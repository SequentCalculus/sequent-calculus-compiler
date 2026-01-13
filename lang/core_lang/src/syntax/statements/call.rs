//! This module defines the call of a top-level function in Core.

use printer::*;

use crate::syntax::*;
use crate::traits::*;

use std::collections::{BTreeSet, HashSet};

/// This struct defines the call of a top-level function in Core. It consists of the name of the
/// top-level function to call, the arguments, and the type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Call {
    /// The name of the top-level function being called
    pub name: Name,
    /// The arguments
    pub args: Arguments,
    /// The type (which is the return type of the definition)
    pub ty: Ty,
}

impl Typed for Call {
    fn get_type(&self) -> Ty {
        self.ty.clone()
    }
}

impl Print for Call {
    fn print<'a>(&'a self, cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        self.name
            .print(cfg, alloc)
            .append(self.args.print(cfg, alloc).parens().group())
    }
}

impl From<Call> for Statement {
    fn from(value: Call) -> Self {
        Statement::Call(value)
    }
}

impl Subst for Call {
    type Target = Call;
    fn subst_sim(
        mut self,
        prod_subst: &[(Var, Term<Prd>)],
        cons_subst: &[(Var, Term<Cns>)],
    ) -> Self::Target {
        self.args = self.args.subst_sim(prod_subst, cons_subst);
        self
    }
}

impl TypedFreeVars for Call {
    fn typed_free_vars(&self, vars: &mut BTreeSet<ContextBinding>) {
        self.args.typed_free_vars(vars)
    }
}

impl Uniquify for Call {
    fn uniquify(mut self, seen_vars: &mut HashSet<Var>, used_vars: &mut HashSet<Var>) -> Call {
        self.args = self.args.uniquify(seen_vars, used_vars);
        self
    }
}

impl Focusing for Call {
    type Target = FsStatement;
    // focus(f(t_i)) = bind(t_i)[λas.f(as)]
    fn focus(self, used_vars: &mut HashSet<Var>) -> FsStatement {
        bind_many(
            self.args.into(),
            Box::new(|bindings, _: &mut HashSet<Var>| {
                FsCall {
                    name: self.name,
                    args: bindings.into(),
                }
                .into()
            }),
            used_vars,
        )
    }
}

/// This struct defines the focused version of [`Call`]s of top-level functions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FsCall {
    /// The name of the top-level function being called
    pub name: Name,
    /// The arguments (only (co)variables here)
    pub args: TypingContext,
}

impl Print for FsCall {
    fn print<'a>(&'a self, cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        self.name
            .print(cfg, alloc)
            .append(self.args.print(cfg, alloc).parens().group())
    }
}

impl From<FsCall> for FsStatement {
    fn from(value: FsCall) -> Self {
        FsStatement::Call(value)
    }
}

impl SubstVar for FsCall {
    type Target = FsCall;
    fn subst_sim(mut self, subst: &[(Var, Var)]) -> FsCall {
        self.args = self.args.subst_sim(subst);
        self
    }
}

impl TypedFreeVars for FsCall {
    fn typed_free_vars(&self, vars: &mut BTreeSet<ContextBinding>) {
        vars.extend(self.args.bindings.iter().cloned())
    }
}

#[cfg(test)]
mod transform_tests {
    use crate::traits::*;
    extern crate self as core_lang;
    use macros::{bind, call, cns, covar, fs_call, prd, var};

    #[test]
    fn transform_call1() {
        let result = call!("main", []).focus(&mut Default::default());
        let expected = fs_call!("main", []).into();
        assert_eq!(result, expected)
    }

    #[test]
    fn transform_call2() {
        let result = call!("fun", [var!("x"), covar!("a")],).focus(&mut Default::default());
        let expected = fs_call!("fun", [bind!("x", prd!()), bind!("a", cns!())]).into();
        assert_eq!(result, expected)
    }
}
