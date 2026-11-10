//! Writer lexical reader recovered from direct arm64 calls; no whole-file TOML parse.
#[derive(Debug, Clone)]
struct Lexical(u8);
impl Lexical {
    fn line_is_structural(&mut self, line: &str) -> bool {
        let started_normal = self.0 == 2;
        let b = line.as_bytes();
        let mut i = 0;
        let mut state = self.0;
        while i < b.len() {
            if state == 2 {
                match b[i] {
                    b'#' => break,
                    q @ (b'"' | b'\'') => {
                        if b.get(i..i + 3) == Some(&[q, q, q]) {
                            state = if q == b'"' { 0 } else { 1 };
                            self.0 = state;
                            i += 3;
                        } else {
                            i += 1;
                            while i < b.len() {
                                if q == b'"' && b[i] == b'\\' {
                                    i = (i + 2).min(b.len());
                                } else {
                                    let close = b[i] == q;
                                    i += 1;
                                    if close {
                                        break;
                                    }
                                }
                            }
                            state = 2;
                        }
                    }
                    _ => i += 1,
                }
            } else if state == 0 && b[i] == b'\\' {
                i = (i + 2).min(b.len());
            } else {
                let q = if state == 0 { b'"' } else { b'\'' };
                if b.get(i..i + 3) == Some(&[q, q, q]) {
                    state = 2;
                    self.0 = 2;
                    i += 3;
                } else {
                    i += 1;
                }
            }
        }
        started_normal
    }
}

const MARKERS: [&str; 6] = [
    "# >>> aimami-relay managed start (DO NOT EDIT MANUALLY)",
    "# >>> aimami-relay managed start (top, DO NOT EDIT MANUALLY)",
    "# >>> aimami-relay codex-router top start (DO NOT EDIT MANUALLY)",
    "# <<< aimami-relay managed end",
    "# <<< aimami-relay managed end (top)",
    "# <<< aimami-relay codex-router top end",
];
fn key(line: &str) -> Option<&str> {
    if line.is_empty() || line.starts_with('#') {
        return None;
    }
    let key = line.split_once('=')?.0.trim();
    (!key.is_empty()
        && key
            .bytes()
            .all(|c| c.is_ascii_alphanumeric() || c == b'_' || c == b'-'))
    .then_some(key)
}
/// Marker lines are ignored; their contents are still read. Once a section is seen,
/// later assignments remain section-local. The first matching assignment wins, even if invalid.
pub fn read_top_level_string_value(text: &str, wanted: &str) -> Option<String> {
    let mut complete = Vec::new();
    let mut lex = Lexical(2);
    let mut start = 0;
    let mut end = 0;
    for raw in text.split_inclusive('\n') {
        end += raw.len();
        lex.line_is_structural(raw);
        if lex.0 == 2 {
            complete.push(&text[start..end]);
            start = end;
        }
    }
    if end > start {
        complete.push(&text[start..end]);
    }
    let mut outer = Lexical(2);
    let mut section = false;
    for raw in complete {
        if !outer.line_is_structural(raw) {
            continue;
        }
        let line = raw.trim();
        if MARKERS.contains(&line) {
            continue;
        }
        if line.starts_with('[') && line.ends_with(']') {
            section = true;
            continue;
        }
        if section {
            continue;
        }
        if key(line) == Some(wanted) {
            let value: toml::Value = toml::from_str(line).ok()?;
            return value.get(wanted)?.as_str().map(str::to_owned);
        }
    }
    None
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn first_match_and_per_assignment_parse() {
        assert_eq!(
            read_top_level_string_value(
                "broken elsewhere\nmodel_catalog_json='x'\n",
                "model_catalog_json"
            ),
            Some("x".into())
        );
        assert_eq!(
            read_top_level_string_value(
                "model_catalog_json=1\nmodel_catalog_json='x'",
                "model_catalog_json"
            ),
            None
        );
        assert_eq!(
            read_top_level_string_value("\"model_catalog_json\"='x'", "model_catalog_json"),
            None
        );
        assert_eq!(
            read_top_level_string_value("model_catalog_json='x' # comment", "model_catalog_json"),
            Some("x".into())
        );
    }
    #[test]
    fn multiline_comments_sections_and_marker_contents() {
        assert_eq!(
            read_top_level_string_value(
                "model_catalog_json=\"\"\"\n/a\nb\n\"\"\"",
                "model_catalog_json"
            ),
            Some("/a\nb\n".into())
        );
        assert_eq!(
            read_top_level_string_value(
                "x='''\nmodel_catalog_json='fake'\n'''\nmodel_catalog_json='real'",
                "model_catalog_json"
            ),
            Some("real".into())
        );
        assert_eq!(
            read_top_level_string_value("[section]\nmodel_catalog_json='x'", "model_catalog_json"),
            None
        );
        assert_eq!(
            read_top_level_string_value(
                "# >>> aimami-relay managed start (DO NOT EDIT MANUALLY)\nmodel_catalog_json='x'",
                "model_catalog_json"
            ),
            Some("x".into())
        );
        assert_eq!(
            read_top_level_string_value(
                "model_catalog_json=\"C:\\\\a\\\\b\"",
                "model_catalog_json"
            ),
            Some("C:\\a\\b".into())
        );
    }
}
