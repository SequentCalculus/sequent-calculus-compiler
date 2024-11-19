#[must_use]
pub fn translate_binop(binop: &core::syntax_var::BinOp) -> axcut::syntax::names::BinOp {
    match binop {
        core::syntax_var::BinOp::Div => axcut::syntax::BinOp::Div,
        core::syntax_var::BinOp::Prod => axcut::syntax::BinOp::Prod,
        core::syntax_var::BinOp::Rem => axcut::syntax::BinOp::Rem,
        core::syntax_var::BinOp::Sum => axcut::syntax::BinOp::Sum,
        core::syntax_var::BinOp::Sub => axcut::syntax::BinOp::Sub,
    }
}
