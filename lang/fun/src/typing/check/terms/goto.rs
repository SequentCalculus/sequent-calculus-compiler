use super::Check;
use crate::{
    parser::util::ToMiette,
    syntax::{
        context::TypingContext,
        terms::Goto,
        types::{OptTyped, Ty},
    },
    typing::{check::lookup_covar, errors::Error, symbol_table::SymbolTable},
};

impl Check for Goto {
    fn check(
        self,
        symbol_table: &SymbolTable,
        context: &TypingContext,
        _expected: &Ty,
    ) -> Result<Goto, Error> {
        let cont_type = lookup_covar(&self.span.to_miette(), context, &self.target)?;
        let term = self.term.check(symbol_table, context, &cont_type)?;
        let ty = term.get_type();
        Ok(Goto {
            span: self.span,
            term,
            target: self.target,
            ty,
        })
    }
}

#[cfg(test)]
mod goto_tests {
    use super::Check;
    use crate::{
        syntax::{
            context::ContextBinding,
            terms::{Goto, Lit},
            types::Ty,
        },
        typing::symbol_table::SymbolTable,
    };
    use codespan::Span;
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
}
