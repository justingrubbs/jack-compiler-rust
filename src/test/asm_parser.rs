use crate::ast::asm::*;

use chumsky::prelude::*;

// This file is not intended to be a full-fledged assembly parser,
// as it is specifically tailored to work on the tests/assembler/ test files.
pub fn parse_assembly() -> impl Parser<char, Vec<Assembly>, Error = Simple<char>> {
    parse_comment()
        .repeated()
        .ignore_then(parse_asm())
        .repeated()
}

fn parse_comment() -> impl Parser<char, (), Error = Simple<char>> {
    just("//")
        .then_ignore(filter(|&c| c != '\n').repeated())
        .padded()
        .ignored()
}

fn parse_asm() -> impl Parser<char, Assembly, Error = Simple<char>> {
    choice((
        parse_a_instruction().map(Assembly::A),
        parse_label().map(Assembly::Label),
        parse_c_instruction().map(Assembly::C),
    ))
}

fn parse_a_instruction() -> impl Parser<char, AInstruction, Error = Simple<char>> {
    just("@")
        .ignore_then(choice((
            parse_num().map(AInstruction::Constant),
            parse_identifier().map(AInstruction::Symbol),
        )))
        .padded()
}

fn parse_c_instruction() -> impl Parser<char, CInstruction, Error = Simple<char>> {
    parse_dest()
        .then_ignore(just("="))
        .or_not()
        .then(parse_comp())
        .then(just(";").ignore_then(parse_jump()).or_not())
        .map(|((o_dest, comp), o_jump)| CInstruction {
            o_dest,
            comp,
            o_jump,
        })
}

fn parse_comp() -> impl Parser<char, Comp, Error = Simple<char>> {
    let c1 = choice((
        just("0").to(Comp::Zero),
        just("1").to(Comp::One),
        just("-1").to(Comp::NegOne),
        just("!D").to(Comp::NotD),
        just("!A").to(Comp::NotA),
        just("-D").to(Comp::NegD),
        just("-A").to(Comp::NegA),
        just("D+1").to(Comp::DPlusOne),
        just("A+1").to(Comp::APlusOne),
        just("D-1").to(Comp::DMinusOne),
        just("A-1").to(Comp::AMinusOne),
        just("D+A").to(Comp::DPlusA),
        just("D-A").to(Comp::DMinusA),
        just("A-D").to(Comp::AMinusD),
        just("D&A").to(Comp::DAndA),
        just("D|A").to(Comp::DOrA),
    ));
    let c2 = choice((
        just("!M").to(Comp::NotM),
        just("-M").to(Comp::NegM),
        just("M+1").to(Comp::MPlusOne),
        just("M-1").to(Comp::MMinusOne),
        just("D+M").to(Comp::DPlusM),
        just("D-M").to(Comp::DMinusM),
        just("M-D").to(Comp::MMinusD),
        just("D&M").to(Comp::DAndM),
        just("D|M").to(Comp::DOrM),
        just("D").to(Comp::D),
        just("A").to(Comp::A),
        just("M").to(Comp::M),
    ));
    choice((c1, c2)).padded()
}

fn parse_dest() -> impl Parser<char, Dest, Error = Simple<char>> {
    choice((
        just("ADM").to(Dest::ADM),
        just("DM").to(Dest::DM),
        just("MD").to(Dest::MD),
        just("AM").to(Dest::AM),
        just("AD").to(Dest::AD),
        just("M").to(Dest::M),
        just("D").to(Dest::D),
        just("A").to(Dest::A),
    ))
    .padded()
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
    .padded()
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
    let starting_char = filter(|&c: &char| c.is_alphabetic() || c == '_');
    let next_char = filter(|&c: &char| c.is_alphanumeric() || c == '_' || c == '.' || c == '$');

    starting_char
        .chain(next_char.repeated())
        .collect::<String>()
        .padded()
}
