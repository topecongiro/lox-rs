use std::str::Chars;

pub(super) const EOF: char = '\0';

#[derive(Debug)]
pub(super) struct Cursor<'a> {
    chars: Chars<'a>,
    line_num: usize,
}

impl<'a> Cursor<'a> {
    /// Creates a new `Cursor`.
    pub fn new(source: &'a str) -> Cursor<'a> {
        Cursor {
            chars: source.chars(),
            line_num: 1,
        }
    }

    pub fn as_str(&self) -> &'a str {
        self.chars.as_str()
    }

    /// Consumes the next character.
    ///
    /// Internally keeps track of the number of new characters.
    pub fn bump(&mut self) -> Option<char> {
        let c = self.chars.next();
        if let Some('\n') = c {
            self.line_num += 1;
        }
        c
    }

    /// Consumes the next character if it matches with the given predicate.
    pub fn bump_if(&mut self, predicate: impl CharPredicate) -> Option<char> {
        if predicate.is_match(self.peek()) {
            self.bump()
        } else {
            None
        }
    }

    /// Returns the next character without consuming it.
    pub fn peek(&self) -> char {
        self.chars.clone().next().unwrap_or(EOF)
    }

    /// Returns the second character without consuming first and second.
    pub fn second(&self) -> char {
        let mut iter = self.chars.clone();
        iter.next();
        iter.next().unwrap_or(EOF)
    }

    /// Returns true if there is no more characters to consume.
    pub fn is_at_end(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    /// Returns the current line number (starting from 1).
    pub fn line_num(&self) -> usize {
        self.line_num
    }
}

/// A convinient trait for matching character.
pub(super) trait CharPredicate {
    fn is_match(&self, c: char) -> bool;
}

impl CharPredicate for char {
    fn is_match(&self, c: char) -> bool {
        *self == c
    }
}

impl<F: Fn(char) -> bool> CharPredicate for F {
    fn is_match(&self, c: char) -> bool {
        self(c)
    }
}
