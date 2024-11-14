use printer::{
    theme::ThemeExt,
    tokens::{CODATA, DATA},
    util::BracesExt,
    DocAllocator, Print,
};

use super::{context::TypingContext, Name};
use crate::traits::focus::{Focusing, FocusingState};

#[derive(Debug, Clone, PartialEq)]
pub struct Data;
#[derive(Debug, Clone, PartialEq)]
pub struct Codata;

#[derive(Debug, Clone, PartialEq)]
pub struct XtorSig<T> {
    pub xtor: T,
    pub name: Name,
    pub args: TypingContext,
}

pub type CtorSig = XtorSig<Data>;
pub type DtorSig = XtorSig<Codata>;

#[derive(Debug, Clone, PartialEq)]
pub struct TypeDeclaration<T> {
    pub dat: T,
    pub name: Name,
    pub xtors: Vec<XtorSig<T>>,
}

pub type DataDeclaration = TypeDeclaration<Data>;
pub type CodataDeclaration = TypeDeclaration<Codata>;

impl Print for Data {
    fn print<'a>(
        &'a self,
        _cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc.keyword(DATA)
    }
}

impl Print for Codata {
    fn print<'a>(
        &'a self,
        _cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc.keyword(CODATA)
    }
}

impl<T> Print for XtorSig<T> {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        if self.args.is_empty() {
            alloc.text(&self.name)
        } else {
            alloc
                .text(&self.name)
                .append(self.args.print(cfg, alloc).parens())
        }
    }
}

impl<T: Print> Print for TypeDeclaration<T> {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        self.dat
            .print(cfg, alloc)
            .append(alloc.space())
            .append(alloc.typ(&self.name))
            .append(alloc.space())
            .append(
                alloc
                    .space()
                    .append(self.xtors.print(cfg, alloc))
                    .append(alloc.space())
                    .braces_anno(),
            )
    }
}

#[cfg(test)]
mod decl_tests {
    use printer::Print;

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
        let result = example_nil().print_to_string(None);
        let expected = "Nil";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_xtor_args() {
        let result = example_cons().print_to_string(None);
        let expected = "Cons(x : Int, xs : ListInt)";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_listint() {
        let result = TypeDeclaration {
            dat: Data,
            name: "ListInt".to_owned(),
            xtors: vec![example_nil(), example_cons()],
        }
        .print_to_string(None);
        let expected = "data ListInt { Nil, Cons(x : Int, xs : ListInt) }";
        assert_eq!(result, expected)
    }
}

impl<T> Focusing for XtorSig<T> {
    type Target = crate::syntax_var::XtorSig;
    fn focus(self, state: &mut FocusingState) -> crate::syntax_var::XtorSig {
        crate::syntax_var::XtorSig {
            name: self.name,
            args: self.args.focus(state),
        }
    }
}

impl<T> Focusing for TypeDeclaration<T> {
    type Target = crate::syntax_var::TypeDeclaration;
    fn focus(self, state: &mut FocusingState) -> crate::syntax_var::TypeDeclaration {
        crate::syntax_var::TypeDeclaration {
            name: self.name,
            xtors: self.xtors.focus(state),
        }
    }
}
