use codespan::Span;
use derivative::Derivative;
use printer::{theme::ThemeExt, tokens::COCASE, DocAllocator, Print};

use crate::syntax::{print_cases, types::Ty, Name};

use super::{Clause, Term};

#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct Cocase {
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    pub cocases: Vec<Clause<Name>>,
    pub ty: Option<Ty>,
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

    use crate::{parser::fun, syntax::terms::Lit};

    use super::{Clause, Cocase, Term};

    fn example_empty() -> Cocase {
        Cocase {
            span: Span::default(),
            cocases: vec![],
            ty: None,
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
            ty: None,
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
