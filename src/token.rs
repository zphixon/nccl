#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenKind {
    Value,
    Tabs(u8),
    Spaces(u8),
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
