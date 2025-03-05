#[cfg(test)]
mod tests {
    use crate::pretty_printer::jack::PrettyPrint;

    fn compare_files(actual: &std::path::Path, expected: &std::path::Path) -> bool {
        let actual_content = std::fs::read_to_string(actual).unwrap();
        let expected_content = std::fs::read_to_string(expected).unwrap();
        expected_content == actual_content
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

    // Testing lexing:
    // ----------------------------------------------------------------------------
    fn test_lexer(file: &str) {
        let jack_path: String = format!("{}.jack", file); // Create a new String
        let exp_path: String = format!("{}Exp.xml", file); // Create a new String
        let act_path: String = format!("{}Act.xml", file); // Create a new String

        let r_tokens = crate::tokenize_jack_file(&jack_path);
        match r_tokens {
            Ok(tokens) => {
                let token_string = crate::pretty_printer::lexer::print_tokens(tokens);
                std::fs::write(act_path.clone(), token_string);
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

    // Testing parser:
    // ----------------------------------------------------------------------------
    fn test_parser(file: &str) {
        let jack_path: String = format!("{}.jack", file);
        let exp_path: String = format!("{}Exp.jack", file);
        let act_path: String = format!("{}Act.jack", file);

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
    fn test_parser_arraytest_main() {
        test_parser("tests/parser/ArrayTest/Main")
    }

    #[test]
    fn test_parser_square_main() {
        test_parser("tests/parser/Square/Main")
    }

    #[test]
    fn test_parser_square_square() {
        test_parser("tests/parser/Square/Square")
    }

    #[test]
    fn test_parser_square_squaregame() {
        test_parser("tests/parser/Square/SquareGame")
    }

    // Testing jack_to_vm:
    // ----------------------------------------------------------------------------
    fn test_jack_to_vm(file: &str) {
        let jack_path: String = format!("{}.jack", file); // Create a new String
        let exp_path: String = format!("{}Exp.vm", file); // Create a new String
        let act_path: String = format!("{}Act.vm", file); // Create a new String

        let r_commands = crate::jack_to_vm(&jack_path);
        match r_commands {
            Ok(commands) => {
                let vm_string = crate::pretty_printer::vm::print_vm(commands);
                std::fs::write(act_path.clone(), vm_string);
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
                eprintln!("Error compiling to VM {}: {:?}", jack_path, e);
                panic!("Failed to compile Jack file to VM: {}", jack_path);
            }
        }
    }

    #[test]
    fn test_jack_to_vm_three_main() {
        test_jack_to_vm("tests/jack_to_vm/Three/Main")
    }

    #[test]
    fn test_jack_to_vm_factorial_main() {
        test_jack_to_vm("tests/jack_to_vm/Factorial/Main")
    }

    #[test]
    fn test_jack_to_vm_alphawhere_main() {
        test_jack_to_vm("tests/jack_to_vm/AlphaWhere/Main")
    }

    #[test]
    fn test_jack_to_vm_alphashow_main() {
        test_jack_to_vm("tests/jack_to_vm/AlphaShow/Main")
    }

    #[test]
    fn test_jack_to_vm_square_main() {
        test_jack_to_vm("tests/jack_to_vm/Square/Main")
    }

    #[test]
    fn test_jack_to_vm_square_square() {
        test_jack_to_vm("tests/jack_to_vm/Square/Square")
    }

    #[test]
    fn test_jack_to_vm_square_squaregame() {
        test_jack_to_vm("tests/jack_to_vm/Square/SquareGame")
    }

    #[test]
    fn test_jack_to_vm_converttobin_main() {
        test_jack_to_vm("tests/jack_to_vm/ConvertToBin/Main")
    }

    #[test]
    fn test_jack_to_vm_arraytest_main() {
        test_jack_to_vm("tests/jack_to_vm/ArrayTest/Main")
    }

    // Testing asm_parser:
    // ----------------------------------------------------------------------------
    fn test_asm_parser(file: &str) {
        let asm_path: String = format!("{}.asm", file); // Create a new String
        let act_path: String = format!("{}Act.asm", file); // Create a new String
        let exp_path: String = format!("{}Exp.asm", file); // Create a new String

        let r_assembly = crate::parse_asm_file(&asm_path);
        match r_assembly {
            Ok(assembly) => {
                let asm_string = crate::pretty_printer::asm::print_asm(assembly);
                std::fs::write(act_path.clone(), asm_string);
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
                eprintln!("Error tokenizing {}: {:?}", asm_path, e);
                panic!("Failed to tokenize Jack file: {}", asm_path);
            }
        }
    }

    #[test]
    fn test_asm_parser_assembler_add_add() {
        test_asm_parser("tests/assembler/add/Add")
    }

    #[test]
    fn test_asm_parser_assembler_max_max() {
        test_asm_parser("tests/assembler/max/Max")
    }

    #[test]
    fn test_asm_parser_assembler_max_maxl() {
        test_asm_parser("tests/assembler/max/MaxL")
    }

    #[test]
    fn test_asm_parser_assembler_pong_pong() {
        test_asm_parser("tests/assembler/pong/Pong")
    }

    #[test]
    fn test_asm_parser_assembler_pong_pongl() {
        test_asm_parser("tests/assembler/pong/PongL")
    }

    #[test]
    fn test_asm_parser_assembler_rect_rect() {
        test_asm_parser("tests/assembler/rect/Rect")
    }

    #[test]
    fn test_asm_parser_assembler_rect_rectl() {
        test_asm_parser("tests/assembler/rect/RectL")
    }

    // Testing assembler:
    // ----------------------------------------------------------------------------
    fn test_assembler(file: &str) {
        let asm_path: String = format!("{}.asm", file); // Create a new String
        let exp_path: String = format!("{}Exp.hack", file); // Create a new String
        let act_path: String = format!("{}Act.hack", file); // Create a new String

        let r_hack = crate::assembler(&asm_path);
        match r_hack {
            Ok(hack) => {
                let hack_string = hack.join("\n") + "\n";
                std::fs::write(act_path.clone(), hack_string);
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
                eprintln!("Error compiling to VM {}: {:?}", asm_path, e);
                panic!("Failed to compile Jack file to VM: {}", asm_path);
            }
        }
    }

    #[test]
    fn test_assembler_add_add() {
        test_assembler("tests/assembler/add/Add")
    }

    #[test]
    fn test_assembler_max_max() {
        test_assembler("tests/assembler/max/Max")
    }

    #[test]
    fn test_assembler_max_maxl() {
        test_assembler("tests/assembler/max/MaxL")
    }

    #[test]
    fn test_assembler_pong_pong() {
        test_assembler("tests/assembler/pong/Pong")
    }

    #[test]
    fn test_assembler_pong_pongl() {
        test_assembler("tests/assembler/pong/PongL")
    }

    #[test]
    fn test_assembler_rect_rect() {
        test_assembler("tests/assembler/rect/Rect")
    }

    #[test]
    fn test_assembler_rect_rectl() {
        test_assembler("tests/assembler/rect/RectL")
    }

    // Testing vm_parser:
    // ----------------------------------------------------------------------------
    // We modify the file extensions as other tests 
    // will be grabbing all `.vm` files in directory.
    fn test_vm_parser(file: &str) {
        let vm_path: String = format!("{}.vm", file); // Create a new String
        let act_path: String = format!("{}.avm", file); // Create a new String
        let exp_path: String = format!("{}.evm", file); // Create a new String

        let r_commands = crate::parse_vm_file(&vm_path);
        match r_commands {
            Ok(commands) => {
                let vm_string = crate::pretty_printer::vm::print_vm(commands);
                std::fs::write(act_path.clone(), vm_string);
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
                eprintln!("Error tokenizing {}: {:?}", vm_path, e);
                panic!("Failed to tokenize Jack file: {}", vm_path);
            }
        }
    }

    #[test]
    fn test_vm_parser_vm_to_asm_simpleadd_simpleadd() {
        test_vm_parser("tests/vm_to_asm/SimpleAdd/SimpleAdd")
    }

    #[test]
    fn test_vm_parser_vm_to_asm_basicloop_basicloop() { 
        test_vm_parser("tests/vm_to_asm/BasicLoop/BasicLoop")
    }

    #[test]
    fn test_vm_parser_vm_to_asm_basictest_basictest() {
        test_vm_parser("tests/vm_to_asm/BasicTest/BasicTest")
    }

    #[test]
    fn test_vm_parser_vm_to_asm_fibonaccielement_main() {
        test_vm_parser("tests/vm_to_asm/FibonacciElement/Main")
    }

    #[test]
    fn test_vm_parser_vm_to_asm_fibonaccielement_sys() {
        test_vm_parser("tests/vm_to_asm/FibonacciElement/Sys")
    }
    
    #[test]
    fn test_vm_parser_vm_to_asm_fibonacciseries_fibonacciseries() {
        test_vm_parser("tests/vm_to_asm/FibonacciSeries/FibonacciSeries")
    }

    #[test]
    fn test_vm_parser_vm_to_asm_nestedcall_sys() {
        test_vm_parser("tests/vm_to_asm/NestedCall/Sys")
    }

    #[test]
    fn test_vm_parser_vm_to_asm_pointertest_pointertest() {
        test_vm_parser("tests/vm_to_asm/PointerTest/PointerTest")
    }

    #[test]
    fn test_vm_parser_vm_to_asm_simplefunction_simplefunction() {
        test_vm_parser("tests/vm_to_asm/SimpleFunction/SimpleFunction")
    }

    #[test]
    fn test_vm_parser_vm_to_asm_stacktest_stacktest() {
        test_vm_parser("tests/vm_to_asm/StackTest/StackTest")
    }

    #[test]
    fn test_vm_parser_vm_to_asm_staticstest_class1() {
        test_vm_parser("tests/vm_to_asm/StaticsTest/Class1")
    }

    #[test]
    fn test_vm_parser_vm_to_asm_staticstest_class2() {
        test_vm_parser("tests/vm_to_asm/StaticsTest/Class2")
    }

    #[test]
    fn test_vm_parser_vm_to_asm_staticstest_sys() {
        test_vm_parser("tests/vm_to_asm/StaticsTest/Sys")
    }

    #[test]
    fn test_vm_parser_vm_to_asm_statictest_statictest() {
        test_vm_parser("tests/vm_to_asm/StaticTest/StaticTest")
    }

}
