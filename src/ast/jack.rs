
// Program Structure (Figure 10.5):
//      A Jack program is a a collection of classes, each appearing in a separate file.
//      The compilation unit is a class. A class is a sequence of tokens, as follows:

// 'class' [className] '{' [classVarDec*] [subRoutineDec*] '}'
pub struct Class {
    class_name: String,
    class_dec: ClassDec,
}

pub struct ClassDec {
    class_var_dec: Vec<ClassVarDec>,
    subroutine_dec: Vec<SubroutineDec>,
}

// ('static' | 'field') [type] [varName* sepBy ','] ';'
pub struct ClassVarDec {
    class_var_type: ClassVarType,
    r#type: Type,
    vars: Vec<String>,
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

// ('constructor' | 'function' | 'method') ('void' | [type]) [subroutineName] '(' [parameterList] ')' [subroutineBody]
pub struct SubroutineDec {
    subroutine_type: SubroutineType,
    subroutine_return_type: SubroutineReturnType,
    subroutine_name: String,
    parameter_list: Vec<Parameter>,
    subroutine_body: Vec<SubroutineBody>,
}

pub enum SubroutineType {
    Constructor,
    Function,
    Method,
}

pub enum SubroutineReturnType {
    Void,
    Type(Type),
}

pub struct Parameter {
    r#type: Type,
    var_name: String,
}

// '{' [varDec*] [statement*] '}'
pub enum SubroutineBody {
    VarDec(VarDec),
    Statement(Statement),
}

// 'var' [type] [varName] (',' [varName])* ';'
pub struct VarDec{
    r#type: Type,
    var_name: Vec<String>,
}


// Statements:

pub enum Statement {
    LetStatement(LetStatement),
    IfStatement(IfStatement),
    WhileStatement(WhileStatement),
    DoStatement(DoStatement),
    ReturnStatement(ReturnStatement),
}

pub struct LetStatement {
    var_name: String,
    option_expression: Option<Expression>,
    expression: Expression,
}

pub struct IfStatement {
    r#if: Expression,
    then: Vec<Statement>,
    r#else: Option<Vec<Statement>>, 
}

pub struct WhileStatement {
    case: Expression,
    body: Vec<Statement>,
}

pub struct DoStatement {
    call: SubroutineCall,
}

pub struct ReturnStatement {
    r#return: Option<Expression>,
}


// Expressions:

pub enum Expression {
    Expr(Term),
    Bin(Term,BinaryOp,Term),
}

pub enum Term {
    IntegerConstant(i16),
    StringConstant(String),
    KeywordConstant(Keyword),
    VarName(String, Option<Box<Expression>>),
    Expression(Box<Expression>),
    UnaryTerm(UnaryOp, Box<Term>),
    SubroutineCall(SubroutineCall),
}

pub enum SubroutineCall {
    Call(String, Vec<Expression>),
    ClassCall(String, String, Vec<Expression>),
}

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

pub enum UnaryOp {
    Negation,
    Tilde,
}

pub enum Keyword {
    True,
    False,
    Null,
    This,
}

