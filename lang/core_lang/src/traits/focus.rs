use crate::syntax::{substitution::SubstitutionBinding, FsStatement, Name, Var};

use std::collections::{HashSet, VecDeque};
use std::rc::Rc;

pub trait Focusing {
    type Target;
    fn focus(self, used_vars: &mut HashSet<Var>) -> Self::Target;
}

impl<T: Focusing + Clone> Focusing for Rc<T> {
    type Target = Rc<T::Target>;
    fn focus(self, used_vars: &mut HashSet<Var>) -> Self::Target {
        Rc::new(Rc::unwrap_or_clone(self).focus(used_vars))
    }
}

impl<T: Focusing> Focusing for Vec<T> {
    type Target = Vec<T::Target>;
    fn focus(self, used_vars: &mut HashSet<Var>) -> Self::Target {
        self.into_iter().map(|x| x.focus(used_vars)).collect()
    }
}

pub type Continuation = Box<dyn FnOnce(Name, &mut HashSet<Var>) -> FsStatement>;
pub type ContinuationVec = Box<dyn FnOnce(VecDeque<Name>, &mut HashSet<Var>) -> FsStatement>;

pub trait Bind: Sized {
    fn bind(self, k: Continuation, used_vars: &mut HashSet<Var>) -> FsStatement;
}

pub fn bind_many(
    mut args: VecDeque<SubstitutionBinding>,
    k: ContinuationVec,
    used_vars: &mut HashSet<Var>,
) -> FsStatement {
    match args.pop_front() {
        None => k(VecDeque::new(), used_vars),
        Some(SubstitutionBinding::ProducerBinding(prd)) => prd.bind(
            Box::new(|name, used_vars| {
                bind_many(
                    args,
                    Box::new(|mut names, used_vars| {
                        names.push_front(name);
                        k(names, used_vars)
                    }),
                    used_vars,
                )
            }),
            used_vars,
        ),
        Some(SubstitutionBinding::ConsumerBinding(cns)) => cns.bind(
            Box::new(|name, used_vars| {
                bind_many(
                    args,
                    Box::new(|mut names, used_vars| {
                        names.push_front(name);
                        k(names, used_vars)
                    }),
                    used_vars,
                )
            }),
            used_vars,
        ),
    }
}
