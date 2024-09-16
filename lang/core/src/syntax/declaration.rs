use super::{context::TypingContext, Name};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Data;
#[derive(Debug, Clone)]
pub struct Codata;

#[derive(Debug, Clone)]
pub struct XtorSig<T> {
    pub xtor: T,
    pub name: Name,
    pub args: TypingContext,
}

pub type CtorSig = XtorSig<Data>;
pub type DtorSig = XtorSig<Codata>;

#[derive(Debug, Clone)]
pub struct TypeDeclaration<T> {
    pub dat: T,
    pub name: Name,
    pub xtors: Vec<XtorSig<T>>,
}

pub type DataDeclaration = TypeDeclaration<Data>;
pub type CodataDeclaration = TypeDeclaration<Codata>;

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("data")
    }
}

impl fmt::Display for Codata {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("codata")
    }
}

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

impl<T: fmt::Display> fmt::Display for TypeDeclaration<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let xtor_strs: Vec<String> = self.xtors.iter().map(|bnd| format!("{}", bnd)).collect();
        write!(
            f,
            "{} {} {{ {} }}",
            self.dat,
            self.name,
            xtor_strs.join(", ")
        )
    }
}

#[cfg(test)]
mod decl_tests {
    use super::{Data, TypeDeclaration, XtorSig};
    use crate::syntax::{context::ContextBinding, types::Ty};

    fn example_nil() -> XtorSig<Data> {
        XtorSig {
            xtor: Data,
            name: "Nil".to_owned(),
            args: vec![],
        }
    }

    fn example_cons() -> XtorSig<Data> {
        XtorSig {
            xtor: Data,
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
                dat: Data,
                name: "ListInt".to_owned(),
                xtors: vec![example_nil(), example_cons()]
            }
        );
        let expected = "data ListInt { Nil, Cons(x : Int, xs : ListInt) }";
        assert_eq!(result, expected)
    }
}
