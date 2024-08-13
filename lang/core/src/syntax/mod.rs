pub mod case;
pub mod clause;
pub mod cocase;
pub mod constructor;
pub mod covariable;
pub mod cut;
pub mod destructor;
pub mod ifz;
pub mod literal;
pub mod mu;
pub mod mutilde;
pub mod names;
pub mod op;
pub mod variable;

pub use case::Case;
pub use clause::Clause;
pub use cocase::Cocase;
pub use constructor::Constructor;
pub use covariable::Covariable;
pub use cut::Cut;
pub use destructor::Destructor;
pub use ifz::IfZ;
pub use literal::Literal;
pub use mu::Mu;
pub use mutilde::MuTilde;
pub use names::{BinOp, Covar, Ctor, Dtor, Name, Var};
pub use op::Op;
pub use variable::Variable;

use super::traits::free_vars::FreeV;
use super::traits::substitution::Subst;
use std::collections::HashSet;
use std::fmt;
use std::rc::Rc;

// Producer
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Producer {
    Variable(Variable),
    Literal(Literal),
    Mu(Mu),
    Constructor(Constructor),
    Cocase(Cocase),
}

impl Producer {
    pub fn is_value(&self) -> bool {
        match self {
            Producer::Literal(_) => true,
            Producer::Variable(_) => true,
            Producer::Cocase(_) => true,
            Producer::Constructor(c) => c.producers.iter().all(|p| p.is_value()),
            Producer::Mu(_) => false,
        }
    }
}

impl std::fmt::Display for Producer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Producer::Variable(v) => v.fmt(f),
            Producer::Literal(i) => i.fmt(f),
            Producer::Mu(m) => m.fmt(f),
            Producer::Constructor(c) => c.fmt(f),
            Producer::Cocase(c) => c.fmt(f),
        }
    }
}

impl FreeV for Producer {
    fn free_vars(self: &Producer) -> HashSet<crate::syntax::Var> {
        match self {
            Producer::Variable(v) => v.free_vars(),
            Producer::Literal(l) => l.free_vars(),
            Producer::Mu(m) => m.free_vars(),
            Producer::Constructor(c) => c.free_vars(),
            Producer::Cocase(c) => c.free_vars(),
        }
    }

    fn free_covars(self: &Producer) -> HashSet<Covar> {
        match self {
            Producer::Variable(v) => v.free_covars(),
            Producer::Literal(l) => l.free_covars(),
            Producer::Mu(m) => m.free_covars(),
            Producer::Constructor(c) => c.free_covars(),
            Producer::Cocase(c) => c.free_covars(),
        }
    }
}

impl Subst for Producer {
    type Target = Producer;
    fn subst_sim(
        self: &Producer,
        prod_subst: &[(Producer, Var)],
        cons_subst: &[(Consumer, Covar)],
    ) -> Producer {
        match self {
            Producer::Variable(v) => v.subst_sim(prod_subst, cons_subst),
            Producer::Literal(l) => l.subst_sim(prod_subst, cons_subst).into(),
            Producer::Mu(m) => m.subst_sim(prod_subst, cons_subst).into(),
            Producer::Constructor(c) => c.subst_sim(prod_subst, cons_subst).into(),
            Producer::Cocase(c) => c.subst_sim(prod_subst, cons_subst).into(),
        }
    }
}

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

// Fun
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fun {
    pub name: Name,
    pub producers: Vec<Producer>,
    pub consumers: Vec<Consumer>,
}

impl std::fmt::Display for Fun {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let args_joined: String = self
            .producers
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        let coargs_joined: String = self
            .consumers
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        write!(f, "{}({};{})", self.name, args_joined, coargs_joined)
    }
}

impl From<Fun> for Statement {
    fn from(value: Fun) -> Self {
        Statement::Fun(value)
    }
}

impl FreeV for Fun {
    fn free_vars(&self) -> HashSet<Var> {
        let free_p = self.producers.free_vars();
        let free_c = self.consumers.free_vars();
        free_p.union(&free_c).cloned().collect()
    }

    fn free_covars(&self) -> HashSet<Covar> {
        let free_p = self.producers.free_covars();
        let free_c = self.consumers.free_covars();
        free_p.union(&free_c).cloned().collect()
    }
}
impl Subst for Fun {
    type Target = Fun;

    fn subst_sim(
        &self,
        prod_subst: &[(Producer, Var)],
        cons_subst: &[(Consumer, Covar)],
    ) -> Self::Target {
        Fun {
            name: self.name.clone(),
            producers: self.producers.subst_sim(prod_subst, cons_subst),
            consumers: self.consumers.subst_sim(prod_subst, cons_subst),
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
