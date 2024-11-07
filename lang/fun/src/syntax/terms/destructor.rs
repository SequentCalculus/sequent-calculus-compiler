use std::rc::Rc;

use codespan::Span;
use derivative::Derivative;
use printer::{theme::ThemeExt, tokens::DOT, Print};

use crate::syntax::{substitution::Substitution, types::Ty, Name};

use super::Term;

#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct Destructor {
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    pub id: Name,
    pub destructee: Rc<Term>,
    pub args: Substitution,
    pub ty: Option<Ty>,
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
            ty: None,
        }
    }

    /// "x.hd.hd"
    fn example_2() -> Destructor {
        Destructor {
            span: Span::default(),
            id: "Hd".to_owned(),
            destructee: Rc::new(example_1().into()),
            args: vec![],
            ty: None,
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
            ty: None,
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
