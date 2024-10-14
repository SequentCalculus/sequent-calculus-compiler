use super::{Covar, Statement, Var};
use crate::{
    syntax::term::{Cns, Prd, Term},
    traits::{free_vars::FreeV, substitution::Subst},
};
use std::{collections::HashSet, fmt, rc::Rc};

// IfZ
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IfZ {
    pub ifc: Rc<Term<Prd>>,
    pub thenc: Rc<Statement>,
    pub elsec: Rc<Statement>,
}

impl std::fmt::Display for IfZ {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "IfZ({}; {}, {})", self.ifc, self.thenc, self.elsec)
    }
}

impl From<IfZ> for Statement {
    fn from(value: IfZ) -> Self {
        Statement::IfZ(value)
    }
}

impl FreeV for IfZ {
    fn free_vars(&self) -> HashSet<Var> {
        let mut free_vars = self.ifc.free_vars();
        free_vars.extend(self.thenc.free_vars());
        free_vars.extend(self.elsec.free_vars());
        free_vars
    }

    fn free_covars(&self) -> HashSet<Covar> {
        let mut free_covars = self.ifc.free_covars();
        free_covars.extend(self.thenc.free_covars());
        free_covars.extend(self.elsec.free_covars());
        free_covars
    }
}

impl Subst for IfZ {
    type Target = IfZ;

    fn subst_sim(
        &self,
        prod_subst: &[(Term<Prd>, Var)],
        cons_subst: &[(Term<Cns>, Covar)],
    ) -> Self::Target {
        IfZ {
            ifc: self.ifc.subst_sim(prod_subst, cons_subst),
            thenc: self.thenc.subst_sim(prod_subst, cons_subst),
            elsec: self.elsec.subst_sim(prod_subst, cons_subst),
        }
    }
}
