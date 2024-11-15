use codespan::Span;
use derivative::Derivative;
use printer::{DocAllocator, Print};

use super::Term;
use crate::{
    parser::util::ToMiette,
    syntax::{
        context::TypingContext,
        types::{OptTyped, Ty},
    },
    typing::{
        check::{check_equality, Check},
        errors::Error,
        symbol_table::SymbolTable,
    },
};

#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct Lit {
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    pub val: i64,
}

impl Lit {
    pub fn mk(val: i64) -> Self {
        Lit {
            span: Span::default(),
            val,
        }
    }
}

impl OptTyped for Lit {
    fn get_type(&self) -> Option<Ty> {
        Some(Ty::mk_int())
    }
}

impl Print for Lit {
    fn print<'a>(
        &'a self,
        _cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc.text(format!("{}", self.val))
    }
}

impl From<Lit> for Term {
    fn from(value: Lit) -> Self {
        Term::Lit(value)
    }
}

impl Check for Lit {
    fn check(
        self,
        _symbol_table: &SymbolTable,
        _context: &TypingContext,
        expected: &Ty,
    ) -> Result<Self, Error> {
        check_equality(&self.span.to_miette(), expected, &Ty::mk_int())?;
        Ok(self)
    }
}

#[cfg(test)]
mod test {
    use super::Check;
    use crate::{
        syntax::{terms::Lit, types::Ty},
        typing::symbol_table::SymbolTable,
    };

    #[test]
    fn check_lit() {
        let result = Lit::mk(1)
            .check(&SymbolTable::default(), &vec![], &Ty::mk_int())
            .unwrap();
        let expected = Lit::mk(1);
        assert_eq!(result, expected)
    }

    #[test]
    fn check_lit_fail() {
        let result = Lit::mk(1).check(&SymbolTable::default(), &vec![], &Ty::mk_decl("ListInt"));
        assert!(result.is_err())
    }
}
