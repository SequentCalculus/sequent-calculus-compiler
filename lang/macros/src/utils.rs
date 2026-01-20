use syn::{Expr, ExprArray, ExprLit, Lit};

pub fn expr_to_str(expr: &Expr) -> String {
    match expr {
        Expr::Lit(ExprLit {
            lit: Lit::Str(s), ..
        }) => s.value(),
        _ => panic!("Please provide string literal"),
    }
}

pub fn expr_to_array(expr: &Expr) -> Vec<Expr> {
    match expr {
        Expr::Array(ExprArray { elems, .. }) => elems.into_iter().cloned().collect(),
        _ => panic!("Please provide an array expression"),
    }
}
