//! This module defines invoking destructors of codata types.

use codespan::Span;
use derivative::Derivative;
use printer::{DocAllocator, Print, theme::ThemeExt, tokens::DOT};

use super::Term;
use crate::{
    parser::util::ToMiette,
    syntax::{
        arguments::Arguments,
        context::TypingContext,
        names::{Name, Var},
        types::{OptTyped, Ty, TypeArgs},
    },
    traits::used_binders::UsedBinders,
    typing::{
        check::{Check, check_args, check_equality},
        errors::Error,
        symbol_table::SymbolTable,
    },
};

use std::{collections::HashSet, rc::Rc};

/// This struct defines an invocation of a destructor of codata type. It consists of the scrutinee
/// on which to invoke the destructor, the name of the destructor, a list of type arguments
/// instantiating the type parameters of the codata type, the arguments of the destructor, and
/// after typechecking also of the inferred type.
///
/// Example:
/// `stream.Head[i64]` invokes the destructor `Head` on a `stream` with type argument `i64`.
#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct Destructor {
    /// The source location
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    /// The term the destructor is invoked on
    pub scrutinee: Rc<Term>,
    /// The destructor name
    pub id: Name,
    /// The type arguments instantiating the type parameters of the type
    pub type_args: TypeArgs,
    /// The arguments of the destructor
    pub args: Arguments,
    /// Type (inferred) of the term
    pub ty: Option<Ty>,
}

impl OptTyped for Destructor {
    fn get_type(&self) -> Option<Ty> {
        self.ty.clone()
    }
}

impl Print for Destructor {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        let args = if self.args.entries.is_empty() {
            alloc.nil()
        } else {
            self.args.print(cfg, alloc).parens()
        };

        if (matches!(*self.scrutinee, Term::XVar(_))
            || matches!(*self.scrutinee, Term::Call(ref call) if call.args.entries.is_empty()))
            && (self.scrutinee.print_to_string(Some(cfg)).len() <= cfg.indent as usize)
        {
            self.scrutinee
                .print(cfg, alloc)
                .append(DOT)
                .append(alloc.dtor(&self.id))
                .append(self.type_args.print(cfg, alloc))
                .append(args.group())
        } else {
            self.scrutinee
                .print(cfg, alloc)
                .append(alloc.line_())
                .append(DOT)
                .append(alloc.dtor(&self.id))
                .append(self.type_args.print(cfg, alloc))
                .append(args.group())
                .nest(cfg.indent)
                .align()
        }
    }
}

impl From<Destructor> for Term {
    fn from(value: Destructor) -> Self {
        Term::Destructor(value)
    }
}

impl Check for Destructor {
    fn check(
        mut self,
        symbol_table: &mut SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<Self, Error> {
        // the name of the constructor in the symbol table for the instantiated data type
        let dtor_name = self.id.clone() + &self.type_args.print_to_string(None);
        let ty = match symbol_table.lookup_ty_for_dtor(&self.span.to_miette(), &dtor_name) {
            Ok(ty) => ty,
            // if there is no instance yet, we create an instance from the template
            Err(_) => symbol_table.lookup_ty_template_for_dtor(&self.id, &self.type_args)?,
        };

        self.scrutinee = self.scrutinee.check(symbol_table, context, &ty)?;

        match symbol_table.dtors.get(&dtor_name) {
            Some(signature) => {
                let (types, ret_ty) = signature.clone();

                self.args = check_args(
                    &self.span.to_miette(),
                    symbol_table,
                    context,
                    self.args,
                    &types,
                )?;

                check_equality(&self.span, symbol_table, expected, &ret_ty)?;

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

impl UsedBinders for Destructor {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        self.scrutinee.used_binders(used);
        self.args.entries.used_binders(used);
    }
}

#[cfg(test)]
mod destructor_tests {
    use super::Check;
    use crate::{
        parser::fun,
        syntax::{
            context::{
                Chirality::{Cns, Prd},
                TypingContext,
            },
            terms::{Destructor, Lit, XVar},
            types::{Ty, TypeArgs},
        },
        test_common::{symbol_table_fun_template, symbol_table_lpair},
        typing::symbol_table::SymbolTable,
    };
    use codespan::Span;
    use printer::Print;
    use std::rc::Rc;

    #[test]
    fn check_fst() {
        let mut ctx = TypingContext::default();
        ctx.add_var(
            "x",
            Ty::mk_decl("LPair", TypeArgs::mk(vec![Ty::mk_i64(), Ty::mk_i64()])),
        );
        let mut symbol_table = symbol_table_lpair();
        let result = Destructor {
            span: Span::default(),
            id: "fst".to_owned(),
            type_args: TypeArgs::mk(vec![Ty::mk_i64(), Ty::mk_i64()]),
            args: vec![].into(),
            scrutinee: Rc::new(XVar::mk("x").into()),
            ty: None,
        }
        .check(&mut symbol_table, &ctx, &Ty::mk_i64())
        .unwrap();
        let expected = Destructor {
            span: Span::default(),
            id: "fst".to_owned(),
            args: vec![].into(),
            type_args: TypeArgs::mk(vec![Ty::mk_i64(), Ty::mk_i64()]),
            scrutinee: Rc::new(
                XVar {
                    span: Span::default(),
                    var: "x".to_owned(),
                    ty: Some(Ty::mk_decl(
                        "LPair",
                        TypeArgs::mk(vec![Ty::mk_i64(), Ty::mk_i64()]),
                    )),
                    chi: Some(Prd),
                }
                .into(),
            ),
            ty: Some(Ty::mk_i64()),
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn check_ap() {
        let mut ctx = TypingContext::default();
        ctx.add_var(
            "x",
            Ty::mk_decl("Fun", TypeArgs::mk(vec![Ty::mk_i64(), Ty::mk_i64()])),
        );
        ctx.add_covar("a", Ty::mk_i64());
        let mut symbol_table = symbol_table_fun_template();
        let result = Destructor {
            span: Span::default(),
            id: "apply".to_owned(),
            type_args: TypeArgs::mk(vec![Ty::mk_i64(), Ty::mk_i64()]),
            args: vec![Lit::mk(1).into(), XVar::mk("a").into()].into(),
            scrutinee: Rc::new(XVar::mk("x").into()),
            ty: None,
        }
        .check(&mut symbol_table, &ctx, &Ty::mk_i64())
        .unwrap();
        let expected = Destructor {
            span: Span::default(),
            id: "apply".to_owned(),
            type_args: TypeArgs::mk(vec![Ty::mk_i64(), Ty::mk_i64()]),
            args: vec![
                Lit::mk(1).into(),
                XVar {
                    span: Span::default(),
                    var: "a".to_owned(),
                    ty: Some(Ty::mk_i64()),
                    chi: Some(Cns),
                }
                .into(),
            ]
            .into(),
            scrutinee: Rc::new(
                XVar {
                    span: Span::default(),
                    var: "x".to_owned(),
                    ty: Some(Ty::mk_decl(
                        "Fun",
                        TypeArgs::mk(vec![Ty::mk_i64(), Ty::mk_i64()]),
                    )),
                    chi: Some(Prd),
                }
                .into(),
            ),
            ty: Some(Ty::mk_i64()),
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn check_dtor_fail() {
        let mut ctx = TypingContext::default();
        ctx.add_var("x", Ty::mk_decl("Stream", TypeArgs::mk(vec![Ty::mk_i64()])));
        let result = Destructor {
            span: Span::default(),
            id: "head".to_owned(),
            type_args: TypeArgs::mk(vec![Ty::mk_i64()]),
            args: vec![].into(),
            scrutinee: Rc::new(XVar::mk("x").into()),
            ty: None,
        }
        .check(&mut SymbolTable::default(), &ctx, &Ty::mk_i64());
        assert!(result.is_err())
    }

    /// "x.head"
    fn example_1() -> Destructor {
        Destructor {
            span: Span::default(),
            id: "head".to_owned(),
            type_args: TypeArgs::mk(vec![Ty::mk_i64()]),
            scrutinee: Rc::new(XVar::mk("x").into()),
            args: vec![].into(),
            ty: None,
        }
    }

    /// "x.head.head"
    fn example_2() -> Destructor {
        Destructor {
            span: Span::default(),
            id: "head".to_owned(),
            type_args: TypeArgs::mk(vec![Ty::mk_i64()]),
            scrutinee: Rc::new(example_1().into()),
            args: vec![].into(),
            ty: None,
        }
    }

    #[test]
    fn display_1() {
        assert_eq!(
            example_1().print_to_string(Default::default()),
            "x.head[i64]"
        )
    }

    #[test]
    fn display_2() {
        assert_eq!(
            example_2().print_to_string(Default::default()),
            "x.head[i64]\n    .head[i64]"
        )
    }

    #[test]
    fn display_3() {
        let dest = Destructor {
            span: Span::default(),
            id: "fst".to_owned(),
            type_args: TypeArgs::mk(vec![Ty::mk_i64(), Ty::mk_i64()]),
            scrutinee: Rc::new(XVar::mk("x").into()),
            args: vec![XVar::mk("y").into(), XVar::mk("z").into()].into(),
            ty: None,
        };
        let result = dest.print_to_string(Default::default());
        let expected = "x.fst[i64, i64](y, z)".to_owned();
        assert_eq!(result, expected)
    }

    #[test]
    fn parse_1() {
        let parser = fun::TermParser::new();
        assert_eq!(parser.parse("x.head[i64]"), Ok(example_1().into()));
    }

    #[test]
    fn parse_2() {
        let parser = fun::TermParser::new();
        assert_eq!(
            parser.parse("x.head[i64].head[i64]"),
            Ok(example_2().into())
        );
    }
}
