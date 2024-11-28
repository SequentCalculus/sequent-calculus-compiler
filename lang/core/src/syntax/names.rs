use crate::traits::*;

pub type Var = String;
pub type Covar = String;
pub type Name = String;

impl SubstVar for Var {
    type Target = Var;

    fn subst_sim(self, subst: &[(Var, Var)]) -> Var {
        match subst.iter().find(|(old, _)| *old == self) {
            None => self,
            Some((_, new)) => new.clone(),
        }
    }
}
