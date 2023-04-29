use std::{fmt::Display, path::PathBuf};

use super::error::LexerError;

pub enum SourceOrigin {
    Stdin,
    File(PathBuf),
}

impl Display for SourceOrigin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SourceOrigin::Stdin => f.write_str("stdin"),
            SourceOrigin::File(path) => path.display().fmt(f),
        }
    }
}

pub struct LexerErrorReporter<'a> {
    origin: SourceOrigin,
    source: &'a str,
}

impl<'a> LexerErrorReporter<'a> {
    pub fn new(origin: SourceOrigin, source: &'a str) -> Self {
        LexerErrorReporter { origin, source }
    }

    // TODO: make output configurable other than stderr.
    pub fn report_lexer_error(&self, err: &LexerError) {
        eprintln!("error: {}\n --> {}:{}", err, self.origin, err.line_num);
        let spacing = err.line_num.to_string().len() + 1;
        eprintln!("{}|", " ".repeat(spacing));
        eprintln!("{} | {}", err.line_num, self.get_line(err.line_num));
        eprintln!("{}|", " ".repeat(spacing));
    }

    fn get_line(&self, line_num: usize) -> &str {
        self.source.lines().nth(line_num - 1).unwrap_or_default()
    }
}
