//! This module defines the control operator for invoking a captured continuation/program context
//! in Fun.

use codespan::Span;
use derivative::Derivative;
use printer::tokens::GOTO;
use printer::*;

use crate::parser::util::ToMiette;
use crate::syntax::*;
use crate::traits::*;
use crate::typing::*;

use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

/// This struct defines the control operator for invoking a captured continuation/program context
/// by control operator [`label`](crate::syntax::terms::Label). It consists of a covariable to
/// which the continuation is bound, the argument of the invocation, and after typechecking also of
/// the inferred type, which can be arbitrary.
///
/// Example:
/// `goto a (0)` invokes the continuation bound to `a` with argument `0`.
#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct Goto {
    /// The source location
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    /// The covariable for the continuation
    pub target: Covar,
    /// The argument
    pub term: Rc<Term>,
    /// The (inferred) type of the term
    pub ty: Option<Ty>,
}

impl Goto {
    pub fn subst_ty(mut self, mappings: &HashMap<Name, Ty>) -> Self {
        self.term = Rc::new(Rc::unwrap_or_clone(self.term).subst_ty(mappings));
        self.ty = self.ty.map(|ty| ty.subst_ty(mappings));
        self
    }
}

impl OptTyped for Goto {
    fn get_type(&self) -> Option<Ty> {
        self.ty.clone()
    }
}

impl Print for Goto {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc
            .keyword(GOTO)
            .append(alloc.space())
            .append(self.target.print(cfg, alloc))
            .append(alloc.space())
            .append(
                alloc
                    .line_()
                    .append(self.term.print(cfg, alloc).group())
                    .nest(cfg.indent)
                    .append(alloc.line_())
                    .parens()
                    .group(),
            )
    }
}

impl From<Goto> for Term {
    fn from(value: Goto) -> Self {
        Term::Goto(value)
    }
}

impl Check for Goto {
    fn check(
        mut self,
        symbol_table: &mut SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<Self, Error> {
        let cont_type = context.lookup_covar(&self.target, &self.span.to_miette())?;
        self.term = self.term.check(symbol_table, context, &cont_type)?;

        self.ty = Some(expected.clone());
        Ok(self)
    }
}

impl UsedBinders for Goto {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        self.term.used_binders(used);
    }
}

#[cfg(test)]
mod test {
    use codespan::Span;
    use printer::Print;

    use crate::parser::fun;
    use crate::syntax::*;
    use crate::typing::*;

    use std::rc::Rc;

    #[test]
    fn check_goto() {
        let mut ctx = TypingContext::default();
        ctx.add_covar("a", Ty::mk_i64());
        let result = Goto {
            span: Span::default(),
            target: "a".to_owned(),
            term: Rc::new(Lit::mk(1).into()),
            ty: None,
        }
        .check(&mut SymbolTable::default(), &ctx, &Ty::mk_i64())
        .unwrap();
        let expected = Goto {
            span: Span::default(),
            target: "a".to_owned(),
            term: Rc::new(Lit::mk(1).into()),
            ty: Some(Ty::mk_i64()),
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn check_goto_fail() {
        let result = Goto {
            span: Span::default(),
            target: "a".to_owned(),
            term: Rc::new(Lit::mk(1).into()),
            ty: None,
        }
        .check(
            &mut SymbolTable::default(),
            &TypingContext::default(),
            &Ty::mk_i64(),
        );
        assert!(result.is_err())
    }

    fn example() -> Goto {
        Goto {
            span: Span::default(),
            target: "x".to_string(),
            term: Rc::new(Term::Lit(Lit::mk(2))),
            ty: None,
        }
    }

    #[test]
    fn display() {
        assert_eq!(example().print_to_string(Default::default()), "goto x (2)")
    }

    #[test]
    fn parse() {
        let parser = fun::TermParser::new();
        assert_eq!(parser.parse("goto x (2)"), Ok(example().into()));
    }
}
