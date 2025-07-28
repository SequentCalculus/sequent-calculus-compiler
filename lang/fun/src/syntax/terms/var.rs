use codespan::Span;
use derivative::Derivative;
use printer::Print;

use super::Term;
use crate::{
    parser::util::ToMiette,
    syntax::{
        context::{Chirality, TypingContext},
        types::{OptTyped, Ty},
        Var,
    },
    typing::{
        check::{check_equality, Check},
        errors::Error,
        symbol_table::SymbolTable,
    },
};

/// A term representing a variable or covariable
/// Example: `x`
#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct XVar {
    /// The source location
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    /// The (co-) variable
    pub var: Var,
    /// The type of the term (inferred)
    pub ty: Option<Ty>,
    /// The chirality, i.e whether this is a variable or covariable
    pub chi: Option<Chirality>,
}

impl XVar {
    pub fn mk(var: &str) -> Self {
        XVar {
            span: Span::default(),
            var: var.to_string(),
            ty: None,
            chi: None,
        }
    }
}

impl OptTyped for XVar {
    fn get_type(&self) -> Option<Ty> {
        self.ty.clone()
    }
}

impl Print for XVar {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        self.var.print(cfg, alloc)
    }
}

impl From<XVar> for Term {
    fn from(value: XVar) -> Self {
        Term::XVar(value)
    }
}

impl Check for XVar {
    fn check(
        mut self,
        symbol_table: &mut SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<Self, Error> {
        use Chirality::*;
        // Free covariables must only occur in special positions (`goto` and `substitution`s)
        // and are thus rejected in all other positions by the `check` function for `XVar`.
        if self.chi == Some(Cns) {
            return Err(Error::ExpectedTermGotCovariable {
                span: self.span.to_miette(),
            });
        }

        let found_ty = context.lookup_var(&self.var, &self.span.to_miette())?;
        if let Some(ty) = self.ty {
            check_equality(&self.span, symbol_table, &ty, &found_ty)?;
        };

        check_equality(&self.span, symbol_table, expected, &found_ty)?;

        self.ty = Some(expected.clone());
        self.chi = Some(Prd);
        Ok(self)
    }
}

#[cfg(test)]
mod test {
    use super::Check;
    use crate::{
        syntax::{
            context::{Chirality::Prd, TypingContext},
            terms::XVar,
            types::{Ty, TypeArgs},
        },
        typing::symbol_table::SymbolTable,
    };
    use codespan::Span;

    #[test]
    fn check_var() {
        let mut ctx = TypingContext::default();
        ctx.add_var("x", Ty::mk_i64());
        let result = XVar::mk("x")
            .check(&mut SymbolTable::default(), &ctx, &Ty::mk_i64())
            .unwrap();
        let expected = XVar {
            span: Span::default(),
            var: "x".to_owned(),
            ty: Some(Ty::mk_i64()),
            chi: Some(Prd),
        };
        assert_eq!(result, expected)
    }
    #[test]
    fn check_var_fail() {
        let mut ctx = TypingContext::default();
        ctx.add_var("x", Ty::mk_i64());
        let result = XVar::mk("x").check(
            &mut SymbolTable::default(),
            &ctx,
            &Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_i64()])),
        );
        assert!(result.is_err())
    }
}
