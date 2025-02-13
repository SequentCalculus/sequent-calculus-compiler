use printer::{
    tokens::{LANGLE, PIPE, RANGLE},
    DocAllocator, Print,
};

use super::{Covar, Statement, Var};
use crate::{
    syntax::{
        terms::{Cns, FsTerm, FsXtor, Prd, Term},
        types::Ty,
        FsStatement,
    },
    traits::*,
};

use std::{collections::HashSet, rc::Rc};

// Unfocused Cut
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cut {
    pub producer: Rc<Term<Prd>>,
    pub ty: Ty,
    pub consumer: Rc<Term<Cns>>,
}

impl Cut {
    pub fn new<T: Into<Term<Prd>>, S: Into<Term<Cns>>>(prd: T, cns: S, ty: Ty) -> Self {
        Cut {
            producer: Rc::new(prd.into()),
            ty,
            consumer: Rc::new(cns.into()),
        }
    }
}

impl Typed for Cut {
    fn get_type(&self) -> Ty {
        self.ty.clone()
    }
}

impl Print for Cut {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        let Cut {
            producer, consumer, ..
        } = self;
        alloc.text(LANGLE).append(
            producer
                .print(cfg, alloc)
                .append(alloc.space())
                .append(alloc.text(PIPE))
                .append(alloc.space())
                .append(consumer.print(cfg, alloc))
                .append(alloc.text(RANGLE)),
        )
    }
}

impl From<Cut> for Statement {
    fn from(value: Cut) -> Self {
        Statement::Cut(value)
    }
}

impl Subst for Cut {
    type Target = Cut;
    fn subst_sim(
        &self,
        prod_subst: &[(Term<Prd>, Var)],
        cons_subst: &[(Term<Cns>, Covar)],
    ) -> Self::Target {
        Cut {
            producer: self.producer.subst_sim(prod_subst, cons_subst),
            ty: self.ty.clone(),
            consumer: self.consumer.subst_sim(prod_subst, cons_subst),
        }
    }
}

impl Uniquify for Cut {
    fn uniquify(self, seen_vars: &mut HashSet<Var>, used_vars: &mut HashSet<Var>) -> Cut {
        Cut {
            producer: self.producer.uniquify(seen_vars, used_vars),
            consumer: self.consumer.uniquify(seen_vars, used_vars),
            ..self
        }
    }
}

impl Focusing for Cut {
    type Target = FsStatement;
    fn focus(self, used_vars: &mut HashSet<Var>) -> FsStatement {
        match (
            Rc::unwrap_or_clone(self.producer),
            Rc::unwrap_or_clone(self.consumer),
        ) {
            // N(⟨K(t_i) | c⟩) = bind(t_i)[λas.⟨K(as) | N(c)⟩]
            (Term::Xtor(constructor), consumer) => bind_many(
                constructor.args.into(),
                Box::new(|arg_vars, used_vars: &mut HashSet<Var>| {
                    FsCut::new(
                        FsXtor {
                            prdcns: constructor.prdcns,
                            id: constructor.id,
                            args: arg_vars.into_iter().collect(),
                        },
                        consumer.focus(used_vars),
                        self.ty,
                    )
                    .into()
                }),
                used_vars,
            ),
            // N(⟨p | D(t_i)⟩) = bind(t_i)[λas⟨ N(p) | D(as)⟩]
            (producer, Term::Xtor(destructor)) => bind_many(
                destructor.args.into(),
                Box::new(|arg_vars, used_vars: &mut HashSet<Var>| {
                    FsCut::new(
                        producer.focus(used_vars),
                        FsXtor {
                            prdcns: destructor.prdcns,
                            id: destructor.id,
                            args: arg_vars.into_iter().collect(),
                        },
                        self.ty,
                    )
                    .into()
                }),
                used_vars,
            ),
            // N(⟨p | c⟩) = ⟨N(p) | N(c)⟩
            (producer, consumer) => FsCut {
                ty: self.ty,
                producer: Rc::new(producer.focus(used_vars)),
                consumer: Rc::new(consumer.focus(used_vars)),
            }
            .into(),
        }
    }
}

// Focused Cut
//
//

/// Focused Cut
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FsCut {
    pub producer: Rc<FsTerm<Prd>>,
    pub ty: Ty,
    pub consumer: Rc<FsTerm<Cns>>,
}

impl FsCut {
    pub fn new<T: Into<FsTerm<Prd>>, S: Into<FsTerm<Cns>>>(prd: T, cns: S, ty: Ty) -> Self {
        FsCut {
            producer: Rc::new(prd.into()),
            ty,
            consumer: Rc::new(cns.into()),
        }
    }
}

impl Print for FsCut {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        let FsCut {
            producer, consumer, ..
        } = self;
        alloc.text(LANGLE).append(
            producer
                .print(cfg, alloc)
                .append(alloc.space())
                .append(alloc.text(PIPE))
                .append(alloc.space())
                .append(consumer.print(cfg, alloc))
                .append(alloc.text(RANGLE)),
        )
    }
}

impl From<FsCut> for FsStatement {
    fn from(value: FsCut) -> Self {
        FsStatement::Cut(value)
    }
}

impl SubstVar for FsCut {
    type Target = FsCut;
    fn subst_sim(self, subst: &[(Var, Var)]) -> FsCut {
        FsCut {
            producer: self.producer.subst_sim(subst),
            ty: self.ty,
            consumer: self.consumer.subst_sim(subst),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Focusing;
    use crate::syntax::{
        statements::{Cut, FsCut},
        substitution::Substitution,
        terms::{FsXtor, Literal, Mu, XVar, Xtor},
        types::Ty,
    };

    #[test]
    // this illustrates the problem
    fn transform_ctor() {
        let result = {
            let mut subst = Substitution::default();
            subst.add_prod(Literal::new(1));
            subst.add_prod(Xtor::ctor(
                "Nil",
                Substitution::default(),
                Ty::Decl("ListInt".to_string()),
            ));
            let cons = Xtor::ctor("Cons", subst, Ty::Decl("ListInt".to_string()));
            Cut::new(
                cons,
                XVar::covar("a", Ty::Decl("ListInt".to_string())),
                Ty::Decl("ListInt".to_string()),
            )
        }
        .focus(&mut Default::default());
        let expected = FsCut::new(
            Literal::new(1),
            Mu::tilde_mu(
                "x0",
                FsCut::new(
                    FsXtor::ctor("Nil", vec![]),
                    Mu::tilde_mu(
                        "x1",
                        FsCut::new(
                            FsXtor::ctor("Cons", vec!["x0".to_string(), "x1".to_string()]),
                            XVar::covar("a", Ty::Decl("ListInt".to_string())),
                            Ty::Decl("ListInt".to_string()),
                        ),
                        Ty::Decl("ListInt".to_string()),
                    ),
                    Ty::Decl("ListInt".to_string()),
                ),
                Ty::I64,
            ),
            Ty::I64,
        )
        .into();

        assert_eq!(result, expected);
    }

    #[test]
    fn transform_dtor() {
        let mut subst = Substitution::default();
        subst.add_prod(XVar::var("y", Ty::I64));
        subst.add_cons(XVar::covar("a", Ty::I64));
        let result = {
            let ap = Xtor::dtor("Ap", subst, Ty::Decl("FunIntInt".to_string()));
            Cut::new(
                XVar::var("x", Ty::Decl("FunIntInt".to_string())),
                ap,
                Ty::Decl("FunIntInt".to_string()),
            )
        }
        .focus(&mut Default::default());
        let expected = {
            let ap = FsXtor::dtor("Ap", vec!["y".to_string(), "a".to_string()]);
            FsCut::new(
                XVar::var("x", Ty::Decl("FunIntInt".to_string())),
                ap,
                Ty::Decl("FunIntInt".to_string()),
            )
        }
        .into();
        assert_eq!(result, expected);
    }

    #[test]
    fn transform_other() {
        let result = Cut::new(XVar::var("x", Ty::I64), XVar::covar("a", Ty::I64), Ty::I64)
            .focus(&mut Default::default());
        let expected =
            FsCut::new(XVar::var("x", Ty::I64), XVar::covar("a", Ty::I64), Ty::I64).into();
        assert_eq!(result, expected);
    }
}
