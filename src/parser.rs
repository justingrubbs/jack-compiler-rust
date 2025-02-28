use crate::ast::jack::*;
use crate::lexer::*;
use chumsky::prelude::*;

// Token helper functions:
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

fn int_const() -> impl Parser<Token, i16, Error = Simple<Token>> {
    select! {
        Token::Integer(i) => i,
    }
}

fn string_const() -> impl Parser<Token, String, Error = Simple<Token>> {
    select! {
        Token::String(s) => s,
    }
}

// Parser specification notation (section 10.2.1):
// 'xxx'        : Represents lanaguage tokens that appear verbatim
// [xxx]        : Represents names of terminal and nonterminal elements
// ()           : Used for grouping
// [x] | [y]    : Either x or y
// [x] [y]      : x is followed by y
// [x]?         : x appears 0 or 1 times
// [x]*         : x appears 0 or more times

// Program structure:

// parse_class:
//  'class' [class_name] '{' [class_dec] '}'
pub fn parse_class() -> impl Parser<Token, Class, Error = Simple<Token>> {
    kw(Keyword::Class)
        .ignore_then(ident())
        .then_ignore(sym(Symbol::LCurly))
        .then(parse_class_dec())
        .then_ignore(sym(Symbol::RCurly))
        .map(|(class_name, class_dec)| Class {
            class_name,
            class_dec,
        })
        .labelled("class")
}

// parse_class_dec:
//  [class_var_dec]* [subroutine_dec]*
fn parse_class_dec() -> impl Parser<Token, ClassDec, Error = Simple<Token>> {
    parse_class_var_dec()
        .repeated()
        .then(parse_subroutine_dec().repeated())
        .map(|(class_var_dec, subroutine_dec)| ClassDec {
            class_var_dec,
            subroutine_dec,
        })
        .labelled("class declaration")
}

// parse_class_var_dec:
//  [class_var_type] [type] [var_name] (',' [var_name])* ';'
fn parse_class_var_dec() -> impl Parser<Token, ClassVarDec, Error = Simple<Token>> {
    parse_kind()
        .then(parse_type())
        .then(ident().separated_by(sym(Symbol::Comma)))
        .then_ignore(sym(Symbol::Semicolon))
        .map(|((kind, r#type), vars)| ClassVarDec { kind, r#type, vars })
        .labelled("class variable declaration")
}

// parse_kind:
//  ('static' | 'field')
fn parse_kind() -> impl Parser<Token, Kind, Error = Simple<Token>> {
    choice((
        kw(Keyword::Static).to(Kind::Static),
        kw(Keyword::Field).to(Kind::Field),
    ))
    .labelled("class variable type")
}

// parse_subroutine_dec:
//  [subroutine_type] [subroutine_return_type] [subroutine_name] '(' [parameter_list] ')' [subroutine_body]
fn parse_subroutine_dec() -> impl Parser<Token, SubroutineDec, Error = Simple<Token>> {
    parse_subroutine_type()
        .then(parse_subroutine_return_type())
        .then(ident())
        .then_ignore(sym(Symbol::LParens))
        .then(parse_parameter_list())
        .then_ignore(sym(Symbol::RParens))
        .then(parse_subroutine_body())
        .map(
            |(
                (((subroutine_type, subroutine_return_type), subroutine_name), parameter_list),
                subroutine_body,
            )| SubroutineDec {
                subroutine_type,
                subroutine_return_type,
                subroutine_name,
                parameter_list,
                subroutine_body,
            },
        )
        .labelled("subroutine declaration")
}

// parse_subroutine_type:
//  ('constructor' | 'function' | 'method')
fn parse_subroutine_type() -> impl Parser<Token, SubroutineType, Error = Simple<Token>> {
    choice((
        kw(Keyword::Constructor).to(SubroutineType::Constructor),
        kw(Keyword::Function).to(SubroutineType::Function),
        kw(Keyword::Method).to(SubroutineType::Method),
    ))
    .labelled("subroutine type")
}

// parse_subroutine_return_type:
//  ('void' | [type])
fn parse_subroutine_return_type() -> impl Parser<Token, SubroutineReturnType, Error = Simple<Token>>
{
    choice((
        kw(Keyword::Void).to(SubroutineReturnType::Void),
        parse_type().map(|t| SubroutineReturnType::Type(t)),
    ))
    .labelled("subroutine return type")
}

// parse_parameter_list:
//  (([type] ([var_name]) (',' [type] [var_name])*))?
fn parse_parameter_list() -> impl Parser<Token, Option<Vec<Parameter>>, Error = Simple<Token>> {
    (parse_type()
        .then(ident())
        .map(|(r#type, var_name)| Parameter { r#type, var_name }))
    .separated_by(sym(Symbol::Comma))
    .or_not()
    .labelled("parameter list")
}

// parse_subroutine_body:
//  '{' [var_dec]* [statement]* '}'
fn parse_subroutine_body() -> impl Parser<Token, SubroutineBody, Error = Simple<Token>> {
    sym(Symbol::LCurly)
        .ignore_then(parse_var_dec().repeated())
        .then(parse_statement().repeated())
        .then_ignore(sym(Symbol::RCurly))
        .map(|(var_decs, stmts)| SubroutineBody { var_decs, stmts })
        .labelled("subroutine body")
}

// parse_var_dec:
//  'var' [type] ([var_name] (','  [var_name])*) ';'
fn parse_var_dec() -> impl Parser<Token, VarDec, Error = Simple<Token>> {
    kw(Keyword::Var)
        .ignore_then(parse_type())
        .then(ident().separated_by(sym(Symbol::Comma)))
        .then_ignore(sym(Symbol::Semicolon))
        .map(|(r#type, var_name)| VarDec { r#type, var_name })
        .labelled("variable declaration")
}

// Statements:

// parse_statement:
//  [let_statement] | [if_statement] | [while_statement] | [do_statement] | [return_statement]
fn parse_statement() -> impl Parser<Token, Statement, Error = Simple<Token>> {
    recursive(|statement| {
        choice((
            parse_let_statement(),
            parse_if_statement(statement.clone()),
            parse_while_statement(statement),
            parse_do_statement(),
            parse_return_statement(),
        ))
        .labelled("statement")
    })
}

// parse_let_statement:
//  'let' [var_name] ('[' [expression] ']')? '=' [expression] ';'
fn parse_let_statement() -> impl Parser<Token, Statement, Error = Simple<Token>> {
    kw(Keyword::Let)
        .ignore_then(ident())
        .then(
            sym(Symbol::LBracket)
                .ignore_then(parse_expression())
                .then_ignore(sym(Symbol::RBracket))
                .or_not(),
        )
        .then_ignore(sym(Symbol::Equal))
        .then(parse_expression())
        .then_ignore(sym(Symbol::Semicolon))
        .map(|((var_name, option_expression), expression)| {
            Statement::LetStatement(var_name, option_expression, expression)
        })
        .labelled("let statement")
}

// parse_if_statement:
//  'if' '(' [expression] ')' '{' [statement]* '}' ('else' '{' [statement]* '}')?
fn parse_if_statement<P: Parser<Token, Statement, Error = Simple<Token>> + Clone>(
    statement: P,
) -> impl Parser<Token, Statement, Error = Simple<Token>> {
    kw(Keyword::If)
        .ignore_then(
            sym(Symbol::LParens)
                .ignore_then(parse_expression())
                .then_ignore(sym(Symbol::RParens)),
        )
        .then(
            sym(Symbol::LCurly)
                .ignore_then(statement.clone().repeated())
                .then_ignore(sym(Symbol::RCurly)),
        )
        .then(
            kw(Keyword::Else)
                .ignore_then(
                    sym(Symbol::LCurly)
                        .ignore_then(statement.repeated())
                        .then_ignore(sym(Symbol::RCurly)),
                )
                .or_not(),
        )
        .map(|((cond, then), else_opt)| Statement::IfStatement(cond, then, else_opt))
        .labelled("if statement")
}

// parse_while_statement:
//  'while' '(' [expression] ')' '{' [statement]* '}'
fn parse_while_statement(
    statement: impl Parser<Token, Statement, Error = Simple<Token>>,
) -> impl Parser<Token, Statement, Error = Simple<Token>> {
    kw(Keyword::While)
        .ignore_then(
            sym(Symbol::LParens)
                .ignore_then(parse_expression())
                .then_ignore(sym(Symbol::RParens)),
        )
        .then(
            sym(Symbol::LCurly)
                .ignore_then(statement.repeated())
                .then_ignore(sym(Symbol::RCurly)),
        )
        .map(|(e, s)| Statement::WhileStatement(e, s))
        .labelled("while statement")
}

// parse_do_statement:
//  'do' [subroutine_call] ';'
fn parse_do_statement() -> impl Parser<Token, Statement, Error = Simple<Token>> {
    kw(Keyword::Do)
        .ignore_then(parse_subroutine_call())
        .then_ignore(sym(Symbol::Semicolon))
        .map(Statement::DoStatement)
        .labelled("do statement")
}

// parse_return_statement:
//  'return' [expression]? ';'
fn parse_return_statement() -> impl Parser<Token, Statement, Error = Simple<Token>> {
    kw(Keyword::Return)
        .ignore_then(parse_expression().or_not())
        .then_ignore(sym(Symbol::Semicolon))
        .map(Statement::ReturnStatement)
        .labelled("return statement")
}

// Expressions:

// neede fixing
// parse_expression:
//  [term] ([binary_op] [term])*
pub fn parse_expression() -> impl Parser<Token, Expression, Error = Simple<Token>> {
    recursive(|expr| {
        parse_term(expr.clone())
            .map(Box::new)
            .then(
                parse_binary_op()
                    .then(parse_term(expr).map(Box::new))
                    .repeated(),
            )
            .map(|(term, bin)| Expression { term, bin })
            .labelled("expression")
    })
    .boxed()
}

// parse_term:
//  [integer_constant] | [string_constant] | [keyword_constant] | [var_name] ('[' [expression] ']')?
//  | '(' [expression] ')' | ([unary_op] [term]) | [subroutine_call]
pub fn parse_term<'a>(
    expr: Recursive<'a, Token, Expression, Simple<Token>>,
) -> impl Parser<Token, Term, Error = Simple<Token>> + 'a {
    recursive(|term| {
        let int_const = int_const().map(Term::IntegerConstant);
        let string_const = string_const().map(Term::StringConstant);
        let keyword_const = parse_keyword_constant().map(Term::KeywordConstant);
        let unary_term = parse_unary_op()
            .then(Box::new(term))
            .map(|(u, t)| Term::UnaryTerm(u, Box::new(t)));
        let parens_expr = sym(Symbol::LParens)
            .ignore_then(expr.clone().map(Box::new))
            .then_ignore(sym(Symbol::RParens))
            .map(Term::ParensExpr);

        let var_or_subroutine = ident()
            .then_ignore(sym(Symbol::Period))
            .then(ident())
            .then_ignore(sym(Symbol::LParens))
            .then(expr.clone().map(Box::new).separated_by(sym(Symbol::Comma)))
            .then_ignore(sym(Symbol::RParens))
            .map(|((c, s), es)| Term::SubroutineCall(SubroutineCall::ClassCall(c, s, es)))
            .or(ident()
                .then_ignore(sym(Symbol::LParens))
                .then(expr.clone().map(Box::new).separated_by(sym(Symbol::Comma)))
                .then_ignore(sym(Symbol::RParens))
                .map(|(s, es)| Term::SubroutineCall(SubroutineCall::Call(s, es))))
            .or(ident()
                .then(
                    (sym(Symbol::LBracket)
                        .ignore_then(expr.map(Box::new))
                        .then_ignore(sym(Symbol::RBracket)))
                    .or_not(),
                )
                .map(|(s, oe)| Term::VarName(s, oe)));

        choice((
            unary_term,
            int_const,
            string_const,
            keyword_const,
            parens_expr,
            var_or_subroutine,
        ))
        .labelled("term")
    })
}

// parse_subroutine_call:
//  ([var_name] | [class_name]) '.' [subroutine_name] '(' [expression_list] ')'
//  | [subroutine_name] '(' [expression_list] ')'
fn parse_subroutine_call() -> impl Parser<Token, SubroutineCall, Error = Simple<Token>> {
    ident()
        .then_ignore(sym(Symbol::Period))
        .then(ident())
        .then_ignore(sym(Symbol::LParens))
        .then(parse_expression_list())
        .then_ignore(sym(Symbol::RParens))
        .map(|((c, s), es)| SubroutineCall::ClassCall(c, s, es))
        .or(ident()
            .then_ignore(sym(Symbol::LParens))
            .then(parse_expression_list())
            .then_ignore(sym(Symbol::RParens))
            .map(|(s, es)| SubroutineCall::Call(s, es)))
        .labelled("subroutine call")
}

// parse_expression_list:
//  ([expression] (',' [expression]) *)?
fn parse_expression_list() -> impl Parser<Token, Vec<Box<Expression>>, Error = Simple<Token>> {
    parse_expression()
        .map(Box::new)
        .separated_by(sym(Symbol::Comma))
        .labelled("expression list")
}

// parse_binary_op:
//  '+' | '-' | '*' | '/' | '&' | '|' | '<' | '>' | '='
fn parse_binary_op() -> impl Parser<Token, BinaryOp, Error = Simple<Token>> {
    choice((
        sym(Symbol::Plus).to(BinaryOp::Plus),
        sym(Symbol::Minus).to(BinaryOp::Minus),
        sym(Symbol::Asterisk).to(BinaryOp::Times),
        sym(Symbol::Slash).to(BinaryOp::Div),
        sym(Symbol::Ampersand).to(BinaryOp::And),
        sym(Symbol::Bar).to(BinaryOp::Or),
        sym(Symbol::Lesser).to(BinaryOp::Lesser),
        sym(Symbol::Greater).to(BinaryOp::Greater),
        sym(Symbol::Equal).to(BinaryOp::Equal),
    ))
}

// parse_unary_op:
//  '-' | '~'
fn parse_unary_op() -> impl Parser<Token, UnaryOp, Error = Simple<Token>> {
    choice((
        sym(Symbol::Minus).to(UnaryOp::Negation),
        sym(Symbol::Tilde).to(UnaryOp::Tilde),
    ))
}

// parse_keyword_constant:
//  'true' | 'false' | 'null' | 'this'
fn parse_keyword_constant() -> impl Parser<Token, KeywordConstant, Error = Simple<Token>> {
    choice((
        kw(Keyword::True).to(KeywordConstant::True),
        kw(Keyword::False).to(KeywordConstant::False),
        kw(Keyword::Null).to(KeywordConstant::Null),
        kw(Keyword::This).to(KeywordConstant::This),
    ))
    .labelled("keyword constant")
}

// parse_type:
//  'int' | 'char' | 'boolean' | [class_name]
fn parse_type() -> impl Parser<Token, Type, Error = Simple<Token>> {
    choice((
        kw(Keyword::Int).to(Type::Int),
        kw(Keyword::Boolean).to(Type::Boolean),
        kw(Keyword::Char).to(Type::Char),
        ident().map(|s| Type::ClassName(s)),
    ))
    .labelled("type")
}
