use std::str::Chars;

pub(super) struct Cursor<'a> {
    chars: Chars<'a>,
    line_num: usize,
}

impl<'a> Cursor<'a> {
    pub fn new(source: &'a str) -> Cursor<'a> {
        Cursor { chars: source.chars(), line_num: 1, }
    }

    pub fn bump(&mut self) -> Option<char> {
        let c = self.chars.next();
        if let Some('\n') = c {
            self.line_num += 1;
        }
        c
    }

    pub fn is_at_end(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    pub fn line_num(&self) -> usize {
        self.line_num
    }
}