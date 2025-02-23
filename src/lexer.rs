use chumsky::prelude::*;
use std::fmt;

// Figure 10.2 on page 194 of "The Elements of Computing Systems"

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Token {
    Keyword(Keyword),
    Symbol(Symbol),
    Integer(i16),
    String(String), // sequence of characters not including double quotes or newlines
    Identifier(String), // sequence of letters, digits, and underscore, not starting with digit
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Keyword {
    Class,
    Constructor,
    Function,
    Method,
    Field,
    Static,
    Var,
    Int,
    Char,
    Boolean,
    Void,
    True,
    False,
    Null,
    This,
    Let,
    Do,
    If,
    Else,
    While,
    Return,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Symbol {
    LCurly,
    RCurly,
    LBracket,
    RBracket,
    LParens,
    RParens,
    Period,
    Comma,
    Semicolon,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Ampersand,
    Bar,
    Lesser,
    Greater,
    Equal,
    Tilde,
}

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

// Printing:
pub fn print_token(token: Token) -> String {
    format!("{}", token.as_str())
}

impl Keyword {
    fn as_str(&self) -> &'static str {
        match self {
            Keyword::Class => "class",
            Keyword::Constructor => "constructor",
            Keyword::Function => "function",
            Keyword::Method => "method",
            Keyword::Field => "field",
            Keyword::Static => "static",
            Keyword::Var => "var",
            Keyword::Int => "int",
            Keyword::Char => "char",
            Keyword::Boolean => "boolean",
            Keyword::Void => "void",
            Keyword::True => "true",
            Keyword::False => "false",
            Keyword::Null => "null",
            Keyword::This => "this",
            Keyword::Let => "let",
            Keyword::Do => "do",
            Keyword::If => "if",
            Keyword::Else => "else",
            Keyword::While => "while",
            Keyword::Return => "return",
        }
    }
}

impl Symbol {
    fn as_str(&self) -> &'static str {
        match self {
            Symbol::LCurly => "{",
            Symbol::RCurly => "}",
            Symbol::LBracket => "[",
            Symbol::RBracket => "]",
            Symbol::LParens => "(",
            Symbol::RParens => ")",
            Symbol::Period => ".",
            Symbol::Comma => ",",
            Symbol::Semicolon => ";",
            Symbol::Plus => "+",
            Symbol::Minus => "-",
            Symbol::Asterisk => "*",
            Symbol::Slash => "/",
            Symbol::Ampersand => "&amp;",
            Symbol::Bar => "|",
            Symbol::Lesser => "&lt;",
            Symbol::Greater => "&gt;",
            Symbol::Equal => "=",
            Symbol::Tilde => "~",
        }
    }
}

impl Token {
    fn as_str(&self) -> String {
        match self {
            Token::Keyword(k) => format!("<keyword> {} </keyword>", k.as_str()),
            Token::Symbol(s) => format!("<symbol> {} </symbol>", s.as_str()),
            Token::Integer(i) => format!("<integerConstant> {} </integerConstant>", i.to_string()),
            Token::String(s) => format!("<stringConstant> {} </stringConstant>", s),
            Token::Identifier(s) => format!("<identifier> {} </identifier>", s),
        }
    }
}
