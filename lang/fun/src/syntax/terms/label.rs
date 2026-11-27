//! This module defines the control operator for capturing current continuation/program context in
//! Fun.

use derivative::Derivative;
use miette::SourceSpan;
use printer::tokens::LABEL;
use printer::*;

use crate::syntax::*;
use crate::traits::*;
use crate::typing::inference::TypeInference;
use crate::typing::inference::TypeInferenceState;
use crate::typing::inference::{TypeVarMapping, World};
use crate::typing::*;

use std::{collections::HashSet, rc::Rc};

/// This struct defines the control operator capturing the current continuation/program context. It
/// consists of a covariable to which the continuation is bound, the body in which the continuation
/// is available, and after typechecking also of the inferred type.
///
/// Example:
/// `label a { goto a (5)}` captures the current continuation, binds it to covariable `a` and
/// invokes it with argument `5`.
#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct Label {
    // The source location
    #[derivative(PartialEq = "ignore")]
    pub span: SourceSpan,
    /// The covariable to which the continuation is bound
    pub label: Covar,
    /// The body in which the continuation is in scope
    pub term: Rc<Term>,
    /// The (inferred) type of the term
    pub ty: Option<Ty>,
}

impl OptTyped for Label {
    fn get_type(&self) -> Option<Ty> {
        self.ty.clone()
    }
}

impl Print for Label {
    fn print<'a>(&'a self, cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        alloc
            .keyword(LABEL)
            .append(alloc.space())
            .append(self.label.clone())
            .append(alloc.space())
            .append(
                alloc
                    .line()
                    .append(self.term.print(cfg, alloc).group())
                    .nest(cfg.indent)
                    .append(alloc.line())
                    .braces_anno()
                    .group(),
            )
    }
}
impl From<Label> for Term {
    fn from(value: Label) -> Self {
        Term::Label(value)
    }
}

impl TypeInference for Label {
    fn collect_constraints(
        &mut self,
        state: &mut TypeInferenceState,
        context: &TypingContext,
        expected_ty: Ty,
    ) -> Result<(), Error> {
        let mut new_context = context.clone();
        new_context.add_covar(&self.label, expected_ty.clone());

        self.ty = Some(expected_ty.clone());

        self.term
            .collect_constraints(state, &new_context, expected_ty)?;

        Ok(())
    }

    fn insert_inferred_types(
        &mut self,
        type_vars: &TypeVarMapping,
        symbol_table: &mut SymbolTable,
        world: &World,
    ) -> Result<(), Error> {
        self.term
            .insert_inferred_types(type_vars, symbol_table, world)?;

        Ty::insert_inferred_annotation(&mut self.ty, self.span, type_vars, symbol_table)
    }
}

impl UsedBinders for Label {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        used.insert(self.label.clone());
        self.term.used_binders(used);
    }
}

#[cfg(test)]
mod test {
    use printer::Print;

    use crate::parser::fun;
    use crate::syntax::util::dummy_span;
    use crate::syntax::*;
    use crate::typing::inference::{TypeInference, TypeInferenceState, VariationalConstraint};

    use std::rc::Rc;

    #[test]
    fn inference_label() {
        let mut term = Label {
            span: dummy_span(),
            label: "a".to_owned(),
            ty: None,
            term: Rc::new(Lit::mk(1).into()),
        };

        let mut state = TypeInferenceState {
            symbol_table: Default::default(),
            fresh_names: Default::default(),
            constraints: Default::default(),
            choices: Default::default(),
        };

        term.collect_constraints(&mut state, &TypingContext::default(), Ty::mk_ty_var("x"))
            .unwrap();

        let expected = vec![VariationalConstraint::equal(
            Ty::mk_ty_var("x"),
            Ty::mk_i64(),
        )];

        let TypeInferenceState {
            constraints: result,
            ..
        } = state;

        assert_eq!(result, expected);
        assert_eq!(term.ty, Some(Ty::mk_ty_var("x")));
    }

    fn example() -> Label {
        Label {
            span: dummy_span(),
            label: "x".to_string(),
            term: Rc::new(Term::Lit(Lit::mk(2))),
            ty: None,
        }
    }

    #[test]
    fn parse() {
        let parser = fun::TermParser::new();
        assert_eq!(parser.parse("label x { 2 }"), Ok(example().into()));
    }

    #[test]
    fn display() {
        assert_eq!(
            example().print_to_string(Default::default()),
            "label x { 2 }"
        )
    }
}
