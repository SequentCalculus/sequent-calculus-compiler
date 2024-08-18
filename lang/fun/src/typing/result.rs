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
