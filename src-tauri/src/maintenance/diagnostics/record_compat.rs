//! Narrow legacy response-item ID compatibility, reconstructed from AiMaMi 1.2.7 arm64.
//! canonical_id_for_item @ 0x1008db528; rollout_history::has_positions @ 0x1008afc44.
//! Only response_item.payload.id changes. call_id, references, body text and unknown IDs stay intact.
use serde_json::Value;
type Result<T> = std::result::Result<T, String>;

fn canonical(item_type: &str, id: &str) -> Option<String> {
    match item_type {
        "custom_tool_call" => {
            let suffix = id.strip_prefix("ct_").or_else(|| id.strip_prefix("fc_"))?;
            if suffix.is_empty()
                || !suffix
                    .bytes()
                    .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'_' | b'-'))
            {
                return None;
            }
            Some(format!("ctc_{suffix}"))
        }
        "tool_search_call" => {
            let suffix = id.strip_prefix("ts_")?;
            if suffix.len() != 12
                || !suffix
                    .bytes()
                    .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
            {
                return None;
            }
            Some(format!("tsc_{suffix}"))
        }
        _ => None,
    }
}
fn parse(line: &[u8]) -> Result<Value> {
    serde_json::from_slice(line).map_err(|_| "会话记录包含无效 JSON，不能执行 ID 修复".into())
}
fn nonempty(line: &[u8]) -> bool {
    line.iter().any(|b| !b.is_ascii_whitespace())
}
fn replacement(value: &Value) -> Option<(&str, String)> {
    if value.get("type")?.as_str()? != "response_item" {
        return None;
    }
    let payload = value.get("payload")?;
    let id = payload.get("id")?.as_str()?;
    Some((id, canonical(payload.get("type")?.as_str()?, id)?))
}
pub(super) fn inspect(bytes: &[u8]) -> Result<usize> {
    let mut count = 0;
    for line in bytes
        .split_inclusive(|b| *b == b'\n')
        .filter(|line| nonempty(line))
    {
        if replacement(&parse(line)?).is_some() {
            count += 1;
        }
    }
    Ok(count)
}
/// Native history positions are encoded in the first session_meta, per the installed release.
pub(super) fn has_positions(bytes: &[u8]) -> Result<bool> {
    for line in bytes
        .split_inclusive(|b| *b == b'\n')
        .filter(|line| nonempty(line))
    {
        let value = parse(line)?;
        if value.get("type").and_then(Value::as_str) == Some("session_meta") {
            if !value.get("payload").is_some_and(Value::is_object) {
                return Err("会话元数据无效，无法确定历史布局".into());
            }
            return Ok(value.get("ordinal").is_some()
                || value
                    .pointer("/payload/history_mode")
                    .and_then(Value::as_str)
                    == Some("paginated")
                || value
                    .pointer("/payload/history_base")
                    .is_some_and(|v| !v.is_null()));
        }
    }
    Err("缺少会话元数据，无法确定历史布局".into())
}
pub(super) fn repair(bytes: &[u8]) -> Result<Vec<u8>> {
    let count = inspect(bytes)?;
    if count == 0 {
        return Ok(bytes.to_vec());
    }
    if has_positions(bytes)? {
        return Err(
            "ROUTER_HISTORY_LAYOUT_PROTECTED：ID 修复会改变原生历史字节偏移，已保留会话文件".into(),
        );
    }
    let mut ids = std::collections::HashSet::new();
    let mut targets = Vec::new();
    for line in bytes
        .split_inclusive(|b| *b == b'\n')
        .filter(|line| nonempty(line))
    {
        let value = parse(line)?;
        if value.get("type").and_then(Value::as_str) == Some("response_item") {
            if let Some(id) = value.pointer("/payload/id").and_then(Value::as_str) {
                ids.insert(id.to_owned());
            }
            if let Some((_, after)) = replacement(&value) {
                targets.push(after);
            }
        }
    }
    let mut new_ids = std::collections::HashSet::new();
    if targets
        .iter()
        .any(|id| ids.contains(id) || !new_ids.insert(id))
    {
        return Err("兼容 ID 与既有响应项冲突，已保留会话文件".into());
    }
    let mut out = Vec::with_capacity(bytes.len() + count);
    for line in bytes.split_inclusive(|b| *b == b'\n') {
        if !nonempty(line) {
            out.extend_from_slice(line);
            continue;
        }
        let value = parse(line)?;
        let Some((before, after)) = replacement(&value) else {
            out.extend_from_slice(line);
            continue;
        };
        let needle = serde_json::to_vec(before).map_err(|_| "ID 编码失败")?;
        let mut matches = line
            .windows(needle.len())
            .enumerate()
            .filter_map(|(i, s)| (s == needle).then_some(i));
        let offset = matches
            .next()
            .ok_or("无法唯一定位响应项 ID；原文件未改动")?;
        if matches.next().is_some() {
            return Err("响应项 ID 在同一行出现多次，不能确定修改位置；原文件未改动".into());
        }
        let after = serde_json::to_vec(&after).map_err(|_| "ID 编码失败")?;
        out.extend_from_slice(&line[..offset]);
        out.extend_from_slice(&after);
        out.extend_from_slice(&line[offset + needle.len()..]);
    }
    if inspect(&out)? != 0 {
        return Err("ID 修复结果校验失败".into());
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    const META: &str = "{\"type\":\"session_meta\",\"payload\":{\"id\":\"fixture\"}}\r\n";
    #[test]
    fn fixes_only_confirmed_legacy_payload_ids_without_reserializing_other_bytes() {
        let source=format!("{META} {{\"type\" : \"response_item\", \"payload\":{{\"type\":\"custom_tool_call\",\"id\":\"ct_abc-X_9\",\"call_id\":\"call_123\",\"input\":\"preserve 中文\"}}}}\r\n{{\"type\":\"response_item\",\"payload\":{{\"type\":\"tool_search_call\",\"id\":\"ts_abcdef012345\"}}}}\n{{\"type\":\"event_msg\",\"payload\":{{\"message\":\"ct_abc-X_9\"}}}}\n");
        assert_eq!(inspect(source.as_bytes()).unwrap(), 2);
        let repaired = String::from_utf8(repair(source.as_bytes()).unwrap()).unwrap();
        assert_eq!(
            repaired,
            source
                .replacen("\"id\":\"ct_abc-X_9\"", "\"id\":\"ctc_abc-X_9\"", 1)
                .replacen(
                    "\"id\":\"ts_abcdef012345\"",
                    "\"id\":\"tsc_abcdef012345\"",
                    1
                )
        );
    }
    #[test]
    fn precise_id_rules_leave_unknown_and_canonical_ids_untouched() {
        assert_eq!(
            canonical("custom_tool_call", "fc_Az19_-"),
            Some("ctc_Az19_-".into())
        );
        for (ty, id) in [
            ("custom_tool_call", "ct_"),
            ("custom_tool_call", "ct_bad!"),
            ("custom_tool_call", "ct_中文"),
            ("custom_tool_call", "ctc_ok"),
            ("function_call", "fc_abc"),
            ("tool_search_call", "ts_ABCDEF012345"),
            ("tool_search_call", "ts_abcdef01234"),
        ] {
            assert!(canonical(ty, id).is_none(), "{ty}:{id}");
        }
    }
    #[test]
    fn blocks_native_history_anchors_even_when_ordinal_is_null() {
        for meta in [
            r#"{"type":"session_meta","ordinal":null,"payload":{}}"#,
            r#"{"type":"session_meta","payload":{"history_mode":"paginated"}}"#,
            r#"{"type":"session_meta","payload":{"history_base":{}}}"#,
        ] {
            let source=format!("{meta}\n{{\"type\":\"response_item\",\"payload\":{{\"type\":\"custom_tool_call\",\"id\":\"ct_abc\"}}}}\n");
            assert!(has_positions(source.as_bytes()).unwrap());
            assert!(repair(source.as_bytes())
                .unwrap_err()
                .contains("LAYOUT_PROTECTED"));
        }
        assert!(
            !has_positions(br#"{"type":"session_meta","payload":{"history_base":null}}"#).unwrap()
        );
    }
    #[test]
    fn refuses_new_id_collisions() {
        let bytes=format!("{META}{{\"type\":\"response_item\",\"payload\":{{\"type\":\"custom_tool_call\",\"id\":\"ct_a\"}}}}\n{{\"type\":\"response_item\",\"payload\":{{\"type\":\"custom_tool_call\",\"id\":\"ctc_a\"}}}}\n");
        assert!(repair(bytes.as_bytes()).unwrap_err().contains("冲突"));
    }
    #[test]
    fn rejects_ambiguous_id_occurrences_unknown_layout_and_broken_json() {
        let item = r#"{"type":"response_item","payload":{"type":"custom_tool_call","id":"ct_abc","input":"ct_abc"}}"#;
        assert!(repair(format!("{META}{item}").as_bytes())
            .unwrap_err()
            .contains("多次"));
        assert!(repair(item.as_bytes()).unwrap_err().contains("布局"));
        assert!(inspect(b"broken").is_err());
        assert_eq!(repair(META.as_bytes()).unwrap(), META.as_bytes());
    }
}
