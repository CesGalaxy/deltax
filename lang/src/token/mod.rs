use symbol::SymbolToken;

pub mod literal;
pub mod symbol;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Symbol(SymbolToken),
    Literal,
    Keyword,
    Identifier(String),
    Tag(String),
}