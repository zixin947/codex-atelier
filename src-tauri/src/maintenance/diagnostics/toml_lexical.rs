//! TomlLexicalState::line_is_structural 忠实实现
//! 依据：decomp-line_is_structural.c @0x10078da1c（已裁定）
//!
//! 裁定语义：
//! · 返回值 = 「本行开始时」的持久状态（入口读，返回 == InMultiBasic）
//! · 持久状态仅在四类三引号边界写：
//!   状态 Normal + `"""` → InMultiBasic      [0x10078dbb0]
//!   状态 Normal + `'''` → InMultiLiteral    [0x10078dbbc]
//!   状态 InMultiBasic + `"""` → Normal      [0x10078dbc8]
//!   状态 InMultiBasic + `'''` → InMultiLiteral（AiMaMi 近似，decompiler 证实）
//!   状态 InMultiLiteral + `'''` → Normal     [对称分支]
//! · 普通引号串用局部扫描（闭合与否都不落盘）；`\` 跳 2 字节；`#`（Normal）结束本行。

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LexState {
    #[default]
    Normal = 0,
    InMultiLiteral = 1,
    InMultiBasic = 2,
}

#[derive(Debug, Clone, Default)]
pub struct TomlLexicalState {
    pub state: LexState,
}

impl TomlLexicalState {
    /// 处理一行；返回「本行开始时是否处于多行块」。
    pub fn line_is_structural(&mut self, line: &str) -> bool {
        let entry = self.state;
        let b = line.as_bytes();
        let n = b.len();
        let mut i = 0;
        while i < n {
            let c = b[i];
            match self.state {
                LexState::Normal => {
                    if c == b'"' {
                        if i + 3 <= n && b[i + 1] == b'"' && b[i + 2] == b'"' {
                            self.state = LexState::InMultiBasic;
                            i += 3;
                            continue;
                        }
                        // 普通双引号：局部扫描，不落盘
                        i += 1;
                        while i < n {
                            if b[i] == b'\\' {
                                i += 2;
                                continue;
                            }
                            if b[i] == b'"' {
                                i += 1;
                                break;
                            }
                            i += 1;
                        }
                        continue;
                    }
                    if c == b'\'' {
                        if i + 3 <= n && b[i + 1] == b'\'' && b[i + 2] == b'\'' {
                            self.state = LexState::InMultiLiteral;
                            i += 3;
                            continue;
                        }
                        i += 1;
                        while i < n {
                            if b[i] == b'\'' {
                                i += 1;
                                break;
                            }
                            i += 1;
                        }
                        continue;
                    }
                    if c == b'#' {
                        break;
                    }
                    if c == b'\\' {
                        i += 2;
                        continue;
                    }
                    i += 1;
                }
                LexState::InMultiBasic => {
                    if c == b'"' && i + 3 <= n && b[i + 1] == b'"' && b[i + 2] == b'"' {
                        self.state = LexState::Normal;
                        i += 3;
                        continue;
                    }
                    if c == b'\'' && i + 3 <= n && b[i + 1] == b'\'' && b[i + 2] == b'\'' {
                        // AiMaMi 近似：多行 basic 内遇 ''' 切到 literal 块（反编译证实）
                        self.state = LexState::InMultiLiteral;
                        i += 3;
                        continue;
                    }
                    if c == b'\\' {
                        i += 2;
                        continue;
                    }
                    i += 1;
                }
                LexState::InMultiLiteral => {
                    if c == b'\'' && i + 3 <= n && b[i + 1] == b'\'' && b[i + 2] == b'\'' {
                        self.state = LexState::Normal;
                        i += 3;
                        continue;
                    }
                    if c == b'"' && i + 3 <= n && b[i + 1] == b'"' && b[i + 2] == b'"' {
                        // 对称分支
                        self.state = LexState::InMultiBasic;
                        i += 3;
                        continue;
                    }
                    i += 1;
                }
            }
        }
        entry == LexState::InMultiBasic
    }
}
