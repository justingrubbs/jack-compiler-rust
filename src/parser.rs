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


pub fn parse_class() -> impl Parser<Token, Class, Error = Simple<Token>> {
    kw(Keyword::Class)
        .ignore_then(ident())
        .then_ignore(sym(Symbol::LCurly))
        .then(parse_class_dec().repeated())
        .then_ignore(sym(Symbol::RCurly))
        .map(|(class_name,class_dec)| Class { class_name, class_dec })
}

// 
fn parse_class_dec() -> impl Parser<Token, ClassDec, Error = Simple<Token>> {
    parse_class_var_dec().repeated()
        .then(parse_subroutine_dec().repeated())
        .map(|(class_var_dec,subroutine_dec)| ClassDec { class_var_dec, subroutine_dec })
        .labelled("class")
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

// Uncertain whether the delimited_by works as expected
fn parse_subroutine_dec() -> impl Parser<Token, SubroutineDec, Error = Simple<Token>> {
    parse_subroutine_type()
        .then(parse_subroutine_return_type())
        .then(ident())
        .then(parse_parameter_list().delimited_by(Symbol::LParens,Symbol::RParens))
        .then(parse_subroutine_body().delimited_by(Symbol::LCurly,Symbol::RCurly))
        // .map(|())
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
    todo!()
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

