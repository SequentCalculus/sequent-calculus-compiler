use super::{Covar, Name, Statement, Var};
use std::fmt;

// Def
//
//

#[derive(Debug, Clone)]
pub struct Def<T> {
    pub name: Name,
    pub pargs: Vec<(Var, T)>,
    pub cargs: Vec<(Covar, T)>,
    pub body: Statement,
}

impl<T> std::fmt::Display for Def<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pargs: Vec<String> = self.pargs.iter().map(|(x, _)| x.to_string()).collect();
        let cargs: Vec<String> = self.cargs.iter().map(|(x, _)| x.to_string()).collect();
        write!(
            f,
            "def {}({}; {}) := {};",
            self.name,
            pargs.join(", "),
            cargs.join(", "),
            self.body
        )
    }
}
