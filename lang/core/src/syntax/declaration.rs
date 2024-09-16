use super::{context::TypingContext, Name};
use std::fmt;

pub struct Ctor;
pub struct Dtor;

pub struct XtorSig<T> {
    pub xtor: T,
    pub name: Name,
    pub args: TypingContext,
}

pub type CtorSig = XtorSig<Ctor>;
pub type DtorSig = XtorSig<Dtor>;

pub struct TypeDeclaration<T> {
    pub name: Name,
    pub xtors: Vec<XtorSig<T>>,
}

pub type DataDeclaration = TypeDeclaration<Ctor>;
pub type CodataDeclaration = TypeDeclaration<Dtor>;

impl<T> fmt::Display for XtorSig<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.args.is_empty() {
            write!(f, "{}", self.name)
        } else {
            let args_strs: Vec<String> = self.args.iter().map(|bnd| format!("{}", bnd)).collect();
            write!(f, "{}({})", self.name, args_strs.join(", "))
        }
    }
}

impl<T> fmt::Display for TypeDeclaration<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let xtor_strs: Vec<String> = self.xtors.iter().map(|bnd| format!("{}", bnd)).collect();
        write!(f, "{} {{ {} }}", self.name, xtor_strs.join(", "))
    }
}

#[cfg(test)]
mod decl_tests {
    use super::{Ctor, TypeDeclaration, XtorSig};
    use crate::syntax::{context::ContextBinding, types::Ty};

    fn example_nil() -> XtorSig<Ctor> {
        XtorSig {
            xtor: Ctor,
            name: "Nil".to_owned(),
            args: vec![],
        }
    }

    fn example_cons() -> XtorSig<Ctor> {
        XtorSig {
            xtor: Ctor,
            name: "Cons".to_owned(),
            args: vec![
                ContextBinding::VarBinding {
                    var: "x".to_owned(),
                    ty: Ty::Int(),
                },
                ContextBinding::VarBinding {
                    var: "xs".to_owned(),
                    ty: Ty::Decl("ListInt".to_owned()),
                },
            ],
        }
    }

    #[test]
    fn display_xtor_simple() {
        let result = format!("{}", example_nil());
        let expected = "Nil";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_xtor_args() {
        let result = format!("{}", example_cons());
        let expected = "Cons(x : Int, xs : ListInt)";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_listint() {
        let result = format!(
            "{}",
            TypeDeclaration {
                name: "ListInt".to_owned(),
                xtors: vec![example_nil(), example_cons()]
            }
        );
        let expected = "ListInt { Nil, Cons(x : Int, xs : ListInt) }";
        assert_eq!(result, expected)
    }
}
