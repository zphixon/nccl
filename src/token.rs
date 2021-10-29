#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum QuoteKind {
    Single,
    Double,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenKind {
    Value,
    QuotedValue(QuoteKind),
    Tabs(usize),
    Spaces(usize),
    Eof,
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
