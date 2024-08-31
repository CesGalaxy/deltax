#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LiteralToken {
    Number(u64),
    String(&str),
    Char(char),
}