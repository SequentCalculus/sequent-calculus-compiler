use printer::Print;

use super::{stringify_and_join, Name, Ty, TypingContext};

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

impl Print for XtorSig {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        todo!()
    }
}

impl fmt::Display for TypeDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let xtor_strs: Vec<String> = self.xtors.iter().map(|bnd| format!("{bnd}")).collect();
        write!(f, "type {} {{ {} }}", self.name, xtor_strs.join(", "))
    }
}

impl Print for TypeDeclaration {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        todo!()
    }
}

#[must_use]
pub fn lookup_type_declaration<'a>(ty: &Ty, types: &'a [TypeDeclaration]) -> &'a TypeDeclaration {
    if let Ty::Decl(type_name) = ty {
        let type_declaration = types
            .iter()
            .find(|declaration| declaration.name == *type_name)
            .expect("Type {type_name} not found");
        type_declaration
    } else {
        panic!("User-defined type cannot be {ty}");
    }
}

#[must_use]
pub fn xtor_position(tag: &Name, type_declaration: &TypeDeclaration) -> usize {
    type_declaration
        .xtors
        .iter()
        .position(|xtor| xtor.name == *tag)
        .expect("Constructor {tag} not found in type declaration {type_declaration}")
}
