mod ast {
    pub mod jack;
    pub mod token;
    pub mod vm;
}
mod compiler {
    pub mod jack_to_vm;
    pub mod lexer;
    pub mod parser;
}
mod pretty_printer {
    pub mod jack;
    pub mod lexer;
    pub mod vm;
}
mod tests;

use pretty_printer::jack::PrettyPrint;
use std::env;
use std::fs;
use std::io::{self, Error};

use chumsky::Parser;

fn main() -> Result<(), Error> {
    // Get the command-line arguments
    let args: Vec<String> = env::args().collect();

    let path = &args[1];

    // Check if the path is a file or directory
    let metadata = fs::metadata(path)?;

    if metadata.is_file() {
        let class = parse_jack_file(path)?;
        let file_name = path.trim_end_matches(".jack");
        let vm = crate::compiler::jack_to_vm::JackToVm::compile(file_name.to_string(), class);
        let vm_string = crate::pretty_printer::vm::print_vm(vm);
        let output_path = format!("{}T.vm", file_name);

        fs::write(output_path, vm_string)?;
    }
    // else if metadata.is_dir() {
    //     // If it's a directory, parse all .jack files
    //     for entry in fs::read_dir(path)? {
    //         let entry = entry?;
    //         let file_path = entry.path();
    //         if file_path.extension().and_then(|s| s.to_str()) == Some("jack") {
    //             let tokens = parse_jack_file(file_path.to_str().unwrap())?;
    //             let token_string = tokens
    //                 .iter()
    //                 .map(|token| format!("{:?}", token)) // Adjust formatting as needed
    //                 .collect::<Vec<String>>()
    //                 .join("\n");
    //             let output_path = format!("{}T.xml", path.trim_end_matches(".jack"));
    //             fs::write(output_path, token_string)?;
    //         }
    //     }
    // }
    else {
        eprintln!("Provided path is neither a file nor a directory.");
    }
    Ok(())
}

// Tokenize a single Jack file
pub fn tokenize_jack_file(file_path: &str) -> Result<Vec<crate::ast::token::Token>, Error> {
    let contents = fs::read_to_string(file_path)?;
    crate::compiler::lexer::tokenize()
        .parse(contents)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("{:#?}", e)))
}

// Parse a single Jack file
pub fn parse_jack_file(file_path: &str) -> Result<crate::ast::jack::Class, Error> {
    let tokens = tokenize_jack_file(file_path)?;
    crate::compiler::parser::parse_class()
        .parse(tokens)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("{:#?}", e)))
}
