use crate::ast::jack::{Kind, Type};

// Virtual machine commands
#[derive(Debug)]
pub enum Command {
    Stack(Stack),
    ACL(ACL),
    Branch(Branch),
    Function(Function),
}

// Commands that move data
#[derive(Debug)]
pub enum Stack {
    Push(Segment, i16),
    Pop(Segment, i16),
}

#[derive(Debug)]
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
#[derive(Debug)]
pub enum ACL {
    Arithmetic(Arithmetic),
    Comparison(Comparison),
    Logical(Logical),
}

#[derive(Debug)]
pub enum Arithmetic {
    Add,
    Sub,
    Neg,
}

#[derive(Debug)]
pub enum Comparison {
    Eq,
    Gt,
    Lt,
}

#[derive(Debug)]
pub enum Logical {
    And,
    Or,
    Not,
}

// Branch commands:
#[derive(Debug)]
pub enum Branch {
    Label(String),
    Goto(String),
    IfGoto(String),
}

// Function commands:
#[derive(Debug)]
pub enum Function {
    Function(String, i16),
    Call(String, i16),
    Return,
}

#[derive(Debug)]
pub struct Var {
    pub r#type: Type,
    pub var_kind: VarKind,
    pub index: i16,
}

#[derive(Debug, Clone)]
pub enum VarKind {
    Global(GlobalKind),
    Local(LocalKind),
}

#[derive(Debug, Clone)]
pub enum GlobalKind {
    Static,
    Field,
}

#[derive(Debug, Clone)]
pub enum LocalKind {
    Arg,
    Var,
}
