use std::fmt;
use std::rc::Rc;

pub type Variable = String;
pub type Covariable = String;
pub type Name = String;

#[derive(Clone)]
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
pub struct Clause<T> {
    pub pt_xtor: T,
    pub pt_vars: Vec<Variable>,
    pub pt_t: Term,
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
pub enum Prog<T> {
    Prog(Vec<Def<T>>),
}

pub fn show_vec<T: fmt::Display>(itms: &Vec<T>) -> String {
    let elems_strs: Vec<String> = itms
        .iter()
        .map(|x| format!("{}", x))
        .collect::<Vec<String>>();
    let elems_joined: &String = &elems_strs.join(", ");
    elems_joined.to_string()
}

fn show_tup<T: fmt::Display>((st, t): &(String, T)) -> String {
    format!("{}::{}", st, t)
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
        if self.pt_vars.len() == 0 {
            write!(f, "{}=>{}", self.pt_xtor, self.pt_t)
        } else {
            write!(
                f,
                "{}({}) => {}",
                self.pt_xtor,
                show_vec(&self.pt_vars),
                self.pt_t
            )
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
                write!(f, "{}({};{})", nm, show_vec(args), coargs.join(", "))
            }
            Term::Constructor(ctor, args) if args.len() == 0 => write!(f, "{}", ctor),
            Term::Constructor(ctor, args) => write!(f, "{}({})", ctor, show_vec(args)),
            Term::Destructor(t, dtor, args) if args.len() == 0 => write!(f, "{}.{}", t, dtor),
            Term::Destructor(t, dtor, args) => write!(f, "{}.{}({})", t, dtor, show_vec(args)),
            Term::Case(t, clauses) => write!(f, "case {} of {{ {} }}", t, show_vec(clauses)),
            Term::Cocase(clauses) => write!(f, "cocase {{ {} }}", show_vec(clauses)),
            Term::Lam(v, t) => write!(f, "\\{}.{}", v, t),
            Term::App(t1, t2) => write!(f, "{} {}", t1, t2),
            Term::Goto(t, cv) => write!(f, "goto({};{})", t, cv),
            Term::Label(cv, t) => write!(f, "label {} {{{}}}", cv, t),
        }
    }
}

impl<T: fmt::Display> fmt::Display for Def<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let args_str: Vec<String> = self.args.iter().map(|x| show_tup(x)).collect();
        let cont_str: Vec<String> = self.cont.iter().map(|x| show_tup(x)).collect();
        write!(
            f,
            "def {}({};{}) := {}",
            self.name,
            show_vec(&args_str),
            show_vec(&cont_str),
            self.body
        )
    }
}

impl<T: fmt::Display> fmt::Display for Prog<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Prog::Prog(defs) => write!(f, "{}", show_vec(defs)),
        }
    }
}
