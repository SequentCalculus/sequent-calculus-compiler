use std::rc::Rc;

use codespan::Span;
use derivative::Derivative;
use printer::{
    theme::ThemeExt,
    tokens::{GOTO, SEMI, TICK},
    DocAllocator, Print,
};

use crate::syntax::Covariable;

use super::Term;

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
        alloc.keyword(GOTO).append(
            self.term
                .print(cfg, alloc)
                .append(SEMI)
                .append(
                    alloc
                        .space()
                        .append(TICK)
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

    use super::{Goto, Term};
    use crate::{parser::fun, syntax::terms::Lit};
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
