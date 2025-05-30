use crate::ast::jack::Type;

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
    Push(Segment, u16),
    Pop(Segment, u16),
}

#[repr(u8)]
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
#[repr(u8)]
#[derive(Clone)]
pub enum ACL {
    Arithmetic(Arithmetic),
    Comparison(Comparison),
    Logical(Logical),
}

#[repr(u8)]
#[derive(Clone)]
pub enum Arithmetic {
    Add,
    Sub,
    Neg,
}

#[repr(u8)]
#[derive(Clone)]
pub enum Comparison {
    Eq,
    Gt,
    Lt,
}

#[repr(u8)]
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
    Body(String, u16),
    Call(String, u16),
    Return,
}

#[derive(Clone)]
pub struct Var {
    pub r#type: Type,
    pub var_kind: VarKind,
    pub index: u16,
}

#[repr(u8)]
#[derive(Clone)]
pub enum VarKind {
    Global(GlobalKind),
    Local(LocalKind),
}

#[repr(u8)]
#[derive(Clone)]
pub enum GlobalKind {
    Static,
    Field,
}

#[repr(u8)]
#[derive(Clone)]
pub enum LocalKind {
    Arg,
    Var,
}
