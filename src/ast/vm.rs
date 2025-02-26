
// Virtual machine commands
pub enum Command {
    Transfer(Transfer),
    ACL,
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

// Branch and function commands:

