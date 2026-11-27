//! This module defines let-bindings of a term in Fun.

use derivative::Derivative;
use miette::SourceSpan;
use printer::tokens::{COLON, EQ, LET, SEMI};
use printer::*;

use crate::syntax::*;
use crate::traits::*;
use crate::typing::inference::{TypeInference, TypeInferenceState};
use crate::typing::inference::{TypeVarMapping, World};
use crate::typing::*;

use std::{collections::HashSet, rc::Rc};

/// This struct defines let-bindings of a term. It consists of the variable the term is bound to,
/// the annotated type of the bound term, the bound term, the remaining term in which the binding
/// is visible, and after typechecking also of the inferred type of the entire term.
///
/// Example:
/// ```text
/// let x: i64 = 2 * 2; x
/// ```
/// This binds the variable `x` of type `i64` to `2 * 2` and then returns it.
#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct Let {
    /// The source location
    #[derivative(PartialEq = "ignore")]
    pub span: SourceSpan,
    /// The bound variable
    pub variable: Var,
    /// The (annotated) type of the bound term
    pub var_ty: Option<Ty>,
    /// The bound term
    pub bound_term: Rc<Term>,
    /// The term in which the variable for the bound term is in scope
    pub in_term: Rc<Term>,
    /// The (inferred) type of the entire term
    pub ty: Option<Ty>,
}

impl OptTyped for Let {
    fn get_type(&self) -> Option<Ty> {
        self.ty.clone()
    }
}

impl Print for Let {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        if self.var_ty.is_some() {
            alloc
                .keyword(LET)
                .append(alloc.space())
                .append(self.variable.print(cfg, alloc))
                .append(COLON)
                .append(alloc.space())
                .append(self.var_ty.print(cfg, alloc))
                .append(alloc.space())
                .append(EQ)
                .append(alloc.space())
                .append(self.bound_term.print(cfg, alloc).group())
                .append(SEMI)
                .append(alloc.hardline())
                .append(self.in_term.print(cfg, alloc).group())
        } else {
            alloc
                .keyword(LET)
                .append(alloc.space())
                .append(self.variable.print(cfg, alloc))
                .append(EQ)
                .append(alloc.space())
                .append(self.bound_term.print(cfg, alloc).group())
                .append(SEMI)
                .append(alloc.hardline())
                .append(self.in_term.print(cfg, alloc).group())
        }
    }
}

impl From<Let> for Term {
    fn from(value: Let) -> Self {
        Term::Let(value)
    }
}

impl TypeInference for Let {
    fn collect_constraints(
        &mut self,
        state: &mut TypeInferenceState,
        context: &TypingContext,
        expected_ty: Ty,
    ) -> Result<(), Error> {
        self.ty = Some(expected_ty.clone());

        // if the bound term has an annotation it is used, else a type variable is substituted
        let bound_term_type = match &self.var_ty {
            Some(ty) => ty.clone(),
            None => {
                let new_ty = state.fresh_names.fresh_ty_var();
                self.var_ty = Some(new_ty.clone());
                new_ty
            }
        };

        self.bound_term
            .collect_constraints(state, context, bound_term_type.clone())?;

        let mut new_context = context.clone();
        new_context.add_var(&self.variable, bound_term_type);
        self.in_term
            .collect_constraints(state, &new_context, expected_ty)?;

        Ok(())
    }

    fn insert_inferred_types(
        &mut self,
        type_vars: &TypeVarMapping,
        symbol_table: &mut SymbolTable,
        world: &World,
    ) -> Result<(), Error> {
        self.bound_term
            .insert_inferred_types(type_vars, symbol_table, world)?;
        self.in_term
            .insert_inferred_types(type_vars, symbol_table, world)?;

        Ty::insert_inferred_annotation(&mut self.var_ty, self.span, type_vars, symbol_table)?;
        Ty::insert_inferred_annotation(&mut self.ty, self.span, type_vars, symbol_table)
    }
}

impl UsedBinders for Let {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        used.insert(self.variable.clone());
        self.bound_term.used_binders(used);
        self.in_term.used_binders(used);
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
    fn inference_let() {
        let mut term = Let {
            span: dummy_span(),
            variable: "x".to_owned(),
            var_ty: None,
            bound_term: Rc::new(Lit::mk(2).into()),
            in_term: Rc::new(XVar::mk("x").into()),
            ty: None,
        };

        let mut state = TypeInferenceState {
            symbol_table: Default::default(),
            fresh_names: Default::default(),
            constraints: Default::default(),
            choices: Default::default(),
        };

        term.collect_constraints(&mut state, &TypingContext::default(), Ty::mk_ty_var("x"))
            .unwrap();

        let expected = vec![
            VariationalConstraint::equal(Ty::mk_ty_var("0"), Ty::mk_i64()),
            VariationalConstraint::equal(Ty::mk_ty_var("x"), Ty::mk_ty_var("0")),
        ];

        let TypeInferenceState {
            constraints: result,
            ..
        } = state;

        assert_eq!(result, expected);
        assert_eq!(term.ty, Some(Ty::mk_ty_var("x")));
    }

    #[test]
    fn inference_let_type_annotation() {
        let mut term = Let {
            span: dummy_span(),
            variable: "x".to_owned(),
            var_ty: Some(Ty::mk_i64()),
            bound_term: Rc::new(Lit::mk(2).into()),
            in_term: Rc::new(XVar::mk("x").into()),
            ty: None,
        };

        let mut state = TypeInferenceState {
            symbol_table: Default::default(),
            fresh_names: Default::default(),
            constraints: Default::default(),
            choices: Default::default(),
        };

        term.collect_constraints(&mut state, &TypingContext::default(), Ty::mk_ty_var("x"))
            .unwrap();

        let expected = vec![
            VariationalConstraint::equal(Ty::mk_i64(), Ty::mk_i64()),
            VariationalConstraint::equal(Ty::mk_ty_var("x"), Ty::mk_i64()),
        ];

        let TypeInferenceState {
            constraints: result,
            ..
        } = state;

        assert_eq!(result, expected);
        assert_eq!(term.ty, Some(Ty::mk_ty_var("x")));
    }

    fn example() -> Let {
        Let {
            span: dummy_span(),
            variable: "x".to_string(),
            var_ty: Some(Ty::mk_i64()),
            bound_term: Rc::new(Term::Lit(Lit::mk(2))),
            in_term: Rc::new(Term::Lit(Lit::mk(4))),
            ty: None,
        }
    }

    #[test]
    fn display() {
        assert_eq!(
            example().print_to_string(Default::default()),
            "let x: i64 = 2;\n4"
        )
    }

    #[test]
    fn parse() {
        let parser = fun::TermParser::new();
        assert_eq!(parser.parse("let x: i64 = 2; 4"), Ok(example().into()));
    }
}
