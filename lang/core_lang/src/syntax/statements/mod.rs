use printer::{theme::ThemeExt, tokens::DONE, Print};

use super::{
    terms::{Cns, Prd, Term},
    types::Ty,
    ContextBinding, Covar, Var,
};
use crate::traits::*;

use std::collections::{BTreeSet, HashSet};

mod call;
mod cut;
mod ifc;
mod ifz;
mod op;
mod print;

pub use call::*;
pub use cut::*;
pub use ifc::*;
pub use ifz::*;
pub use op::*;
pub use print::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    Cut(Cut),
    Op(Op),
    IfC(IfC),
    IfZ(IfZ),
    PrintI64(PrintI64),
    Call(Call),
    Done(Ty),
}

impl Typed for Statement {
    fn get_type(&self) -> Ty {
        match self {
            Statement::Cut(cut) => cut.get_type(),
            Statement::Op(op) => op.get_type(),
            Statement::IfC(ifc) => ifc.get_type(),
            Statement::IfZ(ifz) => ifz.get_type(),
            Statement::PrintI64(print) => print.get_type(),
            Statement::Call(call) => call.get_type(),
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
            Statement::PrintI64(print) => print.print(cfg, alloc),
            Statement::Call(call) => call.print(cfg, alloc),
            Statement::Done(_) => alloc.keyword(DONE),
        }
    }
}

impl Subst for Statement {
    type Target = Statement;
    fn subst_sim(
        self,
        prod_subst: &[(Var, Term<Prd>)],
        cons_subst: &[(Covar, Term<Cns>)],
    ) -> Statement {
        match self {
            Statement::Cut(cut) => cut.subst_sim(prod_subst, cons_subst).into(),
            Statement::Op(op) => op.subst_sim(prod_subst, cons_subst).into(),
            Statement::IfC(ifc) => ifc.subst_sim(prod_subst, cons_subst).into(),
            Statement::IfZ(ifz) => ifz.subst_sim(prod_subst, cons_subst).into(),
            Statement::PrintI64(print) => print.subst_sim(prod_subst, cons_subst).into(),
            Statement::Call(call) => call.subst_sim(prod_subst, cons_subst).into(),
            Statement::Done(ty) => Statement::Done(ty.clone()),
        }
    }
}

impl TypedFreeVars for Statement {
    fn typed_free_vars(&self, vars: &mut BTreeSet<ContextBinding>, state: &TypedFreeVarsState) {
        match self {
            Statement::Cut(cut) => cut.typed_free_vars(vars, state),
            Statement::Op(op) => op.typed_free_vars(vars, state),
            Statement::IfC(ifc) => ifc.typed_free_vars(vars, state),
            Statement::IfZ(ifz) => ifz.typed_free_vars(vars, state),
            Statement::PrintI64(print) => print.typed_free_vars(vars, state),
            Statement::Call(call) => call.typed_free_vars(vars, state),
            Statement::Done(_ty) => {}
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
            Statement::PrintI64(print) => print.uniquify(seen_vars, used_vars).into(),
            Statement::Call(call) => call.uniquify(seen_vars, used_vars).into(),
            Statement::Done(ref _ty) => self,
        }
    }
}

impl Focusing for Statement {
    type Target = FsStatement;
    fn focus(self: Statement, used_vars: &mut HashSet<Var>) -> FsStatement {
        match self {
            Statement::Cut(cut) => cut.focus(used_vars),
            Statement::Op(op) => op.focus(used_vars),
            Statement::IfC(ifc) => ifc.focus(used_vars),
            Statement::IfZ(ifz) => ifz.focus(used_vars),
            Statement::PrintI64(print) => print.focus(used_vars),
            Statement::Call(call) => call.focus(used_vars),
            Statement::Done(_) => FsStatement::Done(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FsStatement {
    Cut(FsCut),
    Op(FsOp),
    IfC(FsIfC),
    IfZ(FsIfZ),
    PrintI64(FsPrintI64),
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
            FsStatement::PrintI64(print) => print.print(cfg, alloc),
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
            FsStatement::PrintI64(print) => print.subst_sim(subst).into(),
            FsStatement::Call(call) => call.subst_sim(subst).into(),
            FsStatement::Done() => self,
        }
    }
}

impl TypedFreeVars for FsStatement {
    fn typed_free_vars(&self, vars: &mut BTreeSet<ContextBinding>, state: &TypedFreeVarsState) {
        match self {
            FsStatement::Cut(cut) => cut.typed_free_vars(vars, state),
            FsStatement::Op(op) => op.typed_free_vars(vars, state),
            FsStatement::IfC(ifc) => ifc.typed_free_vars(vars, state),
            FsStatement::IfZ(ifz) => ifz.typed_free_vars(vars, state),
            FsStatement::PrintI64(print) => print.typed_free_vars(vars, state),
            FsStatement::Call(call) => call.typed_free_vars(vars, state),
            FsStatement::Done() => {}
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        syntax::{
            statements::IfZSort, substitution::Substitution, terms::XVar, types::Ty, Statement,
        },
        test_common::example_subst,
        traits::*,
    };
    use std::rc::Rc;

    use super::{BinOp, Call, Cut, IfZ, Op};

    fn example_cut() -> Statement {
        Cut::new(XVar::var("x", Ty::I64), XVar::covar("a", Ty::I64), Ty::I64).into()
    }

    fn example_op() -> Statement {
        Op {
            fst: Rc::new(XVar::var("x", Ty::I64).into()),
            op: BinOp::Prod,
            snd: Rc::new(XVar::var("x", Ty::I64).into()),
            next: Rc::new(XVar::covar("a", Ty::I64).into()),
        }
        .into()
    }

    fn example_ifz() -> Statement {
        IfZ {
            sort: IfZSort::Equal,
            ifc: Rc::new(XVar::var("x", Ty::I64).into()),
            thenc: Rc::new(
                Cut::new(XVar::var("x", Ty::I64), XVar::covar("a", Ty::I64), Ty::I64).into(),
            ),
            elsec: Rc::new(
                Cut::new(XVar::var("x", Ty::I64), XVar::covar("a", Ty::I64), Ty::I64).into(),
            ),
        }
        .into()
    }

    fn example_call() -> Statement {
        let mut subst = Substitution::default();
        subst.add_prod(XVar::var("x", Ty::I64));
        subst.add_cons(XVar::covar("a", Ty::I64));
        Call {
            name: "main".to_string(),
            args: subst,
            ty: Ty::I64,
        }
        .into()
    }

    #[test]
    fn subst_cut() {
        let subst = example_subst();
        let result = example_cut().subst_sim(&subst.0, &subst.1);
        let expected = Cut::new(XVar::var("y", Ty::I64), XVar::covar("b", Ty::I64), Ty::I64).into();
        assert_eq!(result, expected)
    }
    #[test]
    fn subst_op() {
        let subst = example_subst();
        let result = example_op().subst_sim(&subst.0, &subst.1);
        let expected = Op {
            fst: Rc::new(XVar::var("y", Ty::I64).into()),
            op: BinOp::Prod,
            snd: Rc::new(XVar::var("y", Ty::I64).into()),
            next: Rc::new(XVar::covar("b", Ty::I64).into()),
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_ifz() {
        let subst = example_subst();
        let result = example_ifz().subst_sim(&subst.0, &subst.1);
        let expected = IfZ {
            sort: IfZSort::Equal,
            ifc: Rc::new(XVar::var("y", Ty::I64).into()),
            thenc: Rc::new(
                Cut::new(XVar::var("y", Ty::I64), XVar::covar("b", Ty::I64), Ty::I64).into(),
            ),
            elsec: Rc::new(
                Cut::new(XVar::var("y", Ty::I64), XVar::covar("b", Ty::I64), Ty::I64).into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_call() {
        let subst = example_subst();
        let result = example_call().subst_sim(&subst.0, &subst.1);
        let mut substitution = Substitution::default();
        substitution.add_prod(XVar::var("y", Ty::I64));
        substitution.add_cons(XVar::covar("b", Ty::I64));
        let expected = Call {
            name: "main".to_string(),
            args: substitution,
            ty: Ty::I64,
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_done() {
        let subst = example_subst();
        let result = Statement::Done(Ty::I64).subst_sim(&subst.0, &subst.1);
        let expected = Statement::Done(Ty::I64);
        assert_eq!(result, expected)
    }
}
