use super::Check;
use crate::{
    syntax::{
        context::{ContextBinding, TypingContext},
        terms::Label,
        types::Ty,
    },
    typing::{errors::Error, symbol_table::SymbolTable},
};

impl Check for Label {
    fn check(
        self,
        symbol_table: &SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<Label, Error> {
        let mut new_context = context.clone();
        new_context.push(ContextBinding::TypedCovar {
            covar: self.label.clone(),
            ty: expected.clone(),
        });
        let new_term = self.term.check(symbol_table, &new_context, expected)?;
        Ok(Label {
            span: self.span,
            label: self.label,
            term: new_term,
            cont_ty: Some(expected.clone()),
        })
    }
}

#[cfg(test)]
mod label_tests {
    use super::Check;
    use crate::{
        syntax::{
            context::ContextBinding,
            terms::{Label, Lit, Var},
            types::Ty,
        },
        typing::symbol_table::SymbolTable,
    };
    use codespan::Span;
    use std::rc::Rc;

    #[test]
    fn check_label() {
        let result = Label {
            span: Span::default(),
            label: "a".to_owned(),
            cont_ty: None,
            term: Rc::new(
                Lit {
                    span: Span::default(),
                    val: 1,
                }
                .into(),
            ),
        }
        .check(&SymbolTable::default(), &vec![], &Ty::mk_int())
        .unwrap();
        let expected = Label {
            span: Span::default(),
            label: "a".to_owned(),
            cont_ty: Some(Ty::mk_int()),
            term: Rc::new(
                Lit {
                    span: Span::default(),
                    val: 1,
                }
                .into(),
            ),
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn check_label_fail() {
        let result = Label {
            span: Span::default(),
            label: "a".to_owned(),
            term: Rc::new(
                Var {
                    span: Span::default(),
                    var: "x".to_owned(),
                    ty: None,
                }
                .into(),
            ),
            cont_ty: None,
        }
        .check(
            &SymbolTable::default(),
            &vec![ContextBinding::TypedVar {
                var: "x".to_owned(),
                ty: Ty::mk_decl("ListInt"),
            }],
            &Ty::mk_int(),
        );
        assert!(result.is_err())
    }
}
