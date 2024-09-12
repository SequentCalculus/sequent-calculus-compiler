use super::{context::TypingContext, Name, Statement};
use std::fmt;

// Def
//
//

#[derive(Debug, Clone)]
pub struct Def {
    pub name: Name,
    pub context: TypingContext,
    pub body: Statement,
}

impl std::fmt::Display for Def {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let args: Vec<String> = self.context.iter().map(|bnd| format!("{bnd}")).collect();
        write!(
            f,
            "def {}({}) := {};",
            self.name,
            args.join(", "),
            self.body
        )
    }
}
