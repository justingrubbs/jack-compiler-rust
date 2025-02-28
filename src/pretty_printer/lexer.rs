use crate::ast::token::*;

// Printing:
pub fn print_tokens(tokens: Vec<Token>) -> String {
    format!(
        "<tokens>\n{}\n</tokens>\n",
        tokens
            .iter()
            .map(|token| format!("{}", print_token(token.clone())))
            .collect::<Vec<String>>()
            .join("\n")
    )
}

fn print_token(token: Token) -> String {
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
