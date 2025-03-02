// Program Structure (Figure 10.5):
//      A Jack program is a a collection of classes, each appearing in a separate file.
//      The compilation unit is a class. A class is a sequence of tokens, as follows:
pub struct Class {
    pub class_name: String,
    pub class_dec: ClassDec,
}

pub struct ClassDec {
    pub class_var_dec: Vec<ClassVarDec>,
    pub subroutine_dec: Vec<SubroutineDec>,
}

pub struct ClassVarDec {
    pub kind: Kind,
    pub r#type: Type,
    pub vars: Vec<String>,
}

#[derive(Clone)]
pub enum Kind {
    Static,
    Field,
}

#[derive(Clone)]
pub enum Type {
    Int,
    Char,
    Boolean,
    ClassName(String),
}

pub struct SubroutineDec {
    pub subroutine_type: SubroutineType,
    pub subroutine_return_type: SubroutineReturnType,
    pub subroutine_name: String,
    pub parameter_list: Vec<Parameter>,
    pub subroutine_body: SubroutineBody,
}

#[derive(Clone)]
pub enum SubroutineType {
    Constructor,
    Function,
    Method,
}

#[derive(Clone)]
pub enum SubroutineReturnType {
    Void,
    Type(Type),
}

#[derive(Clone)]
pub struct Parameter {
    pub r#type: Type,
    pub var_name: String,
}

pub struct SubroutineBody {
    pub var_decs: Vec<VarDec>,
    pub stmts: Vec<Statement>,
}

pub struct VarDec {
    pub r#type: Type,
    pub var_name: Vec<String>,
}

// Statements:
#[derive(Clone)]
pub enum Statement {
    LetStatement(String, Option<Expression>, Expression),
    IfStatement(Expression, Vec<Statement>, Option<Vec<Statement>>),
    WhileStatement(Expression, Vec<Statement>),
    DoStatement(SubroutineCall),
    ReturnStatement(Option<Expression>),
}

// Expressions:
#[derive(Clone)]
pub struct Expression {
    pub term: Box<Term>,
    pub bin: Vec<(BinaryOp, Box<Term>)>,
}

#[derive(Clone)]
pub enum Term {
    IntegerConstant(i16),
    StringConstant(String),
    KeywordConstant(KeywordConstant),
    VarName(String, Option<Box<Expression>>),
    UnaryTerm(UnaryOp, Box<Term>),
    ParensExpr(Box<Expression>),
    SubroutineCall(SubroutineCall),
}

#[derive(Clone)]
pub enum SubroutineCall {
    Call(String, Vec<Box<Expression>>),
    ClassCall(String, String, Vec<Box<Expression>>),
}

#[derive(Clone)]
pub enum BinaryOp {
    Plus,
    Minus,
    Times,
    Div,
    And,
    Or,
    Lesser,
    Greater,
    Equal,
}

#[derive(Clone)]
pub enum UnaryOp {
    Negation,
    Tilde,
}

#[derive(Clone)]
pub enum KeywordConstant {
    True,
    False,
    Null,
    This,
}
