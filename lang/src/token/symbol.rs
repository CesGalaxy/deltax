#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SymbolToken {
    Eq,
    DoubleEq,
    TripleEq,
    Arrow,
    FatArrow,
    LessThan,
    GreaterThan,
    LessOrEq,
    GreaterOrEq,
    Exclamation,
    Ampersand,
    DoubleAmpersand,
    Pipe,
    DoublePipe,
    Dollar,
    At,
    Hashtag,
    Question,
    Plus,
    Asterisk,
    DoubleAsterisk,
    Dash,
    Underscore,
    Dot,
    Colon,
    Comma,
    Semicolon,
    Slash,
    Backslash,
    Percent,
    Tilde,
    Caret,
    Backquote,
    OpenParen,
    CloseParen,
    OpenBracker,
    CloseBracker,
    OpenBrace,
    CloseBrace,
    LineBreak,
    PlusEq,
    DashEq,
    AsteriskEq,
    SlashEq,
    PercentEq,
}

impl From<SymbolToken> for &'static str {
    fn from(token: SymbolToken) -> &'static str {
        match token {
            SymbolToken::Eq => "=",
            SymbolToken::DoubleEq => "==",
            SymbolToken::TripleEq => "===",
            SymbolToken::Arrow => "->",
            SymbolToken::FatArrow => "=>",
            SymbolToken::LessThan => "<",
            SymbolToken::GreaterThan => ">",
            SymbolToken::LessOrEq => "<=",
            SymbolToken::GreaterOrEq => ">=",
            SymbolToken::Exclamation => "!",
            SymbolToken::Ampersand => "&",
            SymbolToken::DoubleAmpersand => "&&",
            SymbolToken::Pipe => "|",
            SymbolToken::DoublePipe => "||",
            SymbolToken::Dollar => "$",
            SymbolToken::At => "@",
            SymbolToken::Hashtag => "#",
            SymbolToken::Question => "?",
            SymbolToken::Plus => "+",
            SymbolToken::Asterisk => "*",
            SymbolToken::DoubleAsterisk => "**",
            SymbolToken::Dash => "-",
            SymbolToken::Underscore => "_",
            SymbolToken::Dot => ".",
            SymbolToken::Colon => ":",
            SymbolToken::Comma => ",",
            SymbolToken::Semicolon => ";",
            SymbolToken::Slash => "/",
            SymbolToken::Backslash => "\\",
            SymbolToken::Percent => "%",
            SymbolToken::Tilde => "~",
            SymbolToken::Caret => "^",
            SymbolToken::Backquote => "`",
            SymbolToken::OpenParen => "(",
            SymbolToken::CloseParen => ")",
            SymbolToken::OpenBracker => "[",
            SymbolToken::CloseBracker => "]",
            SymbolToken::OpenBrace => "{",
            SymbolToken::CloseBrace => "}",
            SymbolToken::LineBreak => "\n",
            SymbolToken::PlusEq => "+=",
            SymbolToken::DashEq => "-=",
            SymbolToken::AsteriskEq => "*=",
            SymbolToken::SlashEq => "/=",
            SymbolToken::PercentEq => "%=",
        }
    }
}
