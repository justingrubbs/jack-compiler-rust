use crate::ast::jack::{Kind, Type};

// Virtual machine commands
#[derive(Clone)]
pub enum Command {
    Stack(Stack),
    ACL(ACL),
    Branch(Branch),
    Function(Function),
}

// Commands that move data
#[derive(Clone)]
pub enum Stack {
    Push(Segment, i16),
    Pop(Segment, i16),
}

#[derive(Clone)]
pub enum Segment {
    Argument,
    Local,
    Static,
    Constant,
    This,
    That,
    Pointer,
    Temp,
}

// Arithemtic-Logical commands
#[derive(Clone)]
pub enum ACL {
    Arithmetic(Arithmetic),
    Comparison(Comparison),
    Logical(Logical),
}

#[derive(Clone)]
pub enum Arithmetic {
    Add,
    Sub,
    Neg,
}

#[derive(Clone)]
pub enum Comparison {
    Eq,
    Gt,
    Lt,
}

#[derive(Clone)]
pub enum Logical {
    And,
    Or,
    Not,
}

// Branch commands:
#[derive(Clone)]
pub enum Branch {
    Label(String),
    Goto(String),
    IfGoto(String),
}

// Function commands:
#[derive(Clone)]
pub enum Function {
    Function(String, i16),
    Call(String, i16),
    Return,
}

#[derive(Clone)]
pub struct Var {
    pub r#type: Type,
    pub var_kind: VarKind,
    pub index: i16,
}

#[derive(Clone)]
pub enum VarKind {
    Global(GlobalKind),
    Local(LocalKind),
}

#[derive(Clone)]
pub enum GlobalKind {
    Static,
    Field,
}

#[derive(Clone)]
pub enum LocalKind {
    Arg,
    Var,
}
