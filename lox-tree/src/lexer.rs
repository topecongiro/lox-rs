use self::{token::{Token, TokenKind}, cursor::Cursor};

mod cursor;
pub mod error_report;
pub mod token;

pub fn tokenize() -> impl Iterator<Item = Token> {
    std::iter::from_fn(|| todo!())
}

impl<'a> Cursor<'a> {
    fn scan_token(&mut self) -> Token {
        let line_num = self.line_num();

        let c = match self.bump() {
            Some(c) => c,
            None => return Token::new(TokenKind::Eof, 0, line_num),
        };

        let (kind, len) = match c {
            _ => todo!(),
        };

        Token::new(kind, len, line_num)
    }
}