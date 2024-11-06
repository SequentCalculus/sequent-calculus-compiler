use std::rc::Rc;

use codespan::Span;
use derivative::Derivative;
use printer::{
    theme::ThemeExt,
    tokens::{LABEL, TICK},
    util::BracesExt,
    DocAllocator, Print,
};

use crate::syntax::Covariable;

use super::Term;

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
            .keyword(LABEL)
            .append(alloc.space())
            .append(TICK)
            .append(self.label.clone())
            .append(alloc.space())
            .append(self.term.print(cfg, alloc).braces_anno())
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

    use super::{Label, Term};
    use crate::{parser::fun, syntax::terms::Lit};
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
