use printer::{
    tokens::{LANGLE, PIPE, RANGLE},
    DocAllocator, Print,
};

use super::{Covar, Statement, Var};
use crate::{
    syntax::{
        statement::FsStatement,
        term::{xtor::FsXtor, Cns, FsTerm, Prd, Term},
        types::Ty,
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
            producer,
            consumer,
            ty: _,
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

impl FreeV for Cut {
    fn free_vars(&self) -> HashSet<Var> {
        let mut free_vars = self.producer.free_vars();
        free_vars.extend(self.consumer.free_vars());
        free_vars
    }

    fn free_covars(&self) -> HashSet<Covar> {
        let mut free_covars = self.producer.free_covars();
        free_covars.extend(self.consumer.free_covars());
        free_covars
    }
}

impl UsedBinders for Cut {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        self.producer.used_binders(used);
        self.consumer.used_binders(used);
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
    fn focus(self, state: &mut FocusingState) -> FsStatement {
        match (
            Rc::unwrap_or_clone(self.producer),
            Rc::unwrap_or_clone(self.consumer),
        ) {
            // N(⟨K(t_i) | c⟩) = bind(t_i)[λas.⟨K(as) | N(c)⟩]
            (Term::Xtor(constructor), consumer) => bind_many(
                constructor.args.into(),
                Box::new(|vars, state: &mut FocusingState| {
                    FsCut::new(
                        FsXtor {
                            id: constructor.id,
                            args: vars.into_iter().collect(),
                        },
                        consumer.focus(state),
                        self.ty,
                    )
                    .into()
                }),
                state,
            ),
            // N(⟨p | D(t_i)⟩) = bind(t_i)[λas⟨D(as) | N(p)⟩]
            (producer, Term::Xtor(destructor)) => bind_many(
                destructor.args.into(),
                Box::new(|args, state: &mut FocusingState| {
                    FsCut {
                        ty: self.ty,
                        producer: Rc::new(
                            FsXtor {
                                id: destructor.id,
                                args: args.into_iter().collect(),
                            }
                            .into(),
                        ),
                        consumer: Rc::new(producer.focus(state)),
                    }
                    .into()
                }),
                state,
            ),
            // N(⟨p | c⟩) = ⟨N(p) | N(c)⟩ OR ⟨N(c) | N(p)⟩
            (producer, consumer) => {
                if self.ty.is_codata(state.codata_types) {
                    FsCut {
                        ty: self.ty,
                        producer: Rc::new(consumer.focus(state)),
                        consumer: Rc::new(producer.focus(state)),
                    }
                    .into()
                } else {
                    FsCut {
                        ty: self.ty,
                        producer: Rc::new(producer.focus(state)),
                        consumer: Rc::new(consumer.focus(state)),
                    }
                    .into()
                }
            }
        }
    }
}

// Focused Cut
//
//

/// Focused Cut
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FsCut {
    pub ty: Ty,
    pub producer: Rc<FsTerm>,
    pub consumer: Rc<FsTerm>,
}

impl FsCut {
    pub fn new<T: Into<FsTerm>, S: Into<FsTerm>>(prd: T, cns: S, ty: Ty) -> Self {
        FsCut {
            ty,
            producer: Rc::new(prd.into()),
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
            ty: _,
            producer,
            consumer,
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
            ty: self.ty,
            producer: self.producer.subst_sim(subst),
            consumer: self.consumer.subst_sim(subst),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Focusing;
    use crate::syntax::statement::FsCut;
    use crate::syntax::term::xvar::FsXVar;
    use crate::syntax::Chirality;
    use crate::syntax::{
        statement::Cut,
        substitution::SubstitutionBinding,
        term::{Literal, XVar, Xtor},
        types::Ty,
    };
    use std::rc::Rc;

    #[test]
    // this illustrates the problem
    fn transform_ctor() {
        let result = {
            let cons = Xtor::ctor(
                "Cons",
                vec![
                    SubstitutionBinding::ProducerBinding(Literal::new(1).into()),
                    SubstitutionBinding::ProducerBinding(
                        Xtor::ctor("Nil", vec![], Ty::Decl("ListInt".to_owned())).into(),
                    ),
                ],
                Ty::Decl("ListInt".to_owned()),
            );
            Cut::new(
                cons,
                XVar::covar("a", Ty::Decl("ListInt".to_owned())),
                Ty::Decl("ListInt".to_owned()),
            )
        }
        .focus(&mut Default::default());
        let expected = crate::syntax::statement::cut::FsCut {
            producer: Rc::new(crate::syntax::term::Literal::new(1).into()),
            ty: crate::syntax::Ty::Int,
            consumer: Rc::new(
                crate::syntax::term::mu::FsMu {
                    chi: Chirality::Cns,
                    variable: "x0".to_owned(),
                    statement: Rc::new(
                        crate::syntax::statement::cut::FsCut {
                            producer: Rc::new(
                                crate::syntax::term::xtor::FsXtor {
                                    id: "Nil".to_string(),
                                    args: vec![],
                                }
                                .into(),
                            ),
                            ty: crate::syntax::Ty::Decl("ListInt".to_owned()),
                            consumer: Rc::new(
                                crate::syntax::term::mu::FsMu {
                                    chi: Chirality::Cns,
                                    variable: "x1".to_owned(),
                                    statement: Rc::new(
                                        crate::syntax::statement::cut::FsCut {
                                            producer: Rc::new(
                                                crate::syntax::term::xtor::FsXtor {
                                                    id: "Cons".to_string(),
                                                    args: vec!["x0".to_string(), "x1".to_string()],
                                                }
                                                .into(),
                                            ),
                                            ty: crate::syntax::Ty::Decl("ListInt".to_owned()),
                                            consumer: Rc::new(
                                                crate::syntax::term::xvar::FsXVar::covar("a")
                                                    .into(),
                                            ),
                                        }
                                        .into(),
                                    ),
                                }
                                .into(),
                            ),
                        }
                        .into(),
                    ),
                }
                .into(),
            ),
        }
        .into();

        assert_eq!(result, expected);
    }

    #[test]
    fn transform_dtor() {
        let result = {
            let ap = Xtor::dtor(
                "Ap",
                vec![
                    SubstitutionBinding::ProducerBinding(XVar::var("y", Ty::Int).into()),
                    SubstitutionBinding::ConsumerBinding(XVar::covar("a", Ty::Int).into()),
                ],
                Ty::Decl("FunIntInt".to_owned()),
            );
            Cut::new(
                XVar::var("x", Ty::Decl("FunIntInt".to_owned())),
                ap,
                Ty::Decl("FunIntInt".to_owned()),
            )
        }
        .focus(&mut Default::default());
        let expected = {
            let ap = crate::syntax::term::xtor::FsXtor {
                id: "Ap".to_string(),
                args: vec!["y".to_string(), "a".to_string()],
            };
            crate::syntax::statement::cut::FsCut::new(
                ap,
                crate::syntax::term::xvar::FsXVar::var("x"),
                crate::syntax::Ty::Decl("FunIntInt".to_owned()),
            )
        }
        .into();
        assert_eq!(result, expected);
    }

    #[test]
    fn transform_other() {
        let result = Cut::new(XVar::var("x", Ty::Int), XVar::covar("a", Ty::Int), Ty::Int)
            .focus(&mut Default::default());
        let expected = FsCut::new(FsXVar::var("x"), FsXVar::covar("a"), Ty::Int).into();
        assert_eq!(result, expected);
    }
}
