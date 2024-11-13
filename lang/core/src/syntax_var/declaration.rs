use super::{stringify_and_join, Chirality, ContextBinding, Name, Ty, TypingContext};

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct XtorSig {
    pub name: Name,
    pub args: TypingContext,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeDeclaration {
    pub name: Name,
    pub xtors: Vec<XtorSig>,
}

#[must_use]
pub fn cont_int() -> TypeDeclaration {
    TypeDeclaration {
        name: "0Cont".to_string(),
        xtors: vec![XtorSig {
            name: "0Ret".to_string(),
            args: vec![ContextBinding {
                var: "x".to_string(),
                chi: Chirality::Prd,
                ty: Ty::Int,
            }],
        }],
    }
}

#[must_use]
pub fn lookup_type_declaration<'a>(
    type_name: &String,
    types: &'a [TypeDeclaration],
) -> &'a TypeDeclaration {
    let type_declaration = types
        .iter()
        .find(|declaration| declaration.name == *type_name)
        .expect("Type {type_name} not found");
    type_declaration
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
