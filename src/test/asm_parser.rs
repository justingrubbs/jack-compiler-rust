use crate::ast::asm::*;

use chumsky::prelude::*;

pub fn parse_assembly() -> impl Parser<char, Vec<Assembly>, Error = Simple<char>> {
    parse_comment()
        .repeated()
        .ignore_then(parse_asm())
        .repeated()
}

fn parse_comment() -> impl Parser<char, (), Error = Simple<char>> {
    let single_line_comment = just("//")
        .then_ignore(filter(|&c| c != '\n').repeated())
        .padded();

    // let multi_line_comment = just("/*").then_ignore(take_until(just("*/"))).padded();

    // let api_comment = just("/**").then_ignore(take_until(just("*/"))).padded();
    single_line_comment.ignored()
    // choice((single_line_comment, multi_line_comment, api_comment)).ignored()
}

fn parse_asm() -> impl Parser<char, Assembly, Error = Simple<char>> {
    choice((
        parse_a_instruction().map(Assembly::A),
        parse_label().map(Assembly::Label),
    ))
}

fn parse_a_instruction() -> impl Parser<char, AInstruction, Error = Simple<char>> {
    just("@").ignore_then(choice((
        parse_num().map(AInstruction::Constant),
        parse_identifier().map(AInstruction::Symbol),
    )))
}

fn parse_label() -> impl Parser<char, String, Error = Simple<char>> {
    parse_identifier().delimited_by('(', ')')
}

fn parse_num() -> impl Parser<char, u16, Error = Simple<char>> {
    text::int(10)
        .map(|c: String| c.parse::<u16>().unwrap())
        .padded()
}

fn parse_identifier() -> impl Parser<char, String, Error = Simple<char>> {
    filter(|&c: &char| c.is_alphabetic() || c == '_')
        .repeated()
        .collect::<String>()
        .padded()
}
