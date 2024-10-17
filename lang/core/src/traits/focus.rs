use crate::{
    syntax::{
        context::{ContextBinding, TypingContext},
        substitution::SubstitutionBinding,
        term::{Cns, Prd, XVar},
        Covar, Name, Statement, Var,
    },
    traits::free_vars::{fresh_covar, fresh_var},
};
use std::collections::{HashSet, VecDeque};
use std::rc::Rc;

#[derive(Default)]
pub struct FocusingState {
    pub used_vars: HashSet<Var>,
    pub used_covars: HashSet<Covar>,
}

impl FocusingState {
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

    pub fn add_context(&mut self, ctx: &TypingContext) {
        for bnd in ctx.iter() {
            match bnd {
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

pub type Continuation = Box<dyn FnOnce(Name, &mut FocusingState) -> Statement>;
pub type ContinuationVec =
    Box<dyn FnOnce(VecDeque<SubstitutionBinding>, &mut FocusingState) -> Statement>;

pub trait Bind: Sized {
    fn bind(self, k: Continuation, state: &mut FocusingState) -> Statement;
}

pub fn bind_many(
    mut args: VecDeque<SubstitutionBinding>,
    k: ContinuationVec,
    state: &mut FocusingState,
) -> Statement {
    match args.pop_front() {
        None => k(VecDeque::new(), state),
        Some(SubstitutionBinding::ProducerBinding(p)) => p.bind(
            Box::new(|name, state| {
                bind_many(
                    args,
                    Box::new(|mut names, state| {
                        names.push_front(SubstitutionBinding::ProducerBinding(
                            XVar {
                                prdcns: Prd,
                                var: name,
                            }
                            .into(),
                        ));
                        k(names, state)
                    }),
                    state,
                )
            }),
            state,
        ),
        Some(SubstitutionBinding::ConsumerBinding(c)) => c.bind(
            Box::new(|name, state| {
                bind_many(
                    args,
                    Box::new(|mut names, state| {
                        names.push_front(SubstitutionBinding::ConsumerBinding(
                            XVar {
                                prdcns: Cns,
                                var: name,
                            }
                            .into(),
                        ));
                        k(names, state)
                    }),
                    state,
                )
            }),
            state,
        ),
    }
}
