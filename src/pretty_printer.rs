use crate::ast::jack::*;

use itertools::Itertools;


// Pretty-printing:
fn tab(i: usize) -> String {
    "\t".repeat(i)
}

pub trait PrettyPrint {
    fn pretty_print(&self, i: usize) -> String;
}

impl PrettyPrint for Class {
    fn pretty_print(&self, i: usize) -> String {
        format!("class {} {{\n{}\n}}\n",self.class_name,self.class_dec.pretty_print(i+1))
    }
}

impl PrettyPrint for ClassDec {
    fn pretty_print(&self, i: usize) -> String {
        let class_var_decs = self.class_var_dec
            .iter()
            .map(|cvd| cvd.pretty_print(i))
            .join("\n");

        let subroutine_decs = self.subroutine_dec
            .iter()
            .map(|sd| sd.pretty_print(i))
            .join("\n");

        format!("{}\n{}",class_var_decs, subroutine_decs)
    }
}

impl PrettyPrint for SubroutineDec {
    fn pretty_print(&self, i: usize) -> String {
        let subroutine_type = self.subroutine_type.pretty_print(i);
        let subroutine_return_type = self.subroutine_return_type.pretty_print(i);

        let parameter_list = self.parameter_list.clone()
            .map_or(
                String::new(),
                |params: Vec<Parameter>| params
                    .iter()
                    .map(|p| p.pretty_print(i))
                    .join(", "));
        
        let subroutine_body = self.subroutine_body.pretty_print(i+1);

        format!("{}{} {} {}({}) {{\n{}\n{}}}",
            tab(i),
            subroutine_type,
            subroutine_return_type,
            self.subroutine_name,
            parameter_list,
            subroutine_body,
            tab(i))
    }
}

impl PrettyPrint for SubroutineBody {
    fn pretty_print(&self, i: usize) -> String {
        let var_decs = self.var_decs
            .iter()
            .map(|vd| format!("{}var {};",tab(i),vd.pretty_print(i)))
            .join("\n");

        let statements = self.stmts
            .iter()
            .map(|stmt| format!("{}{}",tab(i),stmt.pretty_print(i)))
            .join("\n");

        format!("{}\n{}", var_decs, statements)
    }
}
impl PrettyPrint for Statement {
    fn pretty_print(&self, i: usize) -> String {
        match self {
            Statement::DoStatement(sc) => format!("do {};",sc.pretty_print(i)),
            Statement::LetStatement(s,oe,e) => todo!(),
            Statement::WhileStatement(e,s) => todo!(),
            Statement::IfStatement(e,s,os) => todo!(),
            Statement::ReturnStatement(oe) => todo!(),
        }
    }
}

impl PrettyPrint for SubroutineCall {
    fn pretty_print(&self, i: usize) -> String {
        match self {
            SubroutineCall::ClassCall(s1,s2,es) => {
                let exprs = es.iter()
                    .map(|e| e.pretty_print(i))
                    .join(", ");
                format!("{}.{}({})",s1,s2,exprs)
            },
            SubroutineCall::Call(s,es) => {
                let exprs = es.iter()
                    .map(|e| e.pretty_print(i))
                    .join(", ");
                format!("{}({})",s,exprs)
            },
        }
    }
}

impl PrettyPrint for Expression {
    fn pretty_print(&self, i:usize) -> String {
        "expr".to_string()
    }
}

impl PrettyPrint for VarDec {
    fn pretty_print(&self, i: usize) -> String {
        let r#type = self.r#type.pretty_print(i);
        let var_names = self.var_name
            .iter()
            .map(|vn| vn.to_string())
            .join(", ");

        format!("{} {}",r#type,var_names)
    }
}

impl PrettyPrint for SubroutineType {
    fn pretty_print(&self, i: usize) -> String {
        match self {
            SubroutineType::Constructor => "constructor".to_string(),
            SubroutineType::Function => "function".to_string(),
            SubroutineType::Method => "method".to_string(),
        }
    }
}

impl PrettyPrint for SubroutineReturnType {
    fn pretty_print(&self, i:usize) -> String {
        match self {
            SubroutineReturnType::Void => "void".to_string(),
            SubroutineReturnType::Type(t) => t.pretty_print(i),
        }
    }
}

impl PrettyPrint for Parameter {
    fn pretty_print(&self, i: usize) -> String {
        format!("{} {}",self.r#type.pretty_print(i),self.var_name.to_string())
    }
}

impl PrettyPrint for ClassVarDec {
    fn pretty_print(&self, i: usize) -> String {
        let class_var_type = self.class_var_type.pretty_print(i);
        let r#type = self.r#type.pretty_print(i);
        let vars = self.vars
            .iter()
            .map(|v| v.to_string())
            .join(", ");

        format!("{}{} {} {};",tab(i),class_var_type,r#type,vars)
    }
}

impl PrettyPrint for ClassVarType {
    fn pretty_print(&self, i: usize) -> String {
        match self {
            ClassVarType::Static => "static".to_string(),
            ClassVarType::Field => "field".to_string(),
        }
    }
}

impl PrettyPrint for Type {
    fn pretty_print(&self, i: usize) -> String {
        match self {
            Type::Int => "int".to_string(),
            Type::Char => "char".to_string(),
            Type::Boolean => "boolean".to_string(),
            Type::ClassName(s) => s.to_string(),
        }
    }
}



