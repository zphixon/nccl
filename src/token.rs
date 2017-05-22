
#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Name,
    Colon,
    Indent,
    EOF,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub line: u64,
}

impl Token {
    pub fn new(kind: TokenKind, lexeme: String, line: u64) -> Self {
        Token {
            kind: kind,
            lexeme: lexeme,
            line: line,
        }
    }
}

