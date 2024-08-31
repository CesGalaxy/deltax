#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token {
    Symbol,
    Literal,
    Keyword,
    Identifier(&str),
    Tag(&str),
}