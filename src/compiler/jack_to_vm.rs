use crate::ast::jack::*;
use crate::ast::vm::*;

use std::collections::HashMap;
use std::mem::take;

pub struct JackToVm {
    file_name: String,
    class_name: String,
    global_ctx: HashMap<String, Var>,
    local_ctx: HashMap<String, Var>,
    global_kind_counts: (u16, u16),
    local_kind_counts: (u16, u16),
    if_count: u16,
    while_count: u16,
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
            if_count: 0,
            while_count: 0,
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
        self.if_count = 0;
        self.while_count = 0;
        self
    }

    fn push(&mut self, command: Command) -> &mut Self {
        self.instruction_stack.push(command);
        self
    }

    fn push_stack(&mut self, stack: Stack) -> &mut Self {
        self.push(Command::Stack(stack))
    }

    fn push_acl(&mut self, acl: ACL) -> &mut Self {
        self.push(Command::ACL(acl))
    }

    fn push_branch(&mut self, branch: Branch) -> &mut Self {
        self.push(Command::Branch(branch))
    }

    fn push_func(&mut self, func: Function) -> &mut Self {
        self.push(Command::Function(func))
    }

    fn if_label(&mut self) -> u16 {
        let l = self.if_count;
        self.if_count += 1;
        l
    }

    fn while_label(&mut self) -> u16 {
        let l = self.while_count;
        self.while_count += 1;
        l
    }

    fn get_global_kind_count(&mut self, global_kind: GlobalKind) -> u16 {
        match global_kind {
            GlobalKind::Field => self.global_kind_counts.0,
            GlobalKind::Static => self.global_kind_counts.1,
        }
    }

    fn get_local_kind_count(&mut self, local_kind: LocalKind) -> u16 {
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
        class_dec.class_var_dec.into_iter().for_each(|cvd| {
            self.compile_class_var_dec(cvd);
        });
        class_dec.subroutine_dec.into_iter().for_each(|sd| {
            self.compile_subroutine_dec(sd);
        });
        self
    }

    fn compile_class_var_dec(&mut self, class_var_dec: ClassVarDec) -> &mut Self {
        class_var_dec.vars.into_iter().for_each(|ident| {
            self.insert_global(
                ident,
                class_var_dec.r#type.clone(),
                kind_to_global(class_var_dec.kind.clone()),
            );
        });
        self
    }

    fn compile_subroutine_dec_head(
        &mut self,
        bool: bool,
        var_decs: Vec<VarDec>,
        subroutine_name: String,
    ) -> &mut Self {
        let mut local_length = 0;
        var_decs.into_iter().for_each(|var_dec| {
            local_length += var_dec.var_name.len() as u16;
            self.compile_var_dec(var_dec);
        });
        self.push_func(Function::Body(
            format!("{}.{}", self.file_name, subroutine_name),
            local_length,
        ));
        if bool {
            self.push_stack(Stack::Push(Segment::Argument, 0))
                .push_stack(Stack::Pop(Segment::Pointer, 0))
        } else {
            self
        }
    }

    fn compile_subroutine_dec(&mut self, subroutine_dec: SubroutineDec) -> &mut Self {
        self.reset_local();
        match subroutine_dec.subroutine_type {
            SubroutineType::Method => {
                self.insert_local(
                    "this".to_string(),
                    Type::ClassName(self.class_name.to_string()),
                    LocalKind::Arg,
                )
                .compile_parameter_list(subroutine_dec.parameter_list)
                .compile_subroutine_dec_head(
                    true,
                    subroutine_dec.subroutine_body.var_decs,
                    subroutine_dec.subroutine_name,
                );
            }
            SubroutineType::Function => {
                self.compile_parameter_list(subroutine_dec.parameter_list)
                    .compile_subroutine_dec_head(
                        false,
                        subroutine_dec.subroutine_body.var_decs,
                        subroutine_dec.subroutine_name,
                    );
            }
            SubroutineType::Constructor => {
                self.compile_parameter_list(subroutine_dec.parameter_list)
                    .compile_subroutine_dec_head(
                        false,
                        subroutine_dec.subroutine_body.var_decs,
                        subroutine_dec.subroutine_name,
                    );
                let i = self.get_global_kind_count(GlobalKind::Field);
                self.push_stack(Stack::Push(Segment::Constant, i))
                    .push_func(Function::Call("Memory.alloc".to_string(), 1))
                    .push_stack(Stack::Pop(Segment::Pointer, 0));
            }
        }
        subroutine_dec
            .subroutine_body
            .stmts
            .into_iter()
            .for_each(|stmt| {
                self.compile_statement(stmt);
            });
        self
    }

    fn compile_parameter_list(&mut self, parameter_list: Vec<Parameter>) -> &mut Self {
        parameter_list.into_iter().for_each(|parameter| {
            self.insert_local(parameter.var_name, parameter.r#type, LocalKind::Arg);
        });
        self
    }

    fn compile_var_dec(&mut self, var_dec: VarDec) -> &mut Self {
        var_dec.var_name.into_iter().for_each(|var| {
            self.insert_local(var, var_dec.r#type.clone(), LocalKind::Var);
        });
        self
    }

    fn compile_statement(&mut self, statement: Statement) -> &mut Self {
        match statement {
            Statement::Return(or) => match or {
                Some(r) => self.compile_expression(r).push_func(Function::Return),
                None => self
                    .push_stack(Stack::Push(Segment::Constant, 0))
                    .push_func(Function::Return),
            },
            Statement::Do(sc) => self
                .compile_subroutine_call(sc)
                .push_stack(Stack::Pop(Segment::Temp, 0)),
            Statement::Let(ident, array, e) => {
                let var = self.lookup(&ident).expect("Variable not in context");
                let var_segment = var_kind_to_segment(var.var_kind.clone());
                let index = var.index;
                match array {
                    None => self
                        .compile_expression(e)
                        .push_stack(Stack::Pop(var_segment, index)),
                    Some(e2) => self
                        .compile_expression(e2)
                        .push_stack(Stack::Push(var_segment, index))
                        .compile_binary_op(BinaryOp::Plus)
                        .compile_expression(e)
                        .push_stack(Stack::Pop(Segment::Temp, 0))
                        .push_stack(Stack::Pop(Segment::Pointer, 1))
                        .push_stack(Stack::Push(Segment::Temp, 0))
                        .push_stack(Stack::Pop(Segment::That, 0)),
                }
            }
            Statement::While(expr, stmts) => {
                let label = self.while_label();
                self.push_branch(Branch::Label(format!("WHILE_EXP{}", label)))
                    .compile_expression(expr)
                    .compile_unary_op(UnaryOp::Tilde)
                    .push_branch(Branch::IfGoto(format!("WHILE_END{}", label)));

                stmts.into_iter().for_each(|stmt| {
                    self.compile_statement(stmt);
                });

                self.push_branch(Branch::Goto(format!("WHILE_EXP{}", label)))
                    .push_branch(Branch::Label(format!("WHILE_END{}", label)))
            }
            Statement::If(e, s1, o_s2) => {
                self.compile_expression(e);
                let label = self.if_label();
                self.push_branch(Branch::IfGoto(format!("IF_TRUE{}", label)))
                    .push_branch(Branch::Goto(format!("IF_FALSE{}", label)))
                    .push_branch(Branch::Label(format!("IF_TRUE{}", label)));

                s1.into_iter().for_each(|stmt| {
                    self.compile_statement(stmt);
                });

                match o_s2 {
                    None => self.push_branch(Branch::Label(format!("IF_FALSE{}", label))),
                    Some(s2) => {
                        self.push_branch(Branch::Goto(format!("IF_END{}", label)))
                            .push_branch(Branch::Label(format!("IF_FALSE{}", label)));
                        s2.into_iter().for_each(|stmt| {
                            self.compile_statement(stmt);
                        });
                        self.push_branch(Branch::Label(format!("IF_END{}", label)))
                    }
                }
            }
        }
    }

    fn compile_subroutine_call(&mut self, subroutine_call: SubroutineCall) -> &mut Self {
        match subroutine_call {
            SubroutineCall::Call(subroutine_name, exprs) => {
                self.push_stack(Stack::Push(Segment::Pointer, 0));
                exprs.into_iter().for_each(|expr| {
                    self.compile_expression(*expr);
                });
                self.push_func(Function::Call(
                    format!("{}.{}", self.class_name, subroutine_name),
                    1,
                ))
            }
            SubroutineCall::ClassCall(name, subroutine_name, exprs) => match self.lookup(&name) {
                Some(Var {
                    r#type: Type::ClassName(c),
                    var_kind,
                    index,
                }) => {
                    let class_name = c.clone();

                    let segment = match var_kind {
                        VarKind::Local(LocalKind::Var) => Segment::Local,
                        _ => Segment::This,
                    };

                    self.push_stack(Stack::Push(segment, *index));

                    let mut exprs_length = 1;

                    exprs.into_iter().for_each(|expr| {
                        self.compile_expression(*expr);
                        exprs_length += 1;
                    });

                    self.push_func(Function::Call(
                        format!("{}.{}", class_name, subroutine_name),
                        exprs_length,
                    ))
                }

                _ => {
                    let mut exprs_length = 0;
                    exprs.into_iter().for_each(|expr| {
                        self.compile_expression(*expr);
                        exprs_length += 1;
                    });
                    self.push_func(Function::Call(
                        format!("{}.{}", name, subroutine_name),
                        exprs_length,
                    ))
                }
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
                self.push_stack(Stack::Push(Segment::Constant, i.try_into().unwrap()))
            }
            Term::StringConstant(s) => {
                let s_length: u16 = s.len().try_into().unwrap();
                self.push_stack(Stack::Push(Segment::Constant, s_length))
                    .push_func(Function::Call("String.new".to_string(), 1));
                s.chars().for_each(|c| {
                    let char_code = c as u16;
                    self.push_stack(Stack::Push(Segment::Constant, char_code))
                        .push_func(Function::Call("String.appendChar".to_string(), 2));
                });
                self
            }
            Term::KeywordConstant(kw) => self.compile_keyword_constant(kw),
            Term::VarName(s, oe) => {
                let o_var = self.lookup(&s);
                match o_var {
                    None => self,
                    Some(var) => {
                        let segment = var_kind_to_segment(var.var_kind.clone());
                        let index = var.index;
                        match oe {
                            None => self.push_stack(Stack::Push(
                                var_kind_to_segment(var.var_kind.clone()),
                                var.index,
                            )),
                            Some(e) => self
                                .compile_expression(*e)
                                .push_stack(Stack::Push(segment, index))
                                .compile_binary_op(BinaryOp::Plus)
                                .push_stack(Stack::Pop(Segment::Pointer, 1))
                                .push_stack(Stack::Push(Segment::That, 0)),
                        }
                    }
                }
            }
            Term::Unary(uop, t) => self.compile_term(*t).compile_unary_op(uop),
            Term::ParensExpr(e) => self.compile_expression(*e),
            Term::SubroutineCall(sc) => self.compile_subroutine_call(sc),
        }
    }

    fn compile_keyword_constant(&mut self, kw: KeywordConstant) -> &mut Self {
        match kw {
            KeywordConstant::False => self.push_stack(Stack::Push(Segment::Constant, 0)),
            KeywordConstant::True => self
                .push_stack(Stack::Push(Segment::Constant, 0))
                .push_acl(ACL::Logical(Logical::Not)),
            KeywordConstant::This => self.push_stack(Stack::Push(Segment::Pointer, 0)),
            KeywordConstant::Null => self.push_stack(Stack::Push(Segment::Constant, 0)),
        }
    }

    fn compile_binary_op(&mut self, op: BinaryOp) -> &mut Self {
        match op {
            BinaryOp::Plus => self.push_acl(ACL::Arithmetic(Arithmetic::Add)),
            BinaryOp::Minus => self.push_acl(ACL::Arithmetic(Arithmetic::Sub)),
            BinaryOp::Times => self.push_func(Function::Call("Math.multiply".to_string(), 2)),
            BinaryOp::Div => self.push_func(Function::Call("Math.divide".to_string(), 2)),
            BinaryOp::And => self.push_acl(ACL::Logical(Logical::And)),
            BinaryOp::Or => self.push_acl(ACL::Logical(Logical::Or)),
            BinaryOp::Lesser => self.push_acl(ACL::Comparison(Comparison::Lt)),
            BinaryOp::Greater => self.push_acl(ACL::Comparison(Comparison::Gt)),
            BinaryOp::Equal => self.push_acl(ACL::Comparison(Comparison::Eq)),
        }
    }

    fn compile_unary_op(&mut self, op: UnaryOp) -> &mut Self {
        match op {
            UnaryOp::Negation => self.push_acl(ACL::Arithmetic(Arithmetic::Neg)),
            UnaryOp::Tilde => self.push_acl(ACL::Logical(Logical::Not)),
        }
    }
}

fn kind_to_global(kind: Kind) -> GlobalKind {
    match kind {
        Kind::Field => GlobalKind::Field,
        Kind::Static => GlobalKind::Static,
    }
}

fn var_kind_to_segment(var_kind: VarKind) -> Segment {
    match var_kind {
        VarKind::Global(g) => match g {
            GlobalKind::Field => Segment::This,
            GlobalKind::Static => Segment::Static,
        },
        VarKind::Local(l) => match l {
            LocalKind::Arg => Segment::Argument,
            LocalKind::Var => Segment::Local,
        },
    }
}
