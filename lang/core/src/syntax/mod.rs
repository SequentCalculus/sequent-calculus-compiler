pub mod case;
pub mod clause;
pub mod cocase;
pub mod constructor;
pub mod covariable;
pub mod cut;
pub mod destructor;
pub mod fun;
pub mod ifz;
pub mod literal;
pub mod mu;
pub mod mutilde;
pub mod names;
pub mod op;
pub mod producer;
pub mod variable;

pub use case::Case;
pub use clause::Clause;
pub use cocase::Cocase;
pub use constructor::Constructor;
pub use covariable::Covariable;
pub use cut::Cut;
pub use destructor::Destructor;
pub use fun::Fun;
pub use ifz::IfZ;
pub use literal::Literal;
pub use mu::Mu;
pub use mutilde::MuTilde;
pub use names::{BinOp, Covar, Ctor, Dtor, Name, Var};
pub use op::Op;
pub use producer::Producer;
pub use variable::Variable;

use super::traits::free_vars::FreeV;
use super::traits::substitution::Subst;
use std::collections::HashSet;
use std::fmt;

// Consumer
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Consumer {
    Covar(Covariable),
    MuTilde(MuTilde),
    Case(Case),
    Destructor(Destructor),
}

impl std::fmt::Display for Consumer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Consumer::Covar(cv) => write!(f, "{}", cv),
            Consumer::MuTilde(m) => m.fmt(f),
            Consumer::Case(case) => case.fmt(f),
            Consumer::Destructor(d) => d.fmt(f),
        }
    }
}

impl FreeV for Consumer {
    fn free_vars(self: &Consumer) -> HashSet<Var> {
        match self {
            Consumer::Covar(_) => HashSet::new(),
            Consumer::MuTilde(m) => m.free_vars(),
            Consumer::Case(pts) => pts.free_vars(),
            Consumer::Destructor(d) => d.free_vars(),
        }
    }

    fn free_covars(self: &Consumer) -> HashSet<Covar> {
        match self {
            Consumer::Covar(covar) => covar.free_covars(),
            Consumer::MuTilde(m) => m.free_covars(),
            Consumer::Case(c) => c.free_covars(),
            Consumer::Destructor(d) => d.free_covars(),
        }
    }
}

impl Subst for Consumer {
    type Target = Consumer;
    fn subst_sim(
        self: &Consumer,
        prod_subst: &[(Producer, Var)],
        cons_subst: &[(Consumer, Covar)],
    ) -> Consumer {
        match self {
            Consumer::Covar(covar) => covar.subst_sim(prod_subst, cons_subst),
            Consumer::MuTilde(m) => m.subst_sim(prod_subst, cons_subst).into(),
            Consumer::Case(pts) => Consumer::Case(pts.subst_sim(prod_subst, cons_subst)),
            Consumer::Destructor(d) => d.subst_sim(prod_subst, cons_subst).into(),
        }
    }
}

// Statement
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    Cut(Cut),
    Op(Op),
    IfZ(IfZ),
    Fun(Fun),
    Done(),
}

impl std::fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Cut(c) => c.fmt(f),
            Statement::Op(op) => op.fmt(f),
            Statement::IfZ(i) => i.fmt(f),
            Statement::Fun(fun) => fun.fmt(f),
            Statement::Done() => write!(f, "Done"),
        }
    }
}

impl FreeV for Statement {
    fn free_vars(self: &Statement) -> HashSet<Var> {
        match self {
            Statement::Cut(c) => c.free_vars(),
            Statement::Op(op) => op.free_vars(),
            Statement::IfZ(i) => i.free_vars(),
            Statement::Fun(f) => f.free_vars(),
            Statement::Done() => HashSet::new(),
        }
    }
    fn free_covars(self: &Statement) -> HashSet<Covar> {
        match self {
            Statement::Cut(c) => c.free_covars(),
            Statement::Op(op) => op.free_covars(),
            Statement::IfZ(i) => i.free_covars(),
            Statement::Fun(f) => f.free_covars(),
            Statement::Done() => HashSet::new(),
        }
    }
}

impl Subst for Statement {
    type Target = Statement;
    fn subst_sim(
        self: &Statement,
        prod_subst: &[(Producer, Var)],
        cons_subst: &[(Consumer, Covar)],
    ) -> Statement {
        match self {
            Statement::Cut(c) => c.subst_sim(prod_subst, cons_subst).into(),
            Statement::Op(o) => o.subst_sim(prod_subst, cons_subst).into(),
            Statement::IfZ(i) => i.subst_sim(prod_subst, cons_subst).into(),
            Statement::Fun(f) => f.subst_sim(prod_subst, cons_subst).into(),
            Statement::Done() => Statement::Done(),
        }
    }
}

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
        let pargs_joined: String = self
            .pargs
            .iter()
            .map(|(x, _)| x.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        let cargs_joined: String = self
            .cargs
            .iter()
            .map(|(x, _)| x.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        write!(
            f,
            "def {}({};{}) := {};",
            self.name, pargs_joined, cargs_joined, self.body
        )
    }
}

// Prog
//
//

#[derive(Debug, Clone)]
pub struct Prog<T> {
    pub prog_defs: Vec<Def<T>>,
}

impl<T> fmt::Display for Prog<T> {
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
