//! This module defines a trait for making all binders in every path through a term or statement
//! unique.

use crate::syntax::Ident;
use std::collections::HashSet;
use std::mem::swap;
use std::rc::Rc;

pub struct UniquifyState {
    pub seen_vars: HashSet<Ident>,
    next_id: usize,
}

impl UniquifyState {
    pub fn new(seen: HashSet<Ident>) -> Self {
        let max_used = seen.iter().map(|ident| ident.id).max().unwrap_or(0);
        Self {
            seen_vars: seen,
            next_id: max_used + 1,
        }
    }

    pub fn uniquify_restore<T>(&mut self, t: T) -> (T, HashSet<Ident>)
    where
        T: Uniquify,
    {
        let mut seen_clone = self.seen_vars.clone();
        let res = t.uniquify(self);
        swap(&mut seen_clone, &mut self.seen_vars);
        (res, seen_clone)
    }

    pub fn next_var(&mut self) -> Ident {
        let new_var = Ident {
            name: "x".to_string(),
            id: self.next_id,
        };
        self.next_id += 1;
        self.seen_vars.insert(new_var.clone());
        new_var
    }
}

/// This trait defines a method for making all binders in every path through a term or statement
/// unique.
pub trait Uniquify {
    /// This method makes all binders in every path through a term or statement unique by renaming
    /// them if needed.
    /// - `seen_vars` is the set of names we have already seen in the path we are currently in.
    /// - `used_vars` is the set of names used in the whole top-level definition being uniquified.
    ///   It is threaded through the uniquification to facilitate generation of fresh
    ///   (co)variables.
    fn uniquify(self, state: &mut UniquifyState) -> Self;
}

impl<T: Uniquify + Clone> Uniquify for Rc<T> {
    fn uniquify(self, state: &mut UniquifyState) -> Self {
        Rc::new(Rc::unwrap_or_clone(self).uniquify(state))
    }
}

impl<T: Uniquify> Uniquify for Option<T> {
    fn uniquify(self, state: &mut UniquifyState) -> Self {
        self.map(|t| t.uniquify(state))
    }
}

impl<T: Uniquify> Uniquify for Vec<T> {
    fn uniquify(self, state: &mut UniquifyState) -> Self {
        self.into_iter()
            .map(|element| element.uniquify(state))
            .collect()
    }
}
