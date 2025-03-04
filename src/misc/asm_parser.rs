use crate::ast::asm::*;

use chumsky::prelude::*;

pub fn parse() -> impl Parser<char, Vec<Assembly>, Error = Simple<char>> {
    parse_comment()
        .repeated()
        .ignore_then(parse_asm())
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

fn parse_asm() -> impl Parser<char, Assembly, Error = Simple<char>> {
    choice((
        parse_a_instruction().map(Assembly::A),
        parse_c_instruction().map(Assembly::C),
        parse_label().map(Assembly::Label),
    ))
}

fn parse_a_instruction() -> impl Parser<char, AInstruction, Error = Simple<char>> {
    just("&").ignore_then(choice((
        parse_num().map(AInstruction::Constant),
        parse_identifier().map(AInstruction::Symbol),
    )))
}

fn parse_c_instruction() -> impl Parser<char, CInstruction, Error = Simple<char>> {
    parse_dest().then_ignore(just("=")).or_not()
    .then(parse_comp())
    .then(just(";").ignore_then(parse_jump()).or_not())
    .map(|((o_dest,comp),o_jump)| CInstruction { o_dest, comp, o_jump })
}

fn parse_dest() -> impl Parser<char, Dest, Error = Simple<char>> {
    choice((
        just("M").to(Dest::M),
        just("D").to(Dest::D),
        just("DM").to(Dest::DM),
        just("A").to(Dest::A),
        just("AM").to(Dest::AM),
        just("AD").to(Dest::AD),
        just("ADM").to(Dest::ADM),
    ))
    .padded()
}

fn parse_comp() -> impl Parser<char, Comp, Error = Simple<char>> {
    just("0").to(Comp::Zero)
        .or(just("1").to(Comp::One))
        .or(just("-1").to(Comp::NegOne))
        .or(just("D").to(Comp::D))
        .or(just("A").to(Comp::A))
        .or(just("!D").to(Comp::NotD))
        .or(just("!A").to(Comp::NotA))
        .or(just("-D").to(Comp::NegD))
        .or(just("-A").to(Comp::NegA))
        .or(just("D+1").to(Comp::DPlusOne))
        .or(just("A+1").to(Comp::APlusOne))
        .or(just("D-1").to(Comp::DMinusOne))
        .or(just("A-1").to(Comp::AMinusOne))
        .or(just("D+A").to(Comp::DPlusA))
        .or(just("D-A").to(Comp::DMinusA))
        .or(just("A-D").to(Comp::AMinusD))
        .or(just("D&A").to(Comp::DAndA))
        .or(just("D|A").to(Comp::DOrA))
        .or(just("M").to(Comp::M))
        .or(just("!M").to(Comp::NotM))
        .or(just("-M").to(Comp::NegM))
        .or(just("M+1").to(Comp::MPlusOne))
        .or(just("M-1").to(Comp::MMinusOne))
        .or(just("D+M").to(Comp::DPlusM))
        .or(just("D-M").to(Comp::DMinusM))
        .or(just("M-D").to(Comp::MMinusD))
        .or(just("D&M").to(Comp::DAndM))
        .or(just("D|M").to(Comp::DOrM))
}


fn parse_jump() -> impl Parser<char, Jump, Error = Simple<char>> {
    choice((
        just("JGT").to(Jump::JGT),
        just("JEQ").to(Jump::JEQ),
        just("JGE").to(Jump::JGE),
        just("JLT").to(Jump::JLT),
        just("JNE").to(Jump::JNE),
        just("JLE").to(Jump::JLE),
        just("JMP").to(Jump::JMP),
    ))
}

fn parse_label() -> impl Parser<char, String, Error = Simple<char>> {
    parse_identifier().delimited_by('(', ')').padded()
}

fn parse_num() -> impl Parser<char, u16, Error = Simple<char>> {
    text::int(10)
        .map(|c: String| c.parse::<u16>().unwrap())
        .padded()
}

fn parse_identifier() -> impl Parser<char, String, Error = Simple<char>> {
    text::ident().map(|s: String| s).padded()
}
