use std::rc::Rc;

use codespan::Span;
use derivative::Derivative;
use printer::{
    theme::ThemeExt,
    tokens::{COLON, COMMA, DOT, EQ, IN},
    DocAllocator, Print,
};

use super::{context::TypingContext, types::Ty, BinOp, Covariable, Name, Variable};
use crate::syntax::substitution::Substitution;

// Clause
//
//

#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct Clause<T> {
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    pub xtor: T,
    pub context: TypingContext,
    pub rhs: Term,
}

impl<T: Print> Print for Clause<T> {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        if self.context.is_empty() {
            self.xtor
                .print(cfg, alloc)
                .append(alloc.space())
                .append("=>")
                .append(alloc.space())
                .append(self.rhs.print(cfg, alloc))
        } else {
            self.xtor
                .print(cfg, alloc)
                .append(self.context.print(cfg, alloc).parens())
                .append(alloc.space())
                .append("=>")
                .append(alloc.space())
                .append(self.rhs.print(cfg, alloc))
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

impl Print for Op {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        self.fst
            .print(cfg, alloc)
            .append(alloc.space())
            .append(self.op.print(cfg, alloc))
            .append(alloc.space())
            .append(self.snd.print(cfg, alloc))
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
    use printer::Print;

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
        assert_eq!(example_prod().print_to_string(Default::default()), "2 * 4")
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
        assert_eq!(example_sum().print_to_string(Default::default()), "2 + 4")
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
        assert_eq!(example_sub().print_to_string(Default::default()), "2 - 4")
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
                    span: Span::default(),
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
        assert_eq!(
            example_parens().print_to_string(Default::default()),
            "(2 * 3) * 4"
        )
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

#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct IfZ {
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    pub ifc: Rc<Term>,
    pub thenc: Rc<Term>,
    pub elsec: Rc<Term>,
}

impl Print for IfZ {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc.keyword("ifz").append(
            self.ifc
                .print(cfg, alloc)
                .append(COMMA)
                .append(self.thenc.print(cfg, alloc))
                .append(COMMA)
                .append(self.elsec.print(cfg, alloc))
                .parens(),
        )
    }
}

impl From<IfZ> for Term {
    fn from(value: IfZ) -> Self {
        Term::IfZ(value)
    }
}

#[cfg(test)]
mod ifz_tests {
    use codespan::Span;
    use printer::Print;

    use crate::parser::fun;
    use std::rc::Rc;

    use super::{IfZ, Lit, Term};

    fn example() -> IfZ {
        IfZ {
            span: Span::default(),
            ifc: Rc::new(Term::Lit(Lit::mk(0))),
            thenc: Rc::new(Term::Lit(Lit::mk(2))),
            elsec: Rc::new(Term::Lit(Lit::mk(4))),
        }
    }

    #[test]
    fn display() {
        assert_eq!(example().print_to_string(Default::default()), "ifz(0,2,4)")
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

#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct Let {
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    pub variable: Variable,
    pub var_ty: Ty,
    pub bound_term: Rc<Term>,
    pub in_term: Rc<Term>,
}

impl Print for Let {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc
            .keyword("let")
            .append(alloc.space())
            .append(self.variable.clone())
            .append(alloc.space())
            .append(COLON)
            .append(alloc.space())
            .append(self.var_ty.print(cfg, alloc))
            .append(alloc.space())
            .append(EQ)
            .append(alloc.space())
            .append(self.bound_term.print(cfg, alloc))
            .append(alloc.space())
            .append(IN)
            .append(alloc.space())
            .append(self.in_term.print(cfg, alloc))
    }
}

impl From<Let> for Term {
    fn from(value: Let) -> Self {
        Term::Let(value)
    }
}

#[cfg(test)]
mod let_tests {
    use codespan::Span;
    use printer::Print;

    use super::{Let, Lit, Term, Ty};
    use crate::parser::fun;
    use std::rc::Rc;

    fn example() -> Let {
        Let {
            span: Span::default(),
            variable: "x".to_string(),
            var_ty: Ty::mk_int(),
            bound_term: Rc::new(Term::Lit(Lit::mk(2))),
            in_term: Rc::new(Term::Lit(Lit::mk(4))),
        }
    }

    #[test]
    fn display() {
        assert_eq!(
            example().print_to_string(Default::default()),
            "let x : Int = 2 in 4"
        )
    }

    #[test]
    fn parse() {
        let parser = fun::TermParser::new();
        assert_eq!(parser.parse("let x : Int = 2 in 4"), Ok(example().into()));
    }
}

// Fun
//
//

#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct Fun {
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    pub name: Name,
    pub args: Substitution,
}

impl Print for Fun {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc
            .text(self.name.clone())
            .append(self.args.print(cfg, alloc).parens())
    }
}

impl From<Fun> for Term {
    fn from(value: Fun) -> Self {
        Term::Fun(value)
    }
}

#[cfg(test)]
mod fun_tests {
    use codespan::Span;
    use printer::Print;

    use crate::{parser::fun, syntax::substitution::SubstitutionBinding};

    use super::{Fun, Lit, Term};

    fn example_simple() -> Fun {
        Fun {
            span: Span::default(),
            name: "foo".to_string(),
            args: vec![],
        }
    }

    #[test]
    fn display_simple() {
        assert_eq!(
            example_simple().print_to_string(Default::default()),
            "foo()"
        )
    }

    #[test]
    fn parse_simple() {
        let parser = fun::TermParser::new();
        assert_eq!(parser.parse("foo()"), Ok(example_simple().into()));
    }

    fn example_extended() -> Fun {
        Fun {
            span: Span::default(),
            name: "foo".to_string(),
            args: vec![
                Term::Lit(Lit::mk(2)).into(),
                SubstitutionBinding::CovarBinding("a".to_string()),
            ],
        }
    }

    #[test]
    fn display_extended() {
        assert_eq!(
            example_extended().print_to_string(Default::default()),
            "foo(2, 'a)"
        )
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

#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct Constructor {
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    pub id: Name,
    pub args: Substitution,
}

impl Print for Constructor {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        if self.args.is_empty() {
            alloc.ctor(&self.id)
        } else {
            alloc
                .ctor(&self.id)
                .append(self.args.print(cfg, alloc).parens())
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
    use codespan::Span;
    use printer::Print;

    use super::{Constructor, Lit, Term};
    use crate::parser::fun;

    fn example_nil() -> Constructor {
        Constructor {
            span: Span::default(),
            id: "Nil".to_owned(),
            args: vec![],
        }
    }

    fn example_tup() -> Constructor {
        Constructor {
            span: Span::default(),
            id: "Tup".to_owned(),
            args: vec![Term::Lit(Lit::mk(2)).into(), Term::Lit(Lit::mk(4)).into()],
        }
    }

    #[test]
    fn display_nil() {
        assert_eq!(example_nil().print_to_string(Default::default()), "Nil")
    }

    #[test]
    fn parse_nil() {
        let parser = fun::TermParser::new();
        assert_eq!(parser.parse("Nil"), Ok(example_nil().into()));
    }

    #[test]
    fn display_tup() {
        assert_eq!(
            example_tup().print_to_string(Default::default()),
            "Tup(2, 4)"
        )
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

#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct Destructor {
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    pub id: Name,
    pub destructee: Rc<Term>,
    pub args: Substitution,
}

impl Print for Destructor {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        if self.args.is_empty() {
            self.destructee
                .print(cfg, alloc)
                .append(DOT)
                .append(alloc.dtor(&self.id))
        } else {
            self.destructee
                .print(cfg, alloc)
                .append(DOT)
                .append(alloc.dtor(&self.id))
                .append(self.args.print(cfg, alloc).parens())
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
    use codespan::Span;
    use printer::Print;

    use super::Destructor;
    use crate::{parser::fun, syntax::terms::Var};
    use std::rc::Rc;

    /// "x.hd"
    fn example_1() -> Destructor {
        Destructor {
            span: Span::default(),
            id: "Hd".to_owned(),
            destructee: Rc::new(Var::mk("x").into()),
            args: vec![],
        }
    }

    /// "x.hd.hd"
    fn example_2() -> Destructor {
        Destructor {
            span: Span::default(),
            id: "Hd".to_owned(),
            destructee: Rc::new(example_1().into()),
            args: vec![],
        }
    }

    #[test]
    fn display_1() {
        assert_eq!(example_1().print_to_string(Default::default()), "x.Hd")
    }

    #[test]
    fn display_2() {
        assert_eq!(example_2().print_to_string(Default::default()), "x.Hd.Hd")
    }

    #[test]
    fn display_3() {
        let dest = Destructor {
            span: Span::default(),
            id: "Fst".to_owned(),
            destructee: Rc::new(Var::mk("x").into()),
            args: vec![Var::mk("y").into(), Var::mk("z").into()],
        };
        let result = dest.print_to_string(Default::default());
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

#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct Case {
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    pub destructee: Rc<Term>,
    pub cases: Vec<Clause<Name>>,
}

impl Print for Case {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        self.destructee
            .print(cfg, alloc)
            .append(DOT)
            .append(alloc.keyword("case"))
            .append(alloc.space())
            .append(self.cases.print(cfg, alloc).braces())
    }
}

impl From<Case> for Term {
    fn from(value: Case) -> Self {
        Term::Case(value)
    }
}

#[cfg(test)]
mod case_tests {
    use codespan::Span;
    use printer::Print;

    use crate::{
        parser::fun,
        syntax::{context::ContextBinding, types::Ty},
    };

    use super::{Case, Clause, Lit, Term, Var};
    use std::rc::Rc;

    fn example_empty() -> Case {
        Case {
            span: Span::default(),
            destructee: Rc::new(Var::mk("x").into()),
            cases: vec![],
        }
    }

    fn example_tup() -> Case {
        Case {
            span: Span::default(),
            destructee: Rc::new(Var::mk("x").into()),
            cases: vec![Clause {
                span: Span::default(),
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
        assert_eq!(
            example_empty().print_to_string(Default::default()),
            "x.case {}"
        )
    }

    #[test]
    fn parse_empty() {
        let parser = fun::TermParser::new();
        assert_eq!(parser.parse("x.case { }"), Ok(example_empty().into()));
    }

    #[test]
    fn display_tup() {
        assert_eq!(
            example_tup().print_to_string(Default::default()),
            "x.case {Tup(x : Int, y : Int) => 2}"
        )
    }

    #[test]
    fn parse_tup() {
        let parser = fun::TermParser::new();
        assert_eq!(
            parser.parse("x.case { Tup(x : Int, y : Int) => 2 }"),
            Ok(example_tup().into())
        );
    }
}

// Cocase
//
//

#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct Cocase {
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    pub cocases: Vec<Clause<Name>>,
}

impl Print for Cocase {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc
            .keyword("cocase")
            .append(alloc.space())
            .append(self.cocases.print(cfg, alloc).braces())
    }
}

impl From<Cocase> for Term {
    fn from(value: Cocase) -> Self {
        Term::Cocase(value)
    }
}

#[cfg(test)]
mod cocase_tests {
    use codespan::Span;
    use printer::Print;

    use crate::parser::fun;

    use super::{Clause, Cocase, Lit, Term};

    fn example_empty() -> Cocase {
        Cocase {
            span: Span::default(),
            cocases: vec![],
        }
    }

    fn example_stream() -> Cocase {
        Cocase {
            span: Span::default(),
            cocases: vec![
                Clause {
                    span: Span::default(),
                    xtor: "Hd".to_owned(),
                    context: vec![],
                    rhs: Term::Lit(Lit::mk(2)),
                },
                Clause {
                    span: Span::default(),
                    xtor: "Tl".to_owned(),
                    context: vec![],
                    rhs: Term::Lit(Lit::mk(4)),
                },
            ],
        }
    }

    #[test]
    fn display_empty() {
        assert_eq!(
            example_empty().print_to_string(Default::default()),
            "cocase {}"
        )
    }

    #[test]
    fn parse_empty() {
        let parser = fun::TermParser::new();
        assert_eq!(parser.parse("cocase { }"), Ok(example_empty().into()));
    }

    #[test]
    fn display_stream() {
        assert_eq!(
            example_stream().print_to_string(Default::default()),
            "cocase {Hd => 2, Tl => 4}"
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

#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct Goto {
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    pub term: Rc<Term>,
    pub target: Covariable,
}

impl Print for Goto {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc.keyword("goto").append(
            self.term
                .print(cfg, alloc)
                .append(";")
                .append(
                    alloc
                        .space()
                        .append("'")
                        .append(self.target.print(cfg, alloc)),
                )
                .parens(),
        )
    }
}

impl From<Goto> for Term {
    fn from(value: Goto) -> Self {
        Term::Goto(value)
    }
}

#[cfg(test)]
mod goto_tests {
    use codespan::Span;
    use printer::Print;

    use super::{Goto, Lit, Term};
    use crate::parser::fun;
    use std::rc::Rc;

    fn example() -> Goto {
        Goto {
            span: Span::default(),
            term: Rc::new(Term::Lit(Lit::mk(2))),
            target: "x".to_string(),
        }
    }

    #[test]
    fn display() {
        assert_eq!(example().print_to_string(Default::default()), "goto(2; 'x)")
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

#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct Label {
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    pub label: Covariable,
    pub term: Rc<Term>,
}

impl Print for Label {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc
            .keyword("label")
            .append(alloc.space())
            .append("'")
            .append(self.label.clone())
            .append(alloc.space())
            .append(self.term.print(cfg, alloc).braces())
    }
}
impl From<Label> for Term {
    fn from(value: Label) -> Self {
        Term::Label(value)
    }
}

#[cfg(test)]
mod label_tests {
    use codespan::Span;
    use printer::Print;

    use super::{Label, Lit, Term};
    use crate::parser::fun;
    use std::rc::Rc;

    fn example() -> Label {
        Label {
            span: Span::default(),
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
        assert_eq!(
            example().print_to_string(Default::default()),
            "label 'x {2}"
        )
    }
}

// Paren
//
//

#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct Paren {
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    pub inner: Rc<Term>,
}

impl Print for Paren {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        self.inner.print(cfg, alloc).parens()
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

impl Print for Lit {
    fn print<'a>(
        &'a self,
        _cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc.text(format!("{}", self.val))
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

impl Print for Var {
    fn print<'a>(
        &'a self,
        _cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc.text(self.var.clone())
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

impl Print for Term {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        match self {
            Term::Var(var) => var.print(cfg, alloc),
            Term::Lit(lit) => lit.print(cfg, alloc),
            Term::Op(op) => op.print(cfg, alloc),
            Term::IfZ(ifz) => ifz.print(cfg, alloc),
            Term::Let(lete) => lete.print(cfg, alloc),
            Term::Fun(fun) => fun.print(cfg, alloc),
            Term::Constructor(constructor) => constructor.print(cfg, alloc),
            Term::Destructor(destructor) => destructor.print(cfg, alloc),
            Term::Case(case) => case.print(cfg, alloc),
            Term::Cocase(cocase) => cocase.print(cfg, alloc),
            Term::Goto(goto) => goto.print(cfg, alloc),
            Term::Label(label) => label.print(cfg, alloc),
            Term::Paren(paren) => paren.print(cfg, alloc),
        }
    }
}
