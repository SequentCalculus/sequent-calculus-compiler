use printer::{
    tokens::{LANGLE, PIPE, RANGLE},
    DocAllocator, Print,
};

use crate::{
    syntax::Ty,
    syntax_var::term::FsTerm,
    syntax_var::{FsStatement, Var},
    traits::substitution::SubstVar,
};

use std::rc::Rc;

/// Focused Cut
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FsCut {
    pub ty: Ty,
    pub producer: Rc<FsTerm>,
    pub consumer: Rc<FsTerm>,
}

impl FsCut {
    pub fn new<T: Into<FsTerm>, S: Into<FsTerm>>(ty: Ty, prd: T, cns: S) -> Self {
        FsCut {
            ty,
            producer: Rc::new(prd.into()),
            consumer: Rc::new(cns.into()),
        }
    }
}

impl Print for FsCut {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        let FsCut {
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

impl From<FsCut> for FsStatement {
    fn from(value: FsCut) -> Self {
        FsStatement::Cut(value)
    }
}

impl SubstVar for FsCut {
    type Target = FsCut;

    fn subst_sim(self, subst: &[(Var, Var)]) -> FsCut {
        FsCut {
            ty: self.ty,
            producer: self.producer.subst_sim(subst),
            consumer: self.consumer.subst_sim(subst),
        }
    }
}
