use printer::{
    tokens::{LANGLE, PIPE, RANGLE},
    DocAllocator, Print,
};

use crate::{
    syntax_var::term::Term,
    syntax_var::{Statement, Ty, Var},
    traits::substitution::SubstVar,
};

use std::rc::Rc;

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

impl Print for Cut {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        let Cut {
            ty: _,
            producer,
            consumer,
        } = self;
        alloc.text(LANGLE).append(
            producer
                .print(cfg, alloc)
                .append(alloc.space())
                .append(alloc.text(PIPE))
                .append(alloc.space())
                .append(consumer.print(cfg, alloc))
                .append(alloc.text(RANGLE)),
        )
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
