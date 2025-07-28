//! Focusing, defines the [Focusing] trait
use crate::syntax::{ContextBinding, FsStatement, Var, substitution::SubstitutionBinding};

use std::collections::{HashSet, VecDeque};
use std::rc::Rc;

/// Focus a term
/// This names all intermediary steps of evaluation
/// Example: `focus(< Cons(1,Nil) | mutilde x. exit 0>)`
/// `= <1 | mutilde x1. <Nil | mutilde x2.<Cons(x1,x2) | mutilde x. <0 | mutilde x0.exit x0> > > >`
pub trait Focusing {
    /// The result of focusing
    /// This is usually a struct with the same fields but some more restrictions
    /// Example: for [crate::syntax::terms::Xtor] the target is [crate::syntax::terms::FsXtor]
    /// After focusing the arguments are no longer a [crate::syntax::substitution::Substitution]
    /// Instead arguments are a [crate::syntax::context::TypingContext]
    type Target;
    /// Focus `self` to `Self::Target`
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

/// Type alias for continuation
/// A continuation is a closure that crates a [Focused Statement][FsStatement]
/// from a given [ContextBinding] and a set of used variables
pub type Continuation = Box<dyn FnOnce(ContextBinding, &mut HashSet<Var>) -> FsStatement>;
/// Type alias for a continuation of many bindings
/// similar to [Continuation] but using multiple context bindings
pub type ContinuationVec =
    Box<dyn FnOnce(VecDeque<ContextBinding>, &mut HashSet<Var>) -> FsStatement>;

/// trait used for combining focused terms into statements
/// It takes the given continuation and combines with with the given `self`
/// by creating a `mu`-binding
/// Example: `bind(1) = <1 | mutilde x.k(x)>`
pub trait Bind: Sized {
    fn bind(self, k: Continuation, used_vars: &mut HashSet<Var>) -> FsStatement;
}

/// [Bind] for [crate::syntax::substitution::Substitution]s
/// This requires a ContinuationVec instead of a single [Continuation]
pub fn bind_many(
    mut args: VecDeque<SubstitutionBinding>,
    k: ContinuationVec,
    used_vars: &mut HashSet<Var>,
) -> FsStatement {
    match args.pop_front() {
        None => k(VecDeque::new(), used_vars),
        Some(SubstitutionBinding::ProducerBinding(prd)) => prd.bind(
            Box::new(|binding, used_vars| {
                bind_many(
                    args,
                    Box::new(|mut bindings, used_vars| {
                        bindings.push_front(binding);
                        k(bindings, used_vars)
                    }),
                    used_vars,
                )
            }),
            used_vars,
        ),
        Some(SubstitutionBinding::ConsumerBinding(cns)) => cns.bind(
            Box::new(|binding, used_vars| {
                bind_many(
                    args,
                    Box::new(|mut bindings, used_vars| {
                        bindings.push_front(binding);
                        k(bindings, used_vars)
                    }),
                    used_vars,
                )
            }),
            used_vars,
        ),
    }
}
