use std::fmt;

use crate::lexer::*;
use crate::ast::jack::*;
use chumsky::prelude::*;


// Token helper functions:
fn kw(expected: Keyword) -> impl Parser<Token, (), Error = Simple<Token>> {
    just(Token::Keyword(expected)).ignored()
}

fn sym(expected: Symbol) -> impl Parser<Token, (), Error = Simple<Token>> {
    just(Token::Symbol(expected)).ignored()
}

fn ident() -> impl Parser<Token, String, Error = Simple<Token>> {
    select! {
        Token::Identifier(s) => s,
    }
}

fn int_const() -> impl Parser<Token, i16, Error = Simple<Token>> {
    select! {
        Token::Integer(i) => i,
    }
}

fn string_const() -> impl Parser<Token, String, Error = Simple<Token>> {
    select! {
        Token::String(s) => s,
    }
}

// Program structure:
pub fn parse_class() -> impl Parser<Token, Class, Error = Simple<Token>> {
    kw(Keyword::Class)
    .ignore_then(ident())
    .then_ignore(sym(Symbol::LCurly))
    .then(parse_class_dec().repeated())
    .then_ignore(sym(Symbol::RCurly))
    .map(|(class_name,class_dec)| Class { class_name, class_dec })
    .labelled("class")
}

fn parse_class_dec() -> impl Parser<Token, ClassDec, Error = Simple<Token>> {
    parse_class_var_dec().repeated()
    .then(parse_subroutine_dec().repeated())
    .map(|(class_var_dec,subroutine_dec)| ClassDec { class_var_dec, subroutine_dec })
    .labelled("class declaration")
}

fn parse_class_var_dec() -> impl Parser<Token, ClassVarDec, Error = Simple<Token>> {
    parse_class_var_type()
    .then(parse_type())
    .then(ident().separated_by(sym(Symbol::Comma)))
    .then_ignore(sym(Symbol::Semicolon))
    .map(|((class_var_type,r#type),vars)| ClassVarDec { class_var_type, r#type, vars })
    .labelled("class variable declaration")
}

fn parse_class_var_type() -> impl Parser<Token, ClassVarType, Error = Simple<Token>> {
    choice((
        kw(Keyword::Static).to(ClassVarType::Static),
        kw(Keyword::Field).to(ClassVarType::Field),
    )).labelled("class variable type")
}

fn parse_subroutine_dec() -> impl Parser<Token, SubroutineDec, Error = Simple<Token>> {
    parse_subroutine_type()
    .then(parse_subroutine_return_type())
    .then(ident())
    .then_ignore(sym(Symbol::LParens)).then(parse_parameter_list()).then_ignore(sym(Symbol::RParens))
    .then(parse_subroutine_body())
    .map(|((((subroutine_type,subroutine_return_type),subroutine_name),parameter_list),subroutine_body)| 
        SubroutineDec {subroutine_type,subroutine_return_type,subroutine_name,parameter_list,subroutine_body})
    .labelled("subroutine declaration")
}

fn parse_subroutine_type() -> impl Parser<Token, SubroutineType, Error = Simple<Token>> {
    choice((
        kw(Keyword::Constructor).to(SubroutineType::Constructor),
        kw(Keyword::Function).to(SubroutineType::Function),
        kw(Keyword::Method).to(SubroutineType::Method),
    )).labelled("subroutine type")
}

fn parse_subroutine_return_type() -> impl Parser<Token, SubroutineReturnType, Error = Simple<Token>> {
    choice((
        kw(Keyword::Void).to(SubroutineReturnType::Void),
        parse_type().map(|t| SubroutineReturnType::Type(t)),
    )).labelled("subroutine return type")
}

fn parse_parameter_list() -> impl Parser<Token, Vec<Parameter>, Error = Simple<Token>> {
    (parse_type()
        .then(ident())
        .map(|(r#type,var_name)| Parameter { r#type, var_name }))
        .separated_by(sym(Symbol::Comma)
    ).labelled("parameter list")
}

fn parse_subroutine_body() -> impl Parser<Token, SubroutineBody, Error = Simple<Token>> {
    sym(Symbol::LCurly)
    .ignore_then(parse_var_dec().repeated())
    .then(parse_statement().repeated())
    .then_ignore(sym(Symbol::RCurly))
    .map(|(var_decs,stmts)| SubroutineBody { var_decs, stmts })
    .labelled("subroutine body")
}

fn parse_var_dec() -> impl Parser<Token, VarDec, Error = Simple<Token>> {
    kw(Keyword::Var)
    .ignore_then(parse_type())
    .then(ident().separated_by(sym(Symbol::Comma)))
    .then_ignore(sym(Symbol::Semicolon))
    .map(|(r#type,var_name)| VarDec { r#type, var_name })
    .labelled("variable declaration")
}


// Statements:
fn parse_statement() -> impl Parser<Token, Statement, Error = Simple<Token>> {
    recursive(|statement| {
        choice((
            parse_let_statement().map(Statement::LetStatement),
            parse_if_statement(statement.clone()).map(Statement::IfStatement),
            parse_while_statement(statement).map(Statement::WhileStatement),
            parse_do_statement().map(Statement::DoStatement),
            parse_return_statement().map(Statement::ReturnStatement),
        )).labelled("statement")
    })
}

fn parse_let_statement() -> impl Parser<Token, LetStatement, Error = Simple<Token>> {
    kw(Keyword::Let)
    .ignore_then(ident())
    .then(
        sym(Symbol::LBracket)
        .ignore_then(parse_expression())
        .then_ignore(sym(Symbol::RBracket))
        .or_not()
    )
    .then_ignore(sym(Symbol::Equal))
    .then(parse_expression())
    .then_ignore(sym(Symbol::Semicolon))
    .map(|((var_name, option_expression), expression)| LetStatement {var_name, option_expression, expression})
    .labelled("let statement")
}

fn parse_if_statement<P: Parser<Token, Statement, Error = Simple<Token>> + Clone>(
    statement: P,
) -> impl Parser<Token, IfStatement, Error = Simple<Token>> {
    kw(Keyword::If)
    .ignore_then(
        sym(Symbol::LParens)
            .ignore_then(parse_expression())
            .then_ignore(sym(Symbol::RParens))
    )
    .then(
        sym(Symbol::LCurly)
            .ignore_then(statement.clone().repeated())
            .then_ignore(sym(Symbol::RCurly))
    )
    .then(
        kw(Keyword::Else)
            .ignore_then(
                sym(Symbol::LCurly)
                    .ignore_then(statement.clone().repeated())
                    .then_ignore(sym(Symbol::RCurly))
            )
            .or_not()
    )
    .map(|((cond, then), else_opt)| IfStatement { r#if: cond, then, r#else: else_opt })
    .labelled("if statement")
}

fn parse_while_statement(statement: impl Parser<Token, Statement, Error = Simple<Token>>) 
    -> impl Parser<Token, WhileStatement, Error = Simple<Token>> {
    kw(Keyword::While)
    .ignore_then(
        sym(Symbol::LParens)
        .ignore_then(parse_expression())
        .then_ignore(sym(Symbol::RParens))
    ).then(
        sym(Symbol::LCurly)
        .ignore_then(statement.repeated())
        .then_ignore(sym(Symbol::RCurly))
    ).map(|(case, body)| WhileStatement { case, body })
    .labelled("while statement")
}

fn parse_do_statement() -> impl Parser<Token, DoStatement, Error = Simple<Token>> {
    kw(Keyword::Do)
    .ignore_then(parse_subroutine_call())
    .then_ignore(sym(Symbol::Semicolon))
    .map(|call| DoStatement { call } )
    .labelled("do statement")
}

fn parse_return_statement() -> impl Parser<Token, ReturnStatement, Error = Simple<Token>> {
    kw(Keyword::Return)
    .ignore_then(parse_expression()).or_not()
    .then_ignore(sym(Symbol::Semicolon))
    .map(|r#return| ReturnStatement { r#return })
    .labelled("return statement")
}


// Expressions:
fn parse_expression() -> impl Parser<Token, Expression, Error = Simple<Token>> {
    parse_bin_expression()
    .or(parse_term().map(|t| Expression::Expr(t)))
    .labelled("expression")
}

fn parse_bin_expression() -> impl Parser<Token, Expression, Error = Simple<Token>> {
    parse_term()
    .then(parse_binary_op())
    .then(parse_term())
    .map(|((t1,bop),t2)| Expression::Bin(t1,bop,t2))
    .labelled("binary expression")
}

fn parse_binary_op() -> impl Parser<Token, BinaryOp, Error = Simple<Token>> {
    choice((
        sym(Symbol::Plus).to(BinaryOp::Plus),
        sym(Symbol::Minus).to(BinaryOp::Minus),
        sym(Symbol::Asterisk).to(BinaryOp::Times),
        sym(Symbol::Slash).to(BinaryOp::Div),
        sym(Symbol::Ampersand).to(BinaryOp::And),
        sym(Symbol::Bar).to(BinaryOp::Or),
        sym(Symbol::Lesser).to(BinaryOp::Lesser),
        sym(Symbol::Greater).to(BinaryOp::Greater),
        sym(Symbol::Equal).to(BinaryOp::Equal),
    ))
}

fn parse_unary_op() -> impl Parser<Token, UnaryOp, Error = Simple<Token>> {
    choice((
        sym(Symbol::Minus).to(UnaryOp::Negation),
        sym(Symbol::Tilde).to(UnaryOp::Tilde),
    ))
}

fn parse_term() -> impl Parser<Token, Term, Error = Simple<Token>> {
    recursive(|term| {
        // subroutine might need to be in .or with VarName
        choice((
            int_const().map(Term::IntegerConstant),
            string_const().map(Term::StringConstant),
            parse_keyword_constant().map(Term::KeywordConstant),
            ident().then(
                (sym(Symbol::LBracket)
                    .ignore_then(parse_expression().map(Box::new))
                    .then_ignore(sym(Symbol::RBracket)))
                    .or_not()
                )
                .map(|(s,oe)| Term::VarName(s,oe)),
            sym(Symbol::LParens)
                .ignore_then(parse_expression().map(Box::new))
                .then_ignore(sym(Symbol::RParens))
                .map(Term::Expression),
            parse_unary_op()
                .then(term)
                .map(|(uop,t)| Term::UnaryTerm(uop,Box::new(t))),
            parse_subroutine_call().map(Term::SubroutineCall),
        )).labelled("term")
    })
}

fn parse_keyword_constant() -> impl Parser<Token, KeywordConstant, Error = Simple<Token>> {
    choice((
        kw(Keyword::True).to(KeywordConstant::True),
        kw(Keyword::False).to(KeywordConstant::False),
        kw(Keyword::Null).to(KeywordConstant::Null),
        kw(Keyword::This).to(KeywordConstant::This),
    )).labelled("keyword constant")
}

fn parse_subroutine_call() -> impl Parser<Token, SubroutineCall, Error = Simple<Token>> {
    ident()
        .then_ignore(sym(Symbol::Period))
        .then(ident())
        .then(parse_expression_list())
        .map(|((c,s),es)| SubroutineCall::ClassCall(c,s,es))
    .or(ident()
        .then(parse_expression_list())
        .map(|(s,es)| SubroutineCall::Call(s,es))
    ).labelled("subroutine call")
}

fn parse_expression_list() -> impl Parser<Token, Vec<Expression>, Error = Simple<Token>> {
    parse_expression().separated_by(sym(Symbol::Comma))
        .labelled("expression(s)")
}

fn parse_type() -> impl Parser<Token, Type, Error = Simple<Token>> {
    choice((
        kw(Keyword::Int).to(Type::Int),
        kw(Keyword::Boolean).to(Type::Boolean),
        kw(Keyword::Char).to(Type::Char),
        ident().map(|s| Type::ClassName(s))
    )).labelled("type")
}


// impl fmt::Display for Class {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "class {} {{\n{}\n}}", self.class_name, self.class_dec)
//     }
// }

// // Pretty-printing:
// pub fn print_class(class: Class) -> Vec<String> {
//     format!("{}",class)
// }

