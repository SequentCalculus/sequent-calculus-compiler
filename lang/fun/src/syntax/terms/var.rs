use codespan::Span;
use derivative::Derivative;
use printer::{DocAllocator, Print};

use super::Term;
use crate::{
    parser::util::ToMiette,
    syntax::{
        context::TypingContext,
        types::{OptTyped, Ty},
        Variable,
    },
    typing::{
        check::{check_equality, Check},
        errors::Error,
        symbol_table::SymbolTable,
    },
};

#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub enum PrdCns {
    Prd,
    Cns,
}

#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct XVar {
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    pub var: Variable,
    pub ty: Option<Ty>,
    pub chi: Option<PrdCns>,
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
        _cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc.text(self.var.clone())
    }
}

impl From<XVar> for Term {
    fn from(value: XVar) -> Self {
        Term::XVar(value)
    }
}

impl Check for XVar {
    fn check(
        self,
        _symbol_table: &SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<Self, Error> {
        use PrdCns::*;
        if self.chi == Some(Cns) {
            Err(Error::ExpectedTermGotCovariable {
                span: self.span.to_miette(),
            })
        } else {
            let found_ty = context.lookup_var(&self.var, &self.span.to_miette())?;
            if self.ty.is_none() {
                Ok(())
            } else {
                check_equality(&self.span.to_miette(), &self.ty.unwrap(), &found_ty)
            }?;

            check_equality(&self.span.to_miette(), expected, &found_ty)?;
            Ok(XVar {
                ty: Some(expected.clone()),
                chi: Some(Prd),
                ..self
            })
        }
    }
}

#[cfg(test)]
mod test {
    use super::Check;
    use crate::{
        syntax::{
            context::TypingContext,
            terms::{PrdCns::Prd, XVar},
            types::Ty,
        },
        typing::symbol_table::SymbolTable,
    };
    use codespan::Span;

    #[test]
    fn check_var() {
        let mut ctx = TypingContext::default();
        ctx.add_var("x", Ty::mk_i64());
        let result = XVar::mk("x")
            .check(&SymbolTable::default(), &ctx, &Ty::mk_i64())
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
        let result = XVar::mk("x").check(&SymbolTable::default(), &ctx, &Ty::mk_decl("ListInt"));
        assert!(result.is_err())
    }
}
