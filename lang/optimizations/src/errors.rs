use axcut::syntax::{Name, statements::Switch};
use miette::Diagnostic;
use std::fmt;

#[derive(Debug, Clone, Diagnostic)]
pub enum Error {
    NoMatchingClause {
        switch: String,
        xtor: Name,
    },
    ClauseArity {
        clause_args: usize,
        xtor_args: usize,
    },
}

impl Error {
    pub fn clause(switch: &Switch, xtor: &str) -> Error {
        Error::NoMatchingClause {
            switch: format!("{switch:#?}"),
            xtor: xtor.to_owned(),
        }
    }

    pub fn arity(clause: usize, xtor: usize) -> Error {
        Error::ClauseArity {
            clause_args: clause,
            xtor_args: xtor,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::NoMatchingClause { switch, xtor } => {
                write!(f, "Could not find clause for {xtor} in {switch}")
            }
            Error::ClauseArity {
                clause_args,
                xtor_args,
            } => write!(
                f,
                "Arity Mismatch: clause has {clause_args} bindings, xtor has {xtor_args}"
            ),
        }
    }
}

impl std::error::Error for Error {}
