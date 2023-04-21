#[derive(Debug, Clone)]
pub struct Token {
    kind: TokenKind,
    len: usize,
    line_num: usize,
}

impl Token {
    pub(crate) fn new(kind: TokenKind, len: usize, line_num: usize) -> Token {
        Token { kind, len, line_num }
    }
}

#[derive(Debug, Clone)]
pub enum TokenKind {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    Identifier(String),
    String(String),
    Number(i64),

    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}