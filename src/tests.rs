use crate::ast::jack::*;
use crate::lexer::*;
use crate::parser::*;

use std::fs;
use std::path::Path;
use std::process::Command;

#[cfg(test)]
mod tests {
    use crate::pretty_printer::PrettyPrint;

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
        let act1 = std::path::Path::new("tests/unit_tests/goodCompareT.txt");
        let exp1 = std::path::Path::new("tests/unit_tests/goodCompare.txt");
        let act2 = std::path::Path::new("tests/unit_tests/badCompareT.txt");
        let exp2 = std::path::Path::new("tests/unit_tests/badCompare.txt");

        let good = compare_files(act1, exp1);
        assert!(good, "Files are not equivalent");

        let bad = compare_files(act2, exp2);
        assert!(!bad, "Files are equivalent");
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
                    format!("<tokens>\n{}\n</tokens>\n", token_string),
                );
                let actual = std::path::Path::new(&act_path);
                let expected = std::path::Path::new(&exp_path);
                assert!(
                    compare_files(actual, expected),
                    "{} and {} do not match",
                    act_path,
                    exp_path
                )
            }
            Err(e) => {
                eprintln!("Error tokenizing {}: {:?}", jack_path, e);
                panic!("Failed to tokenize Jack file: {}", jack_path);
            }
        }
    }

    // Might need to manually convert to LF every time

    // Lexer tests:
    #[test]
    fn test_lexer_arraytest_main() {
        test_lexer("tests/lexer/ArrayTest/Main");
    }

    #[test]
    fn test_lexer_square_main() {
        test_lexer("tests/lexer/Square/Main");
    }

    #[test]
    fn test_lexer_square_square() {
        test_lexer("tests/lexer/Square/Square");
    }

    #[test]
    fn test_lexer_square_squaregame() {
        test_lexer("tests/lexer/Square/SquareGame");
    }

    // Parser tests:
    fn test_parser(file: &str) {
        let jack_path: String = format!("{}.jack", file);
        let exp_path: String = format!("{}.jack", file);
        let act_path: String = format!("{}T.jack", file);

        let r_class = crate::parse_jack_file(&jack_path);
        match r_class {
            Ok(class) => {
                let class_string = class.pretty_print(0);
                std::fs::write(act_path.clone(), class_string);
                let actual = std::path::Path::new(&act_path);
                let expected = std::path::Path::new(&exp_path);
                assert!(
                    compare_files(actual, expected),
                    "{} and {} do not match",
                    act_path,
                    exp_path
                )
            }
            Err(e) => {
                eprintln!("Error parsing {}: {:?}", jack_path, e);
                panic!("Failed to parse Jack file: {}", jack_path);
            }
        }
    }

    #[test]
    fn test_parser_square_main() {
        test_parser("tests/parser/Square/Test");
    }
}
