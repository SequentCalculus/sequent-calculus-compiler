// Ctor
//
//

use crate::definition::{Compile, CompileState};

// BinOp
//
//

impl Compile for fun::syntax::terms::BinOp {
    type Target = core::syntax::BinOp;
    fn compile(self, _state: &mut CompileState) -> Self::Target {
        match self {
            fun::syntax::terms::BinOp::Div => core::syntax::BinOp::Div,
            fun::syntax::terms::BinOp::Prod => core::syntax::BinOp::Prod,
            fun::syntax::terms::BinOp::Rem => core::syntax::BinOp::Rem,
            fun::syntax::terms::BinOp::Sum => core::syntax::BinOp::Sum,
            fun::syntax::terms::BinOp::Sub => core::syntax::BinOp::Sub,
        }
    }
}
