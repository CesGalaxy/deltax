#[derive(Debug, Clone, PartialEq)]
pub enum LiteralToken {
    Number(u64),
    String(String),
    Char(char),
}