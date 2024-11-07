use codespan::Span;
use derivative::Derivative;
use printer::{theme::ThemeExt, Print};

use crate::syntax::{substitution::Substitution, Name};

use super::Term;

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

    use super::{Constructor, Term};
    use crate::{parser::fun, syntax::terms::Lit};

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
