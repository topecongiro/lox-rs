use std::str::Chars;

pub(crate) struct Cursor<'a> {
    chars: Chars<'a>,
    rest: usize,
}

impl<'a> Cursor<'a> {
    pub fn new(input: &'a str) -> Cursor<'a> {
        Cursor { chars: input.chars(), rest: input.len() }
    }

    pub fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    pub fn consumed(&self) -> usize {
        self.rest - self.chars.as_str().len()
    }

    pub fn reset(&mut self) {
        self.rest = self.chars.as_str().len()
    }

    pub fn peek(&self) -> Option<char> {
        self.chars.clone().next()
    }

    pub fn next_if(&mut self, pred: impl Fn(char) -> bool) -> Option<char> {
        if self.peek().map_or(false, pred) {
            self.next()
        } else {
            None
        }
    }

    pub fn eat_while(&mut self, pred: impl Fn(char) -> bool) {
        while self.peek().map_or(false, &pred) {
            self.next();
        }
    }
}

impl<'a> Iterator for Cursor<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.chars.next()
    }
}