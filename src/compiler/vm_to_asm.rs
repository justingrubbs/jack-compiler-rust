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

    fn push_c(&mut self, c_instr: CInstruction) -> &mut Self {
        self.push(Assembly::C(c_instr))
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
            Command::ACL(acl) => todo!(),
            Command::Function(f) => todo!(),
            Command::Branch(b) => todo!(),
        }
    }

    fn compile_push(&mut self, segment: Segment, i: u16) -> &mut Self {
        match segment {
            Segment::Constant => self.push_a(AInstruction::Constant(i)).push_c(CInstruction {
                comp: Comp::A,
                o_dest: Some(Dest::D),
                o_jump: None,
            }),
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
                    .push_c(CInstruction {
                        comp: Comp::M,
                        o_dest: Some(Dest::D),
                        o_jump: None,
                    })
            },
            Segment::Temp => todo!(),
            _ => todo!(),
        }
    }
}
