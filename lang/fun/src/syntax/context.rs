use crate::syntax::{types::Ty, Covariable, Variable};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ContextBinding {
    TypedVar { var: Variable, ty: Ty },
    TypedCovar { covar: Covariable, ty: Ty },
}

pub type TypingContext = Vec<ContextBinding>;

pub fn context_vars(ctx: &TypingContext) -> Vec<Variable> {
    ctx.iter()
        .filter_map(|bnd| match bnd {
            ContextBinding::TypedVar { var, ty: _ } => Some(var),
            _ => None,
        })
        .cloned()
        .collect()
}

pub fn context_covars(ctx: &TypingContext) -> Vec<Covariable> {
    ctx.iter()
        .filter_map(|bnd| match bnd {
            ContextBinding::TypedCovar { covar, ty: _ } => Some(covar),
            _ => None,
        })
        .cloned()
        .collect()
}

impl fmt::Display for ContextBinding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ContextBinding::TypedVar { var, ty } => write!(f, "{var} : {ty}"),
            ContextBinding::TypedCovar { covar, ty } => write!(f, "'{covar} :cnt {ty}"),
        }
    }
}

#[cfg(test)]
mod context_tests {
    use super::{context_covars, context_vars, ContextBinding, Ty, TypingContext};

    fn example_contextitem_var() -> ContextBinding {
        ContextBinding::TypedVar {
            var: "x".to_owned(),
            ty: Ty::Int(),
        }
    }

    fn example_contextitem_covar() -> ContextBinding {
        ContextBinding::TypedCovar {
            covar: "a".to_owned(),
            ty: Ty::Int(),
        }
    }

    fn example_context() -> TypingContext {
        vec![example_contextitem_var(), example_contextitem_covar()]
    }

    #[test]
    fn display_contextitem_var() {
        let result = format!("{}", example_contextitem_var());
        let expected = "x : Int";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_contextitem_covar() {
        let result = format!("{}", example_contextitem_covar());
        let expected = "'a :cnt Int";
        assert_eq!(result, expected)
    }

    #[test]
    fn ctx_vars() {
        let result = context_vars(&example_context());
        let expected = vec!["x".to_owned()];
        assert_eq!(result, expected)
    }

    #[test]
    fn ctx_covars() {
        let result = context_covars(&example_context());
        let expected = vec!["a".to_owned()];
        assert_eq!(result, expected)
    }
}
