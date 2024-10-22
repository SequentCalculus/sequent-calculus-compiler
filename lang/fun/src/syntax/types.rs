use codespan::Span;
use derivative::Derivative;
use printer::{DocAllocator, Print};

use crate::syntax::Name;

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

impl Print for Ty {
    fn print<'a>(
        &'a self,
        _cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        match self {
            Ty::Int { .. } => alloc.text("Int"),
            Ty::Decl { name, .. } => alloc.text(name),
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
