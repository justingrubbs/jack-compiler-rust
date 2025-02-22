mod ast {
    pub mod jack;
    pub mod vm;
    pub mod asm;
}
mod lexer;
mod parser;


use std::env;
use std::fs;
use std::io::{self,Error};

use chumsky::Parser;

fn main() -> Result<(), Error> {
    // Get the command-line arguments
    let args: Vec<String> = env::args().collect();

    // // Check if the user provided a file path
    // if args.len() < 2 {
    //     eprintln!("Usage: {} <file_or_directory>", args[0]);
    //     return Ok(());
    // }

    let path = &args[1];

    // Check if the path is a file or directory
    let metadata = fs::metadata(path)?;

    if metadata.is_file() {
        // If it's a file, parse it
        let tokens = parse_jack_file(path)?;
        let token_string = tokens
        .iter()
        .map(|token| format!("{:?}", token)) // Adjust formatting as needed
        .collect::<Vec<String>>()
        .join("\n");
        let output_path = format!("{}.xml", path.trim_end_matches(".jack")); 

        // Write the token string to the file
        fs::write(output_path, token_string)?;
    } else if metadata.is_dir() {
        // If it's a directory, parse all .jack files
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let file_path = entry.path();
            if file_path.extension().and_then(|s| s.to_str()) == Some("jack") {
                parse_jack_file(file_path.to_str().unwrap())?;
            }
        }
    } else {
        eprintln!("Provided path is neither a file nor a directory.");
    }

    Ok(())
}

// Function to parse a single Jack file
fn parse_jack_file(file_path: &str) -> Result<Vec<crate::lexer::Token>, Error> {
    let contents = fs::read_to_string(file_path)?;
    crate::lexer::tokenize().parse(contents)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("{:?}", e)))
}

