# Vakya Interpreter

A lightweight expression interpreter written in Rust. Vakya can evaluate basic mathematical expressions and comparisons, featuring a tokenizer, recursive descent parser, and expression evaluator.

## Features

- Tokenization and lexical analysis
- Recursive descent parsing
- Expression evaluation with support for:
  - Numeric operations (+, -, *, /)
  - Boolean operations (true, false)
  - Comparison operators (==, !=, <, <=, >, >=)
  - Grouping with parentheses
  - Unary operations (-, !)

## Getting Started

### Prerequisites

- Rust (stable version)
- Cargo

### Setting up Git Hooks

The project includes pre-commit hooks to ensure code quality. To enable them, run:

```bash
# Set up pre-commit hooks
git config core.hooksPath .githooks
```

### Building and Running

```bash
# Clone the repository
git clone git@github.com:sheki/vakya.git
cd vakya

# Build
cargo build

# Run the REPL (interactive mode)
cargo run

# Run with a source file
cargo run -- path/to/your/file
```

## Usage

Vakya currently provides an interactive REPL mode where you can enter expressions and see the results immediately.

### Examples

```
1 + 2             # Results in 3
5 * (3 - 1)       # Results in 10
-5 < 3            # Results in true
!(3 > 2)          # Results in false
```

## Project Structure

- `src/scanner.rs`: Tokenization and lexical analysis
- `src/parser.rs`: Recursive descent parser implementation
- `src/evaluate.rs`: Expression evaluation logic
- `src/expr.rs`: Expression tree data structures
- `src/token.rs` & `src/token_type.rs`: Token representation

## License

MIT