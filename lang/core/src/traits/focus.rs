use crate::{
    syntax::{
        context::{ContextBinding, TypingContext},
        declaration::CodataDeclaration,
        substitution::SubstitutionBinding,
        Covar, Var,
    },
    traits::free_vars::{fresh_covar, fresh_var},
};

use std::collections::{HashSet, VecDeque};
use std::rc::Rc;

#[derive(Default)]
pub struct FocusingState<'a> {
    pub used_vars: HashSet<Var>,
    pub used_covars: HashSet<Covar>,
    pub codata_types: &'a [CodataDeclaration],
}

impl FocusingState<'_> {
    pub fn fresh_var(&mut self) -> Var {
        let new_var = fresh_var(&self.used_vars);
        self.used_vars.insert(new_var.clone());
        new_var
    }

    pub fn fresh_covar(&mut self) -> Covar {
        let new_covar = fresh_covar(&self.used_covars);
        self.used_covars.insert(new_covar.clone());
        new_covar
    }

    pub fn add_context(&mut self, context: &TypingContext) {
        for binding in context.iter() {
            match binding {
                ContextBinding::VarBinding { var, ty: _ } => {
                    self.used_vars.insert(var.clone());
                }
                ContextBinding::CovarBinding { covar, ty: _ } => {
                    self.used_covars.insert(covar.clone());
                }
            }
        }
    }
}

pub trait Focusing {
    type Target;
    fn focus(self, state: &mut FocusingState) -> Self::Target;
}

impl<T: Focusing + Clone> Focusing for Rc<T> {
    type Target = Rc<T::Target>;
    fn focus(self, state: &mut FocusingState) -> Self::Target {
        Rc::new(Rc::unwrap_or_clone(self).focus(state))
    }
}

impl<T: Focusing> Focusing for Vec<T> {
    type Target = Vec<T::Target>;
    fn focus(self, state: &mut FocusingState) -> Self::Target {
        self.into_iter().map(|x| x.focus(state)).collect()
    }
}

pub type Continuation =
    Box<dyn FnOnce(crate::syntax_var::Name, &mut FocusingState) -> crate::syntax_var::Statement>;
pub type ContinuationVec = Box<
    dyn FnOnce(
        VecDeque<crate::syntax_var::Name>,
        &mut FocusingState,
    ) -> crate::syntax_var::Statement,
>;

pub trait Bind: Sized {
    fn bind(self, k: Continuation, state: &mut FocusingState) -> crate::syntax_var::Statement;
}

pub fn bind_many(
    mut args: VecDeque<SubstitutionBinding>,
    k: ContinuationVec,
    state: &mut FocusingState,
) -> crate::syntax_var::Statement {
    match args.pop_front() {
        None => k(VecDeque::new(), state),
        Some(SubstitutionBinding::ProducerBinding(prd)) => prd.bind(
            Box::new(|name, state| {
                bind_many(
                    args,
                    Box::new(|mut names, state| {
                        names.push_front(name);
                        k(names, state)
                    }),
                    state,
                )
            }),
            state,
        ),
        Some(SubstitutionBinding::ConsumerBinding(cns)) => cns.bind(
            Box::new(|name, state| {
                bind_many(
                    args,
                    Box::new(|mut names, state| {
                        names.push_front(name);
                        k(names, state)
                    }),
                    state,
                )
            }),
            state,
        ),
    }
}
