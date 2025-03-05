use crate::ast::vm::*;

use chumsky::prelude::*;

// This file is not intended to be a full-fledged VM parser,
// as it is specifically tailored to work on the tests/vm_to_asm/ test files.
pub fn parse_vm() -> impl Parser<char, Vec<Command>, Error = Simple<char>> {
    parse_comment()
        .repeated()
        .ignore_then(parse_command())
        .repeated()
}

fn parse_comment() -> impl Parser<char, (), Error = Simple<char>> {
    just("//")
        .then_ignore(filter(|&c| c != '\n').repeated())
        .padded()
        .ignored()
}

fn parse_command() -> impl Parser<char, Command, Error = Simple<char>> {
    choice((
        parse_stack().map(Command::Stack),
        parse_acl().map(Command::ACL),
        parse_function().map(Command::Function),
        parse_branch().map(Command::Branch),
    ))
}

fn parse_stack() -> impl Parser<char, Stack, Error = Simple<char>> {
    choice((
        just("push")
            .padded()
            .ignore_then(parse_segment())
            .then(parse_num())
            .map(|(s, i)| Stack::Push(s, i)),
        just("pop")
            .padded()
            .ignore_then(parse_segment())
            .then(parse_num())
            .map(|(s, i)| Stack::Pop(s, i)),
    ))
    .padded()
}

fn parse_segment() -> impl Parser<char, Segment, Error = Simple<char>> {
    choice((
        just("argument").to(Segment::Argument),
        just("local").to(Segment::Local),
        just("static").to(Segment::Static),
        just("constant").to(Segment::Constant),
        just("this").to(Segment::This),
        just("that").to(Segment::That),
        just("pointer").to(Segment::Pointer),
        just("temp").to(Segment::Temp),
    ))
    .padded()
}

fn parse_acl() -> impl Parser<char, ACL, Error = Simple<char>> {
    choice((
        just("add").to(ACL::Arithmetic(Arithmetic::Add)),
        just("sub").to(ACL::Arithmetic(Arithmetic::Sub)),
        just("neg").to(ACL::Arithmetic(Arithmetic::Neg)),
        just("eq").to(ACL::Comparison(Comparison::Eq)),
        just("lt").to(ACL::Comparison(Comparison::Lt)),
        just("gt").to(ACL::Comparison(Comparison::Gt)),
        just("and").to(ACL::Logical(Logical::And)),
        just("or").to(ACL::Logical(Logical::Or)),
        just("not").to(ACL::Logical(Logical::Not)),
    ))
    .padded()
}

fn parse_function() -> impl Parser<char, Function, Error = Simple<char>> {
    choice((
        just("function")
            .padded()
            .ignore_then(parse_identifier())
            .then(parse_num())
            .map(|(s, i)| Function::Function(s, i)),
        just("call")
            .padded()
            .ignore_then(parse_identifier())
            .then(parse_num())
            .map(|(s, i)| Function::Call(s, i)),
        just("return").padded().to(Function::Return),
    ))
    .padded()
}

fn parse_branch() -> impl Parser<char, Branch, Error = Simple<char>> {
    choice((
        just("label")
            .padded()
            .ignore_then(parse_identifier())
            .map(Branch::Label),
        just("if-goto")
            .padded()
            .ignore_then(parse_identifier())
            .map(Branch::IfGoto),
        just("goto")
            .padded()
            .ignore_then(parse_identifier())
            .map(Branch::Goto),
    ))
    .padded()
}

fn parse_num() -> impl Parser<char, u16, Error = Simple<char>> {
    text::int(10)
        .map(|c: String| c.parse::<u16>().unwrap())
        .padded()
}

// Refine as needed
fn parse_identifier() -> impl Parser<char, String, Error = Simple<char>> {
    let starting_char = filter(|&c: &char| c.is_alphabetic() || c == '_');
    let next_char = filter(|&c: &char| c.is_alphanumeric() || c == '_' || c == '.' || c == '$');

    starting_char
        .chain(next_char.repeated())
        .collect::<String>()
        .padded()
}
