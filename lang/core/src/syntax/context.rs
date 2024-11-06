use super::{types::Ty, Covar, Var};
use std::{
    collections::{HashMap, HashSet},
    fmt,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ContextBinding {
    VarBinding { var: Var, ty: Ty },
    CovarBinding { covar: Covar, ty: Ty },
}

pub type TypingContext = Vec<ContextBinding>;

impl fmt::Display for ContextBinding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ContextBinding::VarBinding { var, ty } => write!(f, "{var} : {ty}"),
            ContextBinding::CovarBinding { covar, ty } => write!(f, "'{covar} :cnt {ty}"),
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
    use super::{ContextBinding, Ty};

    #[test]
    fn display_var() {
        let result = format!(
            "{}",
            ContextBinding::VarBinding {
                var: "x".to_owned(),
                ty: Ty::Int()
            }
        );
        let expected = "x : Int";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_covar() {
        let result = format!(
            "{}",
            ContextBinding::CovarBinding {
                covar: "a".to_owned(),
                ty: Ty::Int()
            }
        );
        let expected = "'a :cnt Int";
        assert_eq!(result, expected)
    }
}
