//! This module defines the control operator for capturing current continuation/program context in
//! Fun.

use derivative::Derivative;
use miette::SourceSpan;
use printer::tokens::LABEL;
use printer::*;

use crate::syntax::*;
use crate::traits::*;
use crate::typing::inference::ConstraintBank;
use crate::typing::inference::Inference;
use crate::typing::*;

use std::collections::HashMap;
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

impl Inference for Label {
    fn gather_constraints(
        &mut self,
        constraint_bank: &mut ConstraintBank,
        context: &TypingContext,
        ty_var: Ty,
    ) -> Result<(), Error> {
        let mut new_context = context.clone();
        new_context.add_covar(&self.label, ty_var.clone());

        self.ty = Some(ty_var.clone());

        self.term
            .gather_constraints(constraint_bank, &new_context, ty_var)?;

        Ok(())
    }

    fn insert_inferred_type(
        &mut self,
        mappings: &HashMap<Name, Ty>,
        symbol_table: &mut SymbolTable,
        choices: &HashMap<u32, usize>,
    ) -> Result<(), Error> {
        self.term
            .insert_inferred_type(mappings, symbol_table, choices)?;

        match &mut self.ty {
            Some(ty_var) => {
                ty_var.mut_subst_ty(mappings);
                ty_var.check(&Some(self.span), symbol_table)
            }
            None => panic!(
                "The Type of the term {:?} is not set after type inference",
                self
            ),
        }
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
    use crate::typing::inference::{Constraint, ConstraintBank, Inference};

    use std::rc::Rc;

    #[test]
    fn inference_label() {
        let mut term = Label {
            span: dummy_span(),
            label: "a".to_owned(),
            ty: None,
            term: Rc::new(Lit::mk(1).into()),
        };

        let mut constraint_bank = ConstraintBank {
            symbol_table: Default::default(),
            var_name_generator: Default::default(),
            constraints: Default::default(),
            possible_choices: Default::default(),
        };

        term.gather_constraints(
            &mut constraint_bank,
            &TypingContext::default(),
            Ty::mk_ty_var("x"),
        )
        .unwrap();

        let expected = vec![Constraint::mk_only_ty(Ty::mk_ty_var("x"), Ty::mk_i64())];

        let ConstraintBank {
            constraints: result,
            ..
        } = constraint_bank;

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
