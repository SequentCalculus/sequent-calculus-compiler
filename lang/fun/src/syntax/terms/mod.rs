use std::rc::Rc;

use codespan::Span;
use derivative::Derivative;
use printer::{
    theme::ThemeExt,
    tokens::{CASE, COCASE, COLON, DOT, EQ, FAT_ARROW, IN, LET},
    DocAllocator, Print,
};

mod constructor;
mod destructor;
mod goto;
mod ifz;
mod label;
mod lit;
mod op;
mod paren;
pub use constructor::*;
pub use destructor::*;
pub use goto::*;
pub use ifz::*;
pub use label::*;
pub use lit::*;
pub use op::*;
pub use paren::*;

use super::{context::TypingContext, print_cases, types::Ty, Name, Variable};
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
                .append(FAT_ARROW)
                .append(alloc.space())
                .append(self.rhs.print(cfg, alloc))
        } else {
            self.xtor
                .print(cfg, alloc)
                .append(self.context.print(cfg, alloc).parens())
                .append(alloc.space())
                .append(FAT_ARROW)
                .append(alloc.space())
                .append(self.rhs.print(cfg, alloc))
        }
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
            .keyword(LET)
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
            .append(alloc.line())
            .append(alloc.keyword(IN))
            .append(alloc.space())
            .append(self.in_term.print(cfg, alloc))
            .align()
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
            "let x : Int = 2\nin 4"
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
            .append(alloc.keyword(CASE))
            .append(alloc.space())
            .append(print_cases(&self.cases, cfg, alloc))
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
            "x.case { }"
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
            "x.case { Tup(x : Int, y : Int) => 2 }"
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
            .keyword(COCASE)
            .append(alloc.space())
            .append(print_cases(&self.cocases, cfg, alloc))
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
            "cocase { }"
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
            "cocase {\n    Hd => 2,\n    Tl => 4\n}"
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
