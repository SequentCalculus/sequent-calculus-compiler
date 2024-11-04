use super::Check;
use crate::{
    parser::util::ToMiette,
    syntax::{context::TypingContext, terms::Lit, types::Ty},
    typing::{check::check_equality, errors::Error, symbol_table::SymbolTable},
};

impl Check for Lit {
    fn check(
        self,
        _symbol_table: &SymbolTable,
        _context: &TypingContext,
        expected: &Ty,
    ) -> Result<Lit, Error> {
        check_equality(&self.span.to_miette(), expected, &Ty::mk_int())?;
        Ok(Lit {
            span: self.span,
            val: self.val,
        })
    }
}

#[cfg(test)]
mod lit_test {
    use super::Check;
    use crate::{
        syntax::{terms::Lit, types::Ty},
        typing::symbol_table::SymbolTable,
    };
    use codespan::Span;

    #[test]
    fn check_lit() {
        let result = Lit {
            span: Span::default(),
            val: 1,
        }
        .check(&SymbolTable::default(), &vec![], &Ty::mk_int())
        .unwrap();
        let expected = Lit {
            span: Span::default(),
            val: 1,
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn check_lit_fail() {
        let result = Lit {
            span: Span::default(),
            val: 1,
        }
        .check(&SymbolTable::default(), &vec![], &Ty::mk_decl("ListInt"));
        assert!(result.is_err())
    }
}
