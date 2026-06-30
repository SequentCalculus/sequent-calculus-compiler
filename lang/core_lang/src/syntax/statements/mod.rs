//! This module defines the statements of Core.

use printer::*;

use crate::mono::constraints::ConstraintCollector;
use crate::mono::constraints::FlowConstraintSet;
use crate::mono::errors::MonoError;
use crate::mono::specialize::Specialize;
use crate::syntax::*;
use crate::traits::*;
use crate::typing::check::Checked;
use crate::typing::env::GlobalEnv;
use crate::typing::errors::LocatedTypeError;

use std::collections::BTreeSet;

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
    fn print<'a>(&'a self, cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
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
        prod_subst: &[(Identifier, Term<Prd>)],
        cons_subst: &[(Identifier, Term<Cns>)],
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
    fn uniquify(self, max_id: &mut ID) -> Statement {
        match self {
            Statement::Cut(cut) => cut.uniquify(max_id).into(),
            Statement::IfC(ifc) => ifc.uniquify(max_id).into(),
            Statement::PrintI64(print) => print.uniquify(max_id).into(),
            Statement::Call(call) => call.uniquify(max_id).into(),
            Statement::Exit(exit) => exit.uniquify(max_id).into(),
        }
    }
}

impl Focusing for Statement {
    type Target = FsStatement;
    fn focus(self: Statement, max_id: &mut ID) -> FsStatement {
        match self {
            Statement::Cut(cut) => cut.focus(max_id),
            Statement::IfC(ifc) => ifc.focus(max_id),
            Statement::PrintI64(print) => print.focus(max_id),
            Statement::Call(call) => call.focus(max_id),
            Statement::Exit(exit) => exit.focus(max_id),
        }
    }
}

impl ConstraintCollector for Statement {
    fn collect_constraints(
        &self,
        data_declarations: &[DataDeclaration],
        codata_declarations: &[CodataDeclaration],
    ) -> Result<FlowConstraintSet, MonoError> {
        let constraints = match self {
            Statement::Cut(cut) => {
                cut.collect_constraints(data_declarations, codata_declarations)?
            }
            Statement::IfC(ifc) => {
                ifc.collect_constraints(data_declarations, codata_declarations)?
            }
            Statement::PrintI64(print) => {
                print.collect_constraints(data_declarations, codata_declarations)?
            }
            Statement::Call(call) => {
                call.collect_constraints(data_declarations, codata_declarations)?
            }
            Statement::Exit(exit) => {
                exit.collect_constraints(data_declarations, codata_declarations)?
            }
        };
        Ok(constraints)
    }
}

impl Specialize for Statement {
    fn specialize(&self, context: crate::mono::specialize::SpecializeContext) -> Self {
        match self {
            Statement::Cut(cut) => cut.specialize(context).into(),
            Statement::IfC(ifc) => ifc.specialize(context).into(),
            Statement::PrintI64(print) => print.specialize(context).into(),
            Statement::Call(call) => call.specialize(context).into(),
            Statement::Exit(exit) => exit.specialize(context).into(),
        }
    }
}

impl Checked for Statement {
    fn check(
        &self,
        type_params: &[Identifier],
        context: &TypingContext,
        env: &GlobalEnv,
    ) -> Result<(), LocatedTypeError> {
        match self {
            Statement::Cut(cut) => cut.check(type_params, context, env),
            Statement::IfC(ifc) => ifc.check(type_params, context, env),
            Statement::PrintI64(print) => print.check(type_params, context, env),
            Statement::Call(call) => call.check(type_params, context, env),
            Statement::Exit(exit) => exit.check(type_params, context, env),
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
    fn print<'a>(&'a self, cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
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
    fn subst_sim(self, subst: &[(ID, Identifier)]) -> FsStatement {
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
    use crate::syntax::*;
    use crate::test_common::example_subst;
    use crate::traits::*;
    extern crate self as core_lang;
    use core_macros::{call, covar, cut, id, ife, var};

    fn example_cut() -> Statement {
        cut!(var!(id!("x")), covar!(id!("a"))).into()
    }

    fn example_ifz() -> Statement {
        ife!(
            var!(id!("x")),
            cut!(var!(id!("x")), covar!(id!("a"))),
            cut!(var!(id!("x")), covar!(id!("a")))
        )
        .into()
    }

    fn example_call() -> Statement {
        call!(id!("main"), [var!(id!("x")), covar!(id!("a"))]).into()
    }

    #[test]
    fn subst_cut() {
        let subst = example_subst();
        let result = example_cut().subst_sim(&subst.0, &subst.1);
        let expected = cut!(var!(id!("y")), covar!(id!("b"))).into();
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_ifz() {
        let subst = example_subst();
        let result = example_ifz().subst_sim(&subst.0, &subst.1);
        let expected = ife!(
            var!(id!("y")),
            cut!(var!(id!("y")), covar!(id!("b"))),
            cut!(var!(id!("y")), covar!(id!("b")))
        )
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_call() {
        let subst = example_subst();
        let result = example_call().subst_sim(&subst.0, &subst.1);
        let expected = call!(id!("main"), [var!(id!("y")), covar!(id!("b"))]).into();
        assert_eq!(result, expected)
    }
}
