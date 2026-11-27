//! This module defines the control operator for invoking a captured continuation/program context
//! in Fun.

use derivative::Derivative;
use miette::SourceSpan;
use printer::tokens::GOTO;
use printer::*;

use crate::syntax::*;
use crate::traits::*;
use crate::typing::inference::TypeInference;
use crate::typing::inference::TypeInferenceState;
use crate::typing::inference::{TypeVarMapping, World};
use crate::typing::*;

use std::{collections::HashSet, rc::Rc};

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
    pub span: SourceSpan,
    /// The covariable for the continuation
    pub target: Covar,
    /// The argument
    pub term: Rc<Term>,
    /// The (inferred) type of the term
    pub ty: Option<Ty>,
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

impl TypeInference for Goto {
    fn collect_constraints(
        &mut self,
        state: &mut TypeInferenceState,
        context: &TypingContext,
        expected_ty: Ty,
    ) -> Result<(), Error> {
        let continuation_type = context.lookup_covar(&self.target, &self.span)?;
        self.ty = Some(expected_ty);

        self.term
            .collect_constraints(state, context, continuation_type)
    }

    fn insert_inferred_types(
        &mut self,
        type_vars: &TypeVarMapping,
        symbol_table: &mut SymbolTable,
        world: &World,
    ) -> Result<(), Error> {
        self.term
            .insert_inferred_types(type_vars, symbol_table, world)?;

        match &mut self.ty {
            Some(ty) => {
                ty.mut_subst_ty(type_vars);
                ty.check(&Some(self.span), symbol_table)
            }
            None => panic!(
                "The Type of the term {:?} is not set after type inference",
                self
            ),
        }
    }
}

impl UsedBinders for Goto {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        self.term.used_binders(used);
    }
}

#[cfg(test)]
mod test {
    use printer::Print;

    use crate::parser::fun;
    use crate::syntax::util::dummy_span;
    use crate::syntax::*;
    use crate::typing::inference::TypeInference;
    use crate::typing::inference::TypeInferenceState;
    use crate::typing::inference::VariationalConstraint;

    use std::rc::Rc;

    #[test]
    fn inference_goto() {
        let mut ctx = TypingContext::default();
        ctx.add_covar("a", Ty::mk_i64());
        let mut term = Goto {
            span: dummy_span(),
            target: "a".to_owned(),
            term: Rc::new(Lit::mk(1).into()),
            ty: None,
        };

        let mut state = TypeInferenceState {
            symbol_table: Default::default(),
            fresh_names: Default::default(),
            constraints: Default::default(),
            choices: Default::default(),
        };

        term.collect_constraints(&mut state, &ctx, Ty::mk_ty_var("x"))
            .unwrap();

        let expected = vec![VariationalConstraint::equal(Ty::mk_i64(), Ty::mk_i64())];

        let TypeInferenceState {
            constraints: result,
            ..
        } = state;

        assert_eq!(result, expected)
    }

    fn example() -> Goto {
        Goto {
            span: dummy_span(),
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
