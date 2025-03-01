use crate::ast::jack::*;
use crate::ast::vm::*;

use std::collections::HashMap;
use std::mem::take;

pub struct JackToVm {
    file_name: String,
    class_name: String,
    global_ctx: HashMap<String, Var>,
    local_ctx: HashMap<String, Var>,
    global_kind_counts: (i16, i16),
    local_kind_counts: (i16, i16),
    label_count: i32,
    instruction_stack: Vec<Command>,
}

impl JackToVm {
    pub fn compile(file_name: String, class: Class) -> Vec<Command> {
        let mut compiler = Self {
            file_name,
            class_name: class.class_name,
            global_ctx: HashMap::new(),
            local_ctx: HashMap::new(),
            global_kind_counts: (0, 0), // (field, static)
            local_kind_counts: (0, 0),  // (argument, variable)
            label_count: 0,
            instruction_stack: Vec::new(),
        };
        let ins = compiler.compile_class_dec(class.class_dec);
        take(&mut ins.instruction_stack)
    }

    // Methods to modify `Compiler`
    fn lookup(&self, name: &str) -> Option<&Var> {
        self.local_ctx
            .get(name)
            .or_else(|| self.global_ctx.get(name))
    }

    fn insert_global(&mut self, name: String, r#type: Type, global_kind: GlobalKind) -> &mut Self {
        let index = self.get_global_kind_count(global_kind.clone());
        self.inc_kind_count(VarKind::Global(global_kind.clone()));
        self.global_ctx.insert(
            name.to_string(),
            Var {
                r#type,
                var_kind: VarKind::Global(global_kind),
                index,
            },
        );
        self
    }

    fn insert_local(&mut self, name: String, r#type: Type, local_kind: LocalKind) -> &mut Self {
        let index = self.get_local_kind_count(local_kind.clone());
        self.inc_kind_count(VarKind::Local(local_kind.clone()));
        self.local_ctx.insert(
            name.to_string(),
            Var {
                r#type,
                var_kind: VarKind::Local(local_kind),
                index,
            },
        );
        self
    }

    fn reset_local(&mut self) -> &mut Self {
        self.local_ctx = HashMap::new();
        self.local_kind_counts.0 = 0;
        self.local_kind_counts.1 = 0;
        self
    }

    fn push(&mut self, command: Command) -> &mut Self {
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

    fn inc_kind_count(&mut self, var_kind: VarKind) -> &mut Self {
        match var_kind {
            VarKind::Global(g) => match g {
                GlobalKind::Field => self.global_kind_counts.0 += 1,
                GlobalKind::Static => self.global_kind_counts.1 += 1,
            },
            VarKind::Local(l) => match l {
                LocalKind::Arg => self.local_kind_counts.0 += 1,
                LocalKind::Var => self.local_kind_counts.1 += 1,
            },
        };
        self
    }

    // Compilation functions:
    fn compile_class_dec(&mut self, class_dec: ClassDec) -> &mut Self {
        for cvd in class_dec.class_var_dec {
            self.compile_class_var_dec(cvd);
        }
        for sd in class_dec.subroutine_dec {
            self.compile_subroutine_dec(sd);
        }
        self
    }

    fn compile_class_var_dec(&mut self, class_var_dec: ClassVarDec) -> &mut Self {
        for ident in class_var_dec.vars {
            self.insert_global(
                ident,
                class_var_dec.r#type.clone(),
                kind_to_global(class_var_dec.kind.clone()),
            );
        }
        self
    }

    fn compile_subroutine_dec(&mut self, subroutine_dec: SubroutineDec) -> &mut Self {
        self.reset_local();
        match subroutine_dec.subroutine_type {
            SubroutineType::Method => self.insert_local(
                "this".to_string(),
                Type::ClassName(self.class_name.to_string()),
                LocalKind::Arg,
            ),
            SubroutineType::Function => {
                let parameter_length: i16 = subroutine_dec
                    .parameter_list
                    .clone()
                    .len()
                    .try_into()
                    .unwrap();
                self.compile_parameter_list(subroutine_dec.parameter_list);
                self.push(Command::Function(Function::Function(
                    format!("{}.{}", self.file_name, subroutine_dec.subroutine_name),
                    parameter_length,
                )));
                for var_dec in subroutine_dec.subroutine_body.var_decs {
                    self.compile_var_dec(var_dec);
                }
                for statement in subroutine_dec.subroutine_body.stmts {
                    self.compile_statement(statement);
                }
                self
            }
            _ => self,
        }
    }

    fn compile_parameter_list(&mut self, parameter_list: Vec<Parameter>) -> &mut Self {
        for parameter in parameter_list {
            self.insert_local(parameter.var_name, parameter.r#type, LocalKind::Arg);
        }
        self
    }

    fn compile_var_dec(&mut self, var_dec: VarDec) -> &mut Self {
        // Needs to take in a VarKind for hwen inserting
        todo!()
    }

    fn compile_statement(&mut self, statement: Statement) -> &mut Self {
        match statement {
            Statement::ReturnStatement(or) => match or {
                Some(r) => self
                    .compile_expression(r)
                    .push(Command::Function(Function::Return)),
                None => self
                    .push(Command::Stack(Stack::Push(Segment::Constant, 0)))
                    .push(Command::Function(Function::Return)),
            },
            Statement::DoStatement(sc) => self
                .compile_subroutine_call(sc)
                .push(Command::Stack(Stack::Pop(Segment::Temp, 0))),
            Statement::LetStatement(ident, array, e) => {
                let var = self.lookup(&ident).expect("Variable not in context");
                let var_segment = match var.var_kind {
                    VarKind::Local(_) => Segment::Local,
                    VarKind::Global(ref g) => match g {
                        GlobalKind::Field => Segment::This,
                        GlobalKind::Static => Segment::Static,
                    },
                };
                let index = var.index;
                match array {
                    None => self
                        .compile_expression(e)
                        .push(Command::Stack(Stack::Pop(var_segment, index))),
                    Some(a) => todo!(),
                }
            }
            _ => todo!(),
        }
    }

    fn compile_subroutine_call(&mut self, subroutine_call: SubroutineCall) -> &mut Self {
        match subroutine_call {
            SubroutineCall::Call(subroutine_name, exprs) => todo!(),
            SubroutineCall::ClassCall(name, subroutine_name, exprs) => match self.lookup(&name) {
                None => {
                    let mut exprs_length = 0;
                    for expr in exprs {
                        self.compile_expression(*expr);
                        exprs_length += 1;
                    }
                    self.push(Command::Function(Function::Call(
                        format!("{}.{}", name, subroutine_name),
                        exprs_length,
                    )))
                }
                Some(t) => todo!(),
            },
        }
    }

    fn compile_expression(&mut self, expression: Expression) -> &mut Self {
        self.compile_term(*expression.term);
        expression.bin.into_iter().for_each(|(b, t)| {
            self.compile_term(*t).compile_binary_op(b);
        });
        self
    }

    fn compile_term(&mut self, term: Term) -> &mut Self {
        match term {
            Term::IntegerConstant(i) => {
                self.push(Command::Stack(Stack::Push(Segment::Constant, i)))
            }
            Term::StringConstant(s) => {
                let s_length: i16 = s.len().try_into().unwrap();
                self.push(Command::Stack(Stack::Push(Segment::Constant, s_length)))
                    .push(Command::Function(Function::Call(
                        "String.new".to_string(),
                        1,
                    )));
                for c in s.chars() {
                    let char_code = c as i16;
                    self.push(Command::Stack(Stack::Push(Segment::Constant, char_code)))
                        .push(Command::Function(Function::Call(
                            "String.appendChar".to_string(),
                            2,
                        )));
                }
                self
            }
            Term::KeywordConstant(kw) => self.compile_keyword_constant(kw),
            Term::VarName(s, oe) => todo!(),
            Term::UnaryTerm(uop, t) => self.compile_term(*t).compile_unary_op(uop),
            Term::ParensExpr(e) => self.compile_expression(*e),
            Term::SubroutineCall(sc) => self.compile_subroutine_call(sc),
        }
    }

    fn compile_keyword_constant(&mut self, kw: KeywordConstant) -> &mut Self {
        match kw {
            KeywordConstant::False => self.push(Command::Stack(Stack::Push(Segment::Constant, 0))),
            KeywordConstant::True => self
                .push(Command::Stack(Stack::Push(Segment::Constant, 1)))
                .push(Command::ACL(ACL::Logical(Logical::Not))),
            KeywordConstant::This => self.push(Command::Stack(Stack::Push(Segment::Pointer, 0))),
            KeywordConstant::Null => self.push(Command::Stack(Stack::Push(Segment::Constant, 0))),
        }
    }

    fn compile_binary_op(&mut self, op: BinaryOp) -> &mut Self {
        match op {
            BinaryOp::Plus => self.push(Command::ACL(ACL::Arithmetic(Arithmetic::Add))),
            BinaryOp::Minus => self.push(Command::ACL(ACL::Arithmetic(Arithmetic::Sub))),
            BinaryOp::Times => self.push(Command::Function(Function::Call(
                "Math.multiply".to_string(),
                2,
            ))),
            BinaryOp::Div => self.push(Command::Function(Function::Call(
                "Math.divide".to_string(),
                2,
            ))),
            BinaryOp::And => self.push(Command::ACL(ACL::Logical(Logical::And))),
            BinaryOp::Or => self.push(Command::ACL(ACL::Logical(Logical::Or))),
            BinaryOp::Lesser => self.push(Command::ACL(ACL::Comparison(Comparison::Lt))),
            BinaryOp::Greater => self.push(Command::ACL(ACL::Comparison(Comparison::Gt))),
            BinaryOp::Equal => self.push(Command::ACL(ACL::Comparison(Comparison::Eq))),
        }
    }

    fn compile_unary_op(&mut self, op: UnaryOp) -> &mut Self {
        match op {
            UnaryOp::Negation => self.push(Command::ACL(ACL::Arithmetic(Arithmetic::Neg))),
            UnaryOp::Tilde => self.push(Command::ACL(ACL::Logical(Logical::Not))),
        }
    }
}

fn kind_to_global(kind: Kind) -> GlobalKind {
    match kind {
        Kind::Field => GlobalKind::Field,
        Kind::Static => GlobalKind::Static,
    }
}
