use super::{stringify_and_join, Name, Statement, TypingContext, Var};

use crate::traits::substitution::SubstVar;

use std::{fmt, rc::Rc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Clause {
    pub xtor: Name,
    pub context: TypingContext,
    pub case: Rc<Statement>,
}

impl fmt::Display for Clause {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let context = stringify_and_join(&self.context, ", ");
        write!(f, "{}({}) => {}", self.xtor, context, self.case)
    }
}

impl SubstVar for Clause {
    type Target = Clause;

    fn subst_sim(self, subst: &[(Var, Var)]) -> Clause {
        Clause {
            case: self.case.subst_sim(subst),
            ..self
        }
    }
}
