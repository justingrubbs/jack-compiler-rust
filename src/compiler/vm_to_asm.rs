use crate::ast::asm::*;
use crate::ast::vm::*;

use std::mem::take;

pub struct VmToAsm {
    file_name: String,
    func_name: String,
    label_count: u64,
    assembly_stack: Vec<Assembly>,
}

impl VmToAsm {
    pub fn compile(file_name: String, commands: Vec<Command>) -> Vec<Assembly> {
        let mut virtual_machine = Self {
            file_name,
            func_name: "".to_string(),
            label_count: 1,
            assembly_stack: Vec::new(),
        };
        let asm = virtual_machine.compile_commands(commands);
        take(&mut asm.assembly_stack)
    }

    fn push(&mut self, asm: Assembly) -> &mut Self {
        self.assembly_stack.push(asm);
        self
    }

    fn push_a(&mut self, a_instr: AInstruction) -> &mut Self {
        self.push(Assembly::A(a_instr))
    }

    fn push_c(&mut self, comp: Comp, o_dest: Option<Dest>, o_jump: Option<Jump>) -> &mut Self {
        self.push(Assembly::C(CInstruction {
            comp,
            o_dest,
            o_jump,
        }))
    }

    fn push_label(&mut self, name: String) -> &mut Self {
        self.push(Assembly::Label(name))
    }

    fn compile_commands(&mut self, commands: Vec<Command>) -> &mut Self {
        commands.into_iter().for_each(|command| {
            self.compile_command(command);
        });
        self
    }

    fn compile_command(&mut self, command: Command) -> &mut Self {
        match command {
            Command::Stack(s) => match s {
                Stack::Push(s, i) => self.compile_push(s, i),
                Stack::Pop(s, i) => todo!(),
            },
            Command::ACL(acl) => self.compile_acl(acl),
            Command::Function(f) => todo!(),
            Command::Branch(b) => todo!(),
        }
    }

    fn compile_push(&mut self, segment: Segment, i: u16) -> &mut Self {
        match segment {
            Segment::Constant => {
                self.push_a(AInstruction::Constant(i))
                    .push_c(Comp::A, Some(Dest::D), None)
            }
            Segment::Pointer => {
                let symbol = match i {
                    0 => "THIS",
                    1 => "THAT",
                    other => panic!(
                        "Invalid pointer index: {}. Only 0 and 1 are allowed.",
                        other
                    ),
                };
                self.push_a(AInstruction::Symbol(symbol.to_string()))
                    .push_c(Comp::M, Some(Dest::D), None)
            }
            Segment::Temp => todo!(),
            _ => todo!(),
        };
        self.push_a(AInstruction::Symbol("SP".to_string()))
            .push_c(Comp::M, Some(Dest::A), None)
            .push_c(Comp::D, Some(Dest::M), None)
            .push_a(AInstruction::Symbol("SP".to_string()))
            .push_c(Comp::MPlusOne,Some(Dest::M), None)
    }

    fn compile_acl(&mut self, acl: ACL) -> &mut Self {
        match acl {
            ACL::Arithmetic(a) => todo!(),
            ACL::Comparison(c) => todo!(),
            ACL::Logical(l) => todo!(),
        }
    }

    fn compile_arithmetic(&mut self, arith: Arithmetic) -> &mut Self {
        self
    }
}
