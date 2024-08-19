use std::{error::Error, fmt::Display};

use crate::syntax::{Ctor, Dtor};

use super::Ty;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeError {
    OccursCheck {
        var: String,
        ty: Ty,
    },
    CannotUnify {
        ty1: Ty,
        ty2: Ty,
    },
    FunNotFound {
        name: String,
    },
    VarNotFound {
        name: String,
    },
    CovarNotFound {
        name: String,
    },
    CtorWrongNumOfArgs {
        ctor: Ctor,
    },
    DtorWrongNumOfArgs {
        dtor: Dtor,
    },
    PatternWrongNumOfArgs {
        ctor: Ctor,
    },
    FunWrongNumOfArgs {
        name: String,
        expected_vars: usize,
        actual_vars: usize,
        expected_covars: usize,
        actual_covars: usize,
    },
    InvalidCase,
    InvalidCocase,
}

impl Display for TypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use TypeError::*;
        match self {
            OccursCheck { var, ty } => write!(f, "Occurs check! {var} occurs in {ty}."),
            CannotUnify { ty1, ty2 } => write!(f, "Cannot unify types: {ty1} and {ty2}."),
            FunNotFound { name } => write!(
                f,
                "A top-level function named {name} is not contained in the program."
            ),
            VarNotFound { name } => write!(f, "Variable {name} not found in environment."),
            CovarNotFound { name } => write!(f, "Covariable {name} not found in environment."),
            CtorWrongNumOfArgs { ctor } => {
                write!(f, "Wrong number of arguments for constructor {ctor}")
            }
            DtorWrongNumOfArgs { dtor } => {
                write!(f, "Wrong number of arguments for destructor {dtor}")
            }
            InvalidCase => write!(f, "Invalid case expression"),
            InvalidCocase => write!(f, "Invalid cocase expression"),
            FunWrongNumOfArgs {
                name,
                expected_vars,
                actual_vars,
                expected_covars,
                actual_covars,
            } => {
                write!(f,"{name} called with wrong number of arguments. Expected: {expected_vars} + {expected_covars} Got: {actual_vars} + {actual_covars}")
            }
            PatternWrongNumOfArgs { ctor } => {
                write!(f, "Wrong number of bound variables for {ctor}")
            }
        }
    }
}

impl Error for TypeError {}

#[cfg(test)]
mod display_error_test {
    use super::{Ty, TypeError};

    #[test]
    fn fmt_occurs() {
        let result = format!(
            "{}",
            TypeError::OccursCheck {
                var: "x".to_owned(),
                ty: Ty::Int()
            }
        );
        let expected = "Occurs check! x occurs in Int.".to_owned();
        assert_eq!(result, expected);
    }

    #[test]
    fn fmt_unify() {
        let result = format!(
            "{}",
            TypeError::CannotUnify {
                ty1: Ty::Int(),
                ty2: Ty::Var("X".to_owned())
            }
        );
        let expected = "Cannot unify types: Int and X.".to_owned();
        assert_eq!(result, expected)
    }

    #[test]
    fn fmt_funnotfound() {
        let result = format!(
            "{}",
            TypeError::FunNotFound {
                name: "main".to_owned()
            }
        );
        let expected = "A top-level function named main is not contained in the program.";
        assert_eq!(result, expected)
    }

    #[test]
    fn fmt_varnotfound() {
        let result = format!(
            "{}",
            TypeError::VarNotFound {
                name: "x".to_owned()
            }
        );
        let expected = "Variable x not found in environment.";
        assert_eq!(result, expected)
    }

    #[test]
    fn fmt_covarnotfound() {
        let result = format!(
            "{}",
            TypeError::CovarNotFound {
                name: "a".to_owned()
            }
        );
        let expected = "Covariable a not found in environment.";
        assert_eq!(result, expected);
    }

    #[test]
    fn fmt_ctorargs() {
        let result = format!(
            "{}",
            TypeError::CtorWrongNumOfArgs {
                ctor: crate::syntax::Ctor::Nil
            }
        );
        let expected = "Wrong number of arguments for constructor Nil".to_owned();
        assert_eq!(result, expected)
    }

    #[test]
    fn fmt_dtorargs() {
        let result = format!(
            "{}",
            TypeError::DtorWrongNumOfArgs {
                dtor: crate::syntax::Dtor::Fst
            }
        );
        let expected = "Wrong number of arguments for destructor fst".to_owned();
        assert_eq!(result, expected)
    }

    #[test]
    fn fmt_case() {
        let result = format!("{}", TypeError::InvalidCase);
        let expected = "Invalid case expression".to_owned();
        assert_eq!(result, expected)
    }

    #[test]
    fn fmt_cocase() {
        let result = format!("{}", TypeError::InvalidCocase);
        let expected = "Invalid cocase expression".to_owned();
        assert_eq!(result, expected)
    }

    #[test]
    fn fmt_funargs() {
        let result = format!(
            "{}",
            TypeError::FunWrongNumOfArgs {
                name: "main".to_owned(),
                expected_vars: 1,
                expected_covars: 2,
                actual_vars: 0,
                actual_covars: 3
            }
        );
        let expected =
            "main called with wrong number of arguments. Expected: 1 + 2 Got: 0 + 3".to_owned();
        assert_eq!(result, expected)
    }

    #[test]
    fn fmt_patternargs() {
        let result = format!(
            "{}",
            TypeError::PatternWrongNumOfArgs {
                ctor: crate::syntax::Ctor::Nil
            }
        );
        let expected = "Wrong number of bound variables for Nil".to_owned();
        assert_eq!(result, expected)
    }
}
