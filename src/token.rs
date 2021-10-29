#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenKind {
    Value,
    QuotedValue,
    Tabs(usize),
    Spaces(usize),
    EOF,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Span {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct Token<'a> {
    pub(crate) kind: TokenKind,
    pub(crate) lexeme: &'a str,
    pub(crate) span: Span,
}
