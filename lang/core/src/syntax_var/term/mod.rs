use printer::Print;

use crate::{
    syntax::term::{mu::FsMu, xtor::FsXtor, Literal},
    syntax_var::Var,
    traits::substitution::SubstVar,
};

pub mod xcase;
pub mod xvar;

pub use xcase::FsXCase;
pub use xvar::FsXVar;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FsTerm {
    XVar(FsXVar),
    Literal(Literal),
    Mu(FsMu),
    Xtor(FsXtor),
    XCase(FsXCase),
}

impl Print for FsTerm {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        match self {
            FsTerm::XVar(var) => var.print(cfg, alloc),
            FsTerm::Literal(lit) => lit.print(cfg, alloc),
            FsTerm::Mu(mu) => mu.print(cfg, alloc),
            FsTerm::Xtor(xtor) => xtor.print(cfg, alloc),
            FsTerm::XCase(xcase) => xcase.print(cfg, alloc),
        }
    }
}

impl SubstVar for FsTerm {
    type Target = FsTerm;
    fn subst_sim(self, subst: &[(Var, Var)]) -> Self::Target {
        match self {
            FsTerm::XVar(var) => var.subst_sim(subst).into(),
            FsTerm::Literal(lit) => FsTerm::Literal(lit),
            FsTerm::Mu(mu) => mu.subst_sim(subst).into(),
            FsTerm::Xtor(xtor) => xtor.subst_sim(subst).into(),
            FsTerm::XCase(xcase) => xcase.subst_sim(subst).into(),
        }
    }
}
