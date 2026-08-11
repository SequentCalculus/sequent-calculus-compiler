use printer::*;

use crate::syntax::*;
use crate::traits::*;

/// This struct defines the unreachable statement in Core.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unreachable {
    pub(crate) ty: Ty,
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
