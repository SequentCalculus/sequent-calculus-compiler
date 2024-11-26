use printer::{
    tokens::{COLON, TICK},
    DocAllocator, Print,
};

use super::{Chirality, Covar, Ty, Var};
use crate::traits::{
    focus::{Focusing, FocusingState},
    substitution::SubstVar,
};

use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ContextBinding {
    VarBinding { var: Var, ty: Ty },
    CovarBinding { covar: Covar, ty: Ty },
}

pub type TypingContext = Vec<ContextBinding>;

impl Print for ContextBinding {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        match self {
            ContextBinding::VarBinding { var, ty } => alloc
                .text(var)
                .append(alloc.text(COLON))
                .append(alloc.space())
                .append(ty.print(cfg, alloc)),
            ContextBinding::CovarBinding { covar, ty } => alloc
                .text(TICK)
                .append(covar.print(cfg, alloc))
                .append(alloc.space())
                .append(alloc.text(":cns"))
                .append(alloc.space())
                .append(ty.print(cfg, alloc)),
        }
    }
}

#[must_use]
pub fn context_vars(ctx: &TypingContext) -> HashSet<Var> {
    ctx.iter()
        .filter_map(|bnd| match bnd {
            ContextBinding::VarBinding { var, ty: _ } => Some(var.clone()),
            ContextBinding::CovarBinding { .. } => None,
        })
        .collect()
}

#[must_use]
pub fn context_covars(ctx: &TypingContext) -> HashSet<Covar> {
    ctx.iter()
        .filter_map(|bnd| match bnd {
            ContextBinding::CovarBinding { covar, ty: _ } => Some(covar.clone()),
            ContextBinding::VarBinding { .. } => None,
        })
        .collect()
}

#[cfg(test)]
mod context_tests {
    use printer::Print;

    use super::{ContextBinding, Ty};

    #[test]
    fn display_var() {
        let result = ContextBinding::VarBinding {
            var: "x".to_owned(),
            ty: Ty::Int(),
        }
        .print_to_string(None);
        let expected = "x: Int";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_covar() {
        let result = ContextBinding::CovarBinding {
            covar: "a".to_owned(),
            ty: Ty::Int(),
        }
        .print_to_string(None);
        let expected = "'a :cns Int";
        assert_eq!(result, expected)
    }
}

impl Focusing for ContextBinding {
    type Target = crate::syntax::context::FsContextBinding;
    fn focus(self, state: &mut FocusingState) -> crate::syntax::context::FsContextBinding {
        state.add_context(&vec![self.clone()]);
        match self {
            ContextBinding::VarBinding { var, ty } => {
                let chi = if ty.is_codata(state.codata_types) {
                    crate::syntax::Chirality::Cns
                } else {
                    crate::syntax::Chirality::Prd
                };
                crate::syntax::context::FsContextBinding { var, chi, ty }
            }
            ContextBinding::CovarBinding { covar, ty } => {
                let chi = if ty.is_codata(state.codata_types) {
                    crate::syntax::Chirality::Prd
                } else {
                    crate::syntax::Chirality::Cns
                };
                crate::syntax::context::FsContextBinding {
                    var: covar,
                    chi,
                    ty,
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FsContextBinding {
    pub var: Var,
    pub chi: Chirality,
    pub ty: Ty,
}

pub type FsTypingContext = Vec<FsContextBinding>;

impl Print for FsContextBinding {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc
            .text(&self.var)
            .append(alloc.space())
            .append(alloc.text(COLON))
            .append(self.chi.print(cfg, alloc))
            .append(alloc.space())
            .append(self.ty.print(cfg, alloc))
    }
}

impl SubstVar for FsContextBinding {
    type Target = FsContextBinding;

    fn subst_sim(self, subst: &[(Var, Var)]) -> FsContextBinding {
        FsContextBinding {
            var: self.var.subst_sim(subst),
            ..self
        }
    }
}

#[must_use]
pub fn fs_context_vars(context: &FsTypingContext) -> Vec<Var> {
    let mut vars = Vec::with_capacity(context.len());
    for binding in context {
        vars.push(binding.var.clone());
    }
    vars
}
