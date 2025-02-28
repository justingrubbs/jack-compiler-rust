use crate::ast::token::*;

use chumsky::prelude::*;

// Lexer:
pub fn tokenize() -> impl Parser<char, Vec<Token>, Error = Simple<char>> {
    parse_comment()
        .repeated()
        .ignore_then(parse_token())
        .repeated()
}

fn parse_comment() -> impl Parser<char, (), Error = Simple<char>> {
    let single_line_comment = just("//")
        .then_ignore(filter(|&c| c != '\n').repeated())
        .padded();

    let multi_line_comment = just("/*").then_ignore(take_until(just("*/"))).padded();

    let api_comment = just("/**").then_ignore(take_until(just("*/"))).padded();

    choice((single_line_comment, multi_line_comment, api_comment)).ignored()
}

fn parse_token() -> impl Parser<char, Token, Error = Simple<char>> {
    choice((
        parse_keyword().map(Token::Keyword),
        parse_symbol().map(Token::Symbol),
        parse_num().map(Token::Integer),
        parse_string().map(Token::String),
        parse_identifier().map(Token::Identifier),
    ))
}

fn parse_keyword() -> impl Parser<char, Keyword, Error = Simple<char>> {
    choice((
        just("class").to(Keyword::Class),
        just("constructor").to(Keyword::Constructor),
        just("function").to(Keyword::Function),
        just("method").to(Keyword::Method),
        just("field").to(Keyword::Field),
        just("static").to(Keyword::Static),
        just("var").to(Keyword::Var),
        just("int").to(Keyword::Int),
        just("char").to(Keyword::Char),
        just("boolean").to(Keyword::Boolean),
        just("void").to(Keyword::Void),
        just("true").to(Keyword::True),
        just("false").to(Keyword::False),
        just("null").to(Keyword::Null),
        just("this").to(Keyword::This),
        just("let").to(Keyword::Let),
        just("do").to(Keyword::Do),
        just("if").to(Keyword::If),
        just("else").to(Keyword::Else),
        just("while").to(Keyword::While),
        just("return").to(Keyword::Return),
    ))
    .padded()
}

fn parse_symbol() -> impl Parser<char, Symbol, Error = Simple<char>> {
    choice((
        just('{').to(Symbol::LCurly),
        just('}').to(Symbol::RCurly),
        just('[').to(Symbol::LBracket),
        just(']').to(Symbol::RBracket),
        just('(').to(Symbol::LParens),
        just(')').to(Symbol::RParens),
        just('.').to(Symbol::Period),
        just(',').to(Symbol::Comma),
        just(';').to(Symbol::Semicolon),
        just('+').to(Symbol::Plus),
        just('-').to(Symbol::Minus),
        just('*').to(Symbol::Asterisk),
        just('/').to(Symbol::Slash),
        just('&').to(Symbol::Ampersand),
        just('|').to(Symbol::Bar),
        just('<').to(Symbol::Lesser),
        just('>').to(Symbol::Greater),
        just('=').to(Symbol::Equal),
        just('~').to(Symbol::Tilde),
    ))
    .padded()
}

fn parse_num() -> impl Parser<char, i16, Error = Simple<char>> {
    text::int(10)
        .map(|c: String| c.parse::<i16>().unwrap())
        .padded()
}

fn parse_string() -> impl Parser<char, String, Error = Simple<char>> {
    let valid_char = filter(|&c: &char| c != '"' && c != '\n');
    valid_char
        .repeated()
        .delimited_by('"', '"')
        .collect::<String>()
        .padded()
}

fn parse_identifier() -> impl Parser<char, String, Error = Simple<char>> {
    let starting_char = filter(|&c: &char| c.is_alphabetic() || c == '_');
    let follow_char = filter(|&c: &char| c.is_alphanumeric() || c == '_');
    starting_char
        .chain(follow_char.repeated())
        .collect::<String>()
        .padded()
}
