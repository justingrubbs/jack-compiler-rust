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


    fn compile_commands(&mut self, commands: Vec<Command>) -> &mut Self {
        commands.into_iter().for_each(|command| {
            self.compile_command(command);
        });
        self
    }

    fn compile_command(&mut self, command: Command) -> &mut Self {
        match command {
            Command::Stack(s) => match s {
                Stack::Push(s,i) => todo!(),
                Stack::Pop(s,i) => todo!(),
            },
            Command::ACL(acl) => todo!(),
            Command::Function(f) => todo!(),
            Command::Branch(b) => todo!(),
        }
    }

}
