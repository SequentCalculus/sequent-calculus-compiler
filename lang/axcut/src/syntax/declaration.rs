use printer::{theme::ThemeExt, tokens::TYPE, util::BracesExt, DocAllocator, Print};

use super::{Name, Ty, TypingContext};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XtorSig {
    pub name: Name,
    pub args: TypingContext,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeDeclaration {
    pub name: Name,
    pub xtors: Vec<XtorSig>,
}

impl Print for XtorSig {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc
            .text(&self.name)
            .append(self.args.print(cfg, alloc).parens())
    }
}

impl Print for TypeDeclaration {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc
            .keyword(TYPE)
            .append(alloc.space())
            .append(alloc.typ(&self.name))
            .append(alloc.space())
            .append(self.xtors.print(cfg, alloc).braces_anno())
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
        panic!("User-defined type cannot be {}", ty.print_to_string(None));
    }
}

#[must_use]
pub fn xtor_position(tag: &Name, type_declaration: &TypeDeclaration) -> usize {
    type_declaration
        .xtors
        .iter()
        .position(|xtor| xtor.name == *tag)
        .unwrap_or_else(|| {
            panic!(
                "Constructor {tag} not found in type declaration {}",
                type_declaration.print_to_string(None)
            )
        })
}

#[cfg(test)]
mod declaration_tests {
    use super::{lookup_type_declaration, xtor_position, TypeDeclaration, XtorSig};
    use crate::syntax::{context::ContextBinding, types::Ty, Chirality};
    use printer::Print;

    fn example_nil() -> XtorSig {
        XtorSig {
            name: "Nil".to_owned(),
            args: vec![],
        }
    }
    fn example_cons() -> XtorSig {
        XtorSig {
            name: "Cons".to_owned(),
            args: vec![
                ContextBinding {
                    var: "x".to_owned(),
                    chi: Chirality::Prd,
                    ty: Ty::Int,
                },
                ContextBinding {
                    var: "xs".to_owned(),
                    chi: Chirality::Prd,
                    ty: Ty::Decl("ListInt".to_owned()),
                },
            ],
        }
    }

    fn example_list() -> TypeDeclaration {
        TypeDeclaration {
            name: "ListInt".to_owned(),
            xtors: vec![example_nil(), example_cons()],
        }
    }

    #[test]
    fn print_nil() {
        let result = example_nil().print_to_string(Default::default());
        let expected = "Nil()";
        assert_eq!(result, expected)
    }

    #[test]
    fn print_cons() {
        let result = example_cons().print_to_string(Default::default());
        let expected = "Cons(x :prd: Int, xs :prd: ListInt)";
        assert_eq!(result, expected)
    }

    #[test]
    fn print_list() {
        let result = example_list().print_to_string(Default::default());
        let expected = "type ListInt {Nil(), Cons(x :prd: Int, xs :prd: ListInt)}";
        assert_eq!(result, expected)
    }

    #[test]
    fn lookup_list() {
        let decls = vec![example_list()];
        let result = lookup_type_declaration(&Ty::Decl("ListInt".to_owned()), &decls);
        let expected = example_list();
        assert_eq!(result, &expected)
    }

    #[test]
    #[should_panic]
    fn lookup_pair() {
        let decls = vec![example_list()];
        let _ = lookup_type_declaration(&Ty::Decl("TupIntInt".to_owned()), &decls);
    }

    #[test]
    fn lookup_nil() {
        let result = xtor_position(&"Nil".to_owned(), &example_list());
        let expected = 0;
        assert_eq!(result, expected)
    }

    #[test]
    fn lookup_cons() {
        let result = xtor_position(&"Cons".to_owned(), &example_list());
        let expected = 1;
        assert_eq!(result, expected)
    }

    #[test]
    #[should_panic]
    fn lookup_tup() {
        let _ = xtor_position(&"Tup".to_owned(), &example_list());
    }
}
