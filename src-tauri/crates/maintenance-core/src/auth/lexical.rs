// Copied from independently recovered config lexical scanner, @10078da1c.
#[derive(Debug, Clone)]
pub(super) struct Lexical(pub(super) u8);
impl Lexical {
    pub(super) fn line_is_structural(&mut self, line: &str) -> bool {
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
