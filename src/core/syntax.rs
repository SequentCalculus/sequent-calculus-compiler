use crate::fun::syntax::show_vec;
use crate::fun::syntax::BinOp;
use crate::fun::syntax::Ctor;
use crate::fun::syntax::Dtor;
use std::fmt;
use std::rc::Rc;

type Variable = String;
type Covariable = String;
type Name = String;

struct Pattern<T> {
    xtor: T,
    patv: Vec<Variable>,
    patcv: Vec<Covariable>,
    patst: Rc<Statement>,
}
enum Producer {
    Var(Variable),
    Lit(i64),
    Mu(Covariable, Rc<Statement>),
    Constructor(Ctor, Vec<Rc<Producer>>, Vec<Rc<Consumer>>),
    Cocase(Vec<Pattern<Dtor>>),
}

enum Consumer {
    Covar(Covariable),
    MuTilde(Variable, Rc<Statement>),
    MuTildeDyn(Variable, Rc<Statement>),
    Case(Vec<Pattern<Dtor>>),
    Destructor(Dtor, Vec<Rc<Producer>>, Vec<Rc<Consumer>>),
}

enum Statement {
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
            show_vec(&self.patv),
            show_vec(&self.patcv),
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
            Producer::Constructor(ctor, args, coargs) => {
                write!(f, "{}({};{})", ctor, show_vec(args), show_vec(coargs))
            }
            Producer::Cocase(pts) => write!(f, "cocase {{ {} }}", show_vec(pts)),
        }
    }
}

impl std::fmt::Display for Consumer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Consumer::Covar(cv) => write!(f, "{}", cv),
            Consumer::MuTilde(v, st) => write!(f, "mutilde {}. {}", v, st),
            Consumer::MuTildeDyn(v, st) => write!(f, "mutilde {}. {}", v, st),
            Consumer::Case(pts) => write!(f, "case {{ {} }}", show_vec(pts)),
            Consumer::Destructor(dt, args, coargs) => {
                write!(f, "{}({};{})", dt, show_vec(args), show_vec(coargs))
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
                write!(f, "{}({};{})", nm, show_vec(args), show_vec(coargs))
            }
            Statement::Done() => write!(f, "Done"),
        }
    }
}
