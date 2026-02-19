//! This module defines cuts in Core.

use printer::tokens::{LANGLE, PIPE, RANGLE};
use printer::*;

use crate::syntax::*;
use crate::traits::*;

use std::collections::{BTreeSet, HashSet};
use std::rc::Rc;

/// This structs defines cuts between a producer and consumer term in Core. It consists of the
/// producer and the consumer to be cut and of their type. The type parameters `P` and `C`
/// determine whether this is the unfocused variant (if `P` and `C` are instantiated with
/// [`Term<Prd>`] and [`Term<Cns>`], which is the default) or the focused variant (if `P` and `C`
/// are instantiated with [`FsTerm<Prd>`] and [`FsTerm<Cns>`]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cut<P = Term<Prd>, C = Term<Cns>> {
    /// The producer
    pub producer: Rc<P>,
    /// The type of the cut
    pub ty: Ty,
    /// The consumer
    pub consumer: Rc<C>,
}

pub type FsCut = Cut<FsTerm<Prd>, FsTerm<Cns>>;

impl<P, C> Cut<P, C> {
    /// This function constructs a cut from a producer and a consumer with a given type.
    pub fn new<T: Into<P>, S: Into<C>>(prd: T, cns: S, ty: Ty) -> Self {
        Cut {
            producer: Rc::new(prd.into()),
            ty,
            consumer: Rc::new(cns.into()),
        }
    }
}

impl<P, C> Typed for Cut<P, C> {
    fn get_type(&self) -> Ty {
        self.ty.clone()
    }
}

impl<P, C> Print for Cut<P, C>
where
    P: Print,
    C: Print,
{
    fn print<'a>(&'a self, cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        let Cut {
            producer, consumer, ..
        } = self;
        alloc
            .text(LANGLE)
            .append(producer.print(cfg, alloc))
            .append(alloc.line())
            .append(alloc.text(PIPE))
            .append(alloc.space())
            .append(consumer.print(cfg, alloc))
            .append(alloc.text(RANGLE))
    }
}

impl From<Cut> for Statement {
    fn from(value: Cut) -> Self {
        Statement::Cut(value)
    }
}

impl From<FsCut> for FsStatement {
    fn from(value: FsCut) -> Self {
        FsStatement::Cut(value)
    }
}

impl Subst for Cut {
    type Target = Cut;
    fn subst_sim(
        mut self,
        prod_subst: &[(Ident, Term<Prd>)],
        cons_subst: &[(Ident, Term<Cns>)],
    ) -> Self::Target {
        self.producer = self.producer.subst_sim(prod_subst, cons_subst);
        self.consumer = self.consumer.subst_sim(prod_subst, cons_subst);
        self
    }
}

impl SubstVar for FsCut {
    type Target = FsCut;
    fn subst_sim(mut self, subst: &[(Ident, Ident)]) -> FsCut {
        self.producer = self.producer.subst_sim(subst);
        self.consumer = self.consumer.subst_sim(subst);
        self
    }
}

impl<P, C> TypedFreeVars for Cut<P, C>
where
    P: TypedFreeVars,
    C: TypedFreeVars,
{
    fn typed_free_vars(&self, vars: &mut BTreeSet<ContextBinding>) {
        self.producer.typed_free_vars(vars);
        self.consumer.typed_free_vars(vars);
    }
}

impl Uniquify for Cut {
    fn uniquify(mut self, state: &mut UniquifyState) -> Cut {
        self.producer = self.producer.uniquify(state);
        self.consumer = self.consumer.uniquify(state);
        self
    }
}

impl Focusing for Cut {
    type Target = FsStatement;
    fn focus(self, used_vars: &mut HashSet<Ident>) -> FsStatement {
        match (
            Rc::unwrap_or_clone(self.producer),
            Rc::unwrap_or_clone(self.consumer),
        ) {
            // focus(⟨K(t_i) | c⟩) = bind(t_i)[λas.⟨K(as) | focus(c)⟩]
            (Term::Xtor(constructor), consumer) => bind_many(
                constructor.args.into(),
                Box::new(|bindings, used_vars: &mut HashSet<Ident>| {
                    FsCut::new(
                        FsXtor {
                            prdcns: constructor.prdcns,
                            id: constructor.id,
                            args: bindings.into(),
                            ty: self.ty.clone(),
                        },
                        consumer.focus(used_vars),
                        self.ty,
                    )
                    .into()
                }),
                used_vars,
            ),
            // focus(⟨p | D(t_i)⟩) = bind(t_i)[λas⟨ focus(p) | D(as)⟩]
            (producer, Term::Xtor(destructor)) => bind_many(
                destructor.args.into(),
                Box::new(|bindings, used_vars: &mut HashSet<Ident>| {
                    FsCut::new(
                        producer.focus(used_vars),
                        FsXtor {
                            prdcns: destructor.prdcns,
                            id: destructor.id,
                            args: bindings.into(),
                            ty: self.ty.clone(),
                        },
                        self.ty,
                    )
                    .into()
                }),
                used_vars,
            ),
            // focus(⟨ +(p_1, p_2) | c⟩) = bind(p_1)[λa1.bind(p_2)[λa_2.⟨ +(a_1, a_2) | focus(c)⟩]]
            (Term::Op(op), consumer) => Rc::unwrap_or_clone(op.fst).bind(
                Box::new(
                    |binding_fst: ContextBinding, used_vars: &mut HashSet<Ident>| {
                        Rc::unwrap_or_clone(op.snd).bind(
                            Box::new(|binding_snd, used_vars: &mut HashSet<Ident>| {
                                FsCut::new(
                                    FsOp {
                                        fst: binding_fst.var,
                                        op: op.op,
                                        snd: binding_snd.var,
                                    },
                                    consumer.focus(used_vars),
                                    self.ty,
                                )
                                .into()
                            }),
                            used_vars,
                        )
                    },
                ),
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

#[cfg(test)]
mod tests {
    use crate::syntax::*;
    use crate::traits::*;
    use core_macros::{
        bind, cns, covar, ctor, cut, dtor, fs_ctor, fs_cut, fs_dtor, fs_mutilde, id, lit, prd, ty,
        var,
    };
    extern crate self as core_lang;

    #[test]
    // this illustrates the problem
    fn transform_ctor() {
        let result = cut!(
            ctor!(
                id!("Cons"),
                [lit!(1), ctor!(id!("Nil"), [], ty!(id!("ListInt")))],
                ty!(id!("ListInt"))
            ),
            covar!(id!("a"), ty!(id!("ListInt"))),
            ty!(id!("ListInt"))
        )
        .focus(&mut Default::default());

        let expected = fs_cut!(
            lit!(1),
            fs_mutilde!(
                id!("x", 0),
                fs_cut!(
                    fs_ctor!(id!("Nil"), [], ty!(id!("ListInt"))),
                    fs_mutilde!(
                        id!("x", 1),
                        fs_cut!(
                            fs_ctor!(
                                id!("Cons"),
                                [
                                    bind!(id!("x", 0), prd!()),
                                    bind!(id!("x", 1), prd!(), ty!(id!("ListInt")))
                                ],
                                ty!(id!("ListInt"))
                            ),
                            covar!(id!("a"), ty!(id!("ListInt"))),
                            ty!(id!("ListInt"))
                        ),
                        ty!(id!("ListInt"))
                    ),
                    ty!(id!("ListInt"))
                )
            )
        )
        .into();

        assert_eq!(result, expected);
    }

    #[test]
    fn transform_dtor() {
        let result = cut!(
            var!(id!("x"), ty!(id!("Fun[i64, i64]"))),
            dtor!(
                id!("apply"),
                [var!(id!("y")), covar!(id!("a"))],
                ty!(id!("Fun[i64, i64]"))
            ),
            ty!(id!("Fun[i64, i64]"))
        )
        .focus(&mut Default::default());

        let expected = fs_cut!(
            var!(id!("x"), ty!(id!("Fun[i64, i64]"))),
            fs_dtor!(
                id!("apply"),
                [bind!(id!("y"), prd!()), bind!(id!("a"), cns!())],
                ty!(id!("Fun[i64, i64]"))
            ),
            ty!(id!("Fun[i64, i64]"))
        )
        .into();
        assert_eq!(result, expected);
    }

    #[test]
    fn transform_other() {
        let result = cut!(var!(id!("x")), covar!(id!("a"))).focus(&mut Default::default());
        let expected = FsCut::new(var!(id!("x")), covar!(id!("a")), Ty::I64).into();
        assert_eq!(result, expected);
    }
}
