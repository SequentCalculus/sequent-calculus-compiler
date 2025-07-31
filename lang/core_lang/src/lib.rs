//! This crate contains the [syntax] for the Core intermediate representation, as well as some
//! infrastructure [traits] and moreover a trait for transforming a Core program into the
//! [focused](traits::focus::Focusing) fragment of Core, where only (co)variables are allowed in
//! argument positions.

pub mod syntax;
pub mod traits;

#[cfg(test)]
pub mod test_common {
    use crate::syntax::{
        Covar, Term, Var,
        terms::{Cns, Prd, XVar},
        types::Ty,
    };

    pub fn example_subst() -> (Vec<(Var, Term<Prd>)>, Vec<(Covar, Term<Cns>)>) {
        let prod_subst = vec![("x".to_string(), XVar::var("y", Ty::I64).into())];
        let cnos_subst = vec![("a".to_string(), XVar::covar("b", Ty::I64).into())];
        (prod_subst, cnos_subst)
    }
}
