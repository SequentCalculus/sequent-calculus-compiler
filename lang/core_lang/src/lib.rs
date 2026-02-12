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
    use core_macros::{covar, var};

    pub fn example_subst() -> (Vec<(Var, Term<Prd>)>, Vec<(Var, Term<Cns>)>) {
        let prod_subst = vec![(
            Var {
                name: "x".to_string(),
                id: 0,
            },
            var!("y", 0).into(),
        )];
        let cons_subst = vec![(
            Var {
                name: "a".to_string(),
                id: 0,
            },
            covar!("b", 0).into(),
        )];
        (prod_subst, cons_subst)
    }
}
