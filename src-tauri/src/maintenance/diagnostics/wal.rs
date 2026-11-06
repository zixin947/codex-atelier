//! Validate the copied WAL before SQLite can silently ignore damaged frames.
//! Format and checksum definition: https://www.sqlite.org/fileformat2.html#wal_file_format
//! This module never opens a source database or writes a journal.
type Result<T> = std::result::Result<T, String>;
fn be(bytes: &[u8]) -> u32 {
    u32::from_be_bytes(bytes.try_into().expect("validated 4-byte field"))
}
fn checksum(bytes: &[u8], big_endian: bool, mut state: (u32, u32)) -> (u32, u32) {
    debug_assert_eq!(bytes.len() % 8, 0);
    for pair in bytes.chunks_exact(8) {
        let first: [u8; 4] = pair[..4].try_into().unwrap();
        let second: [u8; 4] = pair[4..].try_into().unwrap();
        let x = if big_endian {
            u32::from_be_bytes(first)
        } else {
            u32::from_le_bytes(first)
        };
        let y = if big_endian {
            u32::from_be_bytes(second)
        } else {
            u32::from_le_bytes(second)
        };
        state.0 = state.0.wrapping_add(x).wrapping_add(state.1);
        state.1 = state.1.wrapping_add(y).wrapping_add(state.0);
    }
    state
}
pub(super) fn validate(bytes: &[u8]) -> Result<()> {
    if bytes.is_empty() {
        return Ok(());
    }
    if bytes.len() < 32 {
        return Err("WAL 头不完整，无法确认索引快照".into());
    }
    let magic = be(&bytes[..4]);
    if !matches!(magic, 0x377f0682 | 0x377f0683) || be(&bytes[4..8]) != 3_007_000 {
        return Err("WAL 头版本无效或不受支持，无法确认索引快照".into());
    }
    let page = be(&bytes[8..12]) as usize;
    if !(512..=65536).contains(&page) || !page.is_power_of_two() {
        return Err("WAL 页大小无效".into());
    }
    let big_endian = magic == 0x377f0683;
    let mut state = checksum(&bytes[..24], big_endian, (0, 0));
    if state != (be(&bytes[24..28]), be(&bytes[28..32])) {
        return Err("WAL 头校验失败，未使用可能过期的主库结果".into());
    }
    let frames = &bytes[32..];
    let frame_size = 24 + page;
    if frames.len() % frame_size != 0 {
        return Err("WAL 含不完整帧，可能正在写入；请稍后重新检测".into());
    }
    let mut committed = frames.is_empty();
    for frame in frames.chunks_exact(frame_size) {
        let number = be(&frame[..4]);
        if number == 0 || number == u32::MAX {
            return Err("WAL 页编号无效".into());
        }
        if frame[8..16] != bytes[16..24] {
            // A reset can leave stale frames with an older salt. Without the live wal-index we
            // cannot distinguish that from corruption; report unknown rather than claim empty.
            return Err("WAL 帧代次不一致，可能处于重置或包含旧尾部；无法确认快照完整性".into());
        }
        state = checksum(&frame[..8], big_endian, state);
        state = checksum(&frame[24..], big_endian, state);
        if state != (be(&frame[16..20]), be(&frame[20..24])) {
            return Err("WAL 帧校验失败，未将主库旧结果冒充完整索引".into());
        }
        let size = be(&frame[4..8]);
        if size == u32::MAX {
            return Err("WAL 提交页数无效".into());
        }
        committed = size != 0;
    }
    if !committed {
        return Err("WAL 尾部事务尚未提交，无法确认稳定快照；请稍后重新检测".into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;
    use std::{fs, path::PathBuf};
    struct Fixture {
        root: PathBuf,
        _connection: Connection,
        bytes: Vec<u8>,
    }
    impl Fixture {
        fn new() -> Self {
            let root =
                std::env::temp_dir().join(format!("atelier-wal-test-{}", uuid::Uuid::new_v4()));
            fs::create_dir_all(&root).unwrap();
            let path = root.join("source.sqlite");
            let connection = Connection::open(&path).unwrap();
            connection.execute_batch("PRAGMA journal_mode=WAL; PRAGMA wal_autocheckpoint=0; CREATE TABLE fixture(id INTEGER); PRAGMA wal_checkpoint(TRUNCATE); INSERT INTO fixture VALUES(1);").unwrap();
            let bytes = fs::read(root.join("source.sqlite-wal")).unwrap();
            Self {
                root,
                _connection: connection,
                bytes,
            }
        }
    }
    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.root);
        }
    }
    fn rechecksum(bytes: &mut [u8], big: bool) {
        let magic: u32 = if big { 0x377f0683 } else { 0x377f0682 };
        bytes[..4].copy_from_slice(&magic.to_be_bytes());
        let mut state = checksum(&bytes[..24], big, (0, 0));
        bytes[24..28].copy_from_slice(&state.0.to_be_bytes());
        bytes[28..32].copy_from_slice(&state.1.to_be_bytes());
        let page = be(&bytes[8..12]) as usize;
        for frame in bytes[32..].chunks_exact_mut(page + 24) {
            state = checksum(&frame[..8], big, state);
            state = checksum(&frame[24..], big, state);
            frame[16..20].copy_from_slice(&state.0.to_be_bytes());
            frame[20..24].copy_from_slice(&state.1.to_be_bytes());
        }
    }
    #[test]
    fn validates_real_sqlite_wal_and_both_checksum_byte_orders() {
        let f = Fixture::new();
        validate(&f.bytes).unwrap();
        let mut bytes = f.bytes.clone();
        rechecksum(&mut bytes, true);
        validate(&bytes).unwrap();
        rechecksum(&mut bytes, false);
        validate(&bytes).unwrap();
        assert!(validate(&[]).is_ok());
    }
    #[test]
    fn corruption_cannot_silently_fall_back_to_old_database() {
        let f = Fixture::new();
        let mut bytes = f.bytes.clone();
        bytes[56 + 100] ^= 1;
        assert!(validate(&bytes).unwrap_err().contains("帧校验"));
        let snapshot = f.root.join("snapshot.sqlite");
        fs::copy(f.root.join("source.sqlite"), &snapshot).unwrap();
        fs::write(f.root.join("snapshot.sqlite-wal"), &bytes).unwrap();
        let read = Connection::open(snapshot).unwrap();
        assert_eq!(
            read.query_row("PRAGMA quick_check(1)", [], |r| r.get::<_, String>(0))
                .unwrap(),
            "ok"
        );
        assert_eq!(
            read.query_row("SELECT count(*) FROM fixture", [], |r| r.get::<_, i64>(0))
                .unwrap(),
            0
        );
    }
    #[test]
    fn rejects_header_salt_and_partial_frame_changes() {
        let f = Fixture::new();
        let mut header = f.bytes.clone();
        header[12] ^= 1;
        assert!(validate(&header).unwrap_err().contains("头校验"));
        let mut salt = f.bytes.clone();
        salt[40] ^= 1;
        assert!(validate(&salt).unwrap_err().contains("代次"));
        assert!(validate(&f.bytes[..f.bytes.len() - 1])
            .unwrap_err()
            .contains("不完整帧"));
    }
    #[test]
    fn valid_checksum_with_uncommitted_tail_reports_unknown() {
        let f = Fixture::new();
        let mut bytes = f.bytes.clone();
        let page = be(&bytes[8..12]) as usize;
        let last = bytes.len() - (page + 24);
        bytes[last + 4..last + 8].copy_from_slice(&0u32.to_be_bytes());
        let big = be(&bytes[..4]) == 0x377f0683;
        rechecksum(&mut bytes, big);
        assert!(validate(&bytes).unwrap_err().contains("尚未提交"));
    }
}
