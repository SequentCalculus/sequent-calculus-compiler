use super::{types::Ty, Covar, Var};
use std::fmt;

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
