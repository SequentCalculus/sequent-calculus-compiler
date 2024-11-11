use super::{check_args, check_equality, terms::Check};
use crate::{
    parser::util::ToMiette,
    syntax::{context::TypingContext, terms::Fun, types::Ty},
    typing::{errors::Error, symbol_table::SymbolTable},
};

impl Check for Fun {
    fn check(
        self,
        symbol_table: &SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<Self, Error> {
        match symbol_table.funs.get(&self.name) {
            Some((types, ret_ty)) => {
                check_equality(&self.span.to_miette(), expected, ret_ty)?;
                let new_args = check_args(
                    &self.span.to_miette(),
                    symbol_table,
                    context,
                    self.args,
                    types,
                )?;
                Ok(Fun {
                    args: new_args,
                    ret_ty: Some(expected.clone()),
                    ..self
                })
            }
            None => Err(Error::Undefined {
                span: self.span.to_miette(),
                name: self.name.clone(),
            }),
        }
    }
}

#[cfg(test)]
mod fun_tests {
    use super::Check;
    use crate::{
        syntax::{
            context::ContextBinding,
            substitution::SubstitutionBinding,
            terms::{Fun, Lit},
            types::Ty,
        },
        typing::symbol_table::SymbolTable,
    };
    use codespan::Span;
    #[test]
    fn check_fun() {
        let mut symbol_table = SymbolTable::default();
        symbol_table.funs.insert(
            "main".to_owned(),
            (
                vec![
                    ContextBinding::TypedVar {
                        var: "x".to_owned(),
                        ty: Ty::mk_int(),
                    },
                    ContextBinding::TypedVar {
                        var: "y".to_owned(),
                        ty: Ty::mk_int(),
                    },
                ],
                Ty::mk_int(),
            ),
        );
        let result = Fun {
            span: Span::default(),
            name: "main".to_owned(),
            args: vec![
                SubstitutionBinding::TermBinding(
                    Lit {
                        span: Span::default(),
                        val: 1,
                    }
                    .into(),
                ),
                SubstitutionBinding::TermBinding(
                    Lit {
                        span: Span::default(),
                        val: 2,
                    }
                    .into(),
                ),
            ],
            ret_ty: None,
        }
        .check(&symbol_table, &vec![], &Ty::mk_int())
        .unwrap();
        let expected = Fun {
            span: Span::default(),
            name: "main".to_owned(),
            args: vec![
                SubstitutionBinding::TermBinding(
                    Lit {
                        span: Span::default(),
                        val: 1,
                    }
                    .into(),
                ),
                SubstitutionBinding::TermBinding(
                    Lit {
                        span: Span::default(),
                        val: 2,
                    }
                    .into(),
                ),
            ],
            ret_ty: Some(Ty::mk_int()),
        };
        assert_eq!(result, expected)
    }
    #[test]
    fn check_fun_fail() {
        let result = Fun {
            span: Span::default(),
            name: "main".to_owned(),
            args: vec![],
            ret_ty: None,
        }
        .check(&SymbolTable::default(), &vec![], &Ty::mk_int());
        assert!(result.is_err())
    }
}
