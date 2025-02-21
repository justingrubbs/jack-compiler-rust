use chumsky::prelude::*;


// Figure 10.2 on page 194 of "The Elements of Computing Systems"

#[derive(Debug,Clone)]
pub enum Token {
    Keyword(Keyword),
    Symbol(Symbol),
    Integer(i16),
    String(String),
    Identifier(String),
}

#[derive(Debug,Clone)]
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

#[derive(Debug,Clone)]
pub enum Symbol {
    LBracket,
    RBracket,
    LParens,
    RParens,
    LCurly,
    RCurly,
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
fn lexer() -> impl Parser<char, Vec<Token>, Error = Simple<char>> {
    todo!()
}


