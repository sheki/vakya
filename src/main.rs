use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

use clap::Parser;
use vakya_interpreter::{evaluate_stmt, Scanner, Stmt};

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

    match res {
        Ok(statements) => interpret(statements),
        Err(error) => println!("error: {:?}", error),
    }

    Ok(())
}

fn interpret(statements: Vec<Stmt>) {
    for stmt in statements {
        let eval_result = evaluate_stmt(stmt);
        if let Err(error) = eval_result {
            println!("error: {:?}", error);
        }
    }
}

fn run_prompt() -> Result<(), std::io::Error> {
    loop {
        println!("> ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        let mut scanner = Scanner::new(&input);
        scanner.scan_tokens();
        let parser = vakya_interpreter::Parser::new(&scanner.tokens);
        let res = parser.parse();

        match res {
            Ok(statements) => interpret(statements),
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
