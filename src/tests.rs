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

    // Input is without file extension
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




    fn test_assembler(file: &str, asm: Vec<crate::ast::asm::Assembly>) {
        let exp_path: String = format!("{}Exp.hack", file); // Create a new String
        let act_path: String = format!("{}Act.hack", file); // Create a new String

        let hack = crate::compiler::assembler::Assembler::assemble(asm);
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

    #[test]
    fn test_assembler_add_add() {
        let asm = vec![
            crate::ast::asm::Assembly::A(crate::ast::asm::AInstruction::Constant(2)),
            crate::ast::asm::Assembly::C(crate::ast::asm::CInstruction {
                comp: crate::ast::asm::Comp::A,
                o_dest: Some(crate::ast::asm::Dest::D),
                o_jump: None,
            }),
            crate::ast::asm::Assembly::A(crate::ast::asm::AInstruction::Constant(3)),
            crate::ast::asm::Assembly::C(crate::ast::asm::CInstruction {
                comp: crate::ast::asm::Comp::DPlusA,
                o_dest: Some(crate::ast::asm::Dest::D),
                o_jump: None,
            }),
            crate::ast::asm::Assembly::A(crate::ast::asm::AInstruction::Constant(0)),
            crate::ast::asm::Assembly::C(crate::ast::asm::CInstruction {
                comp: crate::ast::asm::Comp::D,
                o_dest: Some(crate::ast::asm::Dest::M),
                o_jump: None,
            }),
        ];
        test_assembler("tests/assembler/Add/Add",asm)
    }

    #[test]
    fn test_assembler_max_max() {
        let asm = vec![
            crate::ast::asm::Assembly::A(crate::ast::asm::AInstruction::Symbol("R0".into())),
            crate::ast::asm::Assembly::C(crate::ast::asm::CInstruction {
                comp: crate::ast::asm::Comp::M,
                o_dest: Some(crate::ast::asm::Dest::D),
                o_jump: None,
            }),
            crate::ast::asm::Assembly::A(crate::ast::asm::AInstruction::Symbol("R1".into())),
            crate::ast::asm::Assembly::C(crate::ast::asm::CInstruction {
                comp: crate::ast::asm::Comp::DMinusM,
                o_dest: Some(crate::ast::asm::Dest::D),
                o_jump: None,
            }),
            crate::ast::asm::Assembly::A(crate::ast::asm::AInstruction::Symbol("ITSR0".into())),
            crate::ast::asm::Assembly::C(crate::ast::asm::CInstruction {
                comp: crate::ast::asm::Comp::D,
                o_dest: None,
                o_jump: Some(crate::ast::asm::Jump::JGT),
            }),
            crate::ast::asm::Assembly::A(crate::ast::asm::AInstruction::Symbol("R1".into())),
            crate::ast::asm::Assembly::C(crate::ast::asm::CInstruction {
                comp: crate::ast::asm::Comp::M,
                o_dest: Some(crate::ast::asm::Dest::D),
                o_jump: None,
            }),
            crate::ast::asm::Assembly::A(crate::ast::asm::AInstruction::Symbol("OUTPUT_D".into())),
            crate::ast::asm::Assembly::C(crate::ast::asm::CInstruction {
                comp: crate::ast::asm::Comp::Zero,
                o_dest: None,
                o_jump: Some(crate::ast::asm::Jump::JMP),
            }),
            crate::ast::asm::Assembly::Label("ITSR0".into()),
            crate::ast::asm::Assembly::A(crate::ast::asm::AInstruction::Symbol("R0".into())),
            crate::ast::asm::Assembly::C(crate::ast::asm::CInstruction {
                comp: crate::ast::asm::Comp::M,
                o_dest: Some(crate::ast::asm::Dest::D),
                o_jump: None,
            }),
            crate::ast::asm::Assembly::Label("OUTPUT_D".into()),
            crate::ast::asm::Assembly::A(crate::ast::asm::AInstruction::Symbol("R2".into())),
            crate::ast::asm::Assembly::C(crate::ast::asm::CInstruction {
                comp: crate::ast::asm::Comp::D,
                o_dest: Some(crate::ast::asm::Dest::M),
                o_jump: None,
            }),
            crate::ast::asm::Assembly::Label("END".into()),
            crate::ast::asm::Assembly::A(crate::ast::asm::AInstruction::Symbol("END".into())),
            crate::ast::asm::Assembly::C(crate::ast::asm::CInstruction {
                comp: crate::ast::asm::Comp::Zero,
                o_dest: None,
                o_jump: Some(crate::ast::asm::Jump::JMP),
            }),
        ];
        test_assembler("tests/assembler/max/Max",asm)
    }

    #[test]
    fn test_assembler_rect_rect() {
        let asm = vec![
            crate::ast::asm::Assembly::A(crate::ast::asm::AInstruction::Symbol("R0".to_string())),
            crate::ast::asm::Assembly::C(crate::ast::asm::CInstruction {
                comp: crate::ast::asm::Comp::M,
                o_dest: Some(crate::ast::asm::Dest::D),
                o_jump: None,
            }),
            crate::ast::asm::Assembly::A(crate::ast::asm::AInstruction::Symbol("END".to_string())),
            crate::ast::asm::Assembly::C(crate::ast::asm::CInstruction {
                comp: crate::ast::asm::Comp::D,
                o_dest: None,
                o_jump: Some(crate::ast::asm::Jump::JLE),
            }),
            crate::ast::asm::Assembly::A(crate::ast::asm::AInstruction::Symbol("n".to_string())),
            crate::ast::asm::Assembly::C(crate::ast::asm::CInstruction {
                comp: crate::ast::asm::Comp::D,
                o_dest: Some(crate::ast::asm::Dest::M),
                o_jump: None,
            }),
            crate::ast::asm::Assembly::A(crate::ast::asm::AInstruction::Symbol("SCREEN".to_string())),
            crate::ast::asm::Assembly::C(crate::ast::asm::CInstruction {
                comp: crate::ast::asm::Comp::A,
                o_dest: Some(crate::ast::asm::Dest::D),
                o_jump: None,
            }),
            crate::ast::asm::Assembly::A(crate::ast::asm::AInstruction::Symbol("addr".to_string())),
            crate::ast::asm::Assembly::C(crate::ast::asm::CInstruction {
                comp: crate::ast::asm::Comp::D,
                o_dest: Some(crate::ast::asm::Dest::M),
                o_jump: None,
            }),
            crate::ast::asm::Assembly::Label("LOOP".to_string()),
            crate::ast::asm::Assembly::A(crate::ast::asm::AInstruction::Symbol("addr".to_string())),
            crate::ast::asm::Assembly::C(crate::ast::asm::CInstruction {
                comp: crate::ast::asm::Comp::M,
                o_dest: Some(crate::ast::asm::Dest::A),
                o_jump: None,
            }),
            crate::ast::asm::Assembly::C(crate::ast::asm::CInstruction {
                comp: crate::ast::asm::Comp::NegOne,
                o_dest: Some(crate::ast::asm::Dest::M),
                o_jump: None,
            }),
            crate::ast::asm::Assembly::A(crate::ast::asm::AInstruction::Symbol("addr".to_string())),
            crate::ast::asm::Assembly::C(crate::ast::asm::CInstruction {
                comp: crate::ast::asm::Comp::M,
                o_dest: Some(crate::ast::asm::Dest::D),
                o_jump: None,
            }),
            crate::ast::asm::Assembly::A(crate::ast::asm::AInstruction::Constant(32)),
            crate::ast::asm::Assembly::C(crate::ast::asm::CInstruction {
                comp: crate::ast::asm::Comp::DPlusA,
                o_dest: Some(crate::ast::asm::Dest::D),
                o_jump: None,
            }),
            crate::ast::asm::Assembly::A(crate::ast::asm::AInstruction::Symbol("addr".to_string())),
            crate::ast::asm::Assembly::C(crate::ast::asm::CInstruction {
                comp: crate::ast::asm::Comp::D,
                o_dest: Some(crate::ast::asm::Dest::M),
                o_jump: None,
            }),
            crate::ast::asm::Assembly::A(crate::ast::asm::AInstruction::Symbol("n".to_string())),
            crate::ast::asm::Assembly::C(crate::ast::asm::CInstruction {
                comp: crate::ast::asm::Comp::MMinusOne,
                o_dest: Some(crate::ast::asm::Dest::DM),
                o_jump: None,
            }),
            crate::ast::asm::Assembly::A(crate::ast::asm::AInstruction::Symbol("LOOP".to_string())),
            crate::ast::asm::Assembly::C(crate::ast::asm::CInstruction {
                comp: crate::ast::asm::Comp::D,
                o_dest: None,
                o_jump: Some(crate::ast::asm::Jump::JGT),
            }),
            crate::ast::asm::Assembly::Label("END".to_string()),
            crate::ast::asm::Assembly::A(crate::ast::asm::AInstruction::Symbol("END".to_string())),
            crate::ast::asm::Assembly::C(crate::ast::asm::CInstruction {
                comp: crate::ast::asm::Comp::Zero,
                o_dest: None,
                o_jump: Some(crate::ast::asm::Jump::JMP),
            }),
        ];
        test_assembler("tests/assembler/rect/Rect", asm)
    }

}
