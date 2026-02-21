//! This crate contains the [syntax] for the Core intermediate representation, as well as some
//! infrastructure [traits] and moreover a trait for transforming a Core program into the
//! [focused](traits::focus::Focusing) fragment of Core, where only (co)variables are allowed in
//! argument positions.

pub mod syntax;
pub mod traits;

#[cfg(test)]
pub mod test_common {
    use crate::syntax::*;
    extern crate self as core_lang;
    use core_macros::{covar, id, var};

    pub fn example_subst() -> (Vec<(Ident, Term<Prd>)>, Vec<(Ident, Term<Cns>)>) {
        let prod_subst = vec![(id!("x"), var!(id!("y")).into())];
        let cons_subst = vec![(id!("a"), covar!(id!("b")).into())];
        (prod_subst, cons_subst)
    }
}
