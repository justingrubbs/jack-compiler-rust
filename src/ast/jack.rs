// Program Structure:
//      A Jack program is a a collection of classes, each appearing in a separate file.
//      The compilation unit is a class. A class is a sequence of tokens, as follows:

pub struct Class {
    // 'class' [className] '{' [classVarDec*] [subRoutineDec*] '}'
    class_name: String,
    class_dec: ClassDec,
}

pub struct ClassDec {
    class_var_dec: Vec<ClassVarDec>,
    // subroutine_dec: Vec<SubroutineDec>,
}

pub struct ClassVarDec {
    class_var_type: ClassVarType,
    r#type: Type,
    
}

pub enum ClassVarType {
    Static,
    Field,
}

pub enum Type {
    Int,
    Char,
    Boolean,
    ClassName(String),
}

// pub enum 






