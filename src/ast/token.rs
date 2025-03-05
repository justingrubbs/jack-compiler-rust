// Figure 10.2 on page 194 of "The Elements of Computing Systems"

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Token {
    Keyword(Keyword),
    Symbol(Symbol),
    Integer(i16),
    String(String), // sequence of characters not including double quotes or newlines
    Identifier(String), // sequence of letters, digits, and underscore, not starting with digit
}

#[repr(u8)]
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

#[repr(u8)]
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
    At,
}
