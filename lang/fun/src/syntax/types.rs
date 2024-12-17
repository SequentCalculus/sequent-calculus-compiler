use codespan::Span;
use derivative::Derivative;
use printer::{theme::ThemeExt, tokens::I64, Print};

use crate::{
    parser::util::ToMiette,
    syntax::Name,
    typing::{errors::Error, symbol_table::SymbolTable},
};

/// Types
#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub enum Ty {
    /// Signed 64-Bit integer.
    I64 {
        #[derivative(PartialEq = "ignore")]
        span: Span,
    },
    /// Declared data or codata type.
    Decl {
        #[derivative(PartialEq = "ignore")]
        span: Span,
        name: Name,
    },
}

impl Ty {
    pub fn check(&self, symbol_table: &SymbolTable) -> Result<(), Error> {
        match self {
            Ty::I64 { .. } => Ok(()),
            Ty::Decl { span, name } => match symbol_table.ty_ctors.get(name) {
                None => Err(Error::Undefined {
                    span: span.to_miette(),
                    name: name.clone(),
                }),
                Some(_) => Ok(()),
            },
        }
    }
}

pub trait OptTyped {
    fn get_type(&self) -> Option<Ty>;
}

impl Print for Ty {
    fn print<'a>(
        &'a self,
        _cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        match self {
            Ty::I64 { .. } => alloc.keyword(I64),
            Ty::Decl { name, .. } => alloc.typ(name),
        }
    }
}

impl Ty {
    pub fn mk_i64() -> Self {
        Ty::I64 {
            span: Span::default(),
        }
    }

    pub fn mk_decl(name: &str) -> Self {
        Ty::Decl {
            span: Span::default(),
            name: name.to_string(),
        }
    }
}

#[cfg(test)]
mod type_tests {
    use printer::Print;

    use super::Ty;

    #[test]
    fn display_i64() {
        assert_eq!(Ty::mk_i64().print_to_string(None), "i64".to_owned())
    }
}
