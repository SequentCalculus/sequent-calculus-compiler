use super::{check_equality, context::lookup_var, terms::Check};
use crate::{
    parser::util::ToMiette,
    syntax::{context::TypingContext, terms::Var, types::Ty},
    typing::{errors::Error, symbol_table::SymbolTable},
};

impl Check for Var {
    fn check(
        self,
        _symbol_table: &SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<Self, Error> {
        let found_ty = lookup_var(&self.span.to_miette(), context, &self.var)?;
        check_equality(&self.span.to_miette(), expected, &found_ty)?;
        Ok(Var {
            ty: Some(expected.clone()),
            ..self
        })
    }
}

#[cfg(test)]
mod var_test {
    use super::Check;
    use crate::{
        syntax::{context::ContextBinding, terms::Var, types::Ty},
        typing::symbol_table::SymbolTable,
    };
    use codespan::Span;
    #[test]
    fn check_var() {
        let result = Var {
            span: Span::default(),
            var: "x".to_owned(),
            ty: None,
        }
        .check(
            &SymbolTable::default(),
            &vec![ContextBinding::TypedVar {
                var: "x".to_owned(),
                ty: Ty::mk_int(),
            }],
            &Ty::mk_int(),
        )
        .unwrap();
        let expected = Var {
            span: Span::default(),
            var: "x".to_owned(),
            ty: Some(Ty::mk_int()),
        };
        assert_eq!(result, expected)
    }
    #[test]
    fn check_var_fail() {
        let result = Var {
            span: Span::default(),
            var: "x".to_owned(),
            ty: None,
        }
        .check(
            &SymbolTable::default(),
            &vec![ContextBinding::TypedVar {
                var: "x".to_owned(),
                ty: Ty::mk_int(),
            }],
            &Ty::mk_decl("ListInt"),
        );
        assert!(result.is_err())
    }
}
