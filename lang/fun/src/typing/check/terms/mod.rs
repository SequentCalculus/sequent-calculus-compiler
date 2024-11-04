use crate::{
    syntax::{
        context::{ContextBinding, TypingContext},
        substitution::{Substitution, SubstitutionBinding},
        terms::Term,
        types::Ty,
    },
    typing::{
        check::{check_annot, check_equality, lookup_covar},
        errors::Error,
        symbol_table::SymbolTable,
    },
};
use miette::SourceSpan;
use std::rc::Rc;

pub mod case;
pub mod cocase;
pub mod ctor;
pub mod dtor;
pub mod fun;
pub mod goto;
pub mod ifz;
pub mod label;
pub mod let_exp;
pub mod lit;
pub mod op;
pub mod paren;
pub mod var;

pub trait Check: Sized {
    fn check(
        self,
        symbol_table: &SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<Self, Error>;
}

impl Check for Term {
    fn check(
        self,
        symbol_table: &SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<Term, Error> {
        match self {
            Term::Var(var) => Ok(var.check(symbol_table, context, expected)?.into()),
            Term::Lit(lit) => Ok(lit.check(symbol_table, context, expected)?.into()),
            Term::Op(op) => Ok(op.check(symbol_table, context, expected)?.into()),
            Term::IfZ(if_z) => Ok(if_z.check(symbol_table, context, expected)?.into()),
            Term::Let(letexp) => Ok(letexp.check(symbol_table, context, expected)?.into()),
            Term::Fun(fun) => Ok(fun.check(symbol_table, context, expected)?.into()),
            Term::Constructor(constructor) => {
                Ok(constructor.check(symbol_table, context, expected)?.into())
            }
            Term::Destructor(destructor) => {
                Ok(destructor.check(symbol_table, context, expected)?.into())
            }
            Term::Case(case) => Ok(case.check(symbol_table, context, expected)?.into()),
            Term::Cocase(cocase) => Ok(cocase.check(symbol_table, context, expected)?.into()),
            Term::Goto(goto) => Ok(goto.check(symbol_table, context, expected)?.into()),
            Term::Label(label) => Ok(label.check(symbol_table, context, expected)?.into()),
            Term::Paren(paren) => Ok(paren.check(symbol_table, context, expected)?.into()),
        }
    }
}

impl<T: Check + Clone> Check for Rc<T> {
    fn check(
        self,
        symbol_table: &SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<Rc<T>, Error> {
        let t_checked = Rc::unwrap_or_clone(self).check(symbol_table, context, expected)?;
        Ok(Rc::new(t_checked))
    }
}

pub fn check_args(
    span: &SourceSpan,
    symbol_table: &SymbolTable,
    context: &TypingContext,
    args: Substitution,
    types: &TypingContext,
) -> Result<Substitution, Error> {
    if types.len() != args.len() {
        return Err(Error::WrongNumberOfArguments {
            span: *span,
            expected: types.len(),
            got: args.len(),
        });
    }
    let mut new_subst = vec![];
    for c in args.into_iter().zip(types.iter()) {
        match c {
            (
                SubstitutionBinding::TermBinding { term, ty: subst_ty },
                ContextBinding::TypedVar { ty: var_ty, .. },
            ) => {
                check_annot(var_ty, &subst_ty, span)?;
                let term_checked = term.check(symbol_table, context, var_ty)?;
                let new_binding = SubstitutionBinding::TermBinding {
                    term: term_checked,
                    ty: Some(var_ty.clone()),
                };
                new_subst.push(new_binding)
            }
            (
                SubstitutionBinding::CovarBinding {
                    covar,
                    ty: subst_ty,
                },
                ContextBinding::TypedCovar { ty: covar_ty, .. },
            ) => {
                check_annot(covar_ty, &subst_ty, span)?;
                let found_ty = lookup_covar(span, context, &covar)?;
                check_equality(span, covar_ty, &found_ty)?;
                let new_binding = SubstitutionBinding::CovarBinding {
                    covar,
                    ty: Some(found_ty),
                };
                new_subst.push(new_binding)
            }
            (SubstitutionBinding::CovarBinding { .. }, ContextBinding::TypedVar { .. }) => {
                return Err(Error::ExpectedTermGotCovariable { span: *span })
            }
            (SubstitutionBinding::TermBinding { .. }, ContextBinding::TypedCovar { .. }) => {
                return Err(Error::ExpectedCovariableGotTerm { span: *span })
            }
        }
    }
    Ok(new_subst)
}
