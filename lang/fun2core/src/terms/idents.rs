// Ctor
//
//

use crate::definition::{Compile, CompileState};

// BinOp
//
//

impl Compile for fun::syntax::BinOp {
    type Target = core::syntax::BinOp;
    fn compile(self, _state: &mut CompileState) -> Self::Target {
        match self {
            fun::syntax::BinOp::Prod => core::syntax::BinOp::Prod,
            fun::syntax::BinOp::Sum => core::syntax::BinOp::Sum,
            fun::syntax::BinOp::Sub => core::syntax::BinOp::Sub,
        }
    }
}
