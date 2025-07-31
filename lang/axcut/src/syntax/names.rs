//! This module defines some utilities to deal with names and lists of names.

use crate::traits::linearize::fresh_var;
use crate::traits::substitution::Subst;

use std::collections::HashSet;

/// Names of top-level functions, user-declared types and xtors.
pub type Name = String;
/// Variables
pub type Var = String;

impl Subst for Var {
    fn subst_sim(self, subst: &[(Var, Var)]) -> Var {
        match subst.iter().find(|(old, _)| *old == self) {
            None => self,
            Some((_, new)) => new.clone(),
        }
    }
}

/// This function picks fresh names for variables that are duplicated in a context.
/// - `context` is the context in which to pick fresh names.
/// - `clashes` is the set of variables for which a fresh name must be picked if they occur in the
///   context.
/// - `used_vars` is the set of variable names already used somwhere, i.e., which cannot be used as
///   fresh name.
pub fn freshen(
    context: &[Var],
    mut clashes: HashSet<Var>,
    used_vars: &mut HashSet<Var>,
) -> Vec<Var> {
    let mut new_context = Vec::with_capacity(context.len());
    for var in context {
        if clashes.contains(var) {
            // if the variable has occurred already we pick a fresh one
            new_context.push(fresh_var(used_vars, var));
        } else {
            // otherwise we keep it, but remember that we have seen it already
            clashes.insert(var.clone());
            new_context.push(var.clone());
        }
    }
    new_context
}

/// This function keeps all bindings in a context which are contained in a given set. It tries to
/// retain the original positions of as many bindings as possible in the context by moving bindings
/// at the end to positions of variables that are not retained.
/// - `context` is the context from which to keep bindings.
/// - `set` is the set of variables for which to keep bindings.
pub fn filter_by_set(context: &[Var], set: &HashSet<Var>) -> Vec<Var> {
    let mut new_context = context.to_owned();
    for (pos, var) in context.iter().enumerate() {
        // if we are beyond the length of the new context, we must have move all variables from
        // this point on already, so we are done
        if pos >= new_context.len() {
            break;
        } else if !set.contains(var) {
            // if we do not keep the binding at the current position, we look for one to keep from
            // the end of the new context
            let mut found_element = false;
            while new_context.len() - 1 > pos {
                if set.contains(&new_context[new_context.len() - 1]) {
                    found_element = true;
                    // if we have found a binding to keep at the end, we move it to the free
                    // position ...
                    new_context.swap_remove(pos);
                    // ... and stop searching
                    break;
                }
                // if we do not keep the binding currently at the end, we remove it
                new_context.pop();
            }
            if !found_element {
                // if we do not keep any of the bindings beyond the current position, we simply
                // remove the one at the current position (and are done now)
                new_context.pop();
                break;
            }
        }
    }
    new_context
}
