mod ast {
    pub mod asm;
    pub mod jack;
    pub mod vm;
}
mod lexer;
mod parser;
mod tests;

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
        // turn to function
        let tokens = tokenize_jack_file(path)?;
        let token_string = tokens
            .iter()
            .map(|token| format!("{}", crate::lexer::print_token(token.clone())))
            .collect::<Vec<String>>()
            .join("\n");
        let output_path = format!("{}T.xml", path.trim_end_matches(".jack"));

        // Write the token string to the file
        fs::write(
            output_path,
            format!("<tokens>\n{}\n</tokens>\n", token_string),
        )?;
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

// Function to parse a single Jack file
pub fn tokenize_jack_file(file_path: &str) -> Result<Vec<crate::lexer::Token>, Error> {
    let contents = fs::read_to_string(file_path)?;
    crate::lexer::tokenize()
        .parse(contents)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("{:#?}", e)))

    // match tokens {
    //     Ok(token_vec) => match crate::parser::parse_class().parse(token_vec)
    //         .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("{:#?}", e))) {
    //         Ok(parsed) => Ok(crate::parser::print_class(parsed)),
    //         Err(e) => Err(e),
    //     },
    //     Err(e) => Err(e),
    // }
}
