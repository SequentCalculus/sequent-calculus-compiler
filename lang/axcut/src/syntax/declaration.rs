use super::{context::TypingContext, names::Name, stringify_and_join};
use std::fmt;

#[derive(Debug, Clone)]
pub struct XtorSig {
    pub name: Name,
    pub args: TypingContext,
}

#[derive(Debug, Clone)]
pub struct TypeDeclaration {
    pub name: Name,
    pub xtors: Vec<XtorSig>,
}

impl fmt::Display for XtorSig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let args = stringify_and_join(&self.args, ", ");
        write!(f, "{}({})", self.name, args)
    }
}

impl fmt::Display for TypeDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let xtor_strs: Vec<String> = self.xtors.iter().map(|bnd| format!("{bnd}")).collect();
        write!(f, "type {} {{ {} }}", self.name, xtor_strs.join(", "))
    }
}
