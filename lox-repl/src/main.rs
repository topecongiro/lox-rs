use std::{path::{PathBuf, Path}, fs::{File, self}};

use clap::Parser;
use reedline::{Reedline, DefaultPrompt, Signal, DefaultPromptSegment};

#[derive(Debug, Parser)]
struct Args {
    file: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    if let Some(_file) = args.file {
        todo!();
    } else {
        repl();
    }

    Ok(())
}

fn repl() {
    let mut line_editor = Reedline::create();
    let prompt = DefaultPrompt::new(
        DefaultPromptSegment::Empty,
        DefaultPromptSegment::Empty,
    );

    loop {
        match line_editor.read_line(&prompt) {
            Ok(Signal::Success(buf)) => {
                println!("{}", buf);
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
    Ok(())
}