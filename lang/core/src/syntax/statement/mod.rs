use super::{
    term::{Cns, Prd, Term},
    types::{Ty, Typed},
    Covar, Var,
};
use crate::traits::{
    focus::{Focusing, FocusingState},
    free_vars::FreeV,
    substitution::Subst,
};
use std::collections::HashSet;

pub mod cut;
pub mod fun;
pub mod ifz;
pub mod op;

pub use cut::*;
pub use fun::*;
pub use ifz::*;
pub use op::*;
use printer::{tokens::DONE, DocAllocator, Print};

// Statement
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    Cut(Cut),
    Op(Op),
    IfZ(IfZ),
    Fun(Fun),
    Done(Ty),
}

impl Typed for Statement {
    fn get_type(&self) -> Ty {
        match self {
            Statement::Cut(cut) => cut.get_type(),
            Statement::Op(op) => op.get_type(),
            Statement::IfZ(ifz) => ifz.get_type(),
            Statement::Fun(fun) => fun.get_type(),
            Statement::Done(ty) => ty.clone(),
        }
    }
}

impl Print for Statement {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        match self {
            Statement::Cut(cut) => cut.print(cfg, alloc),
            Statement::Op(op) => op.print(cfg, alloc),
            Statement::IfZ(if_z) => if_z.print(cfg, alloc),
            Statement::Fun(fun) => fun.print(cfg, alloc),
            Statement::Done(_) => alloc.text(DONE),
        }
    }
}

impl FreeV for Statement {
    fn free_vars(self: &Statement) -> HashSet<Var> {
        match self {
            Statement::Cut(c) => c.free_vars(),
            Statement::Op(op) => op.free_vars(),
            Statement::IfZ(i) => i.free_vars(),
            Statement::Fun(f) => f.free_vars(),
            Statement::Done(_) => HashSet::new(),
        }
    }
    fn free_covars(self: &Statement) -> HashSet<Covar> {
        match self {
            Statement::Cut(c) => c.free_covars(),
            Statement::Op(op) => op.free_covars(),
            Statement::IfZ(i) => i.free_covars(),
            Statement::Fun(f) => f.free_covars(),
            Statement::Done(_) => HashSet::new(),
        }
    }
}

impl Subst for Statement {
    type Target = Statement;
    fn subst_sim(
        self: &Statement,
        prod_subst: &[(Term<Prd>, Var)],
        cons_subst: &[(Term<Cns>, Covar)],
    ) -> Statement {
        match self {
            Statement::Cut(c) => c.subst_sim(prod_subst, cons_subst).into(),
            Statement::Op(o) => o.subst_sim(prod_subst, cons_subst).into(),
            Statement::IfZ(i) => i.subst_sim(prod_subst, cons_subst).into(),
            Statement::Fun(f) => f.subst_sim(prod_subst, cons_subst).into(),
            Statement::Done(ty) => Statement::Done(ty.clone()),
        }
    }
}

impl Focusing for Statement {
    type Target = Statement;
    fn focus(self: Statement, state: &mut FocusingState) -> Statement {
        match self {
            Statement::Cut(cut) => cut.focus(state),
            Statement::Op(op) => op.focus(state),
            Statement::IfZ(ifz) => ifz.focus(state),
            Statement::Fun(fun) => fun.focus(state),
            Statement::Done(ty) => Statement::Done(ty),
        }
    }
}

#[cfg(test)]
mod statement_tests {
    use super::Focusing;
    use crate::syntax::{
        statement::{Cut, Fun, IfZ, Op},
        substitution::SubstitutionBinding,
        term::{Cns, Literal, Prd, XVar},
        types::Ty,
        BinOp, Statement,
    };
    use std::rc::Rc;

    fn example_cut() -> Cut {
        Cut {
            producer: Rc::new(
                XVar {
                    prdcns: Prd,
                    var: "x".to_owned(),
                    ty: Ty::Int(),
                }
                .into(),
            ),
            ty: Ty::Int(),
            consumer: Rc::new(
                XVar {
                    prdcns: Cns,
                    var: "a".to_owned(),
                    ty: Ty::Int(),
                }
                .into(),
            ),
        }
    }
    fn example_op() -> Op {
        Op {
            fst: Rc::new(Literal { lit: 1 }.into()),
            op: BinOp::Prod,
            snd: Rc::new(Literal { lit: 2 }.into()),
            continuation: Rc::new(
                XVar {
                    prdcns: Cns,
                    var: "a".to_owned(),
                    ty: Ty::Int(),
                }
                .into(),
            ),
        }
    }

    fn example_ifz() -> IfZ {
        IfZ {
            ifc: Rc::new(Literal { lit: 0 }.into()),
            thenc: Rc::new(Statement::Done(Ty::Int())),
            elsec: Rc::new(Statement::Done(Ty::Int())),
        }
    }

    fn example_fun() -> Fun {
        Fun {
            name: "multFast".to_owned(),
            args: vec![
                SubstitutionBinding::ProducerBinding(
                    XVar {
                        prdcns: Prd,
                        var: "x".to_owned(),
                        ty: Ty::Decl("ListInt".to_owned()),
                    }
                    .into(),
                ),
                SubstitutionBinding::ConsumerBinding(
                    XVar {
                        prdcns: Cns,
                        var: "a".to_owned(),
                        ty: Ty::Int(),
                    }
                    .into(),
                ),
            ],
            ty: Ty::Int(),
        }
    }

    fn example_done() -> Statement {
        Statement::Done(Ty::Int())
    }

    #[test]
    fn transform_cut() {
        let result = <Cut as Into<Statement>>::into(example_cut()).focus(&mut Default::default());
        let expected = example_cut().focus(&mut Default::default());
        assert_eq!(result, expected)
    }

    #[test]
    fn transform_op() {
        let result = <Op as Into<Statement>>::into(example_op()).focus(&mut Default::default());
        let expected = example_op().focus(&mut Default::default());
        assert_eq!(result, expected)
    }

    #[test]
    fn transform_ifz() {
        let result = <IfZ as Into<Statement>>::into(example_ifz()).focus(&mut Default::default());
        let expected = example_ifz().focus(&mut Default::default());
        assert_eq!(result, expected)
    }

    #[test]
    fn transform_fun() {
        let result = <Fun as Into<Statement>>::into(example_fun()).focus(&mut Default::default());
        let expected = example_fun().focus(&mut Default::default());
        assert_eq!(result, expected)
    }

    #[test]
    fn transform_done() {
        let result = example_done().focus(&mut Default::default());
        let expected = Statement::Done(Ty::Int());
        assert_eq!(result, expected)
    }
}

#[cfg(test)]
mod statement_tests2 {
    use printer::Print;

    use crate::{
        syntax::{
            substitution::SubstitutionBinding,
            term::{Cns, Prd, Term, XVar},
            types::Ty,
            BinOp, Covar, Statement, Var,
        },
        traits::{free_vars::FreeV, substitution::Subst},
    };
    use std::{collections::HashSet, rc::Rc};

    use super::{Cut, Fun, IfZ, Op};

    fn example_cut() -> Statement {
        Cut {
            producer: Rc::new(
                XVar {
                    prdcns: Prd,
                    var: "x".to_owned(),
                    ty: Ty::Int(),
                }
                .into(),
            ),
            ty: Ty::Int(),
            consumer: Rc::new(
                XVar {
                    prdcns: Cns,
                    var: "a".to_owned(),
                    ty: Ty::Int(),
                }
                .into(),
            ),
        }
        .into()
    }

    fn example_op() -> Statement {
        Op {
            fst: Rc::new(
                XVar {
                    prdcns: Prd,
                    var: "x".to_owned(),
                    ty: Ty::Int(),
                }
                .into(),
            ),
            op: BinOp::Prod,
            snd: Rc::new(
                XVar {
                    prdcns: Prd,
                    var: "x".to_owned(),
                    ty: Ty::Int(),
                }
                .into(),
            ),
            continuation: Rc::new(
                XVar {
                    prdcns: Cns,
                    var: "a".to_owned(),
                    ty: Ty::Int(),
                }
                .into(),
            ),
        }
        .into()
    }

    fn example_ifz() -> Statement {
        IfZ {
            ifc: Rc::new(
                XVar {
                    prdcns: Prd,
                    var: "x".to_owned(),
                    ty: Ty::Int(),
                }
                .into(),
            ),
            thenc: Rc::new(
                Cut {
                    producer: Rc::new(
                        XVar {
                            prdcns: Prd,
                            var: "x".to_owned(),
                            ty: Ty::Int(),
                        }
                        .into(),
                    ),
                    ty: Ty::Int(),
                    consumer: Rc::new(
                        XVar {
                            prdcns: Cns,
                            var: "a".to_owned(),
                            ty: Ty::Int(),
                        }
                        .into(),
                    ),
                }
                .into(),
            ),
            elsec: Rc::new(
                Cut {
                    producer: Rc::new(
                        XVar {
                            prdcns: Prd,
                            var: "x".to_owned(),
                            ty: Ty::Int(),
                        }
                        .into(),
                    ),
                    ty: Ty::Int(),
                    consumer: Rc::new(
                        XVar {
                            prdcns: Cns,
                            var: "a".to_owned(),
                            ty: Ty::Int(),
                        }
                        .into(),
                    ),
                }
                .into(),
            ),
        }
        .into()
    }

    fn example_fun() -> Statement {
        Fun {
            name: "main".to_owned(),
            args: vec![
                SubstitutionBinding::ProducerBinding(
                    XVar {
                        prdcns: Prd,
                        var: "x".to_owned(),
                        ty: Ty::Int(),
                    }
                    .into(),
                ),
                SubstitutionBinding::ConsumerBinding(
                    XVar {
                        prdcns: Cns,
                        var: "a".to_owned(),
                        ty: Ty::Int(),
                    }
                    .into(),
                ),
            ],
            ty: Ty::Int(),
        }
        .into()
    }

    fn example_prodsubst() -> Vec<(Term<Prd>, Var)> {
        vec![(
            XVar {
                prdcns: Prd,
                var: "y".to_owned(),
                ty: Ty::Int(),
            }
            .into(),
            "x".to_owned(),
        )]
    }

    fn example_conssubst() -> Vec<(Term<Cns>, Covar)> {
        vec![(
            XVar {
                prdcns: Cns,
                var: "b".to_owned(),
                ty: Ty::Int(),
            }
            .into(),
            "a".to_owned(),
        )]
    }

    #[test]
    fn display_cut() {
        let result = example_cut().print_to_string(None);
        let expected = "<x | 'a>".to_owned();
        assert_eq!(result, expected)
    }

    #[test]
    fn display_op() {
        let result = example_op().print_to_string(None);
        let expected = "*(x, x; 'a)".to_owned();
        assert_eq!(result, expected)
    }

    #[test]
    fn display_ifz() {
        let result = example_ifz().print_to_string(None);
        let expected = "IfZ(x; <x | 'a>, <x | 'a>)".to_owned();
        assert_eq!(result, expected)
    }

    #[test]
    fn display_fun() {
        let result = example_fun().print_to_string(None);
        let expected = "main(x, 'a)";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_done() {
        let result = Statement::Done(Ty::Int()).print_to_string(None);
        let expected = "Done".to_owned();
        assert_eq!(result, expected)
    }

    #[test]
    fn free_vars_cut() {
        let result = example_cut().free_vars();
        let expected = HashSet::from(["x".to_owned()]);
        assert_eq!(result, expected)
    }

    #[test]
    fn free_vars_op() {
        let result = example_op().free_vars();
        let expected = HashSet::from(["x".to_owned()]);
        assert_eq!(result, expected)
    }

    #[test]
    fn free_vars_ifz() {
        let result = example_ifz().free_vars();
        let expected = HashSet::from(["x".to_owned()]);
        assert_eq!(result, expected)
    }

    #[test]
    fn free_vars_fun() {
        let result = example_fun().free_vars();
        let expected = HashSet::from(["x".to_owned()]);
        assert_eq!(result, expected)
    }

    #[test]
    fn free_vars_done() {
        let result = Statement::Done(Ty::Int()).free_vars();
        let expected = HashSet::new();
        assert_eq!(result, expected)
    }

    #[test]
    fn free_covars_cut() {
        let result = example_cut().free_covars();
        let expected = HashSet::from(["a".to_owned()]);
        assert_eq!(result, expected)
    }

    #[test]
    fn free_covars_op() {
        let result = example_op().free_covars();
        let expected = HashSet::from(["a".to_owned()]);
        assert_eq!(result, expected)
    }

    #[test]
    fn free_covars_ifz() {
        let result = example_ifz().free_covars();
        let expected = HashSet::from(["a".to_owned()]);
        assert_eq!(result, expected)
    }

    #[test]
    fn free_covars_fun() {
        let result = example_fun().free_covars();
        let expected = HashSet::from(["a".to_owned()]);
        assert_eq!(result, expected)
    }

    #[test]
    fn free_covars_done() {
        let result = Statement::Done(Ty::Int()).free_covars();
        let expected = HashSet::new();
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_cut() {
        let result = example_cut().subst_sim(&example_prodsubst(), &example_conssubst());
        let expected = Cut {
            producer: Rc::new(
                XVar {
                    prdcns: Prd,
                    var: "y".to_owned(),
                    ty: Ty::Int(),
                }
                .into(),
            ),
            ty: Ty::Int(),
            consumer: Rc::new(
                XVar {
                    prdcns: Cns,
                    var: "b".to_owned(),
                    ty: Ty::Int(),
                }
                .into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }
    #[test]
    fn subst_op() {
        let result = example_op().subst_sim(&example_prodsubst(), &example_conssubst());
        let expected = Op {
            fst: Rc::new(
                XVar {
                    prdcns: Prd,
                    var: "y".to_owned(),
                    ty: Ty::Int(),
                }
                .into(),
            ),
            op: BinOp::Prod,
            snd: Rc::new(
                XVar {
                    prdcns: Prd,
                    var: "y".to_owned(),
                    ty: Ty::Int(),
                }
                .into(),
            ),
            continuation: Rc::new(
                XVar {
                    prdcns: Cns,
                    var: "b".to_owned(),
                    ty: Ty::Int(),
                }
                .into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_ifz() {
        let result = example_ifz().subst_sim(&example_prodsubst(), &example_conssubst());
        let expected = IfZ {
            ifc: Rc::new(
                XVar {
                    prdcns: Prd,
                    var: "y".to_owned(),
                    ty: Ty::Int(),
                }
                .into(),
            ),
            thenc: Rc::new(
                Cut {
                    producer: Rc::new(
                        XVar {
                            prdcns: Prd,
                            var: "y".to_owned(),
                            ty: Ty::Int(),
                        }
                        .into(),
                    ),
                    ty: Ty::Int(),
                    consumer: Rc::new(
                        XVar {
                            prdcns: Cns,
                            var: "b".to_owned(),
                            ty: Ty::Int(),
                        }
                        .into(),
                    ),
                }
                .into(),
            ),
            elsec: Rc::new(
                Cut {
                    producer: Rc::new(
                        XVar {
                            prdcns: Prd,
                            var: "y".to_owned(),
                            ty: Ty::Int(),
                        }
                        .into(),
                    ),
                    ty: Ty::Int(),
                    consumer: Rc::new(
                        XVar {
                            prdcns: Cns,
                            var: "b".to_owned(),
                            ty: Ty::Int(),
                        }
                        .into(),
                    ),
                }
                .into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_fun() {
        let result = example_fun().subst_sim(&example_prodsubst(), &example_conssubst());
        let expected = Fun {
            name: "main".to_owned(),
            args: vec![
                SubstitutionBinding::ProducerBinding(
                    XVar {
                        prdcns: Prd,
                        var: "y".to_owned(),
                        ty: Ty::Int(),
                    }
                    .into(),
                ),
                SubstitutionBinding::ConsumerBinding(
                    XVar {
                        prdcns: Cns,
                        var: "b".to_owned(),
                        ty: Ty::Int(),
                    }
                    .into(),
                ),
            ],
            ty: Ty::Int(),
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_done() {
        let result =
            Statement::Done(Ty::Int()).subst_sim(&example_prodsubst(), &example_conssubst());
        let expected = Statement::Done(Ty::Int());
        assert_eq!(result, expected)
    }
}
