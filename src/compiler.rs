use crate::ast::jack::*;
use crate::ast::vm::*;

use ::std::collections::HashMap;

pub struct Compiler {
    file_name: String,
    class_name: String,
    global_ctx: HashMap<String, Var>,
    local_ctx: HashMap<String, Var>,
    global_kind_counts: (i16, i16),
    local_kind_counts: (i16, i16),
    label_count: i32,
    instruction_stack: Vec<Command>,
}

impl Compiler {
    pub fn compile(file_name: String, class: Class) -> Vec<Command> {
        Self {
            file_name,
            class_name: class.class_name,
            global_ctx: HashMap::new(),
            local_ctx: HashMap::new(),
            global_kind_counts: (0, 0), // (field, static)
            local_kind_counts: (0, 0), // (argument, variable)
            label_count: 0,
            instruction_stack: Vec::new(),
        }.compile_class_dec(class.class_dec)
        .instruction_stack

    }

    // Methods to modify `Compiler`
    fn lookup(&self, name: &str) -> Option<&Var> {
        self.local_ctx
            .get(name)
            .or_else(|| self.global_ctx.get(name))
    }

    fn insert_global(&mut self, name: String, r#type: Type, global_kind: GlobalKind) {
        let index = self.get_global_kind_count(global_kind.clone());
        self.inc_kind_count(VarKind::Global(global_kind.clone()));
        self.global_ctx.insert(
            name.to_string(),
            Var {
                r#type,
                var_kind: VarKind::Global(global_kind),
                index,
            }
        );
    }

    fn insert_local(&mut self, name: String, r#type: Type, local_kind: LocalKind) {
        let index = self.get_local_kind_count(local_kind.clone());
        self.inc_kind_count(VarKind::Local(local_kind.clone()));
        self.local_ctx.insert(
            name.to_string(), 
            Var { 
                r#type, 
                var_kind: VarKind::Local(local_kind), 
                index
            }
        );
    }

    fn reset_local(&mut self) {
        self.local_ctx = HashMap::new();
        self.local_kind_counts.0 = 0;
        self.local_kind_counts.1 = 0;
    }

    fn push(mut self, command: Command) {
        self.instruction_stack.push(command);
        self
    }

    fn inc_label(&mut self) -> i32 {
        let l = self.label_count;
        self.label_count += 1;
        l
    }

    fn get_global_kind_count(&mut self, global_kind: GlobalKind) -> i16 {
        match global_kind {
            GlobalKind::Field => self.global_kind_counts.0,
            GlobalKind::Static => self.global_kind_counts.1,
        }
    }

    fn get_local_kind_count(&mut self, local_kind: LocalKind) -> i16 {
        match local_kind {
            LocalKind::Arg => self.local_kind_counts.0,
            LocalKind::Var => self.local_kind_counts.1,
        }
    }

    fn inc_kind_count(&mut self, var_kind: VarKind) {
        match var_kind {
            VarKind::Global(g) => match g {
                GlobalKind::Field => self.global_kind_counts.0 += 1,
                GlobalKind::Static => self.global_kind_counts.1 += 1,
            },
            VarKind::Local(l) => match l {
                LocalKind::Arg => self.local_kind_counts.0 += 1,
                LocalKind::Var => self.local_kind_counts.1 += 1,
            }
        }
    }

    // Compilation functions:
    fn compile_class_dec(mut self, class_dec: ClassDec) -> Self {
        for cvd in class_dec.class_var_dec {
            self = self.compile_class_var_dec(cvd);
        }
        for sd in class_dec.subroutine_dec {
            self = self.compile_subroutine_dec(sd);
        }
        self
    }

    fn compile_class_var_dec(mut self, class_var_dec: ClassVarDec) -> Self {
        for ident in class_var_dec.vars {
            self.insert_global(
                ident, 
                class_var_dec.r#type.clone(), 
                kind_to_global(class_var_dec.kind.clone())
            );
        }
        self
    }

    fn compile_subroutine_dec(&mut self, subroutine_dec: SubroutineDec) -> Self {
        self.reset_local();
        let _ = match subroutine_dec.subroutine_type {
            SubroutineType::Method => self.insert_local(
                "this".to_string(), 
                Type::ClassName(self.class_name.to_string()), 
                LocalKind::Arg),
            _ => (),
        };

        todo!()
    } 





    fn compile_expression(mut self, expression: Expression) -> Self {
        self.compile_term(*expression.term);
        expression
        .bin
        .into_iter()
        .for_each(|(b, t)| {
            self.compile_term(*t).compile_op(b);
        });
        self
    }

    fn compile_term(&mut self, term: Term) -> Self {
        todo!()
    }

    fn compile_op(&mut self, op: BinaryOp) -> &mut Self {
        match op {
            BinaryOp::Plus => self.push(Command::ACL(ACL::Arithmetic(Arithmetic::Add))),
            BinaryOp::Minus => self.push(Command::ACL(ACL::Arithmetic(Arithmetic::Sub))),
            BinaryOp::Times => self.push(Command::Function(Function::Call("Math.multiply".to_string(),2))),
            BinaryOp::Div => self.push(Command::Function(Function::Call("Math.divide".to_string(),2))),

            BinaryOp::And => todo!(),
            BinaryOp::Or => todo!(),
            BinaryOp::Lesser =>  todo!(),
            BinaryOp::Greater => todo!(),
            BinaryOp::Equal => todo!(),
        }
        self
    }







}

fn kind_to_global(kind: Kind) -> GlobalKind {
    match kind {
        Kind::Field => GlobalKind::Field,
        Kind::Static => GlobalKind::Static,
    }
}

