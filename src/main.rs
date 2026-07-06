use anyhow::{Context, Result};
use jlox_rs::{Scanner, Parser};
use std::env;
use std::io;
use std::io::Read;
use std::io::prelude::*;
use std::iter::ExactSizeIterator;
use std::path::Path;
use std::process::exit;
fn main() -> Result<()> {
    let mut args = env::args();
    let length = args.len();
    if length > 2 {
        eprintln!("Usage: jlox [script]");
        exit(64);
    } else if length == 2 {
        let _ = args.next();
        let file_path = args.next().expect("already check this value exists");
        run_file(file_path)?;
    } else {
        run_prompt()?;
    }
    Ok(())
}

fn run_file(path: String) -> Result<()> {
    let path = Path::new(&path);
    let mut file = std::fs::File::open(path).context("can not open file")?;
    let mut body = String::new();
    file.read_to_string(&mut body)?;
    run(body)?;
    Ok(())
}

fn run_prompt() -> Result<()> {
    let stdin = io::stdin();
    print!("> ");
    let _ = io::stdout().flush();
    for line in stdin.lock().lines() {
        let line = line?;
        if line == "" {
            break;
        }
        run(line)?;
        print!("> ");
        let _ = io::stdout().flush();
    }
    Ok(())
}

fn run(source: String) -> Result<()> {
    let scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
    println!("==========tokens==========");
    for t in tokens.iter() {
        println!("{:?}", t);
    }
    println!("==========exprs==========");
    let mut parser = Parser::new(tokens);
    let expr = parser.parse();
    println!("{:?}", expr);
    Ok(())
}
