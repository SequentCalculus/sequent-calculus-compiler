use std::collections::HashSet;
use std::fmt;
use std::rc::Rc;

use super::traits::free_vars::{fresh_covar, fresh_var, FreeV};
use super::traits::substitution::Subst;

pub type Var = String;
pub type Covar = String;
pub type Name = String;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinOp {
    Prod,
    Sum,
    Sub,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ctor {
    Nil,
    Cons,
    Tup,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Dtor {
    Hd,
    Tl,
    Fst,
    Snd,
    Ap,
}

impl fmt::Display for BinOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BinOp::Prod => write!(f, "*"),
            BinOp::Sum => write!(f, "+"),
            BinOp::Sub => write!(f, "-"),
        }
    }
}

impl fmt::Display for Ctor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Ctor::Nil => write!(f, "Nil"),
            Ctor::Cons => write!(f, "Cons"),
            Ctor::Tup => write!(f, "Tup"),
        }
    }
}

impl fmt::Display for Dtor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Dtor::Hd => write!(f, "hd"),
            Dtor::Tl => write!(f, "tl"),
            Dtor::Fst => write!(f, "fst"),
            Dtor::Snd => write!(f, "snd"),
            Dtor::Ap => write!(f, "ap"),
        }
    }
}

// Clause
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Clause<T> {
    pub xtor: T,
    pub vars: Vec<Var>,
    pub covars: Vec<Covar>,
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
    fn free_vars(self: &Clause<T>) -> HashSet<Var> {
        let free_pt = self.rhs.free_vars();
        let unfree = HashSet::from_iter(self.vars.iter().cloned());
        free_pt.difference(&unfree).cloned().collect()
    }
    fn free_covars(self: &Clause<T>) -> HashSet<Covar> {
        let free_pt = self.rhs.free_covars();
        let unfree = HashSet::from_iter(self.covars.iter().cloned());
        free_pt.difference(&unfree).cloned().collect()
    }
}

impl<T: Clone> Subst for Clause<T> {
    type Target = Clause<T>;
    fn subst_sim(
        self: &Clause<T>,
        prod_subst: &[(Producer, Var)],
        cons_subst: &[(Consumer, Covar)],
    ) -> Clause<T> {
        let mut fr_v = self.rhs.free_vars();
        let mut fr_cv = self.rhs.free_covars();
        for (prod, var) in prod_subst.iter() {
            fr_v.extend(prod.free_vars());
            fr_v.insert(var.clone());

            fr_cv.extend(prod.free_covars());
        }
        for (cons, covar) in cons_subst.iter() {
            fr_v.extend(cons.free_vars());

            fr_cv.extend(cons.free_covars());
            fr_cv.insert(covar.clone());
        }

        let mut new_vars: Vec<Var> = vec![];
        let mut var_subst: Vec<(Producer, Var)> = vec![];

        for old_var in self.vars.iter() {
            let new_var: Var = fresh_var(&fr_v);
            fr_v.insert(new_var.clone());
            new_vars.insert(0, new_var.clone());
            var_subst.insert(
                0,
                (
                    crate::syntax::Variable { var: new_var }.into(),
                    old_var.clone(),
                ),
            )
        }

        let mut new_covars: Vec<Covar> = vec![];
        let mut covar_subst: Vec<(Consumer, Covar)> = vec![];

        for old_covar in self.covars.iter() {
            let new_covar: Covar = fresh_covar(&fr_cv);
            fr_cv.insert(new_covar.clone());
            new_covars.insert(0, new_covar.clone());
            covar_subst.insert(
                0,
                (Covariable { covar: new_covar }.into(), old_covar.clone()),
            )
        }

        let new_st = self.rhs.subst_sim(&var_subst, &covar_subst);

        Clause {
            xtor: self.xtor.clone(),
            vars: new_vars,
            covars: new_covars,
            rhs: new_st.subst_sim(prod_subst, cons_subst),
        }
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
    fn free_vars(&self) -> HashSet<Var> {
        HashSet::from([self.var.clone()])
    }

    fn free_covars(&self) -> HashSet<Covar> {
        HashSet::new()
    }
}

impl From<Variable> for Producer {
    fn from(value: Variable) -> Self {
        Producer::Variable(value)
    }
}

impl Subst for Variable {
    type Target = Producer;

    fn subst_sim(
        &self,
        prod_subst: &[(Producer, Var)],
        _cons_subst: &[(Consumer, Covar)],
    ) -> Self::Target {
        let crate::syntax::Variable { var } = self;
        match prod_subst.iter().find(|(_, v)| v == var) {
            None => crate::syntax::Variable { var: var.clone() }.into(),
            Some((p, _)) => p.clone(),
        }
    }
}

#[cfg(test)]
mod variable_tests {
    use std::collections::HashSet;

    use crate::{syntax::Variable, traits::free_vars::FreeV};

    #[test]
    fn display() {
        let ex = Variable {
            var: "x".to_string(),
        };
        assert_eq!(format!("{ex}"), "x")
    }

    #[test]
    fn free_vars() {
        let ex = Variable {
            var: "x".to_string(),
        };
        let mut res = HashSet::new();
        res.insert("x".to_string());
        assert_eq!(ex.free_vars(), res)
    }

    #[test]
    fn free_covars() {
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
    fn free_vars(&self) -> HashSet<Var> {
        HashSet::new()
    }

    fn free_covars(&self) -> HashSet<Covar> {
        HashSet::new()
    }
}

impl From<Literal> for Producer {
    fn from(value: Literal) -> Self {
        Producer::Literal(value)
    }
}

impl Subst for Literal {
    type Target = Literal;

    fn subst_sim(
        &self,
        _prod_subst: &[(Producer, Var)],
        _cons_subst: &[(Consumer, Covar)],
    ) -> Self::Target {
        self.clone()
    }
}

#[cfg(test)]
mod literal_tests {
    use crate::syntax::Literal;

    #[test]
    fn display() {
        let ex = Literal { lit: 20 };
        assert_eq!(format!("{ex}"), "20".to_string())
    }
}

// Mu
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mu {
    pub covariable: Covar,
    pub statement: Rc<Statement>,
}

impl std::fmt::Display for Mu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "mu {}.{}", self.covariable, self.statement)
    }
}

impl FreeV for Mu {
    fn free_vars(&self) -> HashSet<Var> {
        FreeV::free_vars(Rc::as_ref(&self.statement))
    }

    fn free_covars(&self) -> HashSet<Covar> {
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

impl Subst for Mu {
    type Target = Mu;

    fn subst_sim(
        &self,
        prod_subst: &[(Producer, Var)],
        cons_subst: &[(Consumer, Covar)],
    ) -> Self::Target {
        let Mu {
            covariable,
            statement,
        } = self;
        let mut fr_cv: HashSet<Covar> = statement.free_vars();
        for (cons, cv) in cons_subst.iter() {
            fr_cv.insert(cv.clone());
            fr_cv.extend(cons.free_covars());
        }
        for (prod, _) in prod_subst.iter() {
            fr_cv.extend(prod.free_covars());
        }
        let new_covar: Covar = fresh_covar(&fr_cv);
        let new_st: Rc<Statement> = statement.subst_covar(
            Covariable {
                covar: new_covar.clone(),
            }
            .into(),
            covariable.clone(),
        );
        Mu {
            covariable: new_covar,
            statement: new_st.subst_sim(prod_subst, cons_subst),
        }
    }
}

#[cfg(test)]
mod mu_tests {
    use std::rc::Rc;

    use crate::syntax::Mu;

    use super::Statement;

    #[test]
    fn display() {
        let ex = Mu {
            covariable: "a".to_string(),
            statement: Rc::new(Statement::Done()),
        };
        assert_eq!(format!("{ex}"), "mu a.Done".to_string())
    }
}

// Constructor
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Constructor {
    pub id: Ctor,
    pub producers: Vec<Producer>,
    pub consumers: Vec<Consumer>,
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
    fn free_vars(&self) -> HashSet<Var> {
        let free_args = self.producers.free_vars();
        let free_coargs = self.consumers.free_vars();
        free_args.union(&free_coargs).cloned().collect()
    }

    fn free_covars(&self) -> HashSet<Covar> {
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

impl Subst for Constructor {
    type Target = Constructor;

    fn subst_sim(
        &self,
        prod_subst: &[(Producer, Var)],
        cons_subst: &[(Consumer, Covar)],
    ) -> Self::Target {
        Constructor {
            id: self.id.clone(),
            producers: self.producers.subst_sim(prod_subst, cons_subst),
            consumers: self.consumers.subst_sim(prod_subst, cons_subst),
        }
    }
}

#[cfg(test)]
mod constructor_tests {

    use crate::syntax::Ctor;

    use super::Constructor;

    #[test]
    fn display() {
        let ex = Constructor {
            id: Ctor::Cons,
            producers: vec![],
            consumers: vec![],
        };
        assert_eq!(format!("{ex}"), "Cons(;)".to_string())
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
    fn free_vars(&self) -> HashSet<Var> {
        self.cocases.free_vars()
    }

    fn free_covars(&self) -> HashSet<Covar> {
        self.cocases.free_covars()
    }
}

impl From<Cocase> for Producer {
    fn from(value: Cocase) -> Self {
        Producer::Cocase(value)
    }
}

impl Subst for Cocase {
    type Target = Cocase;

    fn subst_sim(
        &self,
        prod_subst: &[(Producer, Var)],
        cons_subst: &[(Consumer, Covar)],
    ) -> Self::Target {
        Cocase {
            cocases: self.cocases.subst_sim(prod_subst, cons_subst),
        }
    }
}

#[cfg(test)]
mod cocase_test {
    use crate::syntax::Cocase;

    #[test]
    fn display() {
        let ex = Cocase { cocases: vec![] };
        assert_eq!(format!("{ex}"), "cocase {  }".to_string());
    }
}

// Covar
//
//
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Covariable {
    pub covar: Covar,
}

impl std::fmt::Display for Covariable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.covar)
    }
}

impl FreeV for Covariable {
    fn free_vars(&self) -> HashSet<Var> {
        HashSet::new()
    }
    fn free_covars(&self) -> HashSet<Covar> {
        HashSet::from([self.covar.clone()])
    }
}

impl From<Covariable> for Consumer {
    fn from(cv: Covariable) -> Consumer {
        Consumer::Covar(cv)
    }
}

impl Subst for Covariable {
    type Target = Consumer;

    fn subst_sim(
        &self,
        _prod_subst: &[(Producer, Var)],
        cons_subst: &[(Consumer, Covar)],
    ) -> Self::Target {
        let crate::syntax::Covariable { covar } = self;
        match cons_subst.iter().find(|(_, cv)| cv == covar) {
            None => crate::syntax::Covariable {
                covar: covar.clone(),
            }
            .into(),
            Some((p, _)) => p.clone(),
        }
    }
}

#[cfg(test)]
mod covariable_tests {
    use std::collections::HashSet;

    use crate::{syntax::Covariable, traits::free_vars::FreeV};

    #[test]
    fn display() {
        let ex = Covariable {
            covar: "a".to_string(),
        };
        assert_eq!(format!("{ex}"), "a")
    }

    #[test]
    fn free_vars() {
        let ex = Covariable {
            covar: "a".to_string(),
        };
        assert_eq!(ex.free_covars(), HashSet::new())
    }

    #[test]
    fn free_covars() {
        let ex = Covariable {
            covar: "a".to_string(),
        };
        let mut res = HashSet::new();
        res.insert("a".to_string());
        assert_eq!(ex.free_vars(), res)
    }
}

// MuTilde
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MuTilde {
    pub variable: Var,
    pub statement: Rc<Statement>,
}

impl std::fmt::Display for MuTilde {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "mutilde {}. {}", self.variable, self.statement)
    }
}

impl FreeV for MuTilde {
    fn free_vars(&self) -> HashSet<Var> {
        let mut fr_st = self.statement.free_vars();
        fr_st.remove(&self.variable);
        fr_st
    }

    fn free_covars(&self) -> HashSet<Covar> {
        self.statement.free_covars()
    }
}

impl Subst for MuTilde {
    type Target = MuTilde;

    fn subst_sim(
        &self,
        prod_subst: &[(Producer, Var)],
        cons_subst: &[(Consumer, Covar)],
    ) -> Self::Target {
        let mut fr_v: HashSet<Var> = self.statement.free_vars();
        for (prod, var) in prod_subst.iter() {
            fr_v.extend(prod.free_vars());
            fr_v.insert(var.clone());
        }
        for (cons, _) in cons_subst.iter() {
            fr_v.extend(cons.free_vars());
        }
        let new_var: Var = fresh_var(&fr_v);
        let new_st = self.statement.subst_var(
            crate::syntax::Variable {
                var: new_var.clone(),
            }
            .into(),
            self.variable.clone(),
        );
        MuTilde {
            variable: new_var,
            statement: new_st.subst_sim(prod_subst, cons_subst),
        }
    }
}

impl From<MuTilde> for Consumer {
    fn from(value: MuTilde) -> Self {
        Consumer::MuTilde(value)
    }
}

// Destructor
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Destructor {
    pub id: Dtor,
    pub producers: Vec<Producer>,
    pub consumers: Vec<Consumer>,
}

impl std::fmt::Display for Destructor {
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

impl From<Destructor> for Consumer {
    fn from(value: Destructor) -> Self {
        Consumer::Destructor(value)
    }
}

impl FreeV for Destructor {
    fn free_vars(&self) -> HashSet<Var> {
        let free_args = self.producers.free_vars();
        let free_coargs = self.consumers.free_vars();
        free_args.union(&free_coargs).cloned().collect()
    }

    fn free_covars(&self) -> HashSet<Covar> {
        let free_args = self.producers.free_covars();
        let free_coargs = self.consumers.free_covars();
        free_args.union(&free_coargs).cloned().collect()
    }
}

impl Subst for Destructor {
    type Target = Destructor;

    fn subst_sim(
        &self,
        prod_subst: &[(Producer, Var)],
        cons_subst: &[(Consumer, Covar)],
    ) -> Self::Target {
        Destructor {
            id: self.id.clone(),
            producers: self.producers.subst_sim(prod_subst, cons_subst),
            consumers: self.consumers.subst_sim(prod_subst, cons_subst),
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
    Case(Vec<Clause<Ctor>>),
    Destructor(Destructor),
}

impl std::fmt::Display for Consumer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Consumer::Covar(cv) => write!(f, "{}", cv),
            Consumer::MuTilde(m) => m.fmt(f),
            Consumer::Case(pts) => {
                let pts_joined: String = pts
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                write!(f, "case {{ {} }}", pts_joined)
            }
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
    fn free_vars(&self) -> HashSet<Var> {
        let Cut { producer, consumer } = self;
        let free_p = producer.free_vars();
        let free_c = consumer.free_vars();
        free_p.union(&free_c).cloned().collect()
    }

    fn free_covars(&self) -> HashSet<Covar> {
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

impl Subst for Cut {
    type Target = Cut;

    fn subst_sim(
        &self,
        prod_subst: &[(Producer, Var)],
        cons_subst: &[(Consumer, Covar)],
    ) -> Self::Target {
        Cut {
            producer: self.producer.subst_sim(prod_subst, cons_subst),
            consumer: self.consumer.subst_sim(prod_subst, cons_subst),
        }
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
    fn free_vars(&self) -> HashSet<Var> {
        let free_p1 = self.fst.free_vars();
        let free_p2 = self.snd.free_vars();
        let free_c = self.continuation.free_vars();
        let free_p: HashSet<Var> = free_p1.union(&free_p2).cloned().collect();
        free_p.union(&free_c).cloned().collect()
    }

    fn free_covars(&self) -> HashSet<Covar> {
        let free_p1 = self.fst.free_covars();
        let free_p2 = self.snd.free_covars();
        let free_c = self.continuation.free_covars();
        let free_p: HashSet<Covar> = free_p1.union(&free_p2).cloned().collect();
        free_p.union(&free_c).cloned().collect()
    }
}

impl From<Op> for Statement {
    fn from(value: Op) -> Self {
        Statement::Op(value)
    }
}

impl Subst for Op {
    type Target = Op;

    fn subst_sim(
        &self,
        prod_subst: &[(Producer, Var)],
        cons_subst: &[(Consumer, Covar)],
    ) -> Self::Target {
        Op {
            fst: self.fst.subst_sim(prod_subst, cons_subst),
            op: self.op.clone(),
            snd: self.snd.subst_sim(prod_subst, cons_subst),
            continuation: self.continuation.subst_sim(prod_subst, cons_subst),
        }
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

impl FreeV for IfZ {
    fn free_vars(&self) -> HashSet<Var> {
        let free_p = self.ifc.free_vars();
        let free_st1 = self.thenc.free_vars();
        let free_st2 = self.elsec.free_vars();
        let free_st: HashSet<Var> = free_st1.union(&free_st2).cloned().collect();
        free_st.union(&free_p).cloned().collect()
    }

    fn free_covars(&self) -> HashSet<Covar> {
        let free_p = self.ifc.free_covars();
        let free_st1 = self.thenc.free_covars();
        let free_st2 = self.elsec.free_covars();
        let free_st: HashSet<Var> = free_st1.union(&free_st2).cloned().collect();
        free_st.union(&free_p).cloned().collect()
    }
}

impl Subst for IfZ {
    type Target = IfZ;

    fn subst_sim(
        &self,
        prod_subst: &[(Producer, Var)],
        cons_subst: &[(Consumer, Covar)],
    ) -> Self::Target {
        IfZ {
            ifc: self.ifc.subst_sim(prod_subst, cons_subst),
            thenc: self.thenc.subst_sim(prod_subst, cons_subst),
            elsec: self.elsec.subst_sim(prod_subst, cons_subst),
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

#[derive(Clone)]
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
