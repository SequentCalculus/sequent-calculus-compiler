use std::fmt;
use std::rc::Rc;

pub type Variable = String;
pub type Covariable = String;
pub type Name = String;

// BinOp
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinOp {
    Prod,
    Sum,
    Sub,
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

// Ctor
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ctor {
    Nil,
    Cons,
    Tup,
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

// Dtor
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Dtor {
    Hd,
    Tl,
    Fst,
    Snd,
}

impl fmt::Display for Dtor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Dtor::Hd => write!(f, "Hd"),
            Dtor::Tl => write!(f, "Tl"),
            Dtor::Fst => write!(f, "Fst"),
            Dtor::Snd => write!(f, "Snd"),
        }
    }
}

// Clause
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Clause<T> {
    pub xtor: T,
    pub vars: Vec<Variable>,
    pub rhs: Term,
}

impl<T: fmt::Display> fmt::Display for Clause<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.vars.is_empty() {
            write!(f, "{}=>{}", self.xtor, self.rhs)
        } else {
            write!(f, "{}({}) => {}", self.xtor, self.vars.join(", "), self.rhs)
        }
    }
}

// Op
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Op {
    pub fst: Rc<Term>,
    pub op: BinOp,
    pub snd: Rc<Term>,
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.fst, self.op, self.snd)
    }
}

impl From<Op> for Term {
    fn from(value: Op) -> Self {
        Term::Op(value)
    }
}

// IfZ
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IfZ {
    pub ifc: Rc<Term>,
    pub thenc: Rc<Term>,
    pub elsec: Rc<Term>,
}

impl fmt::Display for IfZ {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "IfZ {} then {} else {})",
            self.ifc, self.thenc, self.elsec
        )
    }
}

impl From<IfZ> for Term {
    fn from(value: IfZ) -> Self {
        Term::IfZ(value)
    }
}

// Let
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Let {
    pub variable: Variable,
    pub bound_term: Rc<Term>,
    pub in_term: Rc<Term>,
}

impl fmt::Display for Let {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Let {}={} in {}",
            self.variable, self.bound_term, self.in_term
        )
    }
}

impl From<Let> for Term {
    fn from(value: Let) -> Self {
        Term::Let(value)
    }
}

// Fun
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fun {
    pub name: Name,
    pub args: Vec<Rc<Term>>,
    pub coargs: Vec<Covariable>,
}

impl fmt::Display for Fun {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let args_joined: String = self
            .args
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        write!(
            f,
            "{}({};{})",
            self.name,
            args_joined,
            self.coargs.join(", ")
        )
    }
}

impl From<Fun> for Term {
    fn from(value: Fun) -> Self {
        Term::Fun(value)
    }
}

// Constructor
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Constructor {
    pub id: Ctor,
    pub args: Vec<Rc<Term>>,
}

impl fmt::Display for Constructor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.args.is_empty() {
            write!(f, "{}", self.id)
        } else {
            let args_joined: String = self
                .args
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(", ");
            write!(f, "{}({})", self.id, args_joined)
        }
    }
}

impl From<Constructor> for Term {
    fn from(value: Constructor) -> Self {
        Term::Constructor(value)
    }
}

// Term
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Term {
    Var(Variable),
    Lit(i64),
    Op(Op),
    IfZ(IfZ),
    Let(Let),
    Fun(Fun),
    Constructor(Constructor),
    Destructor(Rc<Term>, Dtor, Vec<Rc<Term>>),
    Case(Rc<Term>, Vec<Rc<Clause<Ctor>>>),
    Cocase(Vec<Rc<Clause<Dtor>>>),
    Lam(Variable, Rc<Term>),
    App(Rc<Term>, Rc<Term>),
    Goto(Rc<Term>, Covariable),
    Label(Covariable, Rc<Term>),
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Term::Var(v) => write!(f, "{}", v),
            Term::Lit(n) => write!(f, "{}", n),
            Term::Op(o) => o.fmt(f),
            Term::IfZ(i) => i.fmt(f),
            Term::Let(l) => l.fmt(f),
            Term::Fun(fun) => fun.fmt(f),
            Term::Constructor(c) => c.fmt(f),
            Term::Destructor(t, dtor, args) if args.is_empty() => write!(f, "{}.{}", t, dtor),
            Term::Destructor(t, dtor, args) => {
                let args_joined: String = args
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                write!(f, "{}.{}({})", t, dtor, args_joined)
            }
            Term::Case(t, clauses) => {
                let clauses_joined: String = clauses
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                write!(f, "case {} of {{ {} }}", t, clauses_joined)
            }
            Term::Cocase(clauses) => {
                let clauses_joined: String = clauses
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                write!(f, "cocase {{ {} }}", clauses_joined)
            }
            Term::Lam(v, t) => write!(f, "\\{}.{}", v, t),
            Term::App(t1, t2) => write!(f, "{} {}", t1, t2),
            Term::Goto(t, cv) => write!(f, "goto({};{})", t, cv),
            Term::Label(cv, t) => write!(f, "label {} {{{}}}", cv, t),
        }
    }
}

// Def
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Def<T> {
    pub name: Name,
    pub args: Vec<(Variable, T)>,
    pub cont: Vec<(Covariable, T)>,
    pub body: Term,
    pub ret_ty: T,
}

impl<T: fmt::Display> fmt::Display for Def<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let args_str: Vec<String> = self
            .args
            .iter()
            .map(|(x, ty)| format!("{}:{}", x, ty))
            .collect();
        let cont_str: Vec<String> = self
            .cont
            .iter()
            .map(|(x, ty)| format!("{}:{}", x, ty))
            .collect();
        write!(
            f,
            "def {}({};{}) := {}",
            self.name,
            args_str.join(", "),
            cont_str.join(", "),
            self.body
        )
    }
}

// Prog
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
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
