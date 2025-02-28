use crate::ast::vm::*;

use itertools::Itertools;



pub trait PrettyPrint {
    fn pretty_print(&self) -> String;
}

impl PrettyPrint for Command {
    fn pretty_print(&self) -> String {
        format!(
            "class {} {{\n{}\n}}\n",
            self.class_name,
            self.class_dec.pretty_print(i + 1)
        )
    }
}

