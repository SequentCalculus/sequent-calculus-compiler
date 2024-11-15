use codespan::Span;
use derivative::Derivative;
use printer::{theme::ThemeExt, Print};

use crate::{
    parser::util::ToMiette,
    syntax::Name,
    typing::{errors::Error, symbol_table::SymbolTable},
};

#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub enum Ty {
    Int {
        #[derivative(PartialEq = "ignore")]
        span: Span,
    },
    Decl {
        #[derivative(PartialEq = "ignore")]
        span: Span,
        name: Name,
    },
}

impl Ty {
    pub fn check(&self, symbol_table: &SymbolTable) -> Result<(), Error> {
        match self {
            Ty::Int { .. } => Ok(()),
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
            Ty::Int { .. } => alloc.keyword("Int"),
            Ty::Decl { name, .. } => alloc.typ(name),
        }
    }
}

impl Ty {
    pub fn mk_int() -> Self {
        Ty::Int {
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
    fn display_int() {
        assert_eq!(
            Ty::mk_int().print_to_string(Default::default()),
            "Int".to_owned()
        )
    }
}
