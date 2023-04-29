use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub len: usize,
    pub line_num: usize,
}

impl Token {
    pub(crate) fn new(kind: TokenKind, len: usize, line_num: usize) -> Token {
        Token {
            kind,
            len,
            line_num,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.kind.fmt(f)
    }
}

#[derive(Debug, Clone, PartialEq)]
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
    String(StringLiteral),
    Number(Number),

    Keyword(Keyword),

    Comment,
    Whitespace,
    Eof,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StringLiteral {
    DoubleQuoted(String),
    /// Invalid string literal that is not delimited with the closing `"`.
    Undelimited,
}

impl Display for StringLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StringLiteral::DoubleQuoted(s) => s.fmt(f),
            StringLiteral::Undelimited => "invalid string literal".fmt(f),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Number {
    /// Float literal.
    Float(f64),
    /// Invalid number literal.
    ///
    /// This may because the number is too big/contains invalid character.
    Invalid,
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Number::Float(num) => num.fmt(f),
            Number::Invalid => f.write_str("invalid number"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Keyword {
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
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use TokenKind::*;
        let s = match self {
            LeftParen => "(",
            RightParen => ")",
            LeftBrace => "{",
            RightBrace => "}",
            Comma => ",",
            Dot => ".",
            Minus => "-",
            Plus => "+",
            Semicolon => ";",
            Slash => "/",
            Star => "*",

            Bang => "!",
            BangEqual => "!=",
            Equal => "=",
            EqualEqual => "==",
            Greater => "<",
            GreaterEqual => "<=",
            Less => ">",
            LessEqual => ">=",

            Keyword(keyword) => return keyword.fmt(f),
            Identifier(identifier) => identifier,
            String(s) => return s.fmt(f),
            Number(num) => return num.fmt(f),

            Comment => "comment",
            Whitespace => "whitespace",
            Eof => "eof",
        };
        f.write_str(&s)
    }
}

impl Display for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Keyword::*;
        f.write_str(match self {
            And => "and",
            Class => "class",
            Else => "else",
            False => "false",
            Fun => "fun",
            For => "for",
            If => "if",
            Nil => "nil",
            Or => "or",
            Print => "print",
            Return => "return",
            Super => "super",
            This => "this",
            True => "true",
            Var => "var",
            While => "while",
        })
    }
}

impl Keyword {
    pub fn from_str(s: &str) -> Option<Keyword> {
        KEYWORD_MAP.get(s).cloned()
    }
}

static KEYWORD_MAP: phf::Map<&'static str, Keyword> = phf::phf_map! {
    "and" => Keyword::And,
    "class" => Keyword::Class,
    "else" => Keyword::Else,
    "false" => Keyword::False,
    "fun" => Keyword::Fun,
    "for" => Keyword::For,
    "if" => Keyword::If,
    "nil" => Keyword::Nil,
    "or" => Keyword::Or,
    "print" => Keyword::Print,
    "return" => Keyword::Return,
    "super" => Keyword::Super,
    "this" => Keyword::This,
    "true" => Keyword::True,
    "var" => Keyword::Var,
    "while" => Keyword::While,
};
