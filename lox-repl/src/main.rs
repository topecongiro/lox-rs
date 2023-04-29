use std::{
    fs,
    path::{Path, PathBuf},
};

use clap::Parser;
use lox::lexer::error_report::{LexerErrorReporter, SourceOrigin};
use reedline::{DefaultPrompt, DefaultPromptSegment, Reedline, Signal};

#[derive(Debug, Parser)]
struct Args {
    file: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    if let Some(file) = args.file {
        run_file(&file)?;
    } else {
        repl();
    }

    Ok(())
}

fn repl() {
    let mut line_editor = Reedline::create();
    let prompt = DefaultPrompt::new(DefaultPromptSegment::Empty, DefaultPromptSegment::Empty);

    loop {
        match line_editor.read_line(&prompt) {
            Ok(Signal::Success(buf)) => {
                if let Err(err) = run_script(&buf) {
                    eprintln!("{err}");
                }
            }
            Ok(Signal::CtrlC | Signal::CtrlD) => {
                break;
            }
            Err(err) => {
                eprintln!("error: {}", err);
                break;
            }
        }
    }
}

fn run_file(path: &Path) -> anyhow::Result<()> {
    let script = fs::read_to_string(path)?;
    run_script(&script)?;

    Ok(())
}

fn run_script(script: &str) -> anyhow::Result<()> {
    let mut lexer = lox::lexer::Lexer::new(script);

    while let Some(token) = lexer.next() {
        println!("{}", token);
    }

    if !lexer.errors().is_empty() {
        let error_reporter = LexerErrorReporter::new(SourceOrigin::Stdin, script);

        for err in lexer.errors() {
            error_reporter.report_lexer_error(err);
        }
    }

    Ok(())
}
