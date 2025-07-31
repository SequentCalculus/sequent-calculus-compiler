//! This module defines the call of a top-level function in Fun.

use codespan::Span;
use derivative::Derivative;
use printer::{DocAllocator, Print};

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

/// This struct defines the call of a top-level function in Fun. It consists of the name of the
/// top-level function to call, the arguments, and after typechecking also the inferred type.
///
/// Example:
/// `fac(10)`, calls the top-level function `fac` with argument `10`.
#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct Call {
    /// The source location
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    /// The name of the top-level function being called
    pub name: Name,
    /// The arguments
    pub args: Substitution,
    /// The (inferred) return type
    pub ret_ty: Option<Ty>,
}

impl OptTyped for Call {
    fn get_type(&self) -> Option<Ty> {
        self.ret_ty.clone()
    }
}

impl Print for Call {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc
            .text(self.name.clone())
            .append(self.args.print(cfg, alloc).parens())
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
        match symbol_table.defs.get(&self.name) {
            Some(signature) => {
                let (types, ret_ty) = signature.clone();
                check_equality(&self.span, symbol_table, expected, &ret_ty)?;

                self.args = check_args(
                    &self.span.to_miette(),
                    symbol_table,
                    context,
                    self.args,
                    &types,
                )?;

                self.ret_ty = Some(expected.clone());
                Ok(self)
            }
            None => Err(Error::Undefined {
                span: self.span.to_miette(),
                name: self.name.clone(),
            }),
        }
    }
}

impl UsedBinders for Call {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        self.args.used_binders(used);
    }
}

#[cfg(test)]
mod test {
    use super::{Call, Check, Term};
    use crate::{
        parser::fun,
        syntax::{
            context::TypingContext,
            terms::{Lit, XVar},
            types::{Ty, TypeArgs},
        },
        test_common::{def_mult, def_mult_typed, symbol_table_list},
        typing::symbol_table::SymbolTable,
    };
    use codespan::Span;
    use printer::Print;

    #[test]
    fn check_mult() {
        let mut symbol_table = symbol_table_list();
        let mut ctx = TypingContext::default();
        ctx.add_var("l", Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_i64()])));
        symbol_table
            .defs
            .insert("mult".to_owned(), (ctx.clone(), Ty::mk_i64()));
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
            span: Span::default(),
            name: "main".to_owned(),
            args: vec![],
            ret_ty: None,
        }
        .check(
            &mut SymbolTable::default(),
            &TypingContext {
                span: Span::default(),
                bindings: vec![],
            },
            &Ty::mk_i64(),
        );
        assert!(result.is_err())
    }

    fn example_simple() -> Call {
        Call {
            span: Span::default(),
            name: "foo".to_string(),
            args: vec![],
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
    fn parse_simple() {
        let parser = fun::TermParser::new();
        assert_eq!(parser.parse("foo()"), Ok(example_simple().into()));
    }

    fn example_extended() -> Call {
        Call {
            span: Span::default(),
            name: "foo".to_string(),
            args: vec![Term::Lit(Lit::mk(2)).into(), XVar::mk("a").into()],
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
