//! This module defines a trait [Focusing] with a method for transforming a Core program into the
//! focused fragment of Core, where only (co)variables are allowed in argument positions. The
//! module also defines a helper trait [Bind] with a method that is used during focusing to avoid
//! administrative redexes.

use crate::syntax::{ContextBinding, FsStatement, ID, arguments::Argument};

use std::collections::VecDeque;
use std::rc::Rc;

/// This trait defines a method for focusing a term or statement. To do so, it lifts all
/// non-variable terms out of argument positions, names them and puts the names in their place.
///
/// Example:
/// ```text
/// focus(< Cons(1, Nil) | mutilde x. exit 0>)
/// = < 1 | mutilde x1. < Nil | mutilde x2.< Cons(x1, x2) | mutilde x. < 0 | mutilde x0. exit x0 > > > >
/// ```
pub trait Focusing {
    /// The result of focusing is usually a struct for the focused version of the term or
    /// statement.
    type Target;
    /// This method peforms the focusing transformation on a term or statement. To do so, it lifts
    /// all non-variable terms out of argument positions, names them and puts the names in their
    /// place.
    /// `max_id` is the highest [`ID`] currently used for [`crate::syntax::Identifier`]s in the
    /// program. It is threaded through the focusing to facilitate generation of fresh (co)variables.
    fn focus(self, max_id: &mut ID) -> Self::Target;
}

impl<T: Focusing + Clone> Focusing for Rc<T> {
    type Target = Rc<T::Target>;
    fn focus(self, max_id: &mut ID) -> Self::Target {
        Rc::new(Rc::unwrap_or_clone(self).focus(max_id))
    }
}

impl<T: Focusing> Focusing for Vec<T> {
    type Target = Vec<T::Target>;
    fn focus(self, max_id: &mut ID) -> Self::Target {
        self.into_iter().map(|x| x.focus(max_id)).collect()
    }
}

/// This is a type alias for a meta-level continuation that abstracts over a (co)variable
/// standing for a term in argument position that has been lifted out of a statement. When the
/// continuation is applied to a (co)variable, it returns the focused statement with the
/// (co)variable in the place of the term that was lifted. The continuation also expects the
/// highest [`ID`] currently used for [`crate::syntax::Identifier`]s in the program, which is used
/// to generate fresh names.
pub type Continuation = Box<dyn FnOnce(ContextBinding, &mut ID) -> FsStatement>;
/// This is a type alias for a meta-level continuation similar to [Continuation], but it abstracts
/// over many (co)variables at once.
pub type ContinuationVec = Box<dyn FnOnce(VecDeque<ContextBinding>, &mut ID) -> FsStatement>;

/// This trait defines a method used during [focusing](Focusing) to avoid administrative redexes.
pub trait Bind: Sized {
    /// This method is used during [focusing](Focusing) to avoid administrative redexes. It takes
    /// a term that has been lifted out of argument position and additionally a meta-level
    /// [continuation](Continuation) that contains the statement from which the term has been
    /// lifted. It eventually yields the focused statement.
    /// - `continuation` is the continuation containing the statement from which the term has been
    ///   lifted.
    /// - `max_id` is the highest [`ID`] currently used for [`crate::syntax::Identifier`]s in the
    ///   program. It is threaded through the focusing to facilitate generation of fresh
    ///   (co)variables.
    fn bind(self, k: Continuation, max_id: &mut ID) -> FsStatement;
}

/// This function is used during [focusing](Focusing) to avoid administrative redexes. It is
/// similar to the [`Bind::bind`]-method, but for a whole list of lifted terms.
/// - `args` is the list of lifted terms.
/// - `continuation` is the continuation containing the statement from which the terms have been
///   lifted.
/// - `max_id` is the highest [`ID`] currently used for [`crate::syntax::Identifier`]s in the
///   program. It is threaded through the focusing to facilitate generation of fresh (co)variables.
pub fn bind_many(mut args: VecDeque<Argument>, k: ContinuationVec, max_id: &mut ID) -> FsStatement {
    match args.pop_front() {
        None => k(VecDeque::new(), max_id),
        Some(arg) => arg.bind(
            Box::new(|binding, max_id| {
                bind_many(
                    args,
                    Box::new(|mut bindings, max_id| {
                        bindings.push_front(binding);
                        k(bindings, max_id)
                    }),
                    max_id,
                )
            }),
            max_id,
        ),
    }
}
