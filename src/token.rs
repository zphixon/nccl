#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Value,
    Indent, // TODO kill
    Tab(u8),
    Space(u8),
    Newline,
    EOF,
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct Span {
    pub(crate) line: usize,
    pub(crate) column: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct Token2<'a> {
    pub(crate) kind: TokenKind,
    pub(crate) lexeme: &'a str,
    pub(crate) span: Span,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub line: u64,
}

impl Token {
    pub fn new(kind: TokenKind, lexeme: String, line: u64) -> Self {
        Token { kind, lexeme, line }
    }
}
