//! This module defines the unreachable statement in Core.

use printer::*;

use crate::mono::constraints::ConstraintCollector;
use crate::mono::constraints::FlowConstraintSet;
use crate::mono::errors::MonoError;
use crate::mono::specialize::Specialize;
use crate::mono::specialize::SpecializeContext;
use crate::splitting::labeling::{DeclSignatures, LabelAndUnify, SplitState};
use crate::splitting::rewrite::Rewrite;
use crate::splitting::split_table::SplitTable;
use crate::syntax::*;
use crate::traits::*;
use crate::typing::check::Checked;
use crate::typing::env::GlobalEnv;
use crate::typing::errors::LocatedTypeError;

/// This struct defines the unreachable statement in Core. It marks a match/comatch clause that
/// [`crate::mono::specialize::specialize_clause`]'s reachability narrowing found structurally
/// impossible for the equivalence class it was specializing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unreachable {
    pub ty: Ty,
}

impl Print for Unreachable {
    fn print<'a>(&'a self, _cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        alloc.keyword("unreachable")
    }
}

impl From<Unreachable> for Statement {
    fn from(value: Unreachable) -> Self {
        Statement::Unreachable(value)
    }
}

impl Typed for Unreachable {
    fn get_type(&self) -> Ty {
        self.ty.clone()
    }
}

impl Focusing for Unreachable {
    type Target = FsStatement;
    fn focus(self, _max_id: &mut ID) -> Self::Target {
        FsUnreachable.into()
    }
}

impl LabelAndUnify for Unreachable {
    fn label_and_unify(
        &self,
        state: &mut SplitState,
        _sigs: &DeclSignatures,
        _scope: &TypingContext,
    ) -> Self {
        Unreachable {
            ty: state.label_ty(&self.ty),
        }
    }
}

impl Rewrite for Unreachable {
    fn rewrite(&self, table: &SplitTable) -> Self {
        Unreachable {
            ty: self.ty.rewrite(table),
        }
    }
}

impl ConstraintCollector for Unreachable {
    fn collect_constraints(&self, _env: &GlobalEnv) -> Result<FlowConstraintSet, MonoError> {
        Ok(FlowConstraintSet::new())
    }
}

impl Specialize for Unreachable {
    fn specialize(&self, context: &SpecializeContext) -> Self {
        Unreachable {
            ty: self.ty.specialize(context),
        }
    }
}

impl Checked for Unreachable {
    fn check(
        &self,
        type_params: &[TypeParam],
        context: &TypingContext,
        env: &GlobalEnv,
    ) -> Result<(), LocatedTypeError> {
        self.ty.check(type_params, context, env)
    }
}

/// This struct defines the focused version of the [`Unreachable`] statement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FsUnreachable;

impl FsUnreachable {
    /// This function constructs an unreachable statement from a given type.
    #[allow(clippy::self_named_constructors)]
    pub fn unreachable() -> Self {
        FsUnreachable
    }
}

impl Print for FsUnreachable {
    fn print<'a>(&'a self, _cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        alloc.keyword("unreachable")
    }
}

impl From<FsUnreachable> for FsStatement {
    fn from(value: FsUnreachable) -> Self {
        FsStatement::Unreachable(value)
    }
}
