// Ctor
//
//

use crate::definition::{Compile, CompileState};

impl Compile for fun::syntax::Ctor {
    type Target = core::syntax::Ctor;
    fn compile(self, _state: &mut CompileState) -> Self::Target {
        match self {
            fun::syntax::Ctor::Nil => core::syntax::Ctor::Nil,
            fun::syntax::Ctor::Cons => core::syntax::Ctor::Cons,
            fun::syntax::Ctor::Tup => core::syntax::Ctor::Tup,
        }
    }
}

// Dtor
//
//

impl Compile for fun::syntax::Dtor {
    type Target = core::syntax::Dtor;
    fn compile(self, _state: &mut CompileState) -> Self::Target {
        match self {
            fun::syntax::Dtor::Hd => core::syntax::Dtor::Hd,
            fun::syntax::Dtor::Tl => core::syntax::Dtor::Tl,
            fun::syntax::Dtor::Fst => core::syntax::Dtor::Fst,
            fun::syntax::Dtor::Snd => core::syntax::Dtor::Snd,
            fun::syntax::Dtor::Ap => core::syntax::Dtor::Ap,
        }
    }
}

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
