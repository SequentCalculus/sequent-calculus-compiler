use printer::{theme::ThemeExt, tokens::DONE, Print};

use super::{
    term::{Cns, Prd, Term},
    types::Ty,
    Covar, Var,
};
use crate::traits::*;

use std::collections::HashSet;

pub mod cut;
pub mod fun;
pub mod ifc;
pub mod ifz;
pub mod op;

pub use cut::*;
pub use fun::*;
pub use ifc::*;
pub use ifz::*;
pub use op::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    Cut(Cut),
    Op(Op),
    IfC(IfC),
    IfZ(IfZ),
    Fun(Fun),
    Done(Ty),
}

impl Typed for Statement {
    fn get_type(&self) -> Ty {
        match self {
            Statement::Cut(cut) => cut.get_type(),
            Statement::Op(op) => op.get_type(),
            Statement::IfC(ifc) => ifc.get_type(),
            Statement::IfZ(ifz) => ifz.get_type(),
            Statement::Fun(call) => call.get_type(),
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
            Statement::IfC(ifc) => ifc.print(cfg, alloc),
            Statement::IfZ(ifz) => ifz.print(cfg, alloc),
            Statement::Fun(call) => call.print(cfg, alloc),
            Statement::Done(_) => alloc.keyword(DONE),
        }
    }
}

impl FreeV for Statement {
    fn free_vars(self: &Statement) -> HashSet<Var> {
        match self {
            Statement::Cut(cut) => cut.free_vars(),
            Statement::Op(op) => op.free_vars(),
            Statement::IfC(ifc) => ifc.free_vars(),
            Statement::IfZ(ifz) => ifz.free_vars(),
            Statement::Fun(call) => call.free_vars(),
            Statement::Done(_) => HashSet::new(),
        }
    }
    fn free_covars(self: &Statement) -> HashSet<Covar> {
        match self {
            Statement::Cut(cut) => cut.free_covars(),
            Statement::Op(op) => op.free_covars(),
            Statement::IfC(ifc) => ifc.free_covars(),
            Statement::IfZ(ifz) => ifz.free_covars(),
            Statement::Fun(call) => call.free_covars(),
            Statement::Done(_) => HashSet::new(),
        }
    }
}

impl UsedBinders for Statement {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        match self {
            Statement::Cut(cut) => cut.used_binders(used),
            Statement::Op(op) => op.used_binders(used),
            Statement::IfC(ifc) => ifc.used_binders(used),
            Statement::IfZ(ifz) => ifz.used_binders(used),
            Statement::Fun(call) => call.used_binders(used),
            Statement::Done(_) => {}
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
            Statement::Cut(cut) => cut.subst_sim(prod_subst, cons_subst).into(),
            Statement::Op(op) => op.subst_sim(prod_subst, cons_subst).into(),
            Statement::IfC(ifc) => ifc.subst_sim(prod_subst, cons_subst).into(),
            Statement::IfZ(ifz) => ifz.subst_sim(prod_subst, cons_subst).into(),
            Statement::Fun(call) => call.subst_sim(prod_subst, cons_subst).into(),
            Statement::Done(ty) => Statement::Done(ty.clone()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FsStatement {
    Cut(FsCut),
    Op(FsOp),
    IfC(FsIfC),
    IfZ(FsIfZ),
    Call(FsCall),
    Done(),
}

impl Print for FsStatement {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        match self {
            FsStatement::Cut(cut) => cut.print(cfg, alloc),
            FsStatement::Op(op) => op.print(cfg, alloc),
            FsStatement::IfC(ifc) => ifc.print(cfg, alloc),
            FsStatement::IfZ(ifz) => ifz.print(cfg, alloc),
            FsStatement::Call(call) => call.print(cfg, alloc),
            FsStatement::Done() => alloc.keyword(DONE),
        }
    }
}

impl SubstVar for FsStatement {
    type Target = FsStatement;
    fn subst_sim(self, subst: &[(Var, Var)]) -> FsStatement {
        match self {
            FsStatement::Cut(cut) => cut.subst_sim(subst).into(),
            FsStatement::Op(op) => op.subst_sim(subst).into(),
            FsStatement::IfC(ifc) => ifc.subst_sim(subst).into(),
            FsStatement::IfZ(ifz) => ifz.subst_sim(subst).into(),
            FsStatement::Call(call) => call.subst_sim(subst).into(),
            FsStatement::Done() => FsStatement::Done(),
        }
    }
}

impl Uniquify for Statement {
    fn uniquify(self, seen_vars: &mut HashSet<Var>, used_vars: &mut HashSet<Var>) -> Statement {
        match self {
            Statement::Cut(cut) => cut.uniquify(seen_vars, used_vars).into(),
            Statement::Op(op) => op.uniquify(seen_vars, used_vars).into(),
            Statement::IfC(ifc) => ifc.uniquify(seen_vars, used_vars).into(),
            Statement::IfZ(ifz) => ifz.uniquify(seen_vars, used_vars).into(),
            Statement::Fun(call) => call.uniquify(seen_vars, used_vars).into(),
            Statement::Done(ty) => Statement::Done(ty),
        }
    }
}

impl Focusing for Statement {
    type Target = crate::syntax::statement::FsStatement;
    fn focus(self: Statement, state: &mut FocusingState) -> crate::syntax::statement::FsStatement {
        match self {
            Statement::Cut(cut) => cut.focus(state),
            Statement::Op(op) => op.focus(state),
            Statement::IfC(ifc) => ifc.focus(state),
            Statement::IfZ(ifz) => ifz.focus(state),
            Statement::Fun(call) => call.focus(state),
            Statement::Done(_) => crate::syntax::statement::FsStatement::Done(),
        }
    }
}

#[cfg(test)]
mod test {
    use printer::Print;

    use crate::{
        syntax::{
            substitution::SubstitutionBinding,
            term::{Cns, Prd, Term, XVar},
            types::Ty,
            Covar, Statement, Var,
        },
        traits::*,
    };
    use std::{collections::HashSet, rc::Rc};

    use super::{BinOp, Cut, Fun, IfZ, Op};

    fn example_cut() -> Statement {
        Cut {
            producer: Rc::new(
                XVar {
                    prdcns: Prd,
                    var: "x".to_owned(),
                    ty: Ty::Int,
                }
                .into(),
            ),
            ty: Ty::Int,
            consumer: Rc::new(
                XVar {
                    prdcns: Cns,
                    var: "a".to_owned(),
                    ty: Ty::Int,
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
                    ty: Ty::Int,
                }
                .into(),
            ),
            op: BinOp::Prod,
            snd: Rc::new(
                XVar {
                    prdcns: Prd,
                    var: "x".to_owned(),
                    ty: Ty::Int,
                }
                .into(),
            ),
            continuation: Rc::new(
                XVar {
                    prdcns: Cns,
                    var: "a".to_owned(),
                    ty: Ty::Int,
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
                    ty: Ty::Int,
                }
                .into(),
            ),
            thenc: Rc::new(
                Cut {
                    producer: Rc::new(
                        XVar {
                            prdcns: Prd,
                            var: "x".to_owned(),
                            ty: Ty::Int,
                        }
                        .into(),
                    ),
                    ty: Ty::Int,
                    consumer: Rc::new(
                        XVar {
                            prdcns: Cns,
                            var: "a".to_owned(),
                            ty: Ty::Int,
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
                            ty: Ty::Int,
                        }
                        .into(),
                    ),
                    ty: Ty::Int,
                    consumer: Rc::new(
                        XVar {
                            prdcns: Cns,
                            var: "a".to_owned(),
                            ty: Ty::Int,
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
                        ty: Ty::Int,
                    }
                    .into(),
                ),
                SubstitutionBinding::ConsumerBinding(
                    XVar {
                        prdcns: Cns,
                        var: "a".to_owned(),
                        ty: Ty::Int,
                    }
                    .into(),
                ),
            ],
            ty: Ty::Int,
        }
        .into()
    }

    fn example_prodsubst() -> Vec<(Term<Prd>, Var)> {
        vec![(
            XVar {
                prdcns: Prd,
                var: "y".to_owned(),
                ty: Ty::Int,
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
                ty: Ty::Int,
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
        let expected = "ifz(x; <x | 'a>, <x | 'a>)".to_owned();
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
        let result = Statement::Done(Ty::Int).print_to_string(None);
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
        let result = Statement::Done(Ty::Int).free_vars();
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
        let result = Statement::Done(Ty::Int).free_covars();
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
                    ty: Ty::Int,
                }
                .into(),
            ),
            ty: Ty::Int,
            consumer: Rc::new(
                XVar {
                    prdcns: Cns,
                    var: "b".to_owned(),
                    ty: Ty::Int,
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
                    ty: Ty::Int,
                }
                .into(),
            ),
            op: BinOp::Prod,
            snd: Rc::new(
                XVar {
                    prdcns: Prd,
                    var: "y".to_owned(),
                    ty: Ty::Int,
                }
                .into(),
            ),
            continuation: Rc::new(
                XVar {
                    prdcns: Cns,
                    var: "b".to_owned(),
                    ty: Ty::Int,
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
                    ty: Ty::Int,
                }
                .into(),
            ),
            thenc: Rc::new(
                Cut {
                    producer: Rc::new(
                        XVar {
                            prdcns: Prd,
                            var: "y".to_owned(),
                            ty: Ty::Int,
                        }
                        .into(),
                    ),
                    ty: Ty::Int,
                    consumer: Rc::new(
                        XVar {
                            prdcns: Cns,
                            var: "b".to_owned(),
                            ty: Ty::Int,
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
                            ty: Ty::Int,
                        }
                        .into(),
                    ),
                    ty: Ty::Int,
                    consumer: Rc::new(
                        XVar {
                            prdcns: Cns,
                            var: "b".to_owned(),
                            ty: Ty::Int,
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
                        ty: Ty::Int,
                    }
                    .into(),
                ),
                SubstitutionBinding::ConsumerBinding(
                    XVar {
                        prdcns: Cns,
                        var: "b".to_owned(),
                        ty: Ty::Int,
                    }
                    .into(),
                ),
            ],
            ty: Ty::Int,
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_done() {
        let result = Statement::Done(Ty::Int).subst_sim(&example_prodsubst(), &example_conssubst());
        let expected = Statement::Done(Ty::Int);
        assert_eq!(result, expected)
    }
}
