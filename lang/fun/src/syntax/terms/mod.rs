use std::{fmt, rc::Rc};

use codespan::Span;
use derivative::Derivative;

use super::{context::TypingContext, BinOp, Covariable, Name, Variable};
use crate::syntax::{stringify_and_join, substitution::Substitution};

// Clause
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Clause<T> {
    pub xtor: T,
    pub context: TypingContext,
    pub rhs: Term,
}

impl<T: fmt::Display> fmt::Display for Clause<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.context.is_empty() {
            write!(f, "{} => {}", self.xtor, self.rhs)
        } else {
            let context_strs: Vec<String> =
                self.context.iter().map(|bnd| format!("{bnd}")).collect();
            write!(
                f,
                "{}({}) => {}",
                self.xtor,
                context_strs.join(", "),
                self.rhs
            )
        }
    }
}

// Op
//
//

#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct Op {
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
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

    use codespan::Span;

    use crate::parser::fun;

    use super::{BinOp, Lit, Op, Paren, Term};

    fn example_prod() -> Op {
        Op {
            span: Span::default(),
            fst: Rc::new(Term::Lit(Lit::mk(2))),
            op: BinOp::Prod,
            snd: Rc::new(Term::Lit(Lit::mk(4))),
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
            span: Span::default(),
            fst: Rc::new(Term::Lit(Lit::mk(2))),
            op: BinOp::Sum,
            snd: Rc::new(Term::Lit(Lit::mk(4))),
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
            span: Span::default(),
            fst: Rc::new(Term::Lit(Lit::mk(2))),
            op: BinOp::Sub,
            snd: Rc::new(Term::Lit(Lit::mk(4))),
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
            span: Span::default(),
            fst: Rc::new(
                Paren {
                    inner: Rc::new(
                        Op {
                            span: Span::default(),
                            fst: Rc::new(Term::Lit(Lit::mk(2))),
                            op: BinOp::Prod,
                            snd: Rc::new(Term::Lit(Lit::mk(3))),
                        }
                        .into(),
                    ),
                }
                .into(),
            ),
            op: BinOp::Prod,
            snd: Rc::new(Term::Lit(Lit::mk(4))),
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

    use super::{IfZ, Lit, Term};

    fn example() -> IfZ {
        IfZ {
            ifc: Rc::new(Term::Lit(Lit::mk(0))),
            thenc: Rc::new(Term::Lit(Lit::mk(2))),
            elsec: Rc::new(Term::Lit(Lit::mk(4))),
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
    use super::{Let, Lit, Term};
    use crate::parser::fun;
    use std::rc::Rc;

    fn example() -> Let {
        Let {
            variable: "x".to_string(),
            bound_term: Rc::new(Term::Lit(Lit::mk(2))),
            in_term: Rc::new(Term::Lit(Lit::mk(4))),
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
    pub args: Substitution,
}

impl fmt::Display for Fun {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let args_joined: String = stringify_and_join(&self.args);
        write!(f, "{}({})", self.name, args_joined,)
    }
}

impl From<Fun> for Term {
    fn from(value: Fun) -> Self {
        Term::Fun(value)
    }
}

#[cfg(test)]
mod fun_tests {
    use crate::{parser::fun, syntax::substitution::SubstitutionBinding};

    use super::{Fun, Lit, Term};

    fn example_simple() -> Fun {
        Fun {
            name: "foo".to_string(),
            args: vec![],
        }
    }

    #[test]
    fn display_simple() {
        assert_eq!(format!("{}", example_simple()), "foo()")
    }

    #[test]
    fn parse_simple() {
        let parser = fun::TermParser::new();
        assert_eq!(parser.parse("foo()"), Ok(example_simple().into()));
    }

    fn example_extended() -> Fun {
        Fun {
            name: "foo".to_string(),
            args: vec![
                Term::Lit(Lit::mk(2)).into(),
                SubstitutionBinding::CovarBinding("a".to_string()),
            ],
        }
    }

    #[test]
    fn display_extended() {
        assert_eq!(format!("{}", example_extended()), "foo(2, 'a)")
    }

    #[test]
    fn parse_extended() {
        let parser = fun::TermParser::new();
        assert_eq!(parser.parse("foo(2, 'a)"), Ok(example_extended().into()));
    }
}

// Constructor
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Constructor {
    pub id: Name,
    pub args: Substitution,
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
    use super::{Constructor, Lit, Term};
    use crate::parser::fun;

    fn example_nil() -> Constructor {
        Constructor {
            id: "Nil".to_owned(),
            args: vec![],
        }
    }

    fn example_tup() -> Constructor {
        Constructor {
            id: "Tup".to_owned(),
            args: vec![Term::Lit(Lit::mk(2)).into(), Term::Lit(Lit::mk(4)).into()],
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
    pub id: Name,
    pub destructee: Rc<Term>,
    pub args: Substitution,
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
    use super::Destructor;
    use crate::{parser::fun, syntax::terms::Var};
    use std::rc::Rc;

    /// "x.hd"
    fn example_1() -> Destructor {
        Destructor {
            id: "Hd".to_owned(),
            destructee: Rc::new(Var::mk("x").into()),
            args: vec![],
        }
    }

    /// "x.hd.hd"
    fn example_2() -> Destructor {
        Destructor {
            id: "Hd".to_owned(),
            destructee: Rc::new(example_1().into()),
            args: vec![],
        }
    }

    #[test]
    fn display_1() {
        assert_eq!(format!("{}", example_1()), "x.Hd")
    }

    #[test]
    fn display_2() {
        assert_eq!(format!("{}", example_2()), "x.Hd.Hd")
    }

    #[test]
    fn display_3() {
        let dest = Destructor {
            id: "Fst".to_owned(),
            destructee: Rc::new(Var::mk("x").into()),
            args: vec![Var::mk("y").into(), Var::mk("z").into()],
        };
        let result = format!("{}", dest);
        let expected = "x.Fst(y, z)".to_owned();
        assert_eq!(result, expected)
    }

    #[test]
    fn parse_1() {
        let parser = fun::TermParser::new();
        assert_eq!(parser.parse("x.Hd"), Ok(example_1().into()));
    }

    #[test]
    fn parse_2() {
        let parser = fun::TermParser::new();
        assert_eq!(parser.parse("x.Hd.Hd"), Ok(example_2().into()));
    }
}

// Case
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Case {
    pub destructee: Rc<Term>,
    pub cases: Vec<Clause<Name>>,
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
    use crate::{
        parser::fun,
        syntax::{context::ContextBinding, types::Ty},
    };

    use super::{Case, Clause, Lit, Term, Var};
    use std::rc::Rc;

    fn example_empty() -> Case {
        Case {
            destructee: Rc::new(Var::mk("x").into()),
            cases: vec![],
        }
    }

    fn example_tup() -> Case {
        Case {
            destructee: Rc::new(Var::mk("x").into()),
            cases: vec![Clause {
                xtor: "Tup".to_owned(),
                context: vec![
                    ContextBinding::TypedVar {
                        var: "x".to_string(),
                        ty: Ty::mk_int(),
                    },
                    ContextBinding::TypedVar {
                        var: "y".to_string(),
                        ty: Ty::mk_int(),
                    },
                ],
                rhs: Term::Lit(Lit::mk(2)),
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
        assert_eq!(
            format!("{}", example_tup()),
            "case x of { Tup(x : Int, y : Int) => 2 }"
        )
    }

    #[test]
    fn parse_tup() {
        let parser = fun::TermParser::new();
        assert_eq!(
            parser.parse("case x of { Tup(x : Int, y : Int) => 2 }"),
            Ok(example_tup().into())
        );
    }
}

// Cocase
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cocase {
    pub cocases: Vec<Clause<Name>>,
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

    use super::{Clause, Cocase, Lit, Term};

    fn example_empty() -> Cocase {
        Cocase { cocases: vec![] }
    }

    fn example_stream() -> Cocase {
        Cocase {
            cocases: vec![
                Clause {
                    xtor: "Hd".to_owned(),
                    context: vec![],
                    rhs: Term::Lit(Lit::mk(2)),
                },
                Clause {
                    xtor: "Tl".to_owned(),
                    context: vec![],
                    rhs: Term::Lit(Lit::mk(4)),
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
            "cocase { Hd => 2, Tl => 4 }"
        )
    }

    #[test]
    fn parse_stream() {
        let parser = fun::TermParser::new();
        assert_eq!(
            parser.parse("cocase { Hd => 2, Tl => 4 }"),
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
        write!(f, "goto({}; '{})", self.term, self.target)
    }
}

impl From<Goto> for Term {
    fn from(value: Goto) -> Self {
        Term::Goto(value)
    }
}

#[cfg(test)]
mod goto_tests {
    use super::{Goto, Lit, Term};
    use crate::parser::fun;
    use std::rc::Rc;

    fn example() -> Goto {
        Goto {
            term: Rc::new(Term::Lit(Lit::mk(2))),
            target: "x".to_string(),
        }
    }

    #[test]
    fn display() {
        assert_eq!(format!("{}", example()), "goto(2; 'x)")
    }

    #[test]
    fn parse() {
        let parser = fun::TermParser::new();
        assert_eq!(parser.parse("goto(2;'x)"), Ok(example().into()));
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
        write!(f, "label '{} {{ {} }}", self.label, self.term)
    }
}

impl From<Label> for Term {
    fn from(value: Label) -> Self {
        Term::Label(value)
    }
}

#[cfg(test)]
mod label_tests {
    use super::{Label, Lit, Term};
    use crate::parser::fun;
    use std::rc::Rc;

    fn example() -> Label {
        Label {
            label: "x".to_string(),
            term: Rc::new(Term::Lit(Lit::mk(2))),
        }
    }

    #[test]
    fn parse() {
        let parser = fun::TermParser::new();
        assert_eq!(parser.parse("label 'x { 2 }"), Ok(example().into()));
    }

    #[test]
    fn display() {
        assert_eq!(format!("{}", example()), "label 'x { 2 }")
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

// Lit
//

#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct Lit {
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    pub val: i64,
}

impl Lit {
    pub fn mk(val: i64) -> Self {
        Lit {
            span: Span::default(),
            val,
        }
    }
}

impl fmt::Display for Lit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.val)
    }
}

impl From<Lit> for Term {
    fn from(value: Lit) -> Self {
        Term::Lit(value)
    }
}

// Var
//
/// Covariables (used in label, goto and toplevel calls) start with ' but this is not saved in the name string
/// that is, in source code 'a is a valid covariable, but in the AST the name is saved as a

#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct Var {
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    pub var: Variable,
}

impl Var {
    pub fn mk(var: &str) -> Self {
        Var {
            span: Span::default(),
            var: var.to_string(),
        }
    }
}

impl fmt::Display for Var {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.var)
    }
}

impl From<Var> for Term {
    fn from(value: Var) -> Self {
        Term::Var(value)
    }
}

// Term
//
/// Covariables (used in label, goto and toplevel calls) start with ' but this is not saved in the name string
/// that is, in source code 'a is a valid covariable, but in the AST the name is saved as a

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Term {
    Var(Var),
    Lit(Lit),
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
            Term::Var(v) => v.fmt(f),
            Term::Lit(l) => l.fmt(f),
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
