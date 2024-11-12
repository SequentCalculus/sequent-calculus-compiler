use codespan::Span;
use derivative::Derivative;
use printer::{DocAllocator, Print};

use crate::syntax::{
    substitution::Substitution,
    types::{OptTyped, Ty},
    Name,
};

use super::Term;

#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct Fun {
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    pub name: Name,
    pub args: Substitution,
    pub ret_ty: Option<Ty>,
}

impl OptTyped for Fun {
    fn get_type(&self) -> Option<Ty> {
        self.ret_ty.clone()
    }
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

    use crate::{
        parser::fun,
        syntax::{substitution::SubstitutionBinding, terms::Lit},
    };

    use super::{Fun, Term};

    fn example_simple() -> Fun {
        Fun {
            span: Span::default(),
            name: "foo".to_string(),
            args: vec![],
            ret_ty: None,
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
                SubstitutionBinding::CovarBinding {
                    covar: "a".to_string(),
                    ty: None,
                },
            ],
            ret_ty: None,
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
