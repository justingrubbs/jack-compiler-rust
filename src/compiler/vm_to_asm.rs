use crate::ast::asm::*;
use crate::ast::vm::*;

use std::mem::take;

enum CallPush {
    True,
    False,
}

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

    fn inc_label(&mut self) {
        self.label_count += 1;
    }

    fn push(&mut self, asm: Assembly) -> &mut Self {
        self.assembly_stack.push(asm);
        self
    }

    fn push_a(&mut self, a_instr: AInstruction) -> &mut Self {
        self.push(Assembly::A(a_instr))
    }

    fn push_c(&mut self, o_dest: Option<Dest>, comp: Comp, o_jump: Option<Jump>) -> &mut Self {
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
        self.compile_bootstrap();
        commands.into_iter().for_each(|command| {
            self.compile_command(command);
        });
        self
    }

    fn compile_bootstrap(&mut self) {
        self.push_a(AInstruction::Constant(256))
            .push_c(Some(Dest::D), Comp::A, None)
            .push_a(AInstruction::Symbol("SP".to_string()))
            .push_c(Some(Dest::M), Comp::D, None)
            .compile_function(Function::Call("Sys.init".to_string(), 0));
    }

    fn compile_command(&mut self, command: Command) -> &mut Self {
        match command {
            Command::Stack(s) => match s {
                Stack::Push(s, i) => self.compile_push(s, i, CallPush::False),
                Stack::Pop(s, i) => self.compile_pop(s, i),
            },
            Command::ACL(acl) => self.compile_acl(acl),
            Command::Branch(b) => self.compile_branch(b),
            Command::Function(f) => self.compile_function(f),
        }
    }

    fn compile_push(&mut self, segment: Segment, i: u16, bool: CallPush) -> &mut Self {
        match segment {
            Segment::Constant => {
                self.push_a(AInstruction::Constant(i))
                    .push_c(Some(Dest::D), Comp::A, None)
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
                    .push_c(Some(Dest::D), Comp::M, None)
            }
            Segment::Static => {
                let file_name = format!("{}.{}", &self.file_name, i);
                self.push_a(AInstruction::Symbol(file_name))
                    .push_c(Some(Dest::D), Comp::M, None)
            }
            Segment::Temp => {
                self.push_a(AInstruction::Constant(i + 5))
                    .push_c(Some(Dest::D), Comp::M, None)
            }
            seg => match bool {
                CallPush::True => self.push_a(AInstruction::Symbol(show_segment(seg))).push_c(
                    Some(Dest::D),
                    Comp::M,
                    None,
                ),
                CallPush::False => self
                    .push_a(AInstruction::Constant(i))
                    .push_c(Some(Dest::D), Comp::A, None)
                    .push_a(AInstruction::Symbol(show_segment(seg)))
                    .push_c(Some(Dest::A), Comp::DPlusM, None)
                    .push_c(Some(Dest::D), Comp::M, None),
            },
        };
        self.push_pattern()
    }

    fn push_pattern(&mut self) -> &mut Self {
        self.push_a(AInstruction::Symbol("SP".to_string()))
            .push_c(Some(Dest::A), Comp::M, None)
            .push_c(Some(Dest::M), Comp::D, None)
            .push_a(AInstruction::Symbol("SP".to_string()))
            .push_c(Some(Dest::M), Comp::MPlusOne, None)
    }

    fn compile_pop(&mut self, segment: Segment, i: u16) -> &mut Self {
        match segment {
            Segment::Constant => {
                panic!("Constant segment is not a valid segment for pop commands.")
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
                self.push_a(AInstruction::Symbol("SP".to_string()))
                    .push_c(Some(Dest::AM), Comp::MMinusOne, None)
                    .push_c(Some(Dest::D), Comp::M, None)
                    .push_a(AInstruction::Symbol(symbol.to_string()))
                    .push_c(Some(Dest::M), Comp::D, None)
            }
            Segment::Temp => self
                .push_a(AInstruction::Symbol("SP".to_string()))
                .push_c(Some(Dest::AM), Comp::MMinusOne, None)
                .push_c(Some(Dest::D), Comp::M, None)
                .push_a(AInstruction::Constant(i + 5))
                .push_c(Some(Dest::M), Comp::D, None),
            Segment::Static => {
                let file_name = format!("{}.{}", &self.file_name, i);
                self.push_a(AInstruction::Symbol("SP".to_string()))
                    .push_c(Some(Dest::AM), Comp::MMinusOne, None)
                    .push_c(Some(Dest::D), Comp::M, None)
                    .push_a(AInstruction::Symbol(file_name))
                    .push_c(Some(Dest::M), Comp::D, None)
            }
            seg => self
                .push_a(AInstruction::Constant(i))
                .push_c(Some(Dest::D), Comp::A, None)
                .push_a(AInstruction::Symbol(show_segment(seg)))
                .push_c(Some(Dest::D), Comp::DPlusM, None)
                .push_a(AInstruction::Symbol("R13".to_string()))
                .push_c(Some(Dest::M), Comp::D, None)
                .push_a(AInstruction::Symbol("SP".to_string()))
                .push_c(Some(Dest::AM), Comp::MMinusOne, None)
                .push_c(Some(Dest::D), Comp::M, None)
                .push_a(AInstruction::Symbol("R13".to_string()))
                .push_c(Some(Dest::A), Comp::M, None)
                .push_c(Some(Dest::M), Comp::D, None),
        }
    }

    fn compile_acl(&mut self, acl: ACL) -> &mut Self {
        match acl {
            ACL::Arithmetic(a) => self.compile_arithmetic(a),
            ACL::Comparison(c) => self.compile_comparison(c),
            ACL::Logical(l) => self.compile_logical(l),
        }
    }

    fn compile_arithmetic(&mut self, arith: Arithmetic) -> &mut Self {
        match arith {
            Arithmetic::Neg => self.compile_unary().push_c(Some(Dest::M), Comp::NegM, None),
            Arithmetic::Add => self
                .compile_binary()
                .push_c(Some(Dest::M), Comp::DPlusM, None),
            Arithmetic::Sub => self
                .compile_binary()
                .push_c(Some(Dest::M), Comp::MMinusD, None),
        }
    }

    fn compile_comparison(&mut self, comp: Comparison) -> &mut Self {
        let i = self.label_count;
        self.inc_label();
        let comp_s = show_comparison(comp.clone());
        let comp_j = comparison_to_jump(comp);
        self.compile_binary()
            .push_c(Some(Dest::D), Comp::MMinusD, None)
            .push_a(AInstruction::Symbol(format!("{}_true_{}", comp_s, i)))
            .push_c(None, Comp::D, Some(comp_j))
            .push_a(AInstruction::Symbol("SP".to_string()))
            .push_c(Some(Dest::A), Comp::MMinusOne, None)
            .push_c(Some(Dest::M), Comp::Zero, None)
            .push_a(AInstruction::Symbol(format!("{}_end_{}", comp_s, i)))
            .push_c(None, Comp::Zero, Some(Jump::JMP))
            .push_label(format!("{}_true_{}", comp_s, i))
            .push_a(AInstruction::Symbol("SP".to_string()))
            .push_c(Some(Dest::A), Comp::MMinusOne, None)
            .push_c(Some(Dest::M), Comp::NegOne, None)
            .push_label(format!("{}_end_{}", comp_s, i))
    }

    fn compile_logical(&mut self, logic: Logical) -> &mut Self {
        match logic {
            Logical::And => self
                .compile_binary()
                .push_c(Some(Dest::M), Comp::DAndM, None),
            Logical::Or => self
                .compile_binary()
                .push_c(Some(Dest::M), Comp::DOrM, None),
            Logical::Not => self.compile_unary().push_c(Some(Dest::M), Comp::NotM, None),
        }
    }

    fn compile_unary(&mut self) -> &mut Self {
        self.push_a(AInstruction::Symbol("SP".to_string())).push_c(
            Some(Dest::A),
            Comp::MMinusOne,
            None,
        )
    }

    fn compile_binary(&mut self) -> &mut Self {
        self.push_a(AInstruction::Symbol("SP".to_string()))
            .push_c(Some(Dest::AM), Comp::MMinusOne, None)
            .push_c(Some(Dest::D), Comp::M, None)
            .push_c(Some(Dest::A), Comp::AMinusOne, None)
    }

    fn compile_branch(&mut self, branch: Branch) -> &mut Self {
        match branch {
            Branch::Label(l) => self.push_label(format!("{}${}", &self.func_name, l)),
            Branch::Goto(l) => self
                .push_a(AInstruction::Symbol(format!("{}${}", &self.func_name, l)))
                .push_c(None, Comp::Zero, Some(Jump::JMP)),
            Branch::IfGoto(l) => {
                let func_name = format!("{}${}", &self.func_name, l);
                self.push_a(AInstruction::Symbol("SP".to_string()))
                    .push_c(Some(Dest::AM), Comp::MMinusOne, None)
                    .push_c(Some(Dest::D), Comp::M, None)
                    .push_c(Some(Dest::A), Comp::AMinusOne, None)
                    .push_a(AInstruction::Symbol(func_name))
                    .push_c(None, Comp::D, Some(Jump::JNE))
            }
        }
    }

    fn compile_function(&mut self, func: Function) -> &mut Self {
        match func {
            Function::Body(s, i) => {
                self.label_count = 1;
                self.push_label(s).compile_function_locals(i)
            }
            Function::Return => {
                self.push_a(AInstruction::Symbol("LCL".to_string()))
                    .push_c(Some(Dest::D), Comp::M, None)
                    .push_a(AInstruction::Symbol("R13".to_string()))
                    .push_c(Some(Dest::M), Comp::D, None)
                    //
                    .push_a(AInstruction::Constant(5))
                    .push_c(Some(Dest::A), Comp::DMinusA, None)
                    .push_c(Some(Dest::D), Comp::M, None)
                    .push_a(AInstruction::Symbol("R14".to_string()))
                    .push_c(Some(Dest::M), Comp::D, None)
                    //
                    .push_a(AInstruction::Symbol("SP".to_string()))
                    .push_c(Some(Dest::AM), Comp::MMinusOne, None)
                    .push_c(Some(Dest::D), Comp::M, None)
                    .push_a(AInstruction::Symbol("ARG".to_string()))
                    .push_c(Some(Dest::A), Comp::M, None)
                    .push_c(Some(Dest::M), Comp::D, None)
                    //
                    .push_a(AInstruction::Symbol("ARG".to_string()))
                    .push_c(Some(Dest::D), Comp::MPlusOne, None)
                    .push_a(AInstruction::Symbol("SP".to_string()))
                    .push_c(Some(Dest::M), Comp::D, None)
                    //
                    .compile_seg_reset(Segment::That)
                    //
                    .compile_seg_reset(Segment::This)
                    //
                    .compile_seg_reset(Segment::Argument)
                    //
                    .compile_seg_reset(Segment::Local)
                    //
                    .push_a(AInstruction::Symbol("R14".to_string()))
                    .push_c(Some(Dest::A), Comp::M, None)
                    .push_c(None, Comp::Zero, Some(Jump::JMP))
            }
            Function::Call(s, args) => {
                let label_count = self.label_count;
                self.label_count += 1;
                let return_address = format!("{}.{}$ret.{}", &self.file_name, s, label_count);
                self.push_a(AInstruction::Symbol(return_address.clone()))
                    .push_c(Some(Dest::D), Comp::A, None)
                    .push_pattern()
                    .compile_push(Segment::Local, 0, CallPush::True)
                    .compile_push(Segment::Argument, 0, CallPush::True)
                    .compile_push(Segment::This, 0, CallPush::True)
                    .compile_push(Segment::That, 0, CallPush::True)
                    //
                    .push_a(AInstruction::Symbol("SP".to_string()))
                    .push_c(Some(Dest::D), Comp::M, None)
                    .push_a(AInstruction::Constant(args + 5))
                    .push_c(Some(Dest::D), Comp::DMinusA, None)
                    .push_a(AInstruction::Symbol("ARG".to_string()))
                    .push_c(Some(Dest::M), Comp::D, None)
                    //
                    .push_a(AInstruction::Symbol("SP".to_string()))
                    .push_c(Some(Dest::D), Comp::M, None)
                    .push_a(AInstruction::Symbol("LCL".to_string()))
                    .push_c(Some(Dest::M), Comp::D, None)
                    //
                    .push_a(AInstruction::Symbol(s.clone()))
                    .push_c(None, Comp::Zero, Some(Jump::JMP))
                    .push_label(return_address)
            }
        }
    }

    fn compile_function_locals(&mut self, i: u16) -> &mut Self {
        match i {
            0 => self,
            _ => self
                .push_a(AInstruction::Symbol("SP".to_string()))
                .push_c(Some(Dest::A), Comp::M, None)
                .push_c(Some(Dest::M), Comp::Zero, None)
                .push_a(AInstruction::Symbol("SP".to_string()))
                .push_c(Some(Dest::M), Comp::MPlusOne, None)
                .compile_function_locals(i - 1),
        }
    }

    fn compile_seg_reset(&mut self, seg: Segment) -> &mut Self {
        self.push_a(AInstruction::Symbol("R13".to_string()))
            .push_c(Some(Dest::D), Comp::MMinusOne, None)
            .push_c(Some(Dest::AM), Comp::D, None)
            .push_c(Some(Dest::D), Comp::M, None)
            .push_a(AInstruction::Symbol(show_segment(seg)))
            .push_c(Some(Dest::M), Comp::D, None)
    }
}

fn show_comparison(comp: Comparison) -> String {
    match comp {
        Comparison::Eq => "EQ".to_string(),
        Comparison::Lt => "LT".to_string(),
        Comparison::Gt => "GT".to_string(),
    }
}

fn comparison_to_jump(comp: Comparison) -> Jump {
    match comp {
        Comparison::Eq => Jump::JEQ,
        Comparison::Lt => Jump::JLT,
        Comparison::Gt => Jump::JGT,
    }
}

fn show_segment(seg: Segment) -> String {
    match seg {
        Segment::This => "THIS".to_string(),
        Segment::That => "THAT".to_string(),
        Segment::Argument => "ARG".to_string(),
        Segment::Local => "LCL".to_string(),
        _ => panic!("Invalid segment shown:"),
    }
}
