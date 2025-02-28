use crate::ast::vm::*;

// Printing:
pub fn print_vm(commands: Vec<Command>) -> String {
    commands
        .iter()
        .map(|command| format!("{}", print_command(command.clone())))
        .collect::<Vec<String>>()
        .join("\n")
}

fn print_command(command: Command) -> String {
    format!("{}", command.as_str())
}

impl Command {
    fn as_str(&self) -> String {
        match self {
            Command::Stack(p) => p.as_str(),
            Command::ACL(acl) => acl.as_str(),
            // Command::Branch(b) => b.as_str(),
            Command::Function(f) => f.as_str(),
            _ => todo!(),
        }
    }
}

impl Stack {
    fn as_str(&self) -> String {
        match self {
            Stack::Push(s, i) => format!("push {} {}", s.as_str(), i),
            Stack::Pop(s, i) => format!("pop {} {}", s.as_str(), i),
        }
    }
}

impl Segment {
    fn as_str(&self) -> String {
        match self {
            Segment::Argument => "argument".to_string(),
            Segment::Local => "local".to_string(),
            Segment::Static => "static".to_string(),
            Segment::Constant => "constant".to_string(),
            Segment::This => "this".to_string(),
            Segment::That => "that".to_string(),
            Segment::Pointer => "pointer".to_string(),
            Segment::Temp => "temp".to_string(),
        }
    }
}

impl ACL {
    fn as_str(&self) -> String {
        match self {
            ACL::Arithmetic(a) => match a {
                Arithmetic::Add => "add".to_string(),
                Arithmetic::Sub => "sub".to_string(),
                Arithmetic::Neg => "neg".to_string(),
            },
            ACL::Comparison(c) => match c {
                Comparison::Eq => "eq".to_string(),
                Comparison::Lt => "lt".to_string(),
                Comparison::Gt => "gt".to_string(),
            },
            ACL::Logical(l) => match l {
                Logical::And => "and".to_string(),
                Logical::Or => "or".to_string(),
                Logical::Not => "not".to_string(),
            },
        }
    }
}

impl Function {
    fn as_str(&self) -> String {
        match self {
            Function::Function(s, i) => format!("function {} {}", s, i),
            Function::Call(s, i) => format!("call {} {}", s, i),
            Function::Return => "return".to_string(),
        }
    }
}

// impl Keyword {
//     fn as_str(&self) -> &'static str {
//         match self {
//             Keyword::Class => "class",
//             Keyword::Constructor => "constructor",
//             Keyword::Function => "function",
//             Keyword::Method => "method",
//             Keyword::Field => "field",
//             Keyword::Static => "static",
//             Keyword::Var => "var",
//             Keyword::Int => "int",
//             Keyword::Char => "char",
//             Keyword::Boolean => "boolean",
//             Keyword::Void => "void",
//             Keyword::True => "true",
//             Keyword::False => "false",
//             Keyword::Null => "null",
//             Keyword::This => "this",
//             Keyword::Let => "let",
//             Keyword::Do => "do",
//             Keyword::If => "if",
//             Keyword::Else => "else",
//             Keyword::While => "while",
//             Keyword::Return => "return",
//         }
//     }
// }

// impl Symbol {
//     fn as_str(&self) -> &'static str {
//         match self {
//             Symbol::LCurly => "{",
//             Symbol::RCurly => "}",
//             Symbol::LBracket => "[",
//             Symbol::RBracket => "]",
//             Symbol::LParens => "(",
//             Symbol::RParens => ")",
//             Symbol::Period => ".",
//             Symbol::Comma => ",",
//             Symbol::Semicolon => ";",
//             Symbol::Plus => "+",
//             Symbol::Minus => "-",
//             Symbol::Asterisk => "*",
//             Symbol::Slash => "/",
//             Symbol::Ampersand => "&amp;",
//             Symbol::Bar => "|",
//             Symbol::Lesser => "&lt;",
//             Symbol::Greater => "&gt;",
//             Symbol::Equal => "=",
//             Symbol::Tilde => "~",
//         }
//     }
// }

// impl Token {
//     fn as_str(&self) -> String {
//         match self {
//             Token::Keyword(k) => format!("<keyword> {} </keyword>", k.as_str()),
//             Token::Symbol(s) => format!("<symbol> {} </symbol>", s.as_str()),
//             Token::Integer(i) => format!("<integerConstant> {} </integerConstant>", i.to_string()),
//             Token::String(s) => format!("<stringConstant> {} </stringConstant>", s),
//             Token::Identifier(s) => format!("<identifier> {} </identifier>", s),
//         }
//     }
// }
