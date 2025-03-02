// Specified in Figure 6.2 on page 107

#[derive(Clone)]
pub enum Assembly {
    A(AInstruction),
    C(CInstruction),
    Label(String),
}

#[derive(Clone)]
pub enum AInstruction {
    Constant(u16),
    Symbol(String),
}

#[derive(Clone)]
pub struct CInstruction {
    pub comp: Comp,
    pub o_dest: Option<Dest>,
    pub o_jump: Option<Jump>,
}

#[repr(u8)]
#[derive(Clone)]
pub enum Comp {
    // a == 0
    Zero,
    One,
    NegOne,
    D,
    A,
    NotD,
    NotA,
    NegD,
    NegA,
    DPlusOne,
    APlusOne,
    DMinusOne,
    AMinusOne,
    DPlusA,
    DMinusA,
    AMinusD,
    DAndA,
    DOrA,

    // a == 1
    M,
    NotM,
    NegM,
    MPlusOne,
    MMinusOne,
    DPlusM,
    DMinusM,
    MMinusD,
    DAndM,
    DOrM,
}

#[repr(u8)]
#[derive(Clone)]
pub enum Dest {
    M,
    D,
    DM,
    A,
    AM,
    AD,
    ADM,
}

#[repr(u8)]
#[derive(Clone)]
pub enum Jump {
    JGT,
    JEQ,
    JGE,
    JLT,
    JNE,
    JLE,
    JMP,
}
