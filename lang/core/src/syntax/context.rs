use printer::{
    tokens::{COLON, TICK},
    DocAllocator, Print,
};

use super::{types::Ty, Covar, Var};
use crate::traits::typed::Typed;
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ContextBinding {
    VarBinding { var: Var, ty: Ty },
    CovarBinding { covar: Covar, ty: Ty },
}

pub type TypingContext = Vec<ContextBinding>;

impl Typed for ContextBinding {
    fn get_type(&self) -> Ty {
        match self {
            ContextBinding::VarBinding { var: _, ty } => ty.clone(),
            ContextBinding::CovarBinding { covar: _, ty } => ty.clone(),
        }
    }
}

impl Print for ContextBinding {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        match self {
            ContextBinding::VarBinding { var, ty } => alloc
                .text(var)
                .append(alloc.space())
                .append(alloc.text(COLON))
                .append(alloc.space())
                .append(ty.print(cfg, alloc)),
            ContextBinding::CovarBinding { covar, ty } => alloc
                .text(TICK)
                .append(covar.print(cfg, alloc))
                .append(alloc.space())
                .append(alloc.text(":cnt"))
                .append(alloc.space())
                .append(ty.print(cfg, alloc)),
        }
    }
}

pub fn context_vars(ctx: &TypingContext) -> HashSet<Var> {
    ctx.iter()
        .filter_map(|bnd| match bnd {
            ContextBinding::VarBinding { var, ty: _ } => Some(var.clone()),
            _ => None,
        })
        .collect()
}

pub fn context_covars(ctx: &TypingContext) -> HashMap<Covar, Ty> {
    let mut covar_map = HashMap::new();
    for bnd in ctx.iter() {
        if let ContextBinding::CovarBinding { covar, ty } = bnd {
            covar_map.insert(covar.clone(), ty.clone());
        }
    }
    covar_map
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
        let expected = "x : Int";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_covar() {
        let result = ContextBinding::CovarBinding {
            covar: "a".to_owned(),
            ty: Ty::Int(),
        }
        .print_to_string(None);
        let expected = "'a :cnt Int";
        assert_eq!(result, expected)
    }
}
