//! This module contains the definition of top-level functions.

use std::collections::HashMap;

use derivative::Derivative;
use miette::SourceSpan;
use printer::tokens::{COLON, DEF};
use printer::*;

use crate::syntax::*;
use crate::typing::inference::{ConstraintBank, Inference};
use crate::typing::*;

/// This struct defines top-level function definitions. A top-level function consists of a name
/// (unique in the program), a typing context defining the parameters, a return type, and the body
/// term.
///
/// Example:
/// ```text
/// def fac(n: i64): i64 { if n == 0 { 1 } else { n * fac(n - 1) } }
/// ```
/// The top-level function named `fac` has a single (producer) parameter of type `i64` and returns
/// an `i64`. Its body is contained within `{...}`
#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct Def {
    /// The Source Location
    #[derivative(PartialEq = "ignore")]
    pub span: SourceSpan,
    /// The name of the definition
    pub name: Name,
    /// The parameters
    pub context: TypingContext,
    /// The return type
    pub ret_ty: Ty,
    /// The body term
    pub body: Term,
}

impl Def {
    pub fn gather_constraints(
        &mut self,
        constraint_bank: &mut ConstraintBank,
    ) -> Result<(), Error> {
        self.context.no_dups(&self.name)?;
        self.ret_ty
            .check(&Some(self.span), &mut constraint_bank.symbol_table)?;
        self.context.check(&mut constraint_bank.symbol_table)?;

        self.body
            .gather_constraints(constraint_bank, &self.context, self.ret_ty.clone())
    }

    pub fn insert_inferred_type(
        &mut self,
        mappings: &std::collections::HashMap<Name, Ty>,
        symbol_table: &mut SymbolTable,
        choices: &HashMap<u32, usize>,
    ) -> Result<(), Error> {
        self.body
            .insert_inferred_type(mappings, symbol_table, choices)
    }
}

impl Print for Def {
    fn print<'a>(&'a self, cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        let head = alloc
            .keyword(DEF)
            .append(alloc.space())
            .append(self.name.print(cfg, alloc))
            .append(self.context.print(cfg, alloc).parens())
            .append(COLON)
            .append(alloc.space())
            .append(self.ret_ty.print(cfg, alloc))
            .append(alloc.space());

        let body = alloc
            .hardline()
            .append(self.body.print(cfg, alloc).group())
            .nest(cfg.indent)
            .append(alloc.hardline())
            .braces_anno();

        head.group().append(body)
    }
}

impl From<Def> for Declaration {
    fn from(value: Def) -> Self {
        Declaration::Def(value)
    }
}

#[cfg(test)]
mod def_tests {
    use printer::Print;

    use crate::{
        parser::fun,
        syntax::{
            context::TypingContext,
            program::Program,
            terms::{Lit, Term},
            types::Ty,
            util::dummy_span,
        },
    };

    use super::Def;

    /// A definition with no arguments.
    fn simple_def() -> Def {
        Def {
            span: dummy_span(),
            name: "x".to_string(),
            context: TypingContext {
                span: None,
                bindings: vec![],
            },
            body: Term::Lit(Lit::mk(4)),
            ret_ty: Ty::mk_i64(),
        }
    }

    #[test]
    fn display_simple() {
        assert_eq!(
            simple_def().print_to_string(Default::default()),
            "def x(): i64 {\n    4\n}".to_string()
        )
    }

    #[test]
    fn parse_simple() {
        let parser = fun::ProgParser::new();
        let module = Program {
            declarations: vec![simple_def().into()],
        };
        assert_eq!(parser.parse("def x(): i64 { 4 }"), Ok(module));
    }
}
