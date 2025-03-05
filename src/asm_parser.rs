use crate::ast::asm::*;
use crate::ast::token::*;

use chumsky::prelude::*;

fn kw(expected: Keyword) -> impl Parser<Token, (), Error = Simple<Token>> {
    just(Token::Keyword(expected)).ignored()
}

fn sym(expected: Symbol) -> impl Parser<Token, (), Error = Simple<Token>> {
    just(Token::Symbol(expected)).ignored()
}

fn ident() -> impl Parser<Token, String, Error = Simple<Token>> {
    select! {
        Token::Identifier(s) => s,
    }
}

fn int_const() -> impl Parser<Token, u16, Error = Simple<Token>> {
    select! {
        Token::Integer(i) => i as u16,
    }
}

pub fn parse() -> impl Parser<Token, Vec<Assembly>, Error = Simple<Token>> {
    parse_asm().repeated()
}

fn parse_asm() -> impl Parser<Token, Assembly, Error = Simple<Token>> {
    choice((
        parse_a_instruction().map(Assembly::A),
        // parse_c_instruction().map(Assembly::C),
        // parse_label().map(Assembly::Label),
    ))
}

fn parse_a_instruction() -> impl Parser<Token, AInstruction, Error = Simple<Token>> {
    sym(Symbol::At)
        .ignore_then(choice((
            int_const().map(AInstruction::Constant),
            ident().map(AInstruction::Symbol),
        )))
}

// fn parse_c_instruction() -> impl Parser<Token, CInstruction, Error = Simple<Token>> {

// }

// fn parse_c_instruction() -> impl Parser<char, CInstruction, Error = Simple<char>> {
//     parse_dest().then_ignore(just("=")).or_not()
//     .then(parse_comp())
//     .then(just(";").ignore_then(parse_jump()).or_not())
//     .map(|((o_dest,comp),o_jump)| CInstruction { o_dest, comp, o_jump })
// }

// fn parse_dest() -> impl Parser<Token, Dest, Error = Simple<Token>> {
//     choice((
//         just("M").to(Dest::M),
//         just("D").to(Dest::D),
//         just("DM").to(Dest::DM),
//         just("A").to(Dest::A),
//         just("AM").to(Dest::AM),
//         just("AD").to(Dest::AD),
//         just("ADM").to(Dest::ADM),
//     ))
// }

// fn parse_comp() -> impl Parser<char, Comp, Error = Simple<char>> {
//     just("0").to(Comp::Zero)
//         .or(just("1").to(Comp::One))
//         .or(just("-1").to(Comp::NegOne))
//         .or(just("D").to(Comp::D))
//         .or(just("A").to(Comp::A))
//         .or(just("!D").to(Comp::NotD))
//         .or(just("!A").to(Comp::NotA))
//         .or(just("-D").to(Comp::NegD))
//         .or(just("-A").to(Comp::NegA))
//         .or(just("D+1").to(Comp::DPlusOne))
//         .or(just("A+1").to(Comp::APlusOne))
//         .or(just("D-1").to(Comp::DMinusOne))
//         .or(just("A-1").to(Comp::AMinusOne))
//         .or(just("D+A").to(Comp::DPlusA))
//         .or(just("D-A").to(Comp::DMinusA))
//         .or(just("A-D").to(Comp::AMinusD))
//         .or(just("D&A").to(Comp::DAndA))
//         .or(just("D|A").to(Comp::DOrA))
//         .or(just("M").to(Comp::M))
//         .or(just("!M").to(Comp::NotM))
//         .or(just("-M").to(Comp::NegM))
//         .or(just("M+1").to(Comp::MPlusOne))
//         .or(just("M-1").to(Comp::MMinusOne))
//         .or(just("D+M").to(Comp::DPlusM))
//         .or(just("D-M").to(Comp::DMinusM))
//         .or(just("M-D").to(Comp::MMinusD))
//         .or(just("D&M").to(Comp::DAndM))
//         .or(just("D|M").to(Comp::DOrM))
// }

// fn parse_jump() -> impl Parser<char, Jump, Error = Simple<char>> {
//     choice((
//         just("JGT").to(Jump::JGT),
//         just("JEQ").to(Jump::JEQ),
//         just("JGE").to(Jump::JGE),
//         just("JLT").to(Jump::JLT),
//         just("JNE").to(Jump::JNE),
//         just("JLE").to(Jump::JLE),
//         just("JMP").to(Jump::JMP),
//     ))
// }

// fn parse_label() -> impl Parser<char, String, Error = Simple<char>> {
//     parse_identifier().delimited_by('(', ')').padded()
// }

// fn parse_num() -> impl Parser<char, u16, Error = Simple<char>> {
//     text::int(10)
//         .map(|c: String| c.parse::<u16>().unwrap())
//         .padded()
// }

// fn parse_identifier() -> impl Parser<char, String, Error = Simple<char>> {
//     text::ident().map(|s: String| s).padded()
// }
