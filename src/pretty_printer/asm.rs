use crate::ast::asm::*;

pub fn print_asm(assembly: Vec<Assembly>) -> String {
    assembly
        .iter()
        .map(|instruction| print_instruction(instruction.clone()))
        .collect::<Vec<String>>()
        .join("\n")
        + "\n"
}

fn print_instruction(instruction: Assembly) -> String {
    format!("{}", instruction.as_str())
}

impl Assembly {
    fn as_str(&self) -> String {
        match self {
            Assembly::A(a) => format!("@{}", a.as_str()),
            Assembly::C(c) => c.as_str(),
            Assembly::Label(s) => format!("({})", s),
        }
    }
}

impl AInstruction {
    fn as_str(&self) -> String {
        match self {
            AInstruction::Constant(i) => i.to_string(),
            AInstruction::Symbol(s) => s.to_string(),
        }
    }
}

impl CInstruction {
    fn as_str(&self) -> String {
        match self {
            CInstruction {
                comp,
                o_dest: Some(dest),
                o_jump: None,
            } => format!("{}={}", dest.as_str(), comp.as_str()),
            CInstruction {
                comp,
                o_dest: None,
                o_jump: Some(jump),
            } => format!("{};{}", comp.as_str(), jump.as_str()),
            CInstruction {
                comp,
                o_dest: Some(dest),
                o_jump: Some(jump),
            } => format!("{}={};{}", dest.as_str(), comp.as_str(), jump.as_str()),
            CInstruction {
                comp,
                o_dest: None,
                o_jump: None,
            } => comp.as_str(),
        }
    }
}

impl Comp {
    fn as_str(&self) -> String {
        match self {
            Comp::Zero => "0".to_string(),
            Comp::One => "1".to_string(),
            Comp::NegOne => "-1".to_string(),
            Comp::D => "D".to_string(),
            Comp::A => "A".to_string(),
            Comp::NotD => "!D".to_string(),
            Comp::NotA => "!A".to_string(),
            Comp::NegD => "-D".to_string(),
            Comp::NegA => "-A".to_string(),
            Comp::DPlusOne => "D+1".to_string(),
            Comp::APlusOne => "A+1".to_string(),
            Comp::DMinusOne => "D-1".to_string(),
            Comp::AMinusOne => "A-1".to_string(),
            Comp::DPlusA => "D+A".to_string(),
            Comp::DMinusA => "D-A".to_string(),
            Comp::AMinusD => "A-D".to_string(),
            Comp::DAndA => "D&A".to_string(),
            Comp::DOrA => "D|A".to_string(),

            Comp::M => "M".to_string(),
            Comp::NotM => "!M".to_string(),
            Comp::NegM => "-M".to_string(),
            Comp::MPlusOne => "M+1".to_string(),
            Comp::MMinusOne => "M-1".to_string(),
            Comp::DPlusM => "D+M".to_string(),
            Comp::DMinusM => "D-M".to_string(),
            Comp::MMinusD => "M-D".to_string(),
            Comp::DAndM => "D&M".to_string(),
            Comp::DOrM => "D|M".to_string(),
        }
    }
}

impl Dest {
    fn as_str(&self) -> String {
        match self {
            Dest::M => "M".to_string(),
            Dest::D => "D".to_string(),
            Dest::DM => "DM".to_string(),
            Dest::MD => "MD".to_string(),
            Dest::A => "A".to_string(),
            Dest::AM => "AM".to_string(),
            Dest::AD => "AD".to_string(),
            Dest::ADM => "ADM".to_string(),
        }
    }
}

impl Jump {
    fn as_str(&self) -> String {
        match self {
            Jump::JGT => "JGT".to_string(),
            Jump::JEQ => "JEQ".to_string(),
            Jump::JGE => "JGE".to_string(),
            Jump::JLT => "JLT".to_string(),
            Jump::JNE => "JNE".to_string(),
            Jump::JLE => "JLE".to_string(),
            Jump::JMP => "JMP".to_string(),
        }
    }
}
