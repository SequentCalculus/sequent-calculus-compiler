use axcut::syntax::{
    Name,
    statements::{Clause, Switch},
};
use miette::Diagnostic;
use std::fmt;

#[derive(Debug, Clone, Diagnostic)]
pub enum Error {
    NoMatchingClause {
        patterns: String,
        xtor: Name,
    },
    ClauseArity {
        clause_args: usize,
        xtor_args: usize,
    },
}

impl Error {
    pub fn switch_clause(switch: &Switch, xtor: &str) -> Error {
        Error::NoMatchingClause {
            patterns: format!("{switch:#?}"),
            xtor: xtor.to_owned(),
        }
    }

    pub fn create_clause(clauses: &Vec<Clause>, xtor: &str) -> Error {
        Error::NoMatchingClause {
            patterns: format!("{clauses:#?}"),
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
            Error::NoMatchingClause {
                patterns: switch,
                xtor,
            } => {
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
