# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Vakya Interpreter Overview

Vakya is a lightweight expression interpreter written in Rust. The interpreter follows a standard implementation approach:

1. **Scanning/Lexing**: Converts source text into tokens
2. **Parsing**: Converts tokens into an abstract syntax tree (AST)
3. **Evaluation**: Executes the AST to produce values

## Repository Structure

- `src/scanner.rs`: Tokenizes source code into tokens
- `src/token.rs` & `src/token_type.rs`: Token representation
- `src/parser.rs`: Recursive descent parser implementation
- `src/expr.rs`: Expression tree data structures
- `src/stmt.rs`: Statement representation
- `src/evaluate.rs`: Expression and statement evaluation
- `src/parser_error.rs`: Error handling
- `src/main.rs`: CLI and REPL implementation

## Common Commands

```bash
# Build the project
cargo build

# Run the REPL (interactive mode)
cargo run

# Run with a source file
cargo run -- path/to/your/file.vak

# Run tests
cargo test

# Check for linting issues
cargo clippy

# Format code
cargo fmt
```

## Language Features

The interpreter supports:
- Numeric operations (+, -, *, /)
- Boolean operations (true, false)
- Comparison operators (==, !=, <, <=, >, >=)
- Grouping with parentheses
- Unary operations (-, !)
- Print statements
- Expression statements

## Expression Grammar

The language uses the following grammar for expressions:
```
expression → equality
equality → comparison ( ( "!=" | "==" ) comparison )*
comparison → term ( ( ">" | ">=" | "<" | "<=" ) term )*
term → factor ( ( "-" | "+" ) factor )*
factor → unary ( ( "/" | "*" ) unary )*
unary → ( "!" | "-" ) unary | primary
primary → NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")"
```

## Statement Grammar

```
statement → printStmt | exprStmt
printStmt → "print" expression ";"
exprStmt → expression ";"
```