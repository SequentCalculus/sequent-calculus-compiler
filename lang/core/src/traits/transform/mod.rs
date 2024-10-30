pub mod clause;
pub mod cut;
pub mod fun;
pub mod ifz;
pub mod lit;
pub mod mu;
pub mod op;
pub mod prog;
pub mod statement;
pub mod subst;
pub mod term;
pub mod xcase;
pub mod xtor;

use crate::{
    syntax::{
        context::{ContextBinding, TypingContext},
        declaration::{CodataDeclaration, DataDeclaration},
        program::Declaration,
        substitution::SubstitutionBinding,
        term::{Cns, Prd, XVar},
        Covar, Name, Prog, Statement, Var,
    },
    traits::free_vars::{fresh_covar, fresh_var},
};
use std::collections::{HashSet, VecDeque};
use std::rc::Rc;

#[derive(Default)]
pub struct TransformState {
    pub used_vars: HashSet<Var>,
    pub used_covars: HashSet<Covar>,
    pub data_decls: Vec<DataDeclaration>,
    pub codata_decls: Vec<CodataDeclaration>,
}

impl TransformState {
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
    pub fn lookup_data(&self, xtor_name: &Name) -> Option<DataDeclaration> {
        for data_decl in self.data_decls.iter() {
            match data_decl.xtors.iter().find(|xtor| xtor.name == *xtor_name) {
                None => continue,
                Some(_) => return Some(data_decl.clone()),
            };
        }
        None
    }
    pub fn lookup_codata(&self, xtor_name: &Name) -> Option<CodataDeclaration> {
        for codata_decl in self.codata_decls.iter() {
            match codata_decl
                .xtors
                .iter()
                .find(|xtor| xtor.name == *xtor_name)
            {
                None => continue,
                Some(_) => return Some(codata_decl.clone()),
            };
        }
        None
    }
}

impl From<&Prog> for TransformState {
    fn from(prog: &Prog) -> TransformState {
        let mut state = TransformState::default();
        for decl in prog.prog_decls.iter() {
            match decl {
                Declaration::Definition(_) => continue,
                Declaration::DataDeclaration(data) => state.data_decls.push(data.clone()),
                Declaration::CodataDeclaration(codata) => state.codata_decls.push(codata.clone()),
            }
        }
        state
    }
}

pub trait NamingTransformation {
    type Target;
    fn transform(self, state: &mut TransformState) -> Self::Target;
}

impl<T: NamingTransformation + Clone> NamingTransformation for Rc<T> {
    type Target = Rc<T::Target>;
    fn transform(self, state: &mut TransformState) -> Self::Target {
        Rc::new(Rc::unwrap_or_clone(self).transform(state))
    }
}

impl<T: NamingTransformation> NamingTransformation for Vec<T> {
    type Target = Vec<T::Target>;
    fn transform(self, state: &mut TransformState) -> Self::Target {
        self.into_iter().map(|x| x.transform(state)).collect()
    }
}

pub type Continuation = Box<dyn FnOnce(Name, &mut TransformState) -> Statement>;
pub type ContinuationVec =
    Box<dyn FnOnce(VecDeque<SubstitutionBinding>, &mut TransformState) -> Statement>;

pub trait Bind: Sized {
    fn bind(self, k: Continuation, state: &mut TransformState) -> Statement;
}

pub fn bind_many(
    mut args: VecDeque<SubstitutionBinding>,
    k: ContinuationVec,
    state: &mut TransformState,
) -> Statement {
    match args.pop_front() {
        None => k(VecDeque::new(), state),
        Some(SubstitutionBinding::ProducerBinding { prd: p, ty }) => p.bind(
            Box::new(|name, state| {
                bind_many(
                    args,
                    Box::new(|mut names, state| {
                        names.push_front(SubstitutionBinding::ProducerBinding {
                            prd: XVar {
                                prdcns: Prd,
                                var: name,
                            }
                            .into(),
                            ty,
                        });
                        k(names, state)
                    }),
                    state,
                )
            }),
            state,
        ),
        Some(SubstitutionBinding::ConsumerBinding { cns: c, ty }) => c.bind(
            Box::new(|name, state| {
                bind_many(
                    args,
                    Box::new(|mut names, state| {
                        names.push_front(SubstitutionBinding::ConsumerBinding {
                            cns: XVar {
                                prdcns: Cns,
                                var: name,
                            }
                            .into(),
                            ty,
                        });
                        k(names, state)
                    }),
                    state,
                )
            }),
            state,
        ),
    }
}
