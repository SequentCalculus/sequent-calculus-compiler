use core_lang::syntax::statements::FsStatement;

use crate::shrinking::{Shrinking, ShrinkingState};

pub mod call;
pub mod clause;
pub mod cut;
pub mod exit;
pub mod ifc;
pub mod ifz;
pub mod print;

impl Shrinking for FsStatement {
    type Target = axcut::syntax::Statement;

    fn shrink(self, state: &mut ShrinkingState) -> axcut::syntax::Statement {
        match self {
            FsStatement::Cut(cut) => cut.shrink(state),
            FsStatement::IfC(ifc) => ifc.shrink(state),
            FsStatement::IfZ(ifz) => ifz.shrink(state),
            FsStatement::PrintI64(print) => print.shrink(state),
            FsStatement::Call(call) => call.shrink(state),
            FsStatement::Exit(exit) => exit.shrink(state),
        }
    }
}
