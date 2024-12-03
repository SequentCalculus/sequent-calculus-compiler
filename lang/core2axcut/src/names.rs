#[must_use]
pub fn translate_binop(binop: &core_lang::syntax::BinOp) -> axcut::syntax::names::BinOp {
    match binop {
        core_lang::syntax::BinOp::Div => axcut::syntax::BinOp::Div,
        core_lang::syntax::BinOp::Prod => axcut::syntax::BinOp::Prod,
        core_lang::syntax::BinOp::Rem => axcut::syntax::BinOp::Rem,
        core_lang::syntax::BinOp::Sum => axcut::syntax::BinOp::Sum,
        core_lang::syntax::BinOp::Sub => axcut::syntax::BinOp::Sub,
    }
}
