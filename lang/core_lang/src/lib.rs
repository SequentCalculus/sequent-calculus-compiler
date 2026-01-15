//! This crate contains the [syntax] for the Core intermediate representation, as well as some
//! infrastructure [traits] and moreover a trait for transforming a Core program into the
//! [focused](traits::focus::Focusing) fragment of Core, where only (co)variables are allowed in
//! argument positions.

pub mod syntax;
pub mod traits;

#[cfg(test)]
pub mod test_common {
    use crate::syntax::*;

    pub fn example_subst() -> (Vec<(Var, Term<Prd>)>, Vec<(Var, Term<Cns>)>) {
        let prod_subst = vec![(
            Var {
                name: "x".to_string(),
                id: 0,
            },
            XVar::var(
                Var {
                    name: "y".to_string(),
                    id: 0,
                },
                Ty::I64,
            )
            .into(),
        )];
        let cons_subst = vec![(
            Var {
                name: "a".to_string(),
                id: 0,
            },
            XVar::covar(
                Var {
                    name: "b".to_string(),
                    id: 0,
                },
                Ty::I64,
            )
            .into(),
        )];
        (prod_subst, cons_subst)
    }
}
