use printer::{tokens::COLON, DocAllocator, Print};

use crate::syntax::{types::Ty, Covariable, Variable};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ContextBinding {
    TypedVar { var: Variable, ty: Ty },
    TypedCovar { covar: Covariable, ty: Ty },
}

pub type TypingContext = Vec<ContextBinding>;

impl Print for ContextBinding {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        match self {
            ContextBinding::TypedVar { var, ty } => alloc
                .text(var)
                .append(alloc.space())
                .append(COLON)
                .append(alloc.space())
                .append(ty.print(cfg, alloc)),
            ContextBinding::TypedCovar { covar, ty } => alloc
                .text("'")
                .append(covar)
                .append(alloc.space())
                .append(":cnt")
                .append(alloc.space())
                .append(ty.print(cfg, alloc)),
        }
    }
}

#[cfg(test)]
mod context_tests {
    use printer::Print;

    use super::{ContextBinding, Ty};

    fn example_contextitem_var() -> ContextBinding {
        ContextBinding::TypedVar {
            var: "x".to_owned(),
            ty: Ty::mk_int(),
        }
    }

    fn example_contextitem_covar() -> ContextBinding {
        ContextBinding::TypedCovar {
            covar: "a".to_owned(),
            ty: Ty::mk_int(),
        }
    }

    #[test]
    fn display_contextitem_var() {
        let result = example_contextitem_var().print_to_string(Default::default());
        let expected = "x : Int";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_contextitem_covar() {
        let result = example_contextitem_covar().print_to_string(Default::default());
        let expected = "'a :cnt Int";
        assert_eq!(result, expected)
    }
}
