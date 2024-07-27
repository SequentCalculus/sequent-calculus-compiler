use crate::fun::syntax::{BinOp, Ctor, Dtor};
use std::collections::HashSet;
use std::fmt;
use std::rc::Rc;

use super::substitution::FreeV;

type Var = String;
type Covariable = String;
type Name = String;

// Pattern
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Clause<T> {
    pub xtor: T,
    pub vars: Vec<Var>,
    pub covars: Vec<Covariable>,
    pub rhs: Rc<Statement>,
}

impl<T: fmt::Display> fmt::Display for Clause<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}({};{}) => {}",
            self.xtor,
            self.vars.join(", "),
            self.covars.join(", "),
            self.rhs
        )
    }
}

impl<T> FreeV for Clause<T> {
    fn free_vars(self: &Clause<T>) -> HashSet<crate::fun::syntax::Variable> {
        let free_pt = self.rhs.free_vars();
        let unfree = HashSet::from_iter(self.vars.iter().cloned());
        free_pt.difference(&unfree).cloned().collect()
    }
    fn free_covars(self: &Clause<T>) -> HashSet<crate::fun::syntax::Covariable> {
        let free_pt = self.rhs.free_covars();
        let unfree = HashSet::from_iter(self.covars.iter().cloned());
        free_pt.difference(&unfree).cloned().collect()
    }
}

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
    fn free_vars(self: &Producer) -> HashSet<crate::fun::syntax::Variable> {
        match self {
            Producer::Variable(v) => v.free_vars(),
            Producer::Literal(l) => l.free_vars(),
            Producer::Mu(m) => m.free_vars(),
            Producer::Constructor(c) => c.free_vars(),
            Producer::Cocase(pts) => FreeV::free_vars(pts),
        }
    }

    fn free_covars(self: &Producer) -> HashSet<Covariable> {
        match self {
            Producer::Variable(v) => v.free_covars(),
            Producer::Literal(l) => l.free_covars(),
            Producer::Mu(m) => m.free_covars(),
            Producer::Constructor(c) => c.free_covars(),
            Producer::Cocase(pts) => FreeV::free_covars(pts),
        }
    }
}

// Variable
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Variable {
    pub var: Var,
}

impl std::fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.var)
    }
}

impl FreeV for Variable {
    fn free_vars(&self) -> HashSet<crate::fun::syntax::Variable> {
        HashSet::from([self.var.clone()])
    }

    fn free_covars(&self) -> HashSet<crate::fun::syntax::Covariable> {
        HashSet::new()
    }
}

impl From<Variable> for Producer {
    fn from(value: Variable) -> Self {
        Producer::Variable(value)
    }
}

#[cfg(test)]
mod variable_tests {
    use std::collections::HashSet;

    use crate::core::{substitution::FreeV, syntax::Variable};

    #[test]
    fn display_test() {
        let ex = Variable {
            var: "x".to_string(),
        };
        assert_eq!(format!("{ex}"), "x")
    }

    #[test]
    fn free_vars_test() {
        let ex = Variable {
            var: "x".to_string(),
        };
        let mut res = HashSet::new();
        res.insert("x".to_string());
        assert_eq!(ex.free_vars(), res)
    }

    #[test]
    fn free_covars_test() {
        let ex = Variable {
            var: "x".to_string(),
        };
        assert_eq!(ex.free_covars(), HashSet::new())
    }
}

// Literal
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Literal {
    pub lit: i64,
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.lit)
    }
}

impl FreeV for Literal {
    fn free_vars(&self) -> HashSet<crate::fun::syntax::Variable> {
        HashSet::new()
    }

    fn free_covars(&self) -> HashSet<crate::fun::syntax::Covariable> {
        HashSet::new()
    }
}

impl From<Literal> for Producer {
    fn from(value: Literal) -> Self {
        Producer::Literal(value)
    }
}

// Mu
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mu {
    pub covariable: Covariable,
    pub statement: Rc<Statement>,
}

impl std::fmt::Display for Mu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "mu {}.{}", self.covariable, self.statement)
    }
}

impl FreeV for Mu {
    fn free_vars(&self) -> HashSet<crate::fun::syntax::Variable> {
        FreeV::free_vars(Rc::as_ref(&self.statement))
    }

    fn free_covars(&self) -> HashSet<crate::fun::syntax::Covariable> {
        let mut fr_cv = FreeV::free_covars(Rc::as_ref(&self.statement));
        fr_cv.remove(&self.covariable);
        fr_cv
    }
}

impl From<Mu> for Producer {
    fn from(value: Mu) -> Self {
        Producer::Mu(value)
    }
}

// Constructor
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Constructor {
    pub id: Ctor,
    pub producers: Vec<Rc<Producer>>,
    pub consumers: Vec<Rc<Consumer>>,
}

impl std::fmt::Display for Constructor {
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
        write!(f, "{}({};{})", self.id, args_joined, coargs_joined)
    }
}

impl FreeV for Constructor {
    fn free_vars(&self) -> HashSet<crate::fun::syntax::Variable> {
        let free_args = self.producers.free_vars();
        let free_coargs = self.consumers.free_vars();
        free_args.union(&free_coargs).cloned().collect()
    }

    fn free_covars(&self) -> HashSet<crate::fun::syntax::Covariable> {
        let free_args = self.producers.free_covars();
        let free_coargs = self.consumers.free_covars();
        free_args.union(&free_coargs).cloned().collect()
    }
}

impl From<Constructor> for Producer {
    fn from(value: Constructor) -> Self {
        Producer::Constructor(value)
    }
}

// Cocase
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cocase {
    pub cocases: Vec<Clause<Dtor>>,
}

impl std::fmt::Display for Cocase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pts_joined: String = self
            .cocases
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        write!(f, "cocase {{ {} }}", pts_joined)
    }
}

impl FreeV for Cocase {
    fn free_vars(&self) -> HashSet<crate::fun::syntax::Variable> {
        self.cocases.free_vars()
    }

    fn free_covars(&self) -> HashSet<crate::fun::syntax::Covariable> {
        self.cocases.free_covars()
    }
}

impl From<Cocase> for Producer {
    fn from(value: Cocase) -> Self {
        Producer::Cocase(value)
    }
}

// Consumer
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Consumer {
    Covar(Covariable),
    MuTilde(Var, Rc<Statement>),
    Case(Vec<Clause<Ctor>>),
    Destructor(Dtor, Vec<Rc<Producer>>, Vec<Rc<Consumer>>),
}

impl std::fmt::Display for Consumer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Consumer::Covar(cv) => write!(f, "{}", cv),
            Consumer::MuTilde(v, st) => write!(f, "mutilde {}. {}", v, st),
            Consumer::Case(pts) => {
                let pts_joined: String = pts
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                write!(f, "case {{ {} }}", pts_joined)
            }
            Consumer::Destructor(dt, args, coargs) => {
                let args_joined: String = args
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                let coargs_joined: String = coargs
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                write!(f, "{}({};{})", dt, args_joined, coargs_joined)
            }
        }
    }
}

// Cut
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cut {
    pub producer: Rc<Producer>,
    pub consumer: Rc<Consumer>,
}

impl std::fmt::Display for Cut {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Cut { producer, consumer } = self;
        write!(f, "<{}|{}>", producer, consumer)
    }
}

impl FreeV for Cut {
    fn free_vars(&self) -> HashSet<crate::fun::syntax::Variable> {
        let Cut { producer, consumer } = self;
        let free_p = producer.free_vars();
        let free_c = consumer.free_vars();
        free_p.union(&free_c).cloned().collect()
    }

    fn free_covars(&self) -> HashSet<crate::fun::syntax::Covariable> {
        let Cut { producer, consumer } = self;
        let free_p = producer.free_covars();
        let free_c = consumer.free_covars();
        free_p.union(&free_c).cloned().collect()
    }
}

impl From<Cut> for Statement {
    fn from(value: Cut) -> Self {
        Statement::Cut(value)
    }
}

// Op
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Op {
    pub fst: Rc<Producer>,
    pub op: BinOp,
    pub snd: Rc<Producer>,
    pub continuation: Rc<Consumer>,
}

impl std::fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}({},{};{})",
            self.op, self.fst, self.snd, self.continuation
        )
    }
}

impl FreeV for Op {
    fn free_vars(&self) -> HashSet<crate::fun::syntax::Variable> {
        let free_p1 = self.fst.free_vars();
        let free_p2 = self.snd.free_vars();
        let free_c = self.continuation.free_vars();
        let free_p: HashSet<crate::fun::syntax::Variable> =
            free_p1.union(&free_p2).cloned().collect();
        free_p.union(&free_c).cloned().collect()
    }

    fn free_covars(&self) -> HashSet<crate::fun::syntax::Covariable> {
        let free_p1 = self.fst.free_covars();
        let free_p2 = self.snd.free_covars();
        let free_c = self.continuation.free_covars();
        let free_p: HashSet<crate::fun::syntax::Covariable> =
            free_p1.union(&free_p2).cloned().collect();
        free_p.union(&free_c).cloned().collect()
    }
}

impl From<Op> for Statement {
    fn from(value: Op) -> Self {
        Statement::Op(value)
    }
}

// IfZ
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IfZ {
    pub ifc: Rc<Producer>,
    pub thenc: Rc<Statement>,
    pub elsec: Rc<Statement>,
}

impl std::fmt::Display for IfZ {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "IfZ({};{},{})", self.ifc, self.thenc, self.elsec)
    }
}

impl From<IfZ> for Statement {
    fn from(value: IfZ) -> Self {
        Statement::IfZ(value)
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
    Fun(Name, Vec<Rc<Producer>>, Vec<Rc<Consumer>>),
    Done(),
}

impl std::fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Cut(c) => c.fmt(f),
            Statement::Op(op) => op.fmt(f),
            Statement::IfZ(i) => i.fmt(f),
            Statement::Fun(nm, args, coargs) => {
                let args_joined: String = args
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                let coargs_joined: String = coargs
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                write!(f, "{}({};{})", nm, args_joined, coargs_joined)
            }
            Statement::Done() => write!(f, "Done"),
        }
    }
}

// Def
//
//

#[derive(Clone)]
pub struct Def<T> {
    pub name: Name,
    pub pargs: Vec<(Var, T)>,
    pub cargs: Vec<(Covariable, T)>,
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
            "def {}({};{}) := {}",
            self.name, pargs_joined, cargs_joined, self.body
        )
    }
}

// Prog
//
//

pub struct Prog<T> {
    pub prog_defs: Vec<Def<T>>,
}

impl<T: fmt::Display> fmt::Display for Prog<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let defs_joined: String = self
            .prog_defs
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        write!(f, "{}", defs_joined)
    }
}
