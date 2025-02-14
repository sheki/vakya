use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

use clap::Parser;
use vakya_interpreter::Scanner;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    path: Option<std::path::PathBuf>,
}

fn run_file(path: std::path::PathBuf) -> Result<(), std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => {
                println!("{}", line);
            }
            Err(e) => return Err(e),
        }
    }
    Ok(())
}

fn run_prompt() -> Result<(), std::io::Error> {
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        let mut scanner = Scanner::new(&input);
        scanner.scan_tokens();
        println!("{:?}", scanner);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();
    match args.path {
        Some(path) => {
            run_file(path)?;
        }
        None => {
            run_prompt()?;
        }
    }
    return Ok(());
}
