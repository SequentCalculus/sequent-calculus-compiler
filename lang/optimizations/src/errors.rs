use axcut::syntax::{
    Name, TypingContext, Var,
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
    CallArity {
        def: Name,
        def_args: usize,
        called_args: usize,
    },
    DefinitionNotFound {
        name: Name,
    },
    VariableNotFound {
        var: Var,
        context: TypingContext,
    },
    XtorNotFound {
        xtor: Name,
        clause_xtors: Vec<Name>,
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
            Error::DefinitionNotFound { name } => write!(f, "Could not find definition {name}"),
            Error::VariableNotFound { var, context } => {
                write!(f, "Could not find variable {var} in context {context:?}")
            }
            Error::CallArity {
                def,
                def_args,
                called_args,
            } => write!(
                f,
                "Wrong number of arguments for {def}, expected: {def_args}, found: {called_args}"
            ),
            Error::XtorNotFound { xtor, clause_xtors } => write!(
                f,
                "Could not find xtor {xtor} in clauses {}",
                clause_xtors.join(",")
            ),
        }
    }
}

impl std::error::Error for Error {}
