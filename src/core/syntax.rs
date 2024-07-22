use crate::fun::syntax::{BinOp, Ctor, Dtor};
use std::fmt;
use std::rc::Rc;

type Variable = String;
type Covariable = String;
type Name = String;

#[derive(Clone)]
pub struct Pattern<T> {
    pub xtor: T,
    pub patv: Vec<Variable>,
    pub patcv: Vec<Covariable>,
    pub patst: Rc<Statement>,
}

#[derive(Clone)]
pub enum Producer {
    Var(Variable),
    Lit(i64),
    Mu(Covariable, Rc<Statement>),
    MuDyn(Covariable, Rc<Statement>),
    Constructor(Ctor, Vec<Rc<Producer>>, Vec<Rc<Consumer>>),
    Cocase(Vec<Pattern<Dtor>>),
}

#[derive(Clone)]
pub enum Consumer {
    Covar(Covariable),
    MuTilde(Variable, Rc<Statement>),
    MuTildeDyn(Variable, Rc<Statement>),
    Case(Vec<Pattern<Ctor>>),
    Destructor(Dtor, Vec<Rc<Producer>>, Vec<Rc<Consumer>>),
}

#[derive(Clone)]
pub enum Statement {
    Cut(Rc<Producer>, Rc<Consumer>),
    Op(Rc<Producer>, BinOp, Rc<Producer>, Rc<Consumer>),
    IfZ(Rc<Producer>, Rc<Statement>, Rc<Statement>),
    Fun(Name, Vec<Rc<Producer>>, Vec<Rc<Consumer>>),
    Done(),
}

impl<T: fmt::Display> fmt::Display for Pattern<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}({};{}) => {}",
            self.xtor,
            self.patv.join(", "),
            self.patcv.join(", "),
            self.patst
        )
    }
}

impl std::fmt::Display for Producer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Producer::Var(v) => write!(f, "{}", v),
            Producer::Lit(i) => write!(f, "{}", i),
            Producer::Mu(cv, st) => write!(f, "mu {}.{}", cv, st),
            Producer::MuDyn(cv, st) => write!(f, "mu {}.{}", cv, st),
            Producer::Constructor(ctor, args, coargs) => {
                let args_joined: String = args
                    .iter()
                    .map(|x| format!("{}", x))
                    .collect::<Vec<String>>()
                    .join(", ");
                let coargs_joined: String = coargs
                    .iter()
                    .map(|x| format!("{}", x))
                    .collect::<Vec<String>>()
                    .join(", ");
                write!(f, "{}({};{})", ctor, args_joined, coargs_joined)
            }
            Producer::Cocase(pts) => {
                let pts_joined: String = pts
                    .iter()
                    .map(|x| format!("{}", x))
                    .collect::<Vec<String>>()
                    .join(", ");
                write!(f, "cocase {{ {} }}", pts_joined)
            }
        }
    }
}

impl std::fmt::Display for Consumer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Consumer::Covar(cv) => write!(f, "{}", cv),
            Consumer::MuTilde(v, st) => write!(f, "mutilde {}. {}", v, st),
            Consumer::MuTildeDyn(v, st) => write!(f, "mutilde {}. {}", v, st),
            Consumer::Case(pts) => {
                let pts_joined: String = pts
                    .iter()
                    .map(|x| format!("{}", x))
                    .collect::<Vec<String>>()
                    .join(", ");
                write!(f, "case {{ {} }}", pts_joined)
            }
            Consumer::Destructor(dt, args, coargs) => {
                let args_joined: String = args
                    .iter()
                    .map(|x| format!("{}", x))
                    .collect::<Vec<String>>()
                    .join(", ");
                let coargs_joined: String = coargs
                    .iter()
                    .map(|x| format!("{}", x))
                    .collect::<Vec<String>>()
                    .join(", ");
                write!(f, "{}({};{})", dt, args_joined, coargs_joined)
            }
        }
    }
}

impl std::fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Cut(p, c) => write!(f, "<{}|{}>", p, c),
            Statement::Op(p1, op, p2, c) => write!(f, "{}({},{};{})", op, p1, p2, c),
            Statement::IfZ(p, st1, st2) => write!(f, "IfZ({};{},{})", p, st1, st2),
            Statement::Fun(nm, args, coargs) => {
                let args_joined: String = args
                    .iter()
                    .map(|x| format!("{}", x))
                    .collect::<Vec<String>>()
                    .join(", ");
                let coargs_joined: String = coargs
                    .iter()
                    .map(|x| format!("{}", x))
                    .collect::<Vec<String>>()
                    .join(", ");
                write!(f, "{}({};{})", nm, args_joined, coargs_joined)
            }
            Statement::Done() => write!(f, "Done"),
        }
    }
}
