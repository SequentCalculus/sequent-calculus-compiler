use std::fmt;
use std::rc::Rc;

pub type Variable = String;
pub type Covariable = String;
pub type Name = String;

#[derive(Clone, PartialEq)]
pub enum BinOp {
    Prod,
    Sum,
    Sub,
}
#[derive(Clone, PartialEq)]
pub enum Ctor {
    Nil,
    Cons,
    Tup,
}
#[derive(Clone, PartialEq)]
pub enum Dtor {
    Hd,
    Tl,
    Fst,
    Snd,
    Ap,
}

#[derive(Clone)]
pub struct Clause<T> {
    pub xtor: T,
    pub vars: Vec<Variable>,
    pub rhs: Term,
}

#[derive(Clone)]
pub enum Term {
    Var(Variable),
    Lit(i64),
    Op(Rc<Term>, BinOp, Rc<Term>),
    IfZ(Rc<Term>, Rc<Term>, Rc<Term>),
    Let(Variable, Rc<Term>, Rc<Term>),
    Fun(Name, Vec<Rc<Term>>, Vec<Covariable>),
    Constructor(Ctor, Vec<Rc<Term>>),
    Destructor(Rc<Term>, Dtor, Vec<Rc<Term>>),
    Case(Rc<Term>, Vec<Rc<Clause<Ctor>>>),
    Cocase(Vec<Rc<Clause<Dtor>>>),
    Lam(Variable, Rc<Term>),
    App(Rc<Term>, Rc<Term>),
    Goto(Rc<Term>, Covariable),
    Label(Covariable, Rc<Term>),
}

#[derive(Clone)]
pub struct Def<T> {
    pub name: Name,
    pub args: Vec<(Variable, T)>,
    pub cont: Vec<(Covariable, T)>,
    pub body: Term,
    pub ret_ty: T,
}

#[derive(Clone)]
pub struct Prog<T> {
    pub prog_defs: Vec<Def<T>>,
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
            Dtor::Hd => write!(f, "Hd"),
            Dtor::Tl => write!(f, "Tl"),
            Dtor::Fst => write!(f, "Fst"),
            Dtor::Snd => write!(f, "Snd"),
            Dtor::Ap => write!(f, "Ap"),
        }
    }
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

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Term::Var(v) => write!(f, "{}", v),
            Term::Lit(n) => write!(f, "{}", n),
            Term::Op(t1, op, t2) => write!(f, "{} {} {}", t1, op, t2),
            Term::IfZ(t1, t2, t3) => write!(f, "IfZ {} then {} else {})", t1, t2, t3),
            Term::Let(v, t1, t2) => write!(f, "Let {}={} in {}", v, t1, t2),
            Term::Fun(nm, args, coargs) => {
                let args_joined: String = args
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                write!(f, "{}({};{})", nm, args_joined, coargs.join(", "))
            }
            Term::Constructor(ctor, args) if args.is_empty() => write!(f, "{}", ctor),
            Term::Constructor(ctor, args) => {
                let args_joined: String = args
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                write!(f, "{}({})", ctor, args_joined)
            }
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
