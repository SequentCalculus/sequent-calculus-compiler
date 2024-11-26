use printer::{DocAllocator, Print};

use super::{Cns, FsTerm, Prd, Term};
use crate::{
    syntax::{
        types::{Ty, Typed},
        Covar, Var,
    },
    traits::{
        focus::{Bind, Continuation, FocusingState},
        free_vars::FreeV,
        substitution::Subst,
    },
};

use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Literal {
    pub lit: i64,
}

impl Literal {
    #[must_use]
    pub fn new(lit: i64) -> Self {
        Literal { lit }
    }
}

impl From<Literal> for FsTerm {
    fn from(value: Literal) -> Self {
        FsTerm::Literal(value)
    }
}

impl Typed for Literal {
    fn get_type(&self) -> Ty {
        Ty::Int()
    }
}

impl Print for Literal {
    fn print<'a>(
        &'a self,
        _cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc.text(format!("{}", self.lit))
    }
}

impl From<Literal> for Term<Prd> {
    fn from(value: Literal) -> Self {
        Term::Literal(value)
    }
}

impl FreeV for Literal {
    fn free_vars(&self) -> HashSet<Var> {
        HashSet::new()
    }

    fn free_covars(&self) -> HashSet<Covar> {
        HashSet::new()
    }
}

impl Subst for Literal {
    type Target = Literal;
    fn subst_sim(
        &self,
        _prod_subst: &[(Term<Prd>, Var)],
        _cons_subst: &[(Term<Cns>, Covar)],
    ) -> Self::Target {
        self.clone()
    }
}

impl Bind for Literal {
    ///bind(⌜n⌝)[k] = ⟨⌜n⌝ | ~μx.k(x)⟩
    fn bind(self, k: Continuation, state: &mut FocusingState) -> crate::syntax_var::FsStatement {
        let new_var = state.fresh_var();
        crate::syntax::statement::cut::FsCut::new(
            crate::syntax::Ty::Int(),
            self,
            crate::syntax::term::mu::FsMu::tilde_mu(&new_var, k(new_var.clone(), state)),
        )
        .into()
    }
}

#[cfg(test)]
mod lit_tests {
    use printer::Print;

    use super::Bind;
    use super::{Cns, FreeV, Literal, Prd, Subst, Term};
    use crate::syntax::types::Ty;
    use crate::syntax::{term::XVar, Covar, Var};
    use crate::syntax_var::Chirality;
    use std::rc::Rc;

    // Display tests

    #[test]
    fn display_lit() {
        let result = Literal::new(1).print_to_string(None);
        let expected = "1".to_owned();
        assert_eq!(result, expected)
    }

    // Free variable tests

    #[test]
    fn free_vars_lit() {
        assert!(Literal::new(1).free_vars().is_empty())
    }
    #[test]
    fn free_covars_lit() {
        assert!(Literal::new(1).free_covars().is_empty())
    }

    // Substitution tests

    #[test]
    fn subst_lit() {
        let prod_subst: Vec<(Term<Prd>, Var)> =
            vec![(XVar::var("y", Ty::Int()).into(), "x".to_owned())];
        let cons_subst: Vec<(Term<Cns>, Covar)> =
            vec![(XVar::covar("b", Ty::Int()).into(), "a".to_owned())];
        let result = Literal::new(1).subst_sim(&prod_subst, &cons_subst);
        let expected = Literal::new(1);
        assert_eq!(result, expected)
    }

    // Focusing tests

    #[test]
    fn bind_lit1() {
        let result = Literal::new(1).bind(
            Box::new(|_, _| crate::syntax_var::FsStatement::Done()),
            &mut Default::default(),
        );
        let expected = crate::syntax::statement::cut::FsCut {
            producer: Rc::new(Literal::new(1).into()),
            ty: crate::syntax::Ty::Int(),
            consumer: Rc::new(
                crate::syntax::term::mu::FsMu {
                    chi: Chirality::Cns,
                    variable: "x0".to_owned(),
                    statement: Rc::new(crate::syntax_var::FsStatement::Done()),
                }
                .into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn bind_lit2() {
        let result = Literal::new(2).bind(
            Box::new(|_, _| crate::syntax_var::FsStatement::Done()),
            &mut Default::default(),
        );
        let expected = crate::syntax::statement::cut::FsCut {
            producer: Rc::new(Literal::new(2).into()),
            ty: crate::syntax::Ty::Int(),
            consumer: Rc::new(
                crate::syntax::term::mu::FsMu {
                    chi: Chirality::Cns,
                    variable: "x0".to_owned(),
                    statement: Rc::new(crate::syntax_var::FsStatement::Done()),
                }
                .into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }
}
