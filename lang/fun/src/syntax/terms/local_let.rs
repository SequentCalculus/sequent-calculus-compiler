use std::rc::Rc;

use codespan::Span;
use derivative::Derivative;
use printer::{
    theme::ThemeExt,
    tokens::{COLON, EQ, IN, LET},
    DocAllocator, Print,
};

use crate::syntax::{types::Ty, Variable};

use super::Term;

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

    use super::{Let, Term, Ty};
    use crate::{parser::fun, syntax::terms::Lit};
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
