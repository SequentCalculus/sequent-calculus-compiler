use crate::{
    syntax_var::term::Term,
    syntax_var::{Statement, Ty, Var},
    traits::substitution::SubstVar,
};

use std::{fmt, rc::Rc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cut {
    pub ty: Ty,
    pub producer: Rc<Term>,
    pub consumer: Rc<Term>,
}

impl Cut {
    pub fn new<T: Into<Term>, S: Into<Term>>(ty: Ty, prd: T, cns: S) -> Self {
        Cut {
            ty,
            producer: Rc::new(prd.into()),
            consumer: Rc::new(cns.into()),
        }
    }
}

impl std::fmt::Display for Cut {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Cut {
            producer,
            consumer,
            ty: _,
        } = self;
        write!(f, "<{producer} | {consumer}>")
    }
}

impl From<Cut> for Statement {
    fn from(value: Cut) -> Self {
        Statement::Cut(value)
    }
}

impl SubstVar for Cut {
    type Target = Cut;

    fn subst_sim(self, subst: &[(Var, Var)]) -> Cut {
        Cut {
            ty: self.ty,
            producer: self.producer.subst_sim(subst),
            consumer: self.consumer.subst_sim(subst),
        }
    }
}
