use crate::lexer::*;
use crate::ast::jack::*;
use chumsky::prelude::*;



fn kw(expected: crate::lexer::Keyword) -> impl Parser<Token, (), Error = Simple<Token>> {
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






fn parse_type() -> impl Parser<Token, Type, Error = Simple<Token>> {
    choice((
        kw(crate::lexer::Keyword::Int).to(Type::Int),
        kw(crate::lexer::Keyword::Boolean).to(Type::Boolean),
        kw(crate::lexer::Keyword::Char).to(Type::Char),
        ident().map(|s| Type::ClassName(s))
    ))
}


