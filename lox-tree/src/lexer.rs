use crate::lexer::token::StringLiteral;

use self::{
    cursor::Cursor,
    error::{LexerError, LexerErrorKind},
    token::{Token, TokenKind},
};

mod cursor;
pub mod error;
pub mod error_report;
pub mod token;

#[derive(Debug)]
pub struct Lexer<'a> {
    cursor: Cursor<'a>,
    errors: Vec<LexerError>,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.scan_token()
    }
}

impl<'a> Lexer<'a> {
    /// Creates a new `Lexer`.
    pub fn new(source: &'a str) -> Self {
        Lexer {
            cursor: Cursor::new(source),
            errors: vec![],
        }
    }

    pub fn collect_tokens(mut self) -> (Vec<Token>, Vec<LexerError>) {
        let mut tokens = vec![];
        while let Some(token) = self.scan_token() {
            tokens.push(token);
        }
        (tokens, self.errors)
    }

    /// Returns a list of lexical errors found during lexical analysis.
    pub fn errors(&self) -> &[LexerError] {
        &self.errors
    }

    fn scan_token(&mut self) -> Option<Token> {
        use TokenKind::*;

        let s = self.cursor.as_str();
        let line_num = self.cursor.line_num();

        let (kind, len) = match self.cursor.bump()? {
            '(' => (LeftParen, 1),
            ')' => (RightParen, 1),
            '{' => (LeftBrace, 1),
            '}' => (RightBrace, 1),
            ',' => (Comma, 1),
            '.' => (Dot, 1),
            '-' => (Minus, 1),
            '+' => (Plus, 1),
            ';' => (Semicolon, 1),
            '*' => (Star, 1),

            '!' if self.cursor.bump_if('=').is_some() => (BangEqual, 2),
            '!' => (Bang, 1),
            '=' if self.cursor.bump_if('=').is_some() => (EqualEqual, 2),
            '=' => (Equal, 1),
            '<' if self.cursor.bump_if('=').is_some() => (LessEqual, 2),
            '<' => (Less, 1),
            '>' if self.cursor.bump_if('=').is_some() => (GreaterEqual, 2),
            '>' => (Greater, 1),

            '/' if self.cursor.bump_if('/').is_some() => {
                let mut len = 2;
                while self.cursor.bump_if(|c| c != '\n').is_some() {
                    len += 1;
                }
                (Comment, len)
            }
            '/' => (Slash, 1),

            c if c.is_whitespace() => {
                let mut len = 1;
                while self.cursor.bump_if(char::is_whitespace).is_some() {
                    len += 1;
                }
                (Whitespace, len)
            }

            '"' => {
                let (s, len) = self.scan_string_literal(s);
                match s {
                    Some(value) => (String(StringLiteral::DoubleQuoted(value)), len),
                    None => (String(StringLiteral::Undelimited), len),
                }
            }

            '0'..='9' => match self.scan_numeral_literal(s) {
                (Some(num), len) => (Number(token::Number::Float(num)), len),
                (None, len) => (Number(token::Number::Invalid), len),
            },

            c if is_ident_alpha(c) => {
                let mut len = 1;
                while self.cursor.bump_if(is_ident_alphanumeric).is_some() {
                    len += 1;
                }
                let ident = &s[..len];
                if let Some(keyword) = token::Keyword::from_str(ident) {
                    (Keyword(keyword), len)
                } else {
                    (Identifier(ident.to_owned()), len)
                }
            }

            _ => todo!(),
        };

        Some(Token::new(kind, len, line_num))
    }

    fn scan_string_literal(&mut self, s: &str) -> (Option<String>, usize) {
        let mut len = 1;
        while self.cursor.bump_if(|c| c != '"').is_some() {
            len += 1;
        }
        if self.cursor.is_at_end() {
            self.errors.push(LexerError::new(
                LexerErrorKind::UndelimitedStringLiteral,
                self.cursor.line_num(),
            ));
            (None, len)
        } else {
            // The closing `"`.
            self.cursor.bump();
            len += 1;
            (Some(s[..len].to_owned()), len)
        }
    }

    fn scan_numeral_literal(&mut self, s: &str) -> (Option<f64>, usize) {
        let mut len = 1;
        while self.cursor.bump_if(char::is_numeric).is_some() {
            len += 1;
        }

        match self.cursor.peek() {
            '.' if self.cursor.second().is_numeric() => {
                len += 1;
                self.cursor.bump();
                while self.cursor.bump_if(char::is_numeric).is_some() {
                    len += 1;
                }
            }
            _ => (),
        }

        match s[..len].parse() {
            Ok(num) => (Some(num), len),
            Err(err) => {
                self.errors.push(LexerError::new(
                    LexerErrorKind::InvalidLiteralNumber { source: err },
                    self.cursor.line_num(),
                ));
                (None, len)
            }
        }
    }
}

fn is_ident_alpha(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

fn is_ident_alphanumeric(c: char) -> bool {
    is_ident_alpha(c) || c.is_numeric()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn lex_normal() {
        use TokenKind::*;

        let test_cases = [
            (
                "1+2*3-4",
                vec![
                    Number(token::Number::Float(1.0f64)),
                    Plus,
                    Number(token::Number::Float(2.0)),
                    Star,
                    Number(token::Number::Float(3.0)),
                    Minus,
                    Number(token::Number::Float(4.0)),
                ],
            ),
            (
                r#"fun foo(x, "hello,world")"#,
                vec![
                    Keyword(token::Keyword::Fun),
                    Whitespace,
                    Identifier("foo".to_owned()),
                    LeftParen,
                    Identifier("x".to_owned()),
                    Comma,
                    Whitespace,
                    String(StringLiteral::DoubleQuoted(r#""hello,world""#.to_owned())),
                    RightParen,
                ],
            ),
        ];
        for (source, expected_tokens) in test_cases {
            let lexer = Lexer::new(source);
            let (tokens, errors) = lexer.collect_tokens();
            assert!(errors.is_empty());
            assert_eq!(tokens.len(), expected_tokens.len());
            for (t, kind) in tokens.into_iter().zip(expected_tokens.into_iter()) {
                assert_eq!(t.kind, kind);
            }
        }
    }
}
