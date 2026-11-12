//! Pure, conservative recovery of literal LF corruption backed by an exact record.
//! No filesystem/process access. Callers own backup, confirmation and atomic writes.
//! A repaired record must match exactly one backup record byte-for-byte (excluding
//! its line ending). Different formatting or other edits in that record are refused.

use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct HistoryRecovery {
    pub bytes: Vec<u8>,
    pub repaired_records: usize,
    pub restored_separators: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum RecoveryError {
    InvalidBackup,
    InvalidCurrent,
    UnprovenRecord,
    AmbiguousRecord,
}

fn body(line: &[u8]) -> &[u8] {
    let line = line.strip_suffix(b"\n").unwrap_or(line);
    line.strip_suffix(b"\r").unwrap_or(line)
}

fn object(bytes: &[u8]) -> bool {
    matches!(
        serde_json::from_slice::<serde_json::Value>(bytes),
        Ok(serde_json::Value::Object(_))
    )
}

/// All-or-nothing: every literal LF inside a string must be corroborated.
/// Valid current records, blank lines, line endings and appended records are
/// copied verbatim. Backup records must themselves be strict JSONL objects.
/// U+2028 spelled as a JSON escape in the backup is deliberately not accepted:
/// this narrow repair proves original bytes, not just equivalent JSON values.
pub(crate) fn recover_history(
    current: &[u8],
    backup: &[u8],
) -> Result<HistoryRecovery, RecoveryError> {
    let mut evidence = HashMap::<&[u8], usize>::new();
    for line in backup.split_inclusive(|b| *b == b'\n') {
        let record = body(line);
        if record.is_empty() {
            continue;
        }
        if !object(record) {
            return Err(RecoveryError::InvalidBackup);
        }
        *evidence.entry(record).or_default() += 1;
    }

    let mut result = HistoryRecovery {
        bytes: Vec::with_capacity(current.len()),
        repaired_records: 0,
        restored_separators: 0,
    };
    let mut record = Vec::new();
    let mut quoted = false;
    let mut escaped = false;
    let mut replacements = 0;
    for &byte in current {
        if quoted && byte == b'\n' {
            // An LF after a backslash is an invalid escape, not this corruption.
            if escaped {
                return Err(RecoveryError::InvalidCurrent);
            }
            record.extend_from_slice("\u{2028}".as_bytes());
            replacements += 1;
            continue;
        }
        record.push(byte);
        if escaped {
            escaped = false;
        } else if quoted && byte == b'\\' {
            escaped = true;
        } else if byte == b'"' {
            quoted = !quoted;
        }
        if byte == b'\n' && !quoted {
            accept(&record, replacements, &mut evidence, &mut result)?;
            record.clear();
            replacements = 0;
        }
    }
    if !record.is_empty() {
        accept(&record, replacements, &mut evidence, &mut result)?;
    }
    Ok(result)
}

fn accept(
    record: &[u8],
    replacements: usize,
    evidence: &mut HashMap<&[u8], usize>,
    result: &mut HistoryRecovery,
) -> Result<(), RecoveryError> {
    let content = body(record);
    if !content.is_empty() && !object(content) {
        return Err(RecoveryError::InvalidCurrent);
    }
    if replacements > 0 {
        match evidence.get_mut(content) {
            Some(count) if *count == 1 => *count = 0,
            Some(_) => return Err(RecoveryError::AmbiguousRecord),
            None => return Err(RecoveryError::UnprovenRecord),
        }
        result.repaired_records += 1;
        result.restored_separators += replacements;
    }
    result.bytes.extend_from_slice(record);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn restores_only_proven_bytes_preserving_settings_and_appended_records() {
        let original = "{\"type\":\"event_msg\",\"payload\":{\"text\":\"a\u{2028}b\u{2028}c\"}}";
        let backup = format!("{{\"settings\":\"old\"}}\n{original}\n");
        let current = format!(
            "\r\n{{\"settings\":\"new\"}}\r\n{}\r\n{{\"appended\":true}}",
            original.replace('\u{2028}', "\n")
        );
        let recovered = recover_history(current.as_bytes(), backup.as_bytes()).unwrap();
        assert_eq!(
            recovered.bytes,
            format!("\r\n{{\"settings\":\"new\"}}\r\n{original}\r\n{{\"appended\":true}}")
                .as_bytes()
        );
        assert_eq!(
            (recovered.repaired_records, recovered.restored_separators),
            (1, 2)
        );
    }

    #[test]
    fn valid_bytes_are_unchanged_including_escapes_and_unicode() {
        let current = "{\"text\":\"quote: \\\" slash: \\\\ escaped LF: \\n 中文\u{2028}\"}\n\n";
        let recovered = recover_history(current.as_bytes(), b"").unwrap();
        assert_eq!(recovered.bytes, current.as_bytes());
        assert_eq!(recovered.restored_separators, 0);
    }

    #[test]
    fn refuses_missing_partial_or_ambiguous_evidence() {
        let current = b"{\"text\":\"a\nb\nc\"}\n";
        for backup in [
            "",
            "{\"text\":\"a\u{2028}b\\nc\"}\n",
            "{\"text\":\"a\u{2028}b\u{2028}c\",\"extra\":1}\n",
        ] {
            assert_eq!(
                recover_history(current, backup.as_bytes()),
                Err(RecoveryError::UnprovenRecord)
            );
        }
        let record = "{\"text\":\"a\u{2028}b\u{2028}c\"}\n";
        assert_eq!(
            recover_history(current, record.repeat(2).as_bytes()),
            Err(RecoveryError::AmbiguousRecord)
        );
    }

    #[test]
    fn refuses_other_corruption_and_invalid_backup() {
        for current in [
            b"{broken}\n".as_slice(),
            b"{\"x\":\"tail",
            b"{\"x\":\"a\\\nb\"}\n",
            b"{\"x\":\"\xff\"}\n",
            b"{\"x\":\"a\tb\"}\n",
        ] {
            assert_eq!(
                recover_history(current, b""),
                Err(RecoveryError::InvalidCurrent)
            );
        }
        assert_eq!(
            recover_history(b"{}\n", b"{bad}\n"),
            Err(RecoveryError::InvalidBackup)
        );
    }

    #[test]
    fn refuses_reusing_one_backup_record_for_two_damaged_records() {
        let backup = "{\"x\":\"a\u{2028}b\"}\n";
        let current = b"{\"x\":\"a\nb\"}\n".repeat(2);
        assert_eq!(
            recover_history(&current, backup.as_bytes()),
            Err(RecoveryError::AmbiguousRecord)
        );
    }

    #[test]
    fn fails_whole_operation_if_later_record_is_unproven() {
        let backup = "{\"x\":\"a\u{2028}b\"}\n";
        assert_eq!(
            recover_history(b"{\"x\":\"a\nb\"}\n{\"x\":\"c\nd\"}\n", backup.as_bytes()),
            Err(RecoveryError::UnprovenRecord)
        );
    }
}
