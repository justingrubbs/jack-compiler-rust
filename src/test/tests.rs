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
    fn lexer(file: &str) {
        let jack_path: String = format!("{}.jack", file);
        let exp_path: String = format!("{}Exp.xml", file);
        let act_path: String = format!("{}Act.xml", file);

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
    fn lexer_arraytest_main() {
        lexer("tests/lexer/ArrayTest/Main");
    }

    #[test]
    fn lexer_square_main() {
        lexer("tests/lexer/Square/Main");
    }

    #[test]
    fn lexer_square_square() {
        lexer("tests/lexer/Square/Square");
    }

    #[test]
    fn lexer_square_squaregame() {
        lexer("tests/lexer/Square/SquareGame");
    }

    // Testing parser:
    // ----------------------------------------------------------------------------
    fn parser(file: &str) {
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
    fn parser_arraytest_main() {
        parser("tests/parser/ArrayTest/Main")
    }

    #[test]
    fn test_parser_square_main() {
        parser("tests/parser/Square/Main")
    }

    #[test]
    fn test_parser_square_square() {
        parser("tests/parser/Square/Square")
    }

    #[test]
    fn parser_square_squaregame() {
        parser("tests/parser/Square/SquareGame")
    }

    // Testing jack_to_vm:
    // ----------------------------------------------------------------------------
    fn jack_to_vm(file: &str) {
        let jack_path: String = format!("{}.jack", file);
        let exp_path: String = format!("{}Exp.vm", file);
        let act_path: String = format!("{}Act.vm", file);

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
    fn jack_to_vm_three_main() {
        jack_to_vm("tests/jack_to_vm/Three/Main")
    }

    #[test]
    fn jack_to_vm_factorial_main() {
        jack_to_vm("tests/jack_to_vm/Factorial/Main")
    }

    #[test]
    fn jack_to_vm_alphawhere_main() {
        jack_to_vm("tests/jack_to_vm/AlphaWhere/Main")
    }

    #[test]
    fn jack_to_vm_alphashow_main() {
        jack_to_vm("tests/jack_to_vm/AlphaShow/Main")
    }

    #[test]
    fn jack_to_vm_square_main() {
        jack_to_vm("tests/jack_to_vm/Square/Main")
    }

    #[test]
    fn jack_to_vm_square_square() {
        jack_to_vm("tests/jack_to_vm/Square/Square")
    }

    #[test]
    fn jack_to_vm_square_squaregame() {
        jack_to_vm("tests/jack_to_vm/Square/SquareGame")
    }

    #[test]
    fn jack_to_vm_converttobin_main() {
        jack_to_vm("tests/jack_to_vm/ConvertToBin/Main")
    }

    #[test]
    fn jack_to_vm_arraytest_main() {
        jack_to_vm("tests/jack_to_vm/ArrayTest/Main")
    }

    // Testing asm_parser:
    // ----------------------------------------------------------------------------
    fn asm_parser(file: &str) {
        let asm_path: String = format!("{}.asm", file);
        let act_path: String = format!("{}Act.asm", file);
        let exp_path: String = format!("{}Exp.asm", file);

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
    fn asm_parser_assembler_add_add() {
        asm_parser("tests/assembler/add/Add")
    }

    #[test]
    fn asm_parser_assembler_max_max() {
        asm_parser("tests/assembler/max/Max")
    }

    #[test]
    fn asm_parser_assembler_max_maxl() {
        asm_parser("tests/assembler/max/MaxL")
    }

    #[test]
    fn asm_parser_assembler_pong_pong() {
        asm_parser("tests/assembler/pong/Pong")
    }

    #[test]
    fn asm_parser_assembler_pong_pongl() {
        asm_parser("tests/assembler/pong/PongL")
    }

    #[test]
    fn asm_parser_assembler_rect_rect() {
        asm_parser("tests/assembler/rect/Rect")
    }

    #[test]
    fn asm_parser_assembler_rect_rectl() {
        asm_parser("tests/assembler/rect/RectL")
    }

    // Testing assembler:
    // ----------------------------------------------------------------------------
    fn assembler(file: &str) {
        let asm_path: String = format!("{}.asm", file);
        let exp_path: String = format!("{}Exp.hack", file);
        let act_path: String = format!("{}Act.hack", file);

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
    fn assembler_add_add() {
        assembler("tests/assembler/add/Add")
    }

    #[test]
    fn assembler_max_max() {
        assembler("tests/assembler/max/Max")
    }

    #[test]
    fn assembler_max_maxl() {
        assembler("tests/assembler/max/MaxL")
    }

    #[test]
    fn assembler_pong_pong() {
        assembler("tests/assembler/pong/Pong")
    }

    #[test]
    fn assembler_pong_pongl() {
        assembler("tests/assembler/pong/PongL")
    }

    #[test]
    fn assembler_rect_rect() {
        assembler("tests/assembler/rect/Rect")
    }

    #[test]
    fn assembler_rect_rectl() {
        assembler("tests/assembler/rect/RectL")
    }

    // Testing vm_parser:
    // ----------------------------------------------------------------------------
    // We modify the file extensions since other tests will
    //  be grabbing all `.vm` files in directory.
    fn vm_parser(file: &str) {
        let vm_path: String = format!("{}.vm", file);
        let act_path: String = format!("{}.avm", file);
        let exp_path: String = format!("{}.evm", file);

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
    fn vm_parser_vm_to_asm_simpleadd_simpleadd() {
        vm_parser("tests/vm_to_asm/SimpleAdd/SimpleAdd")
    }

    #[test]
    fn vm_parser_vm_to_asm_basicloop_basicloop() {
        vm_parser("tests/vm_to_asm/BasicLoop/BasicLoop")
    }

    #[test]
    fn vm_parser_vm_to_asm_basictest_basictest() {
        vm_parser("tests/vm_to_asm/BasicTest/BasicTest")
    }

    #[test]
    fn vm_parser_vm_to_asm_fibonaccielement_main() {
        vm_parser("tests/vm_to_asm/FibonacciElement/Main")
    }

    #[test]
    fn vm_parser_vm_to_asm_fibonaccielement_sys() {
        vm_parser("tests/vm_to_asm/FibonacciElement/Sys")
    }

    #[test]
    fn vm_parser_vm_to_asm_fibonacciseries_fibonacciseries() {
        vm_parser("tests/vm_to_asm/FibonacciSeries/FibonacciSeries")
    }

    #[test]
    fn vm_parser_vm_to_asm_nestedcall_sys() {
        vm_parser("tests/vm_to_asm/NestedCall/Sys")
    }

    #[test]
    fn vm_parser_vm_to_asm_pointertest_pointertest() {
        vm_parser("tests/vm_to_asm/PointerTest/PointerTest")
    }

    #[test]
    fn vm_parser_vm_to_asm_simplefunction_simplefunction() {
        vm_parser("tests/vm_to_asm/SimpleFunction/SimpleFunction")
    }

    #[test]
    fn vm_parser_vm_to_asm_stacktest_stacktest() {
        vm_parser("tests/vm_to_asm/StackTest/StackTest")
    }

    #[test]
    fn vm_parser_vm_to_asm_staticstest_class1() {
        vm_parser("tests/vm_to_asm/StaticsTest/Class1")
    }

    #[test]
    fn vm_parser_vm_to_asm_staticstest_class2() {
        vm_parser("tests/vm_to_asm/StaticsTest/Class2")
    }

    #[test]
    fn vm_parser_vm_to_asm_staticstest_sys() {
        vm_parser("tests/vm_to_asm/StaticsTest/Sys")
    }

    #[test]
    fn vm_parser_vm_to_asm_statictest_statictest() {
        vm_parser("tests/vm_to_asm/StaticTest/StaticTest")
    }

    // Testing virtual machine:
    // ----------------------------------------------------------------------------
    fn vm(file: &str) {
        let vm_path: String = format!("{}.vm", file);
        // let exp_path: String = format!("{}Exp.asm", file);
        let act_path: String = format!("{}.asm", file);

        let r_asm = crate::vm_to_asm(&vm_path);
        match r_asm {
            Ok(asm) => {
                let asm_string = crate::pretty_printer::asm::print_asm(asm);
                std::fs::write(act_path.clone(), asm_string);
                // let actual = std::path::Path::new(&act_path);
                // let expected = std::path::Path::new(&exp_path);
                // assert!(
                //     compare_files(actual, expected),
                //     "{} and {} do not match",
                //     act_path,
                //     exp_path
                // )
            }
            Err(e) => {
                eprintln!("Error transforming VM to ASM {}: {:?}", vm_path, e);
                panic!("Failed to transform VM to ASM: {}", vm_path);
            }
        }
    }

    // Following tests will eventually target a directory rather than file
    // Furthermore, will try to run the CPUEmulator to automate tests
    #[test]
    fn vm_to_asm_simpleadd() {
        // passes without bootstrap code
        vm("tests/vm_to_asm/SimpleAdd/SimpleAdd")
    }

    #[test]
    fn vm_to_asm_stacktest() {
        // passes without bootstrap code
        vm("tests/vm_to_asm/StackTest/StackTest")
    }

    #[test]
    fn vm_to_asm_basictest() {
        // passes without bootstrap code
        vm("tests/vm_to_asm/BasicTest/BasicTest")
    }

    #[test]
    fn vm_to_asm_pointertest() {
        // passes without bootstrap code
        vm("tests/vm_to_asm/PointerTest/PointerTest")
    }

    #[test]
    fn vm_to_asm_statictest() {
        // passes without bootstrap code
        vm("tests/vm_to_asm/StaticTest/StaticTest")
    }

    #[test]
    fn vm_to_asm_nestedcall() {
        // does not crate .asm file
        vm("tests/vm_to_asm/NestedCall/Sys")
    }

    #[test]
    fn vm_to_asm_basicloop() {
        // passes without bootstrap code
        vm("tests/vm_to_asm/BasicLoop/BasicLoop")
    }

    #[test]
    fn vm_to_asm_fibonacciseries() {
        // passes with or without bootstrap code
        vm("tests/vm_to_asm/FibonacciSeries/FibonacciSeries")
    }

    #[test]
    fn vm_to_asm_simplefunction() {
        // does not create .asm file
        vm("tests/vm_to_asm/SimpleFunction/SimpleFunction")
    }

    // #[test]
    fn vm_to_asm_staticstest() {
        // will not work until vm_to_asm is updated to handle multiple files
        vm("tests/vm_to_asm/StaticsTest/")
    }

    // #[test]
    fn vm_to_asm_fibonaccielement() {
        // will not work until vm_to_asm is updated to handle multiple files
        vm("tests/vm_to_asm/FibonacciElement/")
    }
}
