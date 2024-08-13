use super::{Consumer, Covar, Producer, Statement, Var};
use crate::traits::{free_vars::FreeV, substitution::Subst};
use std::{collections::HashSet, fmt, rc::Rc};
// IfZ
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IfZ {
    pub ifc: Rc<Producer>,
    pub thenc: Rc<Statement>,
    pub elsec: Rc<Statement>,
}

impl std::fmt::Display for IfZ {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "IfZ({};{},{})", self.ifc, self.thenc, self.elsec)
    }
}

impl From<IfZ> for Statement {
    fn from(value: IfZ) -> Self {
        Statement::IfZ(value)
    }
}

impl FreeV for IfZ {
    fn free_vars(&self) -> HashSet<Var> {
        let free_p = self.ifc.free_vars();
        let free_st1 = self.thenc.free_vars();
        let free_st2 = self.elsec.free_vars();
        let free_st: HashSet<Var> = free_st1.union(&free_st2).cloned().collect();
        free_st.union(&free_p).cloned().collect()
    }

    fn free_covars(&self) -> HashSet<Covar> {
        let free_p = self.ifc.free_covars();
        let free_st1 = self.thenc.free_covars();
        let free_st2 = self.elsec.free_covars();
        let free_st: HashSet<Var> = free_st1.union(&free_st2).cloned().collect();
        free_st.union(&free_p).cloned().collect()
    }
}

impl Subst for IfZ {
    type Target = IfZ;

    fn subst_sim(
        &self,
        prod_subst: &[(Producer, Var)],
        cons_subst: &[(Consumer, Covar)],
    ) -> Self::Target {
        IfZ {
            ifc: self.ifc.subst_sim(prod_subst, cons_subst),
            thenc: self.thenc.subst_sim(prod_subst, cons_subst),
            elsec: self.elsec.subst_sim(prod_subst, cons_subst),
        }
    }
}
