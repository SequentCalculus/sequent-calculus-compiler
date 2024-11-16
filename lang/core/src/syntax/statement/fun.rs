use printer::{DocAllocator, Print};

use crate::{
    syntax::{
        substitution::Substitution,
        term::{Cns, Prd, Term},
        types::{Ty, Typed},
        Covar, Name, Statement, Var,
    },
    traits::{
        focus::{bind_many, Focusing, FocusingState},
        free_vars::FreeV,
        substitution::Subst,
        uniquify::Uniquify,
        used_binders::UsedBinders,
    },
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
    type Target = crate::syntax_var::Statement;
    ///N(f(t_i)) = bind(t_i)[λas.f(as)]
    fn focus(self, state: &mut FocusingState) -> crate::syntax_var::Statement {
        bind_many(
            self.args.into(),
            Box::new(|args, _: &mut FocusingState| {
                crate::syntax_var::statement::Call {
                    name: self.name,
                    args: args.into_iter().collect(),
                }
                .into()
            }),
            state,
        )
    }
}

#[cfg(test)]
mod transform_tests {
    use super::Focusing;
    use crate::syntax::{statement::Fun, substitution::SubstitutionBinding, term::XVar, types::Ty};

    fn example_fun1() -> Fun {
        Fun {
            name: "main".to_owned(),
            args: vec![],
            ty: Ty::Int(),
        }
    }
    fn example_fun2() -> Fun {
        Fun {
            name: "fun".to_owned(),
            args: vec![
                SubstitutionBinding::ProducerBinding(XVar::var("x", Ty::Int()).into()),
                SubstitutionBinding::ConsumerBinding(XVar::covar("a", Ty::Int()).into()),
            ],
            ty: Ty::Int(),
        }
    }

    #[test]
    fn transform_fun1() {
        let result = example_fun1().focus(&mut Default::default());
        let expected = crate::syntax_var::statement::Call {
            name: "main".to_owned(),
            args: vec![],
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn transform_fun2() {
        let result = example_fun2().focus(&mut Default::default());
        let expected = crate::syntax_var::statement::Call {
            name: "fun".to_owned(),
            args: vec!["x".to_string(), "a".to_string()],
        }
        .into();
        assert_eq!(result, expected)
    }
}
