//! This module defines the call of a top-level function in Fun.

use derivative::Derivative;
use miette::SourceSpan;
use printer::*;

use crate::syntax::*;
use crate::traits::*;
use crate::typing::*;

use std::collections::HashSet;

/// This struct defines the call of a top-level function in Fun. It consists of the name of the
/// top-level function to call, the type arguments, the arguments, and after typechecking also the inferred type.
///
/// Examples:
///
/// `fac(10)`, calls the top-level function `fac` with argument `10`.
///
/// `id[i64](x)` calls the top-level function `id` with type argument `i64` and argument `x`.
#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct Call {
    /// The source location
    #[derivative(PartialEq = "ignore")]
    pub span: SourceSpan,
    /// The name of the top-level function being called
    pub name: Name,
    /// The type arguments
    pub type_args: TypeArgs,
    /// The arguments
    pub args: Arguments,
    /// The (inferred) return type
    pub ret_ty: Option<Ty>,
}

impl OptTyped for Call {
    fn get_type(&self) -> Option<Ty> {
        self.ret_ty.clone()
    }
}

impl Print for Call {
    fn print<'a>(&'a self, cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        self.name
            .print(cfg, alloc)
            .append(self.type_args.print(cfg, alloc))
            .append(self.args.print(cfg, alloc).parens().group())
    }
}

impl From<Call> for Term {
    fn from(value: Call) -> Self {
        Term::Call(value)
    }
}

impl Check for Call {
    fn check(
        mut self,
        symbol_table: &mut SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<Self, Error> {
        let (types, ret_ty) =
            symbol_table.instantiate_def_signature(Some(self.span), &self.name, &self.type_args)?;

        check_equality(&self.span, symbol_table, expected, &ret_ty)?;
        self.args = check_args(&self.span, symbol_table, context, self.args, &types)?;
        self.ret_ty = Some(expected.clone());
        Ok(self)
    }
}

impl UsedBinders for Call {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        self.args.entries.used_binders(used);
    }
}

#[cfg(test)]
mod test {
    use printer::Print;

    use crate::parser::fun;
    use crate::syntax::util::dummy_span;
    use crate::syntax::*;
    use crate::test_common::*;
    use crate::typing::*;

    #[test]
    fn check_mult() {
        let mut symbol_table = symbol_table_list();
        let mut ctx = TypingContext::default();
        ctx.add_var("l", Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_i64()])));
        symbol_table.defs.insert(
            "mult".to_owned(),
            (TypeParams::default(), ctx.clone(), Ty::mk_i64()),
        );
        let result = def_mult()
            .body
            .check(&mut symbol_table, &ctx, &Ty::mk_i64())
            .unwrap();
        let expected = def_mult_typed().body;
        assert_eq!(result, expected)
    }

    #[test]
    fn check_call_fail() {
        let result = Call {
            span: dummy_span(),
            name: "main".to_owned(),
            type_args: TypeArgs::default(),
            args: vec![].into(),
            ret_ty: None,
        }
        .check(
            &mut SymbolTable::default(),
            &TypingContext {
                span: None,
                bindings: vec![],
            },
            &Ty::mk_i64(),
        );
        assert!(result.is_err())
    }

    #[test]
    fn check_poly_call_missing_type_args_fails() {
        let mut symbol_table = SymbolTable::default();
        let mut poly_ctx = TypingContext::default();
        poly_ctx.add_var("x", Ty::mk_decl("A", TypeArgs::default()));
        symbol_table.defs.insert(
            "id".to_owned(),
            (
                TypeParams::mk(&[("A", Polarity::Data)]),
                poly_ctx,
                Ty::mk_decl("A", TypeArgs::default()),
            ),
        );

        let result = Call {
            span: dummy_span(),
            name: "id".to_owned(),
            type_args: TypeArgs::default(),
            args: vec![Term::Lit(Lit::mk(1)).into()].into(),
            ret_ty: None,
        }
        .check(&mut symbol_table, &TypingContext::default(), &Ty::mk_i64());

        assert!(result.is_err());
    }

    #[test]
    fn check_mono_call_with_type_args_fails() {
        let mut symbol_table = SymbolTable::default();
        let mut mono_ctx = TypingContext::default();
        mono_ctx.add_var("x", Ty::mk_i64());
        symbol_table.defs.insert(
            "id".to_owned(),
            (TypeParams::default(), mono_ctx, Ty::mk_i64()),
        );

        let result = Call {
            span: dummy_span(),
            name: "id".to_owned(),
            type_args: TypeArgs::mk(vec![Ty::mk_i64()]),
            args: vec![Term::Lit(Lit::mk(1)).into()].into(),
            ret_ty: None,
        }
        .check(&mut symbol_table, &TypingContext::default(), &Ty::mk_i64());

        assert!(result.is_err());
    }

    fn example_simple() -> Call {
        Call {
            span: dummy_span(),
            name: "foo".to_string(),
            type_args: TypeArgs::default(),
            args: vec![].into(),
            ret_ty: None,
        }
    }

    fn example_id() -> Call {
        Call {
            span: dummy_span(),
            name: "id".to_string(),
            type_args: TypeArgs::mk(vec![Ty::mk_i64()]),
            args: vec![XVar::mk("x").into()].into(),
            ret_ty: None,
        }
    }

    #[test]
    fn display_simple() {
        assert_eq!(
            example_simple().print_to_string(Default::default()),
            "foo()"
        )
    }

    #[test]
    fn display_id() {
        assert_eq!(
            example_id().print_to_string(Default::default()),
            "id[i64](x)"
        )
    }

    #[test]
    fn parse_simple() {
        let parser = fun::TermParser::new();
        assert_eq!(parser.parse("foo()"), Ok(example_simple().into()));
    }

    #[test]
    fn parse_id() {
        let parser = fun::TermParser::new();
        assert_eq!(parser.parse("id[i64](x)"), Ok(example_id().into()));
    }

    fn example_extended() -> Call {
        Call {
            span: dummy_span(),
            name: "foo".to_string(),
            type_args: TypeArgs::default(),
            args: vec![Term::Lit(Lit::mk(2)).into(), XVar::mk("a").into()].into(),
            ret_ty: None,
        }
    }

    #[test]
    fn display_extended() {
        assert_eq!(
            example_extended().print_to_string(Default::default()),
            "foo(2, a)"
        )
    }

    #[test]
    fn parse_extended() {
        let parser = fun::TermParser::new();
        assert_eq!(parser.parse("foo(2, a)"), Ok(example_extended().into()));
    }
}
