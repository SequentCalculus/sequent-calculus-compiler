use std::rc::Rc;

use codespan::Span;
use derivative::Derivative;
use printer::{
    theme::ThemeExt,
    tokens::{GOTO, SEMI, TICK},
    DocAllocator, Print,
};

use crate::{
    parser::util::ToMiette,
    syntax::{
        context::TypingContext,
        types::{OptTyped, Ty},
        Covariable,
    },
    typing::{
        check::{context::lookup_covar, Check},
        errors::Error,
        symbol_table::SymbolTable,
    },
};

use super::Term;

#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct Goto {
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    pub term: Rc<Term>,
    pub target: Covariable,
    pub ty: Option<Ty>,
}

impl OptTyped for Goto {
    fn get_type(&self) -> Option<Ty> {
        self.ty.clone()
    }
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

impl Check for Goto {
    fn check(
        self,
        symbol_table: &SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<Self, Error> {
        let cont_type = lookup_covar(&self.span.to_miette(), context, &self.target)?;
        let term_checked = self.term.check(symbol_table, context, &cont_type)?;
        Ok(Goto {
            term: term_checked,
            ty: Some(expected.clone()),
            ..self
        })
    }
}

#[cfg(test)]
mod test {
    use super::Check;
    use super::Term;
    use crate::parser::fun;
    use crate::{
        syntax::{
            context::ContextBinding,
            terms::{Goto, Lit},
            types::Ty,
        },
        typing::symbol_table::SymbolTable,
    };
    use codespan::Span;
    use printer::Print;
    use std::rc::Rc;

    #[test]
    fn check_goto() {
        let result = Goto {
            span: Span::default(),
            target: "a".to_owned(),
            term: Rc::new(
                Lit {
                    span: Span::default(),
                    val: 1,
                }
                .into(),
            ),
            ty: None,
        }
        .check(
            &SymbolTable::default(),
            &vec![ContextBinding::TypedCovar {
                covar: "a".to_owned(),
                ty: Ty::mk_int(),
            }],
            &Ty::mk_int(),
        )
        .unwrap();
        let expected = Goto {
            span: Span::default(),
            target: "a".to_owned(),
            term: Rc::new(
                Lit {
                    span: Span::default(),
                    val: 1,
                }
                .into(),
            ),
            ty: Some(Ty::mk_int()),
        };
        assert_eq!(result, expected)
    }
    #[test]
    fn check_goto_fail() {
        let result = Goto {
            span: Span::default(),
            target: "a".to_owned(),
            term: Rc::new(
                Lit {
                    span: Span::default(),
                    val: 1,
                }
                .into(),
            ),
            ty: None,
        }
        .check(&SymbolTable::default(), &vec![], &Ty::mk_int());
        assert!(result.is_err())
    }

    fn example() -> Goto {
        Goto {
            span: Span::default(),
            term: Rc::new(Term::Lit(Lit::mk(2))),
            target: "x".to_string(),
            ty: None,
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
