use std::rc::Rc;

use codespan::Span;
use derivative::Derivative;
use printer::{
    theme::ThemeExt,
    tokens::{CASE, DOT},
    DocAllocator, Print,
};

use crate::syntax::{
    print_cases,
    types::{OptTyped, Ty},
    Name,
};

use super::{Clause, Term};

#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct Case {
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    pub destructee: Rc<Term>,
    pub cases: Vec<Clause<Name>>,
    pub ty: Option<Ty>,
}

impl OptTyped for Case {
    fn get_type(&self) -> Option<Ty> {
        self.ty.clone()
    }
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
        syntax::{
            context::ContextBinding,
            terms::{Lit, Var},
            types::Ty,
        },
    };

    use super::{Case, Clause, Term};
    use std::rc::Rc;

    fn example_empty() -> Case {
        Case {
            span: Span::default(),
            destructee: Rc::new(Var::mk("x").into()),
            cases: vec![],
            ty: None,
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
            ty: None,
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
