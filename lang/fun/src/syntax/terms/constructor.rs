//! This module defines constructor terms of data types.

use codespan::Span;
use derivative::Derivative;
use printer::{Print, theme::ThemeExt};

use super::Term;
use crate::{
    parser::util::ToMiette,
    syntax::{
        context::TypingContext,
        names::{Name, Var},
        substitution::Substitution,
        types::{OptTyped, Ty},
    },
    traits::used_binders::UsedBinders,
    typing::{
        check::{Check, check_args, check_equality},
        errors::Error,
        symbol_table::SymbolTable,
    },
};

use std::collections::HashSet;

/// This struct defines a constructor term of a data type. It consists of a name for the
/// constructor, a substitution for the arguments of the constructor, and after typechecking also
/// of the inferred type.
///
/// Example:
/// `Cons(2, Nil)` is the constructor `Cons` with arguments `2` and constructor `Nil`.
#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct Constructor {
    /// The source location
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    /// The constructor name
    pub id: Name,
    /// The arguments of the constructor
    pub args: Substitution,
    /// The (inferred) type of the constructor
    pub ty: Option<Ty>,
}

impl OptTyped for Constructor {
    fn get_type(&self) -> Option<Ty> {
        self.ty.clone()
    }
}

impl Print for Constructor {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        let args = if self.args.bindings.is_empty() {
            self.args.print(cfg, alloc)
        } else {
            self.args.print(cfg, alloc).parens()
        };

        alloc.ctor(&self.id).append(args.group())
    }
}

impl From<Constructor> for Term {
    fn from(value: Constructor) -> Self {
        Term::Constructor(value)
    }
}

impl Check for Constructor {
    fn check(
        mut self,
        symbol_table: &mut SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<Self, Error> {
        let type_args = match expected {
            Ty::Decl { type_args, .. } => type_args,
            Ty::I64 { .. } => {
                return Err(Error::ExpectedI64ForConstructor {
                    span: self.span.to_miette(),
                    name: self.id,
                });
            }
        };

        // the name of the constructor in the symbol table for the instantiated data type, the
        // instance must exists already
        let name = self.id.clone() + &type_args.print_to_string(None);
        match symbol_table.ctors.get(&name) {
            Some(types) => {
                let (ty, _) = symbol_table.lookup_ty_for_ctor(&self.span.to_miette(), &name)?;

                self.args = check_args(
                    &self.span.to_miette(),
                    symbol_table,
                    context,
                    self.args,
                    &types.clone(),
                )?;

                check_equality(&self.span, symbol_table, expected, &ty)?;

                self.ty = Some(expected.clone());
                Ok(self)
            }
            None => Err(Error::Undefined {
                span: self.span.to_miette(),
                name: self.id.clone(),
            }),
        }
    }
}

impl UsedBinders for Constructor {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        self.args.bindings.used_binders(used);
    }
}

#[cfg(test)]
mod test {
    use super::{Check, Term};
    use crate::{
        parser::fun,
        syntax::context::{Chirality::Prd, TypingContext},
        syntax::terms::Lit,
        syntax::{
            terms::{Constructor, XVar},
            types::{Ty, TypeArgs},
        },
        test_common::symbol_table_list,
    };
    use codespan::Span;
    use printer::Print;

    #[test]
    fn check_nil() {
        let result = Constructor {
            span: Span::default(),
            id: "Nil".to_owned(),
            args: vec![].into(),
            ty: None,
        }
        .check(
            &mut symbol_table_list(),
            &TypingContext::default(),
            &Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_i64()])),
        )
        .unwrap();
        let expected = Constructor {
            span: Span::default(),
            id: "Nil".to_owned(),
            args: vec![].into(),
            ty: Some(Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_i64()]))),
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn check_cons() {
        let mut ctx = TypingContext::default();
        ctx.add_var("x", Ty::mk_i64());
        let result = Constructor {
            span: Span::default(),
            id: "Cons".to_owned(),
            args: vec![
                XVar::mk("x").into(),
                Constructor {
                    span: Span::default(),
                    id: "Nil".to_owned(),
                    args: vec![].into(),
                    ty: None,
                }
                .into(),
            ]
            .into(),
            ty: None,
        }
        .check(
            &mut symbol_table_list(),
            &ctx,
            &Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_i64()])),
        )
        .unwrap();
        let expected = Constructor {
            span: Span::default(),
            id: "Cons".to_owned(),
            args: vec![
                XVar {
                    span: Span::default(),
                    var: "x".to_owned(),
                    ty: Some(Ty::mk_i64()),
                    chi: Some(Prd),
                }
                .into(),
                Constructor {
                    span: Span::default(),
                    id: "Nil".to_owned(),
                    args: vec![].into(),
                    ty: Some(Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_i64()]))),
                }
                .into(),
            ]
            .into(),
            ty: Some(Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_i64()]))),
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn check_ctor_fail() {
        let result = Constructor {
            span: Span::default(),
            id: "Cons".to_owned(),
            args: vec![
                Constructor {
                    span: Span::default(),
                    id: "Nil".to_owned(),
                    args: vec![].into(),
                    ty: None,
                }
                .into(),
                Constructor {
                    span: Span::default(),
                    id: "Nil".to_owned(),
                    args: vec![].into(),
                    ty: None,
                }
                .into(),
            ]
            .into(),
            ty: None,
        }
        .check(
            &mut symbol_table_list(),
            &TypingContext {
                span: Span::default(),
                bindings: vec![],
            },
            &Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_i64()])),
        );
        assert!(result.is_err());
    }

    fn example_nil() -> Constructor {
        Constructor {
            span: Span::default(),
            id: "Nil".to_owned(),
            args: vec![].into(),
            ty: None,
        }
    }

    fn example_tup() -> Constructor {
        Constructor {
            span: Span::default(),
            id: "Tup".to_owned(),
            args: vec![Term::Lit(Lit::mk(2)).into(), Term::Lit(Lit::mk(4)).into()].into(),
            ty: None,
        }
    }

    #[test]
    fn display_nil() {
        assert_eq!(example_nil().print_to_string(Default::default()), "Nil")
    }

    #[test]
    fn parse_nil() {
        let parser = fun::TermParser::new();
        assert_eq!(parser.parse("Nil"), Ok(example_nil().into()));
    }

    #[test]
    fn display_tup() {
        assert_eq!(
            example_tup().print_to_string(Default::default()),
            "Tup(2, 4)"
        )
    }

    #[test]
    fn parse_tup() {
        let parser = fun::TermParser::new();
        assert_eq!(parser.parse("Tup(2,4)"), Ok(example_tup().into()));
    }
}
