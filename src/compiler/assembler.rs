use crate::ast::asm::*;

use std::collections::HashMap;
use std::mem::take;

pub struct Assembler {
    ctx: HashMap<String, u16>,
    index: u16,
    hack_stack: Vec<String>,
}

impl Assembler {
    pub fn assemble(assembly_stack: Vec<Assembly>) -> Vec<String> {
        let mut assembler = Self {
            ctx: HashMap::new(),
            index: 16,
            hack_stack: Vec::new(),
        };
        assembler.set_labels(&assembly_stack);
        assembler.translate_assembly_stack(assembly_stack);
        take(&mut assembler.hack_stack)
    }

    // Functions to modify `Assembler` struct
    fn set_labels(&mut self, assembly_stack: &Vec<Assembly>) {
        let mut i = 0;
        for assembly in assembly_stack {
            match assembly {
                Assembly::Label(l) => self.insert(l.to_string(), i),
                _ => i += 1,
            }
        }
    }

    fn lookup(&mut self, name: String) -> u16 {
        match self.ctx.get(&name) {
            Some(i) => *i,
            None => {
                let i = self.index;
                self.index += 1;
                self.insert(name, i);
                i
            }
        }
    }

    fn insert(&mut self, name: String, i: u16) {
        self.ctx.insert(name, i);
    }

    fn push_hack(&mut self, hack: String) {
        self.hack_stack.push(hack);
    }

    fn label(&mut self) -> u16 {
        let i = self.index;
        self.index += 1;
        i
    }

    // Functions for assembly translation
    fn translate_assembly_stack(&mut self, assembly_stack: Vec<Assembly>) {
        assembly_stack
            .into_iter()
            .for_each(|cmd| self.translate_assembly(cmd));
    }

    fn translate_assembly(&mut self, assembly: Assembly) {
        match assembly {
            Assembly::A(a) => self.compile_a_instruction(a),
            Assembly::C(c) => self.compile_c_instruction(c),
            _ => (),
        }
    }

    fn compile_a_instruction(&mut self, a_instr: AInstruction) {
        match a_instr {
            AInstruction::Constant(i) => {
                if i < 16384 {
                    let binary = format!("{:015b}", i);
                    self.push_hack(format!("0{}", binary))
                } else {
                    let binary = format!("{:016b}", i);
                    self.push_hack(format!("{}", binary))
                }
            }
            AInstruction::Symbol(s) => {
                let new = self.compile_a_symbol(s);
                self.compile_a_instruction(AInstruction::Constant(new))
            }
        }
    }

    fn compile_a_symbol(&mut self, var: String) -> u16 {
        match var.as_str() {
            "SCREEN" => 16384,
            "KBD" => 24576,
            "SP" => 0,
            "LCL" => 1,
            "ARG" => 2,
            "THIS" => 3,
            "THAT" => 4,
            "R0" => 0,
            "R1" => 1,
            "R2" => 2,
            "R3" => 3,
            "R4" => 4,
            "R5" => 5,
            "R6" => 6,
            "R7" => 7,
            "R8" => 8,
            "R9" => 9,
            "R10" => 10,
            "R11" => 11,
            "R12" => 12,
            "R13" => 13,
            "R14" => 14,
            "R15" => 15,
            ident => self.lookup(ident.to_string()),
        }
    }

    fn compile_c_instruction(&mut self, c_instr: CInstruction) {
        let comp = compile_comp(c_instr.comp);
        let dest = compile_dest(c_instr.o_dest);
        let jump = compile_jump(c_instr.o_jump);
        self.push_hack(format!("111{}{}{}", comp, dest, jump));
    }
}

fn compile_comp(comp: Comp) -> String {
    match comp {
        // a == 0
        Comp::Zero => "0101010".to_string(),
        Comp::One => "0111111".to_string(),
        Comp::NegOne => "0111010".to_string(),
        Comp::D => "0001100".to_string(),
        Comp::A => "0110000".to_string(),
        Comp::NotD => "0001101".to_string(),
        Comp::NotA => "0110001".to_string(),
        Comp::NegD => "0001111".to_string(),
        Comp::NegA => "0110011".to_string(),
        Comp::DPlusOne => "0011111".to_string(),
        Comp::APlusOne => "0110111".to_string(),
        Comp::DMinusOne => "0001110".to_string(),
        Comp::AMinusOne => "0110010".to_string(),
        Comp::DPlusA => "0000010".to_string(),
        Comp::DMinusA => "0010011".to_string(),
        Comp::AMinusD => "0000111".to_string(),
        Comp::DAndA => "0000000".to_string(),
        Comp::DOrA => "0010101".to_string(),

        // a == 1
        Comp::M => "1110000".to_string(),
        Comp::NotM => "1110001".to_string(),
        Comp::NegM => "1110011".to_string(),
        Comp::MPlusOne => "1110111".to_string(),
        Comp::MMinusOne => "1110010".to_string(),
        Comp::DPlusM => "1000010".to_string(),
        Comp::DMinusM => "1010011".to_string(),
        Comp::MMinusD => "1000111".to_string(),
        Comp::DAndM => "1000000".to_string(),
        Comp::DOrM => "1010101".to_string(),
    }
}

fn compile_dest(o_dest: Option<Dest>) -> String {
    match o_dest {
        None => "000".to_string(),
        Some(dest) => match dest {
            Dest::M => "001".to_string(),
            Dest::D => "010".to_string(),
            Dest::DM => "011".to_string(),
            Dest::MD => "011".to_string(),
            Dest::A => "100".to_string(),
            Dest::AM => "101".to_string(),
            Dest::AD => "110".to_string(),
            Dest::ADM => "111".to_string(),
        },
    }
}

fn compile_jump(o_jump: Option<Jump>) -> String {
    match o_jump {
        None => "000".to_string(),
        Some(jump) => match jump {
            Jump::JGT => "001".to_string(),
            Jump::JEQ => "010".to_string(),
            Jump::JGE => "011".to_string(),
            Jump::JLT => "100".to_string(),
            Jump::JNE => "101".to_string(),
            Jump::JLE => "110".to_string(),
            Jump::JMP => "111".to_string(),
        },
    }
}
