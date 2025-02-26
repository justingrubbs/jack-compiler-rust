use crate::ast::jack::*;

use itertools::Itertools;

// Pretty-printing:
fn tab(i: usize) -> String {
    "    ".repeat(i)
}

pub trait PrettyPrint {
    fn pretty_print(&self, i: usize) -> String;
}

impl PrettyPrint for Class {
    fn pretty_print(&self, i: usize) -> String {
        format!(
            "class {} {{\n{}\n}}\n",
            self.class_name,
            self.class_dec.pretty_print(i + 1)
        )
    }
}

impl PrettyPrint for ClassDec {
    fn pretty_print(&self, i: usize) -> String {
        let class_var_decs = self
            .class_var_dec
            .iter()
            .map(|cvd| format!("{}\n", cvd.pretty_print(i)))
            .join("");

        let subroutine_decs = self
            .subroutine_dec
            .iter()
            .map(|sd| sd.pretty_print(i))
            .join("\n");

        format!("{}{}", class_var_decs, subroutine_decs)
    }
}

impl PrettyPrint for ClassVarDec {
    fn pretty_print(&self, i: usize) -> String {
        let class_var_type = self.class_var_type.pretty_print(i);
        let r#type = self.r#type.pretty_print(i);
        let vars = self.vars.iter().map(|v| v.to_string()).join(", ");

        format!("{}{} {} {};", tab(i), class_var_type, r#type, vars)
    }
}

impl PrettyPrint for Kind {
    fn pretty_print(&self, _: usize) -> String {
        match self {
            Kind::Static => "static".to_string(),
            Kind::Field => "field".to_string(),
        }
    }
}

impl PrettyPrint for Type {
    fn pretty_print(&self, _: usize) -> String {
        match self {
            Type::Int => "int".to_string(),
            Type::Char => "char".to_string(),
            Type::Boolean => "boolean".to_string(),
            Type::ClassName(s) => s.to_string(),
        }
    }
}

impl PrettyPrint for SubroutineDec {
    fn pretty_print(&self, i: usize) -> String {
        let subroutine_type = self.subroutine_type.pretty_print(i);
        let subroutine_return_type = self.subroutine_return_type.pretty_print(i);

        let parameter_list = self
            .parameter_list
            .clone()
            .map_or(String::new(), |params: Vec<Parameter>| {
                params.iter().map(|p| p.pretty_print(i)).join(", ")
            });

        let subroutine_body = self.subroutine_body.pretty_print(i + 1);

        format!(
            "{}{} {} {}({}) {{\n{}\n{}}}",
            tab(i),
            subroutine_type,
            subroutine_return_type,
            self.subroutine_name,
            parameter_list,
            subroutine_body,
            tab(i)
        )
    }
}

impl PrettyPrint for SubroutineType {
    fn pretty_print(&self, _: usize) -> String {
        match self {
            SubroutineType::Constructor => "constructor".to_string(),
            SubroutineType::Function => "function".to_string(),
            SubroutineType::Method => "method".to_string(),
        }
    }
}

impl PrettyPrint for SubroutineReturnType {
    fn pretty_print(&self, i: usize) -> String {
        match self {
            SubroutineReturnType::Void => "void".to_string(),
            SubroutineReturnType::Type(t) => t.pretty_print(i),
        }
    }
}

impl PrettyPrint for Parameter {
    fn pretty_print(&self, i: usize) -> String {
        format!(
            "{} {}",
            self.r#type.pretty_print(i),
            self.var_name.to_string()
        )
    }
}

impl PrettyPrint for SubroutineBody {
    fn pretty_print(&self, i: usize) -> String {
        let var_decs = self
            .var_decs
            .iter()
            .map(|vd| format!("{}var {};\n", tab(i), vd.pretty_print(i)))
            .join("");

        let statements = self
            .stmts
            .iter()
            .map(|stmt| format!("{}{}", tab(i), stmt.pretty_print(i)))
            .join("\n");

        format!("{}{}", var_decs, statements)
    }
}

impl PrettyPrint for VarDec {
    fn pretty_print(&self, i: usize) -> String {
        let r#type = self.r#type.pretty_print(i);
        let var_names = self.var_name.iter().map(|vn| vn.to_string()).join(", ");

        format!("{} {}", r#type, var_names)
    }
}

impl PrettyPrint for Statement {
    fn pretty_print(&self, i: usize) -> String {
        match self {
            Statement::DoStatement(sc) => format!("do {};", sc.pretty_print(i)),
            Statement::LetStatement(s, oe, e) => {
                let expr = oe
                    .clone()
                    .map_or(String::new(), |expr| format!("[{}]", expr.pretty_print(i)));
                format!("let {}{} = {};", s, expr, e.pretty_print(i))
            }
            Statement::WhileStatement(e, s) => {
                let stmts = s
                    .iter()
                    .map(|stmt| format!("{}{}", tab(i + 1), stmt.pretty_print(i + 1)))
                    .join("\n");
                format!("while ({}) {{\n{}\n{}}}", e.pretty_print(i), stmts, tab(i))
            }
            Statement::IfStatement(e, s, os) => {
                let stmts = s
                    .iter()
                    .map(|s| format!("{}{}", tab(i + 1), s.pretty_print(i + 1)))
                    .join("\n");

                let elsey = os.clone().map_or(String::new(), |stmts| {
                    format!(
                        " else {{\n{}\n{}}}",
                        stmts
                            .iter()
                            .map(|s| format!("{}{}", tab(i + 1), s.pretty_print(i + 1)))
                            .join("\n"),
                        tab(i)
                    )
                });

                format!(
                    "if ({}) {{\n{}\n{}}}{}",
                    e.pretty_print(i),
                    stmts,
                    tab(i),
                    elsey
                )
            }
            Statement::ReturnStatement(oe) => {
                let expr = oe
                    .clone()
                    .map_or(String::new(), |expr| format!(" {}", expr.pretty_print(i)));
                format!("return{};", expr)
            }
        }
    }
}

impl PrettyPrint for Expression {
    fn pretty_print(&self, i: usize) -> String {
        let term = self.term.pretty_print(i);
        let bin = self
            .bin
            .iter()
            .map(|(b, t)| format!(" {} {}", b.pretty_print(i), t.pretty_print(i)))
            .join("");
        format!("{}{}", term, bin)
    }
}

// FINISH
impl PrettyPrint for Term {
    fn pretty_print(&self, i: usize) -> String {
        match self {
            Term::IntegerConstant(i) => i.to_string(),
            Term::StringConstant(s) => format!("\"{}\"", s),
            Term::KeywordConstant(k) => k.pretty_print(i),
            Term::ParensExpr(e) => format!("({})", e.pretty_print(i)),
            Term::SubroutineCall(sc) => sc.pretty_print(i),
            Term::UnaryTerm(uop, t) => format!("{}{}", uop.pretty_print(i), t.pretty_print(i)),
            Term::VarName(s, oe) => {
                let expr = oe
                    .clone()
                    .map_or(String::new(), |expr| format!("[{}]", expr.pretty_print(i)));
                format!("{}{}", s, expr)
            }
        }
    }
}

impl PrettyPrint for SubroutineCall {
    fn pretty_print(&self, i: usize) -> String {
        match self {
            SubroutineCall::ClassCall(s1, s2, es) => {
                let exprs = es.iter().map(|e| e.pretty_print(i)).join(", ");
                format!("{}.{}({})", s1, s2, exprs)
            }
            SubroutineCall::Call(s, es) => {
                let exprs = es.iter().map(|e| e.pretty_print(i)).join(", ");
                format!("{}({})", s, exprs)
            }
        }
    }
}

impl PrettyPrint for BinaryOp {
    fn pretty_print(&self, _: usize) -> String {
        match self {
            BinaryOp::Plus => "+".to_string(),
            BinaryOp::Minus => "-".to_string(),
            BinaryOp::Times => "*".to_string(),
            BinaryOp::Div => "/".to_string(),
            BinaryOp::And => "&".to_string(),
            BinaryOp::Or => "|".to_string(),
            BinaryOp::Lesser => "<".to_string(),
            BinaryOp::Greater => ">".to_string(),
            BinaryOp::Equal => "=".to_string(),
        }
    }
}

impl PrettyPrint for UnaryOp {
    fn pretty_print(&self, _: usize) -> String {
        match self {
            UnaryOp::Negation => "-".to_string(),
            UnaryOp::Tilde => "~".to_string(),
        }
    }
}

impl PrettyPrint for KeywordConstant {
    fn pretty_print(&self, _: usize) -> String {
        match self {
            KeywordConstant::True => "true".to_string(),
            KeywordConstant::False => "false".to_string(),
            KeywordConstant::Null => "null".to_string(),
            KeywordConstant::This => "this".to_string(),
        }
    }
}
