use std::num::ParseFloatError;

use thiserror::Error;

use super::token::TokenKind;

#[derive(Debug, Error)]
#[error("{kind}")]
pub struct LexerError {
    pub kind: LexerErrorKind,
    pub line_num: usize,
}

impl LexerError {
    pub fn new(kind: LexerErrorKind, line_num: usize) -> LexerError {
        LexerError { kind, line_num }
    }
}

#[derive(Debug, Error)]
pub enum LexerErrorKind {
    #[error("unexpected token: `{0}`")]
    UnexpectedTokend(TokenKind),
    #[error("undelimited string literal")]
    UndelimitedStringLiteral,
    #[error(transparent)]
    InvalidLiteralNumber {
        #[from]
        source: ParseFloatError,
    },
}
