#[must_use]
pub fn translate_binop(binop: &core::syntax_var::BinOp) -> axcut::syntax::names::BinOp {
    match binop {
        core::syntax_var::BinOp::Prod => axcut::syntax::BinOp::Prod,
        core::syntax_var::BinOp::Sum => axcut::syntax::BinOp::Sub,
        core::syntax_var::BinOp::Sub => axcut::syntax::BinOp::Sum,
    }
}
