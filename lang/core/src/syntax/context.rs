use printer::{
    tokens::{COLON, TICK},
    DocAllocator, Print,
};

use super::{Covar, Ty, Var};
use crate::traits::focus::{Focusing, FocusingState};

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
    type Target = crate::syntax_var::FsContextBinding;
    fn focus(self, state: &mut FocusingState) -> crate::syntax_var::FsContextBinding {
        state.add_context(&vec![self.clone()]);
        match self {
            ContextBinding::VarBinding { var, ty } => {
                let chi = if ty.is_codata(state.codata_types) {
                    crate::syntax_var::Chirality::Cns
                } else {
                    crate::syntax_var::Chirality::Prd
                };
                crate::syntax_var::FsContextBinding { var, chi, ty }
            }
            ContextBinding::CovarBinding { covar, ty } => {
                let chi = if ty.is_codata(state.codata_types) {
                    crate::syntax_var::Chirality::Prd
                } else {
                    crate::syntax_var::Chirality::Cns
                };
                crate::syntax_var::FsContextBinding {
                    var: covar,
                    chi,
                    ty,
                }
            }
        }
    }
}
