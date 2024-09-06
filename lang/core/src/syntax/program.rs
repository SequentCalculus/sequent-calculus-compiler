use super::Def;
use std::fmt;

// Prog
//
//

#[derive(Debug, Clone)]
pub struct Prog {
    pub prog_defs: Vec<Def>,
}

impl fmt::Display for Prog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let defs_joined: String = self
            .prog_defs
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "{}", defs_joined)
    }
}
