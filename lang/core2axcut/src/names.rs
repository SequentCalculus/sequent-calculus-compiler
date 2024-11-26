#[must_use]
pub fn translate_binop(binop: &core::syntax::BinOp) -> axcut::syntax::names::BinOp {
    match binop {
        core::syntax::BinOp::Div => axcut::syntax::BinOp::Div,
        core::syntax::BinOp::Prod => axcut::syntax::BinOp::Prod,
        core::syntax::BinOp::Rem => axcut::syntax::BinOp::Rem,
        core::syntax::BinOp::Sum => axcut::syntax::BinOp::Sum,
        core::syntax::BinOp::Sub => axcut::syntax::BinOp::Sub,
    }
}
