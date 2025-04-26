# Jack Compiler (Rust)

A compiler for the Jack programming language, written in Rust. This project is based on the Jack language specification used in the [Nand2Tetris](https://www.nand2tetris.org/) course and implements the various stages of a compiler pipeline from tokenization to hack code generation.

## Features

- Lexer: Tokenizes Jack source code
- Parser: Constructs abstract syntax trees
- Code Generator: Outputs VM code
- Modular architecture in Rust
- Unit tests for core components
- Integration testing for each stage in the pipeline

## Getting Started

### Prerequisites

- Rust (latest stable version recommended)
d
### Running
```bash
cargo run [path to directory or file]
