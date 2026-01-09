//! This module defines variables and covariables in Fun.

use codespan::Span;
use derivative::Derivative;
use printer::*;
use std::collections::HashMap;

use crate::parser::util::ToMiette;
use crate::syntax::*;
use crate::typing::*;

/// This struct defines variables and covariables. It consists of the name of the (co)variable, and
/// after typechecking also of the chirality which determines whether this is a variable or
/// covariable and the inferred type.
#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct XVar {
    /// The source location
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    /// The name of the (co)variable
    pub var: Var,
    /// The (inferred) type
    pub ty: Option<Ty>,
    /// The chirality, i.e, whether this is a variable or covariable
    pub chi: Option<Chirality>,
}

impl XVar {
    /// This function returns a (co)variable from a given string, without chirality and type
    /// information.
    pub fn mk(var: &str) -> Self {
        XVar {
            span: Span::default(),
            var: var.to_string(),
            ty: None,
            chi: None,
        }
    }

    pub fn subst_ty(self, mappings: &HashMap<Name, Ty>) -> Self {
        todo!()
    }
}

impl OptTyped for XVar {
    fn get_type(&self) -> Option<Ty> {
        self.ty.clone()
    }
}

impl Print for XVar {
    fn print<'a>(&'a self, cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
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
        // Free covariables must only occur in special positions (`goto` and `arguments`)
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
    use codespan::Span;

    use crate::syntax::*;
    use crate::typing::*;

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
