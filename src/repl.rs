use rustyline::{Editor, error::ReadlineError};

fn main() -> anyhow::Result<()> {
    let mut rl = Editor::<()>::new()?;
    loop {
        let input = match rl.readline(">> ") {
            Ok(line) => line,
            Err(ReadlineError::Interrupted | ReadlineError::Eof) => break,
            Err(err) => {
                eprintln!("error: {}", err);
                break;
            }
        };
        if input.is_empty() || input.trim() == "exit" {
            break;
        }
    }
    Ok(())
}