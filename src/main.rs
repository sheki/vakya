use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

use clap::Parser;
use vakya_interpreter::{Interpreter, Scanner};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    path: Option<std::path::PathBuf>,
}

fn run_file(path: std::path::PathBuf) -> Result<(), std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut source = String::new();
    for line in reader.lines() {
        match line {
            Ok(line) => {
                source.push_str(&line);
                source.push('\n');
            }
            Err(e) => return Err(e),
        }
    }

    let mut scanner = Scanner::new(&source);
    scanner.scan_tokens();
    let parser = vakya_interpreter::Parser::new(&scanner.tokens);
    let res = parser.parse();
    let mut interpreter = Interpreter::new();

    match res {
        Ok(statements) => interpreter.interpret(statements),
        Err(error) => println!("error: {:?}", error),
    }

    Ok(())
}

fn run_prompt() -> Result<(), std::io::Error> {
    let mut interpreter = Interpreter::new();
    loop {
        println!("> ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        let mut scanner = Scanner::new(&input);
        scanner.scan_tokens();
        let parser = vakya_interpreter::Parser::new(&scanner.tokens);
        let res = parser.parse();

        match res {
            Ok(statements) => interpreter.interpret(statements),
            Err(error) => println!("error: {:?}", error),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();
    match args.path {
        Some(path) => run_file(path)?,
        None => run_prompt()?,
    }
    Ok(())
}
