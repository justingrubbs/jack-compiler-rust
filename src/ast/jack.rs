
// Program Structure (Figure 10.5):
//      A Jack program is a a collection of classes, each appearing in a separate file.
//      The compilation unit is a class. A class is a sequence of tokens, as follows:
#[derive(Debug,Clone)]
pub struct Class {
    pub class_name: String,
    pub class_dec: Vec<ClassDec>,
}

#[derive(Debug,Clone)]
pub struct ClassDec {
    pub class_var_dec: Vec<ClassVarDec>,
    pub subroutine_dec: Vec<SubroutineDec>,
}

#[derive(Debug,Clone)]
pub struct ClassVarDec {
    pub class_var_type: ClassVarType,
    pub r#type: Type,
    pub vars: Vec<String>,
}

#[derive(Debug,Clone)]
pub enum ClassVarType {
    Static,
    Field,
}

#[derive(Debug,Clone)]
pub enum Type {
    Int,
    Char,
    Boolean,
    ClassName(String),
}

#[derive(Debug,Clone)]
pub struct SubroutineDec {
    pub subroutine_type: SubroutineType,
    pub subroutine_return_type: SubroutineReturnType,
    pub subroutine_name: String,
    pub parameter_list: Vec<Parameter>,
    pub subroutine_body: SubroutineBody,
}

#[derive(Debug,Clone)]
pub enum SubroutineType {
    Constructor,
    Function,
    Method,
}

#[derive(Debug,Clone)]
pub enum SubroutineReturnType {
    Void,
    Type(Type),
}

#[derive(Debug,Clone)]
pub struct Parameter {
    pub r#type: Type,
    pub var_name: String,
}

#[derive(Debug,Clone)]
pub struct SubroutineBody {
    var_decs: Vec<VarDec>,
    stmts: Vec<Statement>,
}

#[derive(Debug,Clone)]
pub struct VarDec{
    pub r#type: Type,
    pub var_name: Vec<String>,
}


// Statements:
#[derive(Debug,Clone)]
pub enum Statement {
    LetStatement(LetStatement),
    IfStatement(IfStatement),
    WhileStatement(WhileStatement),
    DoStatement(DoStatement),
    ReturnStatement(ReturnStatement),
}

#[derive(Debug,Clone)]
pub struct LetStatement {
    pub var_name: String,
    pub option_expression: Option<Expression>,
    pub expression: Expression,
}

#[derive(Debug,Clone)]
pub struct IfStatement {
    pub r#if: Expression,
    pub then: Vec<Statement>,
    pub r#else: Option<Vec<Statement>>, 
}

#[derive(Debug,Clone)]
pub struct WhileStatement {
    pub case: Expression,
    pub body: Vec<Statement>,
}

#[derive(Debug,Clone)]
pub struct DoStatement {
    pub call: SubroutineCall,
}

#[derive(Debug,Clone)]
pub struct ReturnStatement {
    pub r#return: Option<Expression>,
}


// Expressions:
#[derive(Debug,Clone)]
pub enum Expression {
    Expr(Term),
    Bin(Term,BinaryOp,Term),
}

#[derive(Debug,Clone)]
pub enum Term {
    IntegerConstant(i16),
    StringConstant(String),
    KeywordConstant(KeywordConstant),
    VarName(String, Option<Box<Expression>>),
    Expression(Box<Expression>),
    UnaryTerm(UnaryOp, Box<Term>),
    SubroutineCall(SubroutineCall),
}

#[derive(Debug,Clone)]
pub enum SubroutineCall {
    Call(String, Vec<Expression>),
    ClassCall(String, String, Vec<Expression>),
}

#[derive(Debug,Clone)]
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

#[derive(Debug,Clone)]
pub enum UnaryOp {
    Negation,
    Tilde,
}

#[derive(Debug,Clone)]
pub enum KeywordConstant {
    True,
    False,
    Null,
    This,
}

