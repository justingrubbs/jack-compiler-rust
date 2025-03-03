use crate::ast::asm::*;

use std::collections::HashMap;
use std::mem::take;

pub struct Assembler {
    ctx: HashMap<String, u32>,
    index: u32,
    hack_stack: Vec<String>,
    current_hack: String,
}

impl Assembler {
    pub fn assemble(assembly_stack: Vec<Assembly>) -> Vec<String> {
        let mut assembler = Self {
            ctx: HashMap::new(),
            index: 16,
            hack_stack: Vec::new(),
            current_hack: "".to_string(),
        };
        assembler.translate_assembly_stack(assembly_stack);
        take(&mut assembler.hack_stack)
    }

    // Functions to modify `Assembler` struct
    fn push_hack(&mut self) -> &mut Self {
        self.hack_stack.push(self.current_hack.clone());
        self
    }

    fn append_hack(&mut self, current_hack: &str) -> &mut Self {
        self.current_hack = self.current_hack.clone() + current_hack;
        self
    }

    fn clear(&mut self) {
        self.current_hack = "".to_string()
    }

    fn label(&mut self) -> u32 {
        let i = self.index;
        self.index += 1;
        i
    }

    fn translate_assembly_stack(&mut self, assembly_stack: Vec<Assembly>) {
        assembly_stack
            .into_iter()
            .for_each(|cmd| self.translate_assembly(cmd));
    }

    fn translate_assembly(&mut self, assembly: Assembly) {
        match assembly {
            Assembly::A(a) => todo!(),
            Assembly::C(c) => self.compile_c_instruction(c),
            Assembly::Label(l) => todo!(),
        }
    }

    fn compile_a_instruction(&mut self, a_instr: AInstruction) {
        todo!()
    }

    fn compile_c_instruction(&mut self, c_instr: CInstruction){
        self.current_hack.clear();
        self.append_hack("111");
        self.compile_comp(c_instr.comp)
            .compile_dest(c_instr.o_dest)
            .compile_jump(c_instr.o_jump)
            .push_hack();
    }

    fn compile_comp(&mut self, comp: Comp) -> &mut Self {
        let comp_str = match comp {
            // a == 0
            Comp::Zero => "0101010",
            Comp::One => "0111111",
            Comp::NegOne => "0111010",
            Comp::D => "0001100",
            Comp::A => "0110000",
            Comp::NotD => "0001101",
            Comp::NotA => "0110001",
            Comp::NegD => "0001111",
            Comp::NegA => "0110011",
            Comp::DPlusOne => "0011111",
            Comp::APlusOne => "0110111",
            Comp::DMinusOne => "0001110",
            Comp::AMinusOne => "0110010",
            Comp::DPlusA => "0000010",
            Comp::DMinusA => "0010011",
            Comp::AMinusD => "0000111",
            Comp::DAndA => "0000000",
            Comp::DOrA => "0010101",

            // a == 1
            Comp::M => "1110000",
            Comp::NotM => "1110001",
            Comp::NegM => "1110011",
            Comp::MPlusOne => "1110111",
            Comp::MMinusOne => "1110010",
            Comp::DPlusM => "1000010",
            Comp::DMinusM => "1010011",
            Comp::MMinusD => "1000111",
            Comp::DAndM => "1000000",
            Comp::DOrM => "1010101",
        };
        self.append_hack(comp_str)
    }

    fn compile_dest(&mut self, o_dest: Option<Dest>) -> &mut Self {
        let dest_str = match o_dest {
            None => "000",
            Some(dest) => match dest {
                Dest::M => "001",
                Dest::D => "010",
                Dest::DM => "011",
                Dest::A => "100",
                Dest::AM => "101",
                Dest::AD => "110",
                Dest::ADM => "111",
            }
        };
        self.append_hack(dest_str)
    }

    fn compile_jump(&mut self, o_jump: Option<Jump>) -> &mut Self {
        let jump_str = match o_jump {
            None => "000",
            Some(jump) => match jump {
                Jump::JGT => "001",
                Jump::JEQ => "010",
                Jump::JGE => "011",
                Jump::JLT => "100",
                Jump::JNE => "101",
                Jump::JLE => "110",
                Jump::JMP => "111",
            }
        };
        self.append_hack(jump_str)
    }
}
