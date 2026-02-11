//! This module defines variables and covariables in Fun.

use derivative::Derivative;
use miette::SourceSpan;
use printer::*;

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
    pub span: SourceSpan,
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
    pub fn mk(var: Var) -> Self {
        XVar {
            span: dummy_span(),
            var: var.clone(),
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
            return Err(Error::ExpectedTermGotCovariable { span: self.span });
        }

        let found_ty = context.lookup_var(&self.var, &self.span)?;
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
    use crate::syntax::util::dummy_span;
    use crate::syntax::*;
    use crate::typing::*;

    #[test]
    fn check_var() {
        let mut ctx = TypingContext::default();
        ctx.add_var("x", 0, Ty::mk_i64());
        let result = XVar::mk(Var {
            name: "x".to_string(),
            id: 0,
        })
        .check(&mut SymbolTable::default(), &ctx, &Ty::mk_i64())
        .unwrap();
        let expected = XVar {
            span: dummy_span(),
            var: Var {
                name: "x".to_owned(),
                id: 0,
            },
            ty: Some(Ty::mk_i64()),
            chi: Some(Prd),
        };
        assert_eq!(result, expected)
    }
    #[test]
    fn check_var_fail() {
        let mut ctx = TypingContext::default();
        ctx.add_var("x", 0, Ty::mk_i64());
        let result = XVar::mk(Var {
            name: "x".to_string(),
            id: 0,
        })
        .check(
            &mut SymbolTable::default(),
            &ctx,
            &Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_i64()])),
        );
        assert!(result.is_err())
    }
}
