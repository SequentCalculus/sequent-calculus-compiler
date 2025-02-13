use crate::traits::{Shrinking, ShrinkingState};
use core_lang::syntax::statement::FsStatement;

pub mod call;
pub mod cut;
pub mod ifc;
pub mod ifz;
pub mod op;
pub mod print;

impl Shrinking for FsStatement {
    type Target = axcut::syntax::Statement;

    fn shrink(self, state: &mut ShrinkingState) -> axcut::syntax::Statement {
        match self {
            FsStatement::Cut(cut) => cut.shrink(state),
            FsStatement::Op(op) => op.shrink(state),
            FsStatement::IfC(ifc) => ifc.shrink(state),
            FsStatement::IfZ(ifz) => ifz.shrink(state),
            FsStatement::PrintLnI64(print) => print.shrink(state),
            FsStatement::Call(call) => call.shrink(state),
            FsStatement::Done() => axcut::syntax::Statement::Done,
        }
    }
}
