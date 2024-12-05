use printer::{
    theme::ThemeExt,
    tokens::{CODATA, DATA},
    util::BracesExt,
    DocAllocator, Print,
};

use super::{context::TypingContext, Name};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Data;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Codata;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XtorSig<T> {
    pub xtor: T,
    pub name: Name,
    pub args: TypingContext,
}

pub type CtorSig = XtorSig<Data>;
pub type DtorSig = XtorSig<Codata>;

#[derive(Debug, Clone, PartialEq, Eq)]
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
        alloc.text(&self.name).append(self.args.print(cfg, alloc))
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

// #[must_use]
// pub fn cont_int() -> TypeDeclaration {
//     FsTypeDeclaration {
//         name: "_Cont".to_string(),
//         xtors: vec![FsXtorSig {
//             name: "_Ret".to_string(),
//             args: Context {
//                 bindings: vec![FsContextBinding {
//                     var: "x".to_string(),
//                     chi: Chirality::Prd,
//                     ty: Ty::Int,
//                 }],
//             },
//         }],
//     }
// }

// #[must_use]
// pub fn lookup_type_declaration<'a>(
//     type_name: &String,
//     types: &'a [FsTypeDeclaration],
// ) -> &'a FsTypeDeclaration {
//     let type_declaration = types
//         .iter()
//         .find(|declaration| declaration.name == *type_name)
//         .expect("Type {type_name} not found");
//     type_declaration
// }

#[cfg(test)]
mod decl_tests {
    use printer::Print;

    use super::{Data, TypeDeclaration, XtorSig};
    use crate::syntax::{
        context::{Context, ContextBinding},
        types::Ty,
    };

    fn example_nil() -> XtorSig<Data> {
        XtorSig {
            xtor: Data,
            name: "Nil".to_owned(),
            args: Context { bindings: vec![] },
        }
    }

    fn example_cons() -> XtorSig<Data> {
        XtorSig {
            xtor: Data,
            name: "Cons".to_owned(),
            args: Context {
                bindings: vec![
                    ContextBinding::VarBinding {
                        var: "x".to_owned(),
                        ty: Ty::Int,
                    },
                    ContextBinding::VarBinding {
                        var: "xs".to_owned(),
                        ty: Ty::Decl("ListInt".to_owned()),
                    },
                ],
            },
        }
    }

    #[test]
    fn display_xtor_simple() {
        assert_eq!(example_nil().print_to_string(None), "Nil")
    }

    #[test]
    fn display_xtor_args() {
        assert_eq!(
            example_cons().print_to_string(None),
            "Cons(x: Int, xs: ListInt)"
        )
    }

    #[test]
    fn display_listint() {
        let result = TypeDeclaration {
            dat: Data,
            name: "ListInt".to_owned(),
            xtors: vec![example_nil(), example_cons()],
        }
        .print_to_string(None);
        let expected = "data ListInt { Nil, Cons(x: Int, xs: ListInt) }";
        assert_eq!(result, expected)
    }
}
