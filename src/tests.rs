use crate::ast::jack::*;
use crate::lexer::*;
use crate::parser::*;

use std::fs;
use std::process::Command;
use std::path::Path;


#[cfg(test)]
mod tests {
    
    fn compare_files(actual: &std::path::Path, expected: &std::path::Path) -> bool {
        let actual_content = std::fs::read_to_string(actual).unwrap();
        let expected_content = std::fs::read_to_string(expected).unwrap();
        expected_content == actual_content
    }

    fn get_file_names(dir: &str) -> Vec<String> {
        let path = std::path::Path::new(dir);
        let mut file_names = Vec::new();
    
        if let Ok(entries) = std::fs::read_dir(path) {
            for entry in entries.flatten() {
                let file_path = entry.path();
                if file_path.extension().and_then(|e| e.to_str()) == Some("jack") {
                    if let Some(name) = file_path.file_name().and_then(|n| n.to_str()) {
                        file_names.push(name.to_string());
                    }
                }
            }
        }
        file_names
    }

    #[test]
    fn test_compare_files() {
        let act = std::path::Path::new("tests/lexer/Square/MainT.xml");
        let exp = std::path::Path::new("tests/lexer/Square/Main.xml");

        let b = compare_files(act,exp);
        assert!(b, "Files are not equivalent")
    }

    // Input is without file extension
    fn test_lexer(file: &str) {
        let jack_path: String = format!("{}.jack", file); // Create a new String
        let exp_path: String = format!("{}.xml", file); // Create a new String
        let act_path: String = format!("{}T.xml", file); // Create a new String

        let r_tokens = crate::tokenize_jack_file(&jack_path);
        match r_tokens {
            Ok(tokens) => {
                let token_string = tokens
                    .iter()
                    .map(|token| format!("{}", crate::lexer::print_token(token.clone())))
                    .collect::<Vec<String>>()
                    .join("\n");
                std::fs::write(
                    act_path.clone(),
                    format!("<tokens>\n{}\n</tokens>\n", token_string),);
                let actual = std::path::Path::new(&act_path);
                let expected = std::path::Path::new(&exp_path);
                assert!(compare_files(actual,expected), "{} and {} do not match", act_path, exp_path)
            }
            Err(e) => todo!(),
        }
    }

    #[test]
    fn run_lexer_tests() {
        test_lexer("tests/lexer/Square/Main");
    }

    // #[test]
    // fn test_get_file_names() {
    //     let files = get_file_names("tests/lexer/Square/"); // Ensure this directory exists
    //     assert!(!files.is_empty(), "No .jack files found!");
        
    //     for file in &files {
    //         println!("{}", file);
    //     }
    // }

    // #[test]
    // fn test_compilation_stage() {
    //     let test_file = "tests/example.jack";
    //     let expected_output = "tests/expected_output.vm";
    //     let actual_output = "output.vm"; // Adjust this to match your output path

    //     // Run your compiler
    //     std::process::Command::new("cargo") // or the command you use to run your compiler
    //         .args(&["run", test_file, "-o", actual_output])
    //         .status()
    //         .expect("Failed to run compiler");

    //     // Compare output
    //     assert!(compare_files(expected_output, actual_output), "Output does not match expected output.");
    // }

}
