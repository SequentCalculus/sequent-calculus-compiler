use printer::{DocAllocator, Print};

use crate::{
    syntax::{
        substitution::Substitution,
        terms::{Cns, Prd, Term},
        types::Ty,
        ContextBinding, Covar, FsStatement, Name, Statement, TypingContext, Var,
    },
    traits::*,
};

use std::collections::{BTreeSet, HashSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Call {
    pub name: Name,
    pub args: Substitution,
    pub ty: Ty,
}

impl Typed for Call {
    fn get_type(&self) -> Ty {
        self.ty.clone()
    }
}

impl Print for Call {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc
            .text(&self.name)
            .append(self.args.print(cfg, alloc).parens())
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
        cons_subst: &[(Covar, Term<Cns>)],
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
    ///N(f(t_i)) = bind(t_i)[λas.f(as)]
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

/// Focused Call
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FsCall {
    pub name: Name,
    pub args: TypingContext,
}

impl Print for FsCall {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc.text(&self.name).append(self.args.print(cfg, alloc))
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
    use super::Focusing;
    use crate::syntax::{
        statements::{Call, FsCall},
        substitution::Substitution,
        terms::XVar,
        types::Ty,
        TypingContext,
    };

    #[test]
    fn transform_call1() {
        let result = Call {
            name: "main".to_string(),
            args: Substitution::default(),
            ty: Ty::I64,
        }
        .focus(&mut Default::default());
        let expected = FsCall {
            name: "main".to_string(),
            args: TypingContext::default(),
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn transform_call2() {
        let mut subst = Substitution::default();
        subst.add_prod(XVar::var("x", Ty::I64));
        subst.add_cons(XVar::covar("a", Ty::I64));

        let mut args = TypingContext::default();
        args.add_var("x", Ty::I64);
        args.add_covar("a", Ty::I64);

        let result = Call {
            name: "fun".to_string(),
            args: subst,
            ty: Ty::I64,
        }
        .focus(&mut Default::default());
        let expected = FsCall {
            name: "fun".to_string(),
            args,
        }
        .into();
        assert_eq!(result, expected)
    }
}
