use super::{Covar, Name, Statement, Var};
use std::fmt;

// Def
//
//

#[derive(Debug, Clone)]
pub struct Def {
    pub name: Name,
    pub pargs: Vec<(Var, ())>,
    pub cargs: Vec<(Covar, ())>,
    pub body: Statement,
}

impl std::fmt::Display for Def {
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
