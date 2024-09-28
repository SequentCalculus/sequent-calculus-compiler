use codespan::Span;
use derivative::Derivative;

use crate::syntax::Name;
use std::fmt;

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

impl fmt::Display for Ty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Ty::Int { .. } => write!(f, "Int"),
            Ty::Decl { name, .. } => write!(f, "{}", name),
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
    use super::Ty;

    #[test]
    fn display_int() {
        assert_eq!(format!("{}", Ty::mk_int()), "Int".to_owned())
    }
}
