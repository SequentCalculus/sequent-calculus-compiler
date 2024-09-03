use std::fmt;
use std::rc::Rc;

pub type Variable = String;
pub type Covariable = String;
pub type Name = String;

fn stringify_and_join<T: fmt::Display>(vec: &[T]) -> String {
    vec.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(", ")
}

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
    Ap,
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
    pub vars: Vec<Variable>,
    pub rhs: Term,
}

impl<T: fmt::Display> fmt::Display for Clause<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.vars.is_empty() {
            write!(f, "{} => {}", self.xtor, self.rhs)
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

#[cfg(test)]
mod op_tests {
    use std::rc::Rc;

    use crate::parser::fun;

    use super::{BinOp, Op, Paren, Term};

    fn example_prod() -> Op {
        Op {
            fst: Rc::new(Term::Lit(2)),
            op: BinOp::Prod,
            snd: Rc::new(Term::Lit(4)),
        }
    }

    #[test]
    fn display_prod() {
        assert_eq!(format!("{}", example_prod()), "2 * 4")
    }

    #[test]
    fn parse_prod() {
        let parser = fun::TermParser::new();
        assert_eq!(parser.parse("2 * 4"), Ok(example_prod().into()));
    }

    fn example_sum() -> Op {
        Op {
            fst: Rc::new(Term::Lit(2)),
            op: BinOp::Sum,
            snd: Rc::new(Term::Lit(4)),
        }
    }

    #[test]
    fn display_sum() {
        assert_eq!(format!("{}", example_sum()), "2 + 4")
    }

    #[test]
    fn parse_sum() {
        let parser = fun::TermParser::new();
        assert_eq!(parser.parse("2 + 4"), Ok(example_sum().into()));
    }

    fn example_sub() -> Op {
        Op {
            fst: Rc::new(Term::Lit(2)),
            op: BinOp::Sub,
            snd: Rc::new(Term::Lit(4)),
        }
    }

    #[test]
    fn display_sub() {
        assert_eq!(format!("{}", example_sub()), "2 - 4")
    }

    #[test]
    fn parse_sub() {
        let parser = fun::TermParser::new();
        assert_eq!(parser.parse("2 - 4"), Ok(example_sub().into()));
    }

    /// (2 * 3) * 4
    fn example_parens() -> Op {
        Op {
            fst: Rc::new(
                Paren {
                    inner: Rc::new(
                        Op {
                            fst: Rc::new(Term::Lit(2)),
                            op: BinOp::Prod,
                            snd: Rc::new(Term::Lit(3)),
                        }
                        .into(),
                    ),
                }
                .into(),
            ),
            op: BinOp::Prod,
            snd: Rc::new(Term::Lit(4)),
        }
    }

    #[test]
    fn display_parens() {
        assert_eq!(format!("{}", example_parens()), "(2 * 3) * 4")
    }

    #[test]
    fn parse_parens() {
        let parser = fun::TermParser::new();
        assert_eq!(parser.parse("(2 * 3) * 4"), Ok(example_parens().into()));
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
        write!(f, "ifz({}, {}, {})", self.ifc, self.thenc, self.elsec)
    }
}

impl From<IfZ> for Term {
    fn from(value: IfZ) -> Self {
        Term::IfZ(value)
    }
}

#[cfg(test)]
mod ifz_tests {
    use crate::parser::fun;
    use std::rc::Rc;

    use super::{IfZ, Term};

    fn example() -> IfZ {
        IfZ {
            ifc: Rc::new(Term::Lit(0)),
            thenc: Rc::new(Term::Lit(2)),
            elsec: Rc::new(Term::Lit(4)),
        }
    }

    #[test]
    fn display() {
        assert_eq!(format!("{}", example()), "ifz(0, 2, 4)")
    }

    #[test]
    fn parse() {
        let parser = fun::TermParser::new();
        assert_eq!(parser.parse("ifz(0,2,4)"), Ok(example().into()));
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
            "let {} = {} in {}",
            self.variable, self.bound_term, self.in_term
        )
    }
}

impl From<Let> for Term {
    fn from(value: Let) -> Self {
        Term::Let(value)
    }
}

#[cfg(test)]
mod let_tests {
    use super::{Let, Term};
    use crate::parser::fun;
    use std::rc::Rc;

    fn example() -> Let {
        Let {
            variable: "x".to_string(),
            bound_term: Rc::new(Term::Lit(2)),
            in_term: Rc::new(Term::Lit(4)),
        }
    }

    #[test]
    fn display() {
        assert_eq!(format!("{}", example()), "let x = 2 in 4")
    }

    #[test]
    fn parse() {
        let parser = fun::TermParser::new();
        assert_eq!(parser.parse("let x = 2 in 4"), Ok(example().into()));
    }
}

// Fun
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fun {
    pub name: Name,
    pub args: Vec<Term>,
    pub coargs: Vec<Covariable>,
}

impl fmt::Display for Fun {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let args_joined: String = stringify_and_join(&self.args);
        write!(
            f,
            "{}({}; {})",
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

#[cfg(test)]
mod fun_tests {
    use crate::parser::fun;

    use super::{Fun, Term};

    fn example_simple() -> Fun {
        Fun {
            name: "foo".to_string(),
            args: vec![],
            coargs: vec![],
        }
    }

    #[test]
    fn display_simple() {
        assert_eq!(format!("{}", example_simple()), "foo(; )")
    }

    #[test]
    fn parse_simple() {
        let parser = fun::TermParser::new();
        assert_eq!(parser.parse("foo(; )"), Ok(example_simple().into()));
    }

    fn example_extended() -> Fun {
        Fun {
            name: "foo".to_string(),
            args: vec![Term::Lit(2)],
            coargs: vec!["a".to_string()],
        }
    }

    #[test]
    fn display_extended() {
        assert_eq!(format!("{}", example_extended()), "foo(2; a)")
    }

    #[test]
    fn parse_extended() {
        let parser = fun::TermParser::new();
        assert_eq!(parser.parse("foo(2;a)"), Ok(example_extended().into()));
    }
}

// Constructor
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Constructor {
    pub id: Ctor,
    pub args: Vec<Term>,
}

impl fmt::Display for Constructor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.args.is_empty() {
            write!(f, "{}", self.id)
        } else {
            let args_joined: String = stringify_and_join(&self.args);
            write!(f, "{}({})", self.id, args_joined)
        }
    }
}

impl From<Constructor> for Term {
    fn from(value: Constructor) -> Self {
        Term::Constructor(value)
    }
}

#[cfg(test)]
mod constructor_tests {
    use super::{Constructor, Ctor, Term};
    use crate::parser::fun;

    fn example_nil() -> Constructor {
        Constructor {
            id: Ctor::Nil,
            args: vec![],
        }
    }

    fn example_tup() -> Constructor {
        Constructor {
            id: Ctor::Tup,
            args: vec![Term::Lit(2), Term::Lit(4)],
        }
    }

    #[test]
    fn display_nil() {
        assert_eq!(format!("{}", example_nil()), "Nil")
    }

    #[test]
    fn parse_nil() {
        let parser = fun::TermParser::new();
        assert_eq!(parser.parse("Nil"), Ok(example_nil().into()));
    }

    #[test]
    fn display_tup() {
        assert_eq!(format!("{}", example_tup()), "Tup(2, 4)")
    }

    #[test]
    fn parse_tup() {
        let parser = fun::TermParser::new();
        assert_eq!(parser.parse("Tup(2,4)"), Ok(example_tup().into()));
    }
}

// Destructor
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Destructor {
    pub id: Dtor,
    pub destructee: Rc<Term>,
    pub args: Vec<Term>,
}

impl fmt::Display for Destructor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.args.is_empty() {
            write!(f, "{}.{}", self.destructee, self.id)
        } else {
            let args_joined: String = stringify_and_join(&self.args);
            write!(f, "{}.{}({})", self.destructee, self.id, args_joined)
        }
    }
}

impl From<Destructor> for Term {
    fn from(value: Destructor) -> Self {
        Term::Destructor(value)
    }
}

#[cfg(test)]
mod destructor_tests {
    use super::{Destructor, Dtor, Term};
    use crate::parser::fun;
    use std::rc::Rc;

    /// "x.hd"
    fn example_1() -> Destructor {
        Destructor {
            id: Dtor::Hd,
            destructee: Rc::new(Term::Var("x".to_string())),
            args: vec![],
        }
    }

    /// "x.hd.hd"
    fn example_2() -> Destructor {
        Destructor {
            id: Dtor::Hd,
            destructee: Rc::new(example_1().into()),
            args: vec![],
        }
    }

    #[test]
    fn display_1() {
        assert_eq!(format!("{}", example_1()), "x.hd")
    }

    #[test]
    fn display_2() {
        assert_eq!(format!("{}", example_2()), "x.hd.hd")
    }

    #[test]
    fn display_3() {
        let dest = Destructor {
            id: Dtor::Fst,
            destructee: Rc::new(Term::Var("x".to_owned())),
            args: vec![Term::Var("y".to_owned()), Term::Var("z".to_owned())],
        };
        let result = format!("{}", dest);
        let expected = "x.fst(y, z)".to_owned();
        assert_eq!(result, expected)
    }

    #[test]
    fn parse_1() {
        let parser = fun::TermParser::new();
        assert_eq!(parser.parse("x.hd"), Ok(example_1().into()));
    }

    #[test]
    fn parse_2() {
        let parser = fun::TermParser::new();
        assert_eq!(parser.parse("x.hd.hd"), Ok(example_2().into()));
    }
}

// Case
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Case {
    pub destructee: Rc<Term>,
    pub cases: Vec<Clause<Ctor>>,
}

impl fmt::Display for Case {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let clauses_joined: String = stringify_and_join(&self.cases);
        write!(f, "case {} of {{ {} }}", self.destructee, clauses_joined)
    }
}

impl From<Case> for Term {
    fn from(value: Case) -> Self {
        Term::Case(value)
    }
}

#[cfg(test)]
mod case_tests {
    use crate::parser::fun;

    use super::{Case, Clause, Ctor, Term};
    use std::rc::Rc;

    fn example_empty() -> Case {
        Case {
            destructee: Rc::new(Term::Var("x".to_string())),
            cases: vec![],
        }
    }

    fn example_tup() -> Case {
        Case {
            destructee: Rc::new(Term::Var("x".to_string())),
            cases: vec![Clause {
                xtor: Ctor::Tup,
                vars: vec!["x".to_string(), "y".to_string()],
                rhs: Term::Lit(2),
            }],
        }
    }

    #[test]
    fn display_empty() {
        assert_eq!(format!("{}", example_empty()), "case x of {  }")
    }

    #[test]
    fn parse_empty() {
        let parser = fun::TermParser::new();
        assert_eq!(parser.parse("case x of { }"), Ok(example_empty().into()));
    }

    #[test]
    fn display_tup() {
        assert_eq!(format!("{}", example_tup()), "case x of { Tup(x, y) => 2 }")
    }

    #[test]
    fn parse_tup() {
        let parser = fun::TermParser::new();
        assert_eq!(
            parser.parse("case x of { Tup(x, y) => 2 }"),
            Ok(example_tup().into())
        );
    }
}

// Cocase
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cocase {
    pub cocases: Vec<Clause<Dtor>>,
}

impl fmt::Display for Cocase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let clauses_joined: String = stringify_and_join(&self.cocases);
        write!(f, "cocase {{ {} }}", clauses_joined)
    }
}

impl From<Cocase> for Term {
    fn from(value: Cocase) -> Self {
        Term::Cocase(value)
    }
}

#[cfg(test)]
mod cocase_tests {
    use crate::parser::fun;

    use super::{Clause, Cocase, Dtor, Term};

    fn example_empty() -> Cocase {
        Cocase { cocases: vec![] }
    }

    fn example_stream() -> Cocase {
        Cocase {
            cocases: vec![
                Clause {
                    xtor: Dtor::Hd,
                    vars: vec![],
                    rhs: Term::Lit(2),
                },
                Clause {
                    xtor: Dtor::Tl,
                    vars: vec![],
                    rhs: Term::Lit(4),
                },
            ],
        }
    }

    #[test]
    fn display_empty() {
        assert_eq!(format!("{}", example_empty()), "cocase {  }")
    }

    #[test]
    fn parse_empty() {
        let parser = fun::TermParser::new();
        assert_eq!(parser.parse("cocase { }"), Ok(example_empty().into()));
    }

    #[test]
    fn display_stream() {
        assert_eq!(
            format!("{}", example_stream()),
            "cocase { hd => 2, tl => 4 }"
        )
    }

    #[test]
    fn parse_stream() {
        let parser = fun::TermParser::new();
        assert_eq!(
            parser.parse("cocase { hd=>2, tl=>4 }"),
            Ok(example_stream().into())
        );
    }
}

// Goto
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Goto {
    pub term: Rc<Term>,
    pub target: Covariable,
}

impl fmt::Display for Goto {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "goto({}; {})", self.term, self.target)
    }
}

impl From<Goto> for Term {
    fn from(value: Goto) -> Self {
        Term::Goto(value)
    }
}

#[cfg(test)]
mod goto_tests {
    use super::{Goto, Term};
    use crate::parser::fun;
    use std::rc::Rc;

    fn example() -> Goto {
        Goto {
            term: Rc::new(Term::Lit(2)),
            target: "x".to_string(),
        }
    }

    #[test]
    fn display() {
        assert_eq!(format!("{}", example()), "goto(2; x)")
    }

    #[test]
    fn parse() {
        let parser = fun::TermParser::new();
        assert_eq!(parser.parse("goto(2;x)"), Ok(example().into()));
    }
}

// Label
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Label {
    pub label: Covariable,
    pub term: Rc<Term>,
}

impl fmt::Display for Label {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "label {} {{ {} }}", self.label, self.term)
    }
}

impl From<Label> for Term {
    fn from(value: Label) -> Self {
        Term::Label(value)
    }
}

#[cfg(test)]
mod label_tests {
    use super::{Label, Term};
    use crate::parser::fun;
    use std::rc::Rc;

    fn example() -> Label {
        Label {
            label: "x".to_string(),
            term: Rc::new(Term::Lit(2)),
        }
    }

    #[test]
    fn parse() {
        let parser = fun::TermParser::new();
        assert_eq!(parser.parse("label x { 2 }"), Ok(example().into()));
    }

    #[test]
    fn display() {
        assert_eq!(format!("{}", example()), "label x { 2 }")
    }
}

// Paren
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Paren {
    pub inner: Rc<Term>,
}

impl fmt::Display for Paren {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({})", self.inner)
    }
}

impl From<Paren> for Term {
    fn from(value: Paren) -> Self {
        Term::Paren(value)
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
    Destructor(Destructor),
    Case(Case),
    Cocase(Cocase),
    Goto(Goto),
    Label(Label),
    Paren(Paren),
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
            Term::Destructor(d) => d.fmt(f),
            Term::Case(c) => c.fmt(f),
            Term::Cocase(c) => c.fmt(f),
            Term::Goto(g) => g.fmt(f),
            Term::Label(l) => l.fmt(f),
            Term::Paren(p) => p.fmt(f),
        }
    }
}
