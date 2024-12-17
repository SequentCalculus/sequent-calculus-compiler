use printer::{DocAllocator, Print};

use crate::{
    syntax::{
        statement::FsStatement,
        substitution::Substitution,
        term::{Cns, Prd, Term},
        types::Ty,
        Covar, Name, Statement, Var,
    },
    traits::*,
};

use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fun {
    pub name: Name,
    pub args: Substitution,
    pub ty: Ty,
}

impl Typed for Fun {
    fn get_type(&self) -> Ty {
        self.ty.clone()
    }
}

impl Print for Fun {
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

impl From<Fun> for Statement {
    fn from(value: Fun) -> Self {
        Statement::Fun(value)
    }
}

impl FreeV for Fun {
    fn free_vars(&self) -> HashSet<Var> {
        self.args.free_vars()
    }

    fn free_covars(&self) -> HashSet<Covar> {
        self.args.free_covars()
    }
}

impl UsedBinders for Fun {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        self.args.used_binders(used);
    }
}

impl Subst for Fun {
    type Target = Fun;
    fn subst_sim(
        &self,
        prod_subst: &[(Term<Prd>, Var)],
        cons_subst: &[(Term<Cns>, Covar)],
    ) -> Self::Target {
        Fun {
            name: self.name.clone(),
            args: self.args.subst_sim(prod_subst, cons_subst),
            ty: self.ty.clone(),
        }
    }
}

impl Uniquify for Fun {
    fn uniquify(self, seen_vars: &mut HashSet<Var>, used_vars: &mut HashSet<Var>) -> Fun {
        Fun {
            args: self.args.uniquify(seen_vars, used_vars),
            ..self
        }
    }
}

impl Focusing for Fun {
    type Target = FsStatement;
    ///N(f(t_i)) = bind(t_i)[λas.f(as)]
    fn focus(self, state: &mut FocusingState) -> FsStatement {
        bind_many(
            self.args.into(),
            Box::new(|args, _: &mut FocusingState| {
                FsCall {
                    name: self.name,
                    args: args.into_iter().collect(),
                }
                .into()
            }),
            state,
        )
    }
}

/// Focused Call
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FsCall {
    pub name: Name,
    pub args: Vec<Var>,
}

impl Print for FsCall {
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

impl From<FsCall> for FsStatement {
    fn from(value: FsCall) -> Self {
        FsStatement::Call(value)
    }
}

impl SubstVar for FsCall {
    type Target = FsCall;

    fn subst_sim(self, subst: &[(Var, Var)]) -> FsCall {
        FsCall {
            name: self.name,
            args: self.args.subst_sim(subst),
        }
    }
}

#[cfg(test)]
mod transform_tests {
    use super::Focusing;
    use crate::syntax::{
        statement::{FsCall, Fun},
        substitution::SubstitutionBinding,
        term::XVar,
        types::Ty,
    };

    #[test]
    fn transform_fun1() {
        let result = Fun {
            name: "main".to_string(),
            args: vec![],
            ty: Ty::I64,
        }
        .focus(&mut Default::default());
        let expected = FsCall {
            name: "main".to_string(),
            args: vec![],
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn transform_fun2() {
        let result = Fun {
            name: "fun".to_string(),
            args: vec![
                SubstitutionBinding::ProducerBinding(XVar::var("x", Ty::I64).into()),
                SubstitutionBinding::ConsumerBinding(XVar::covar("a", Ty::I64).into()),
            ],
            ty: Ty::I64,
        }
        .focus(&mut Default::default());
        let expected = FsCall {
            name: "fun".to_string(),
            args: vec!["x".to_string(), "a".to_string()],
        }
        .into();
        assert_eq!(result, expected)
    }
}
