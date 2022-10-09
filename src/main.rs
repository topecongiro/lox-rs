use std::{path::{PathBuf, Path}, io::{self, Read}, fs::{self, File}};

use clap::Parser;
use lox::scanner::Scanner;

#[derive(Parser, Debug)]
struct Args {
    input: PathBuf,    
}

fn main() {
    let args = Args::parse();
    let _scanner = Scanner::new();
    println!("Hello, world!");
}

fn read_file(path: &Path) -> io::Result<Vec<u8>> {
    let f = File::open(path)?;
    let len = f.metadata()?.len();
    let mut buf = Vec::with_capacity(len as usize);
    File::open(path)?.read_to_end(&mut buf)?;
    Ok(buf)
}