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


fn parse_class() -> impl Parser<Token, Class, Error = Simple<Token>> {
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
}

fn parse_class_var_dec() -> impl Parser<Token, ClassVarDec, Error = Simple<Token>> {
    todo!()
}

fn parse_subroutine_dec() -> impl Parser<Token, SubroutineDec, Error = Simple<Token>> {
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


