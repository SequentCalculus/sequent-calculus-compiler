use crate::{
    syntax::{
        stringify_and_join,
        substitution::Substitution,
        term::{Cns, Prd, Term},
        types::Ty,
        Covar, Name, Var,
    },
    traits::{
        focus::{bind_many, Focusing, FocusingState},
        free_vars::FreeV,
        substitution::Subst,
        typed::Typed,
    },
};
use std::{collections::HashSet, fmt};

use super::Statement;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fun {
    pub name: Name,
    pub args: Substitution,
    pub ret_ty: Ty,
}

impl Typed for Fun {
    fn get_type(&self) -> Ty {
        self.ret_ty.clone()
    }
}

impl std::fmt::Display for Fun {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let args_joined: String = stringify_and_join(&self.args);
        write!(f, "{}({})", self.name, args_joined)
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
            ret_ty: self.ret_ty.clone(),
        }
    }
}

impl Focusing for Fun {
    type Target = Statement;
    ///N(f(p_i; c_j)) = bind(p_i)[λas.bind(c_j)[λbs.f(as; bs)]]
    fn focus(self, state: &mut FocusingState) -> Statement {
        let ty = self.ret_ty.clone();
        bind_many(
            self.args.into(),
            Box::new(|args, _: &mut FocusingState| {
                Fun {
                    name: self.name,
                    args: args.into_iter().collect(),
                    ret_ty: ty,
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
            ret_ty: Ty::Int(),
        }
    }
    fn example_fun2() -> Fun {
        Fun {
            name: "fun".to_owned(),
            args: vec![
                SubstitutionBinding::ProducerBinding {
                    prd: XVar::var("x", Ty::Int()).into(),
                    ty: Ty::Int(),
                },
                SubstitutionBinding::ConsumerBinding {
                    cns: XVar::covar("a", Ty::Int()).into(),
                    ty: Ty::Int(),
                },
            ],
            ret_ty: Ty::Int(),
        }
    }

    #[test]
    fn transform_fun1() {
        let result = example_fun1().focus(&mut Default::default());
        let expected = Fun {
            name: "main".to_owned(),
            args: vec![],
            ret_ty: Ty::Int(),
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn transform_fun2() {
        let result = example_fun2().focus(&mut Default::default());
        let expected = Fun {
            name: "fun".to_owned(),
            args: vec![
                SubstitutionBinding::ProducerBinding {
                    prd: XVar::var("x", Ty::Int()).into(),
                    ty: Ty::Int(),
                },
                SubstitutionBinding::ConsumerBinding {
                    cns: XVar::covar("a", Ty::Int()).into(),
                    ty: Ty::Int(),
                },
            ],
            ret_ty: Ty::Int(),
        }
        .into();
        assert_eq!(result, expected)
    }
}
