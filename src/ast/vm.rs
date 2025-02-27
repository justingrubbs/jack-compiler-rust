use crate::ast::jack::{Kind, Type};

// Virtual machine commands
pub enum Command {
    Transfer(Transfer),
    ACL(ACL),
    Branch(Branch),
    Function(Function),
}

// Commands that move data
pub enum Transfer {
    Push(Segment, i16),
    Pop(Segment, i16),
}

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
pub enum ACL {
    Arithmetic(Arithmetic),
    Comparison(Comparison),
    Logical(Logical),
}

pub enum Arithmetic {
    Add,
    Sub,
    Neg,
}

pub enum Comparison {
    Eq,
    Gt,
    Lt,
}

pub enum Logical {
    And,
    Or,
    Not,
}

// Branch commands:
pub enum Branch {
    Label(String),
    Goto(String),
    IfGoto(String),
}

// Function commands:
pub enum Function {
    Function(String, i16),
    Call(String, i16),
    Return,
}

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
