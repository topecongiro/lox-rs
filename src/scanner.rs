use thiserror::Error;

use crate::{token::{Token, TokenKind}, cursor::Cursor};

pub struct Scanner<'a> {
    cursor: Cursor<'a>,
    errors: Vec<ScannerError>,
    current_line: usize,
    tokens: Vec<Token>,
    pos: usize,
}

#[derive(Debug, Error)]
#[error("[line {line}@{place}]: {kind}")]
pub struct ScannerError {
    line: usize,
    place: String,
    kind: ScannerErrorKind,
}

#[derive(Debug, Error)]
pub enum ScannerErrorKind {
    #[error("unexpected character `{0}`")]
    UnexpectedChar(char),
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Scanner {
            cursor: Cursor::new(source),
            errors: vec![],
            current_line: 0,
            tokens: vec![],
            pos: 0,
        }
    }

    pub fn scan_tokens(mut self) -> Result<Vec<Token>, ScannerError> {
        while !self.cursor.is_eof() {
            self.scan_token();
        }
        self.tokens.push(Token::new(TokenKind::EOF, self.current_line));
        Ok(self.tokens)
    }

    fn scan_token(&mut self) -> Option<Token> {
        let lo = self.pos;

        let token_kind = match self.cursor.next() {
            Some('{') => TokenKind::LeftBrace,
            Some('}') => TokenKind::RightBrace,
            Some('(') => TokenKind::LeftParen,
            Some(')') => TokenKind::RightParen,
            Some(',') => TokenKind::Comma,
            Some('.') => TokenKind::Dot,
            Some('-') => TokenKind::Minus,
            Some('+') => TokenKind::Plus,
            Some(';') => TokenKind::Semicolon,
            Some('*') => TokenKind::Star,
            Some('!') => {
                if self.cursor.next_if(|c| c == '=').is_some() {
                    TokenKind::BangEqual 
                } else {
                    TokenKind::Bang
                }
            }
            Some('=') => {
                if self.cursor.next_if(|c| c == '=').is_some() {
                    TokenKind::EqualEqual
                } else {
                    TokenKind::Equal
                }
            }
            Some('<') => {
                if self.cursor.next_if(|c| c == '=').is_some() {
                    TokenKind::LessEqual
                } else {
                    TokenKind::Less
                }
            }
            Some('>') => {
                if self.cursor.next_if(|c| c == '=').is_some() {
                    TokenKind::GreaterEqual
                } else {
                    TokenKind::Greater
                }
            }
            Some('/') => {
                if self.cursor.next_if(|c| c == '/').is_some() {
                    self.cursor.eat_while(|c| c != '\n');
                    return None;
                } else {
                    TokenKind::Slash
                }
            }
            Some(' ') | Some('\r') | Some('\t') => return None,
            Some('\n') => {
                self.current_line += 1;
                return None;
            }
            None => todo!(),
            Some(c) => {
                self.errors.push(ScannerError { line: self.current_line, place: String::new(), kind: ScannerErrorKind::UnexpectedChar(c) });
                return None;
            },
        };
        todo!()
    }
}