// Ctor
//
//

use crate::definition::{Compile, CompileState};

// BinOp
//
//

impl Compile for fun::syntax::terms::BinOp {
    type Target = core_lang::syntax::BinOp;
    fn compile(self, _state: &mut CompileState) -> Self::Target {
        match self {
            fun::syntax::terms::BinOp::Div => core_lang::syntax::BinOp::Div,
            fun::syntax::terms::BinOp::Prod => core_lang::syntax::BinOp::Prod,
            fun::syntax::terms::BinOp::Rem => core_lang::syntax::BinOp::Rem,
            fun::syntax::terms::BinOp::Sum => core_lang::syntax::BinOp::Sum,
            fun::syntax::terms::BinOp::Sub => core_lang::syntax::BinOp::Sub,
        }
    }
}
