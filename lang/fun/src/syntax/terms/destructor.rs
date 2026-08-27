//! This module defines invoking destructors of codata types.

use derivative::Derivative;
use miette::SourceSpan;
use printer::tokens::DOT;
use printer::*;

use crate::syntax::*;
use crate::traits::*;
use crate::typing::*;

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
    pub span: SourceSpan,
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
    fn print<'a>(&'a self, cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        let args = if self.args.entries.is_empty() {
            alloc.nil()
        } else {
            self.args.print(cfg, alloc).parens()
        };

        if (matches!(*self.scrutinee, Term::XVar(_))
            || matches!(*self.scrutinee, Term::Call(ref call) if call.args.entries.is_empty()))
            && (self.scrutinee.print_to_string(Some(cfg)).len() <= cfg.indent.cast_unsigned())
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
        // Resolve the destructor's scrutinee type, argument context, and return type, fully
        // instantiated by splitting `self.type_args` into the codata type's own type arguments
        // and the destructor's own (existential) type arguments.
        let (scrutinee_ty, types, ret_ty) =
            symbol_table.lookup_dtor_signature(&self.span, &self.id, &self.type_args)?;

        self.scrutinee = self.scrutinee.check(symbol_table, context, &scrutinee_ty)?;

        self.args = check_args(&self.span, symbol_table, context, self.args, &types)?;

        check_equality(&self.span, symbol_table, expected, &ret_ty)?;

        self.ty = Some(expected.clone());
        Ok(self)
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
    use printer::Print;

    use crate::parser::fun;
    use crate::syntax::context::ContextBinding;
    use crate::syntax::util::dummy_span;
    use crate::syntax::*;
    use crate::test_common::*;
    use crate::typing::*;

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
            span: dummy_span(),
            id: "fst".to_owned(),
            type_args: TypeArgs::mk(vec![Ty::mk_i64(), Ty::mk_i64()]),
            args: vec![].into(),
            scrutinee: Rc::new(XVar::mk("x").into()),
            ty: None,
        }
        .check(&mut symbol_table, &ctx, &Ty::mk_i64())
        .unwrap();
        let expected = Destructor {
            span: dummy_span(),
            id: "fst".to_owned(),
            args: vec![].into(),
            type_args: TypeArgs::mk(vec![Ty::mk_i64(), Ty::mk_i64()]),
            scrutinee: Rc::new(
                XVar {
                    span: dummy_span(),
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
            span: dummy_span(),
            id: "apply".to_owned(),
            type_args: TypeArgs::mk(vec![Ty::mk_i64(), Ty::mk_i64()]),
            args: vec![Lit::mk(1).into(), XVar::mk("a").into()].into(),
            scrutinee: Rc::new(XVar::mk("x").into()),
            ty: None,
        }
        .check(&mut symbol_table, &ctx, &Ty::mk_i64())
        .unwrap();
        let expected = Destructor {
            span: dummy_span(),
            id: "apply".to_owned(),
            type_args: TypeArgs::mk(vec![Ty::mk_i64(), Ty::mk_i64()]),
            args: vec![
                Lit::mk(1).into(),
                XVar {
                    span: dummy_span(),
                    var: "a".to_owned(),
                    ty: Some(Ty::mk_i64()),
                    chi: Some(Cns),
                }
                .into(),
            ]
            .into(),
            scrutinee: Rc::new(
                XVar {
                    span: dummy_span(),
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
            span: dummy_span(),
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
            span: dummy_span(),
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
            span: dummy_span(),
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
            span: dummy_span(),
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

    /// Builds a symbol table containing `codata Const[A] { run[B](x: B) : A }`, i.e. a codata type
    /// with type parameter `A` whose single destructor has its own (universal) type parameter `B`
    /// for an argument unrelated to the return type.
    fn symbol_table_const_universal() -> SymbolTable {
        let mut symbol_table = SymbolTable::default();

        symbol_table.type_templates.insert(
            "Const".to_owned(),
            (
                Polarity::Codata,
                TypeParams::mk(&[("A", Polarity::Data)]),
                vec!["run".to_owned()],
            ),
        );

        symbol_table.dtor_templates.insert(
            "run".to_owned(),
            (
                TypeParams::mk(&[("B", Polarity::Data)]),
                TypingContext {
                    span: None,
                    bindings: vec![ContextBinding {
                        var: "x".to_owned(),
                        chi: Prd,
                        ty: Ty::mk_decl("B", TypeArgs::default()),
                    }],
                },
                Ty::mk_decl("A", TypeArgs::default()),
            ),
        );

        symbol_table
    }

    #[test]
    fn check_universal_dtor() {
        // "c.run[i64, i64](5)" where the first `i64` instantiates `A` (the codata type's own
        // parameter) and the second `i64` instantiates `B` (the destructor's own parameter).
        let mut ctx = TypingContext::default();
        ctx.add_var("c", Ty::mk_decl("Const", TypeArgs::mk(vec![Ty::mk_i64()])));

        let result = Destructor {
            span: dummy_span(),
            id: "run".to_owned(),
            type_args: TypeArgs::mk(vec![Ty::mk_i64(), Ty::mk_i64()]),
            args: vec![Lit::mk(5).into()].into(),
            scrutinee: Rc::new(XVar::mk("c").into()),
            ty: None,
        }
        .check(&mut symbol_table_const_universal(), &ctx, &Ty::mk_i64())
        .unwrap();

        let expected = Destructor {
            span: dummy_span(),
            id: "run".to_owned(),
            type_args: TypeArgs::mk(vec![Ty::mk_i64(), Ty::mk_i64()]),
            args: vec![Lit::mk(5).into()].into(),
            scrutinee: Rc::new(
                XVar {
                    span: dummy_span(),
                    var: "c".to_owned(),
                    ty: Some(Ty::mk_decl("Const", TypeArgs::mk(vec![Ty::mk_i64()]))),
                    chi: Some(Prd),
                }
                .into(),
            ),
            ty: Some(Ty::mk_i64()),
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn check_universal_dtor_wrong_arity() {
        // "c.run[i64](5)" is missing the type argument for the destructor's own parameter `B`.
        let mut ctx = TypingContext::default();
        ctx.add_var("c", Ty::mk_decl("Const", TypeArgs::mk(vec![Ty::mk_i64()])));

        let result = Destructor {
            span: dummy_span(),
            id: "run".to_owned(),
            type_args: TypeArgs::mk(vec![Ty::mk_i64()]),
            args: vec![Lit::mk(5).into()].into(),
            scrutinee: Rc::new(XVar::mk("c").into()),
            ty: None,
        }
        .check(&mut symbol_table_const_universal(), &ctx, &Ty::mk_i64());
        assert!(result.is_err())
    }
}
