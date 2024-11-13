use crate::traits::free_vars::FreeVars;
use crate::traits::shrink::Shrinking;
use crate::traits::substitution::SubstVar;

use super::{Chirality, Ty, TypeDeclaration, Var};
use std::{collections::HashSet, fmt};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ContextBinding {
    pub var: Var,
    pub chi: Chirality,
    pub ty: Ty,
}

pub type TypingContext = Vec<ContextBinding>;

impl fmt::Display for ContextBinding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} :{}: {}", self.var, self.chi, self.ty)
    }
}

impl FreeVars for ContextBinding {
    fn free_vars(&self, vars: &mut HashSet<Var>) {
        vars.insert(self.var.clone());
    }
}

impl SubstVar for ContextBinding {
    type Target = ContextBinding;

    fn subst_sim(self, subst: &[(Var, Var)]) -> ContextBinding {
        ContextBinding {
            var: self.var.subst_sim(subst),
            ..self
        }
    }
}

impl Shrinking for ContextBinding {
    type Target = axcut::syntax::ContextBinding;

    fn shrink(
        self,
        used_vars: &mut HashSet<Var>,
        types: &[TypeDeclaration],
    ) -> axcut::syntax::ContextBinding {
        if self.ty == Ty::Int {
            if self.chi == Chirality::Prd {
                axcut::syntax::ContextBinding {
                    var: self.var,
                    chi: axcut::syntax::Chirality::Ext,
                    ty: axcut::syntax::Ty::Int,
                }
            } else {
                axcut::syntax::ContextBinding {
                    var: self.var,
                    chi: axcut::syntax::Chirality::Cns,
                    ty: axcut::syntax::Ty::Decl(crate::syntax_var::cont_int().name),
                }
            }
        } else {
            axcut::syntax::ContextBinding {
                var: self.var,
                chi: self.chi.shrink(used_vars, types),
                ty: self.ty.shrink(used_vars, types),
            }
        }
    }
}

#[must_use]
pub fn context_vars(context: &TypingContext) -> Vec<Var> {
    let mut vars = Vec::with_capacity(context.len());
    for binding in context {
        vars.push(binding.var.clone());
    }
    vars
}
