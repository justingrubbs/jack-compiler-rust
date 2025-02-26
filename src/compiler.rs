use crate::ast::jack::*;
use crate::ast::vm::*;

use ::std::collections::HashMap;

pub struct Compiler {
    file_name: String,
    class_name: String,
    global_ctx: HashMap<String, Var>,
    local_ctx: HashMap<String, Var>,
    var_counts: (i16, i16, i16, i16),
    label_count: i32,
    instruction_stack: Vec<Command>,
}

impl Compiler {
    pub fn new(file_name: String, class_name: String) -> Self {
        Self {
            file_name,
            class_name,
            global_ctx: HashMap::new(),
            local_ctx: HashMap::new(),
            var_counts: (0, 0, 0, 0), // (field, static, argument, variable)
            label_count: 0,
            instruction_stack: Vec::new(),
        }
    }

    // Methods to modify `Compiler`
    pub fn lookup(&self, name: &str) -> Option<&Var> {
        self.local_ctx
            .get(name)
            .or_else(|| self.global_ctx.get(name))
    }

    pub fn insert_local(&mut self, name: &str, var: Var) {
        &self.local_ctx.insert(name.to_string(), var);
    }

    pub fn insert_global(&mut self, name: &str, var: Var) {
        &self.global_ctx.insert(name.to_string(), var);
    }

    pub fn push(&mut self, command: Command) {
        self.instruction_stack.push(command)
    }

    pub fn inc_label(&mut self) -> i32 {
        let l = self.label_count;
        self.label_count += 1;
        l
    }

    pub fn inc_var_count(&mut self, i: i8) {
        match i {
            0 => self.var_counts.0 += 1,
            1 => self.var_counts.1 += 1,
            2 => self.var_counts.2 += 1,
            3 => self.var_counts.3 += 1,
            _ => return,
        }
    }
}

pub fn compile_class(file_name: String, class: Class) -> Compiler {
    let class_name = class.class_name;
    let compiler = Compiler::new(file_name, class_name);
    compile_class_dec(compiler, class.class_dec)
}

fn compile_class_dec(compiler: Compiler, class_dec: ClassDec) -> Compiler {
    todo!()
}
