//! This module defines the statements of Core.

use printer::Print;

use super::{
    ContextBinding, Covar, Var,
    terms::{Cns, Prd, Term},
    types::Ty,
};
use crate::traits::*;

use std::collections::{BTreeSet, HashSet};

pub mod call;
pub mod cut;
pub mod exit;
pub mod ifc;
pub mod print;

pub use call::*;
pub use cut::*;
pub use exit::*;
pub use ifc::*;
pub use print::*;

/// This enum defines the statements of Core. It contains one variant for each construct which
/// simply wraps the struct defining the corresponding construct.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    /// Cut between a producer and a consumer
    Cut(Cut),
    /// Conditional comparing two integers
    IfC(IfC),
    /// Printing an integer
    PrintI64(PrintI64),
    /// Call of a top-level function
    Call(Call),
    /// Exiting the program
    Exit(Exit),
}

impl Typed for Statement {
    fn get_type(&self) -> Ty {
        match self {
            Statement::Cut(cut) => cut.get_type(),
            Statement::IfC(ifc) => ifc.get_type(),
            Statement::PrintI64(print) => print.get_type(),
            Statement::Call(call) => call.get_type(),
            Statement::Exit(exit) => exit.get_type(),
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
            Statement::IfC(ifc) => ifc.print(cfg, alloc),
            Statement::PrintI64(print) => print.print(cfg, alloc),
            Statement::Call(call) => call.print(cfg, alloc),
            Statement::Exit(exit) => exit.print(cfg, alloc),
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
            Statement::IfC(ifc) => ifc.subst_sim(prod_subst, cons_subst).into(),
            Statement::PrintI64(print) => print.subst_sim(prod_subst, cons_subst).into(),
            Statement::Call(call) => call.subst_sim(prod_subst, cons_subst).into(),
            Statement::Exit(exit) => exit.subst_sim(prod_subst, cons_subst).into(),
        }
    }
}

impl TypedFreeVars for Statement {
    fn typed_free_vars(&self, vars: &mut BTreeSet<ContextBinding>) {
        match self {
            Statement::Cut(cut) => cut.typed_free_vars(vars),
            Statement::IfC(ifc) => ifc.typed_free_vars(vars),
            Statement::PrintI64(print) => print.typed_free_vars(vars),
            Statement::Call(call) => call.typed_free_vars(vars),
            Statement::Exit(exit) => exit.typed_free_vars(vars),
        }
    }
}

impl Uniquify for Statement {
    fn uniquify(self, seen_vars: &mut HashSet<Var>, used_vars: &mut HashSet<Var>) -> Statement {
        match self {
            Statement::Cut(cut) => cut.uniquify(seen_vars, used_vars).into(),
            Statement::IfC(ifc) => ifc.uniquify(seen_vars, used_vars).into(),
            Statement::PrintI64(print) => print.uniquify(seen_vars, used_vars).into(),
            Statement::Call(call) => call.uniquify(seen_vars, used_vars).into(),
            Statement::Exit(exit) => exit.uniquify(seen_vars, used_vars).into(),
        }
    }
}

impl Focusing for Statement {
    type Target = FsStatement;
    fn focus(self: Statement, used_vars: &mut HashSet<Var>) -> FsStatement {
        match self {
            Statement::Cut(cut) => cut.focus(used_vars),
            Statement::IfC(ifc) => ifc.focus(used_vars),
            Statement::PrintI64(print) => print.focus(used_vars),
            Statement::Call(call) => call.focus(used_vars),
            Statement::Exit(exit) => exit.focus(used_vars),
        }
    }
}

/// This struct defines the focused version of [`Statement`]s. In focused statements only
/// (co)variables can occur in argument positions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FsStatement {
    /// Cut between a producer and a consumer
    Cut(FsCut),
    /// Conditional comparing two integers
    IfC(FsIfC),
    /// Printing an integer
    PrintI64(FsPrintI64),
    /// Call of a top-level function
    Call(FsCall),
    /// Exiting the program
    Exit(FsExit),
}

impl Print for FsStatement {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        match self {
            FsStatement::Cut(cut) => cut.print(cfg, alloc),
            FsStatement::IfC(ifc) => ifc.print(cfg, alloc),
            FsStatement::PrintI64(print) => print.print(cfg, alloc),
            FsStatement::Call(call) => call.print(cfg, alloc),
            FsStatement::Exit(exit) => exit.print(cfg, alloc),
        }
    }
}

impl SubstVar for FsStatement {
    type Target = FsStatement;
    fn subst_sim(self, subst: &[(Var, Var)]) -> FsStatement {
        match self {
            FsStatement::Cut(cut) => cut.subst_sim(subst).into(),
            FsStatement::IfC(ifc) => ifc.subst_sim(subst).into(),
            FsStatement::PrintI64(print) => print.subst_sim(subst).into(),
            FsStatement::Call(call) => call.subst_sim(subst).into(),
            FsStatement::Exit(exit) => exit.subst_sim(subst).into(),
        }
    }
}

impl TypedFreeVars for FsStatement {
    fn typed_free_vars(&self, vars: &mut BTreeSet<ContextBinding>) {
        match self {
            FsStatement::Cut(cut) => cut.typed_free_vars(vars),
            FsStatement::IfC(ifc) => ifc.typed_free_vars(vars),
            FsStatement::PrintI64(print) => print.typed_free_vars(vars),
            FsStatement::Call(call) => call.typed_free_vars(vars),
            FsStatement::Exit(exit) => exit.typed_free_vars(vars),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        syntax::{Statement, arguments::Arguments, statements::IfSort, terms::XVar, types::Ty},
        test_common::example_subst,
        traits::*,
    };
    use std::rc::Rc;

    use super::{Call, Cut, IfC};

    fn example_cut() -> Statement {
        Cut::new(XVar::var("x", Ty::I64), XVar::covar("a", Ty::I64), Ty::I64).into()
    }

    fn example_ifz() -> Statement {
        IfC {
            sort: IfSort::Equal,
            fst: Rc::new(XVar::var("x", Ty::I64).into()),
            snd: None,
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
        let mut arguments = Arguments::default();
        arguments.add_prod(XVar::var("x", Ty::I64));
        arguments.add_cons(XVar::covar("a", Ty::I64));
        Call {
            name: "main".to_string(),
            args: arguments,
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
    fn subst_ifz() {
        let subst = example_subst();
        let result = example_ifz().subst_sim(&subst.0, &subst.1);
        let expected = IfC {
            sort: IfSort::Equal,
            fst: Rc::new(XVar::var("y", Ty::I64).into()),
            snd: None,
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
        let mut arguments = Arguments::default();
        arguments.add_prod(XVar::var("y", Ty::I64));
        arguments.add_cons(XVar::covar("b", Ty::I64));
        let expected = Call {
            name: "main".to_string(),
            args: arguments,
            ty: Ty::I64,
        }
        .into();
        assert_eq!(result, expected)
    }
}
