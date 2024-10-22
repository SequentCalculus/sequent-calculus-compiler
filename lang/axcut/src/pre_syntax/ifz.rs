use super::Statement;
use crate::syntax::{TypingContext, Var};
use crate::traits::free_vars::FreeVars;
use crate::traits::linearize::{Linearizing, UsedBinders};
use crate::traits::substitution::Subst;

use std::collections::HashSet;
use std::{fmt, rc::Rc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IfZ {
    pub ifc: Var,
    pub thenc: Rc<Statement>,
    pub elsec: Rc<Statement>,
}

impl std::fmt::Display for IfZ {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ifz {} {{\n    () =>\n  {}\n    () =>\n  {} }}",
            self.ifc, self.thenc, self.elsec
        )
    }
}

impl From<IfZ> for Statement {
    fn from(value: IfZ) -> Self {
        Statement::IfZ(value)
    }
}

impl FreeVars for IfZ {
    fn free_vars(&self, vars: &mut HashSet<Var>) {
        self.thenc.free_vars(vars);
        self.elsec.free_vars(vars);
        vars.insert(self.ifc.clone());
    }
}

impl Subst for IfZ {
    type Target = IfZ;

    fn subst_sim(self, subst: &[(Var, Var)]) -> IfZ {
        IfZ {
            ifc: self.ifc.subst_sim(subst),
            thenc: self.thenc.subst_sim(subst),
            elsec: self.elsec.subst_sim(subst),
        }
    }
}

impl UsedBinders for IfZ {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        self.thenc.used_binders(used);
        self.elsec.used_binders(used);
    }
}

impl Linearizing for IfZ {
    type Target = crate::syntax::IfZ;
    fn linearize(self, context: TypingContext, used_vars: &mut HashSet<Var>) -> crate::syntax::IfZ {
        crate::syntax::IfZ {
            ifc: self.ifc,
            thenc: self.thenc.linearize(context.clone(), used_vars),
            elsec: self.elsec.linearize(context, used_vars),
        }
    }
}
