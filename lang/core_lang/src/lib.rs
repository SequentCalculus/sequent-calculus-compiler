pub mod syntax;
pub mod traits;

#[cfg(test)]
pub mod test_common {
    use crate::syntax::{
        term::{Cns, Prd, XVar},
        types::Ty,
        Covar, Term, Var,
    };

    pub fn example_subst() -> (Vec<(Term<Prd>, Var)>, Vec<(Term<Cns>, Covar)>) {
        let prd_subst: Vec<(Term<Prd>, Var)> =
            vec![(XVar::var("y", Ty::I64).into(), "x".to_string())];
        let cns_subst: Vec<(Term<Cns>, Covar)> =
            vec![(XVar::covar("b", Ty::I64).into(), "a".to_string())];
        (prd_subst, cns_subst)
    }
}
