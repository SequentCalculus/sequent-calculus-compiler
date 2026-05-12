//! This module defines the trivial translation on types.

use core_lang::syntax::{ID, fresh_identifier, names::Identifier};
use printer::Print;

/// This function converts [types in Fun](fun::syntax::types::Ty) to
/// [types in Core](core_lang::syntax::types::Ty).
pub fn compile_ty(ty: &fun::syntax::types::Ty) -> core_lang::syntax::types::Ty {
    match ty {
        fun::syntax::types::Ty::I64 { .. } => core_lang::syntax::types::Ty::I64,
        fun::syntax::types::Ty::Decl { .. } => {
            core_lang::syntax::types::Ty::Decl(Identifier::new(ty.print_to_string(None)))
        }
    }
}

/// This function converts [type parameters in Fun](fun::syntax::context::TypeContext) to
/// [type parameters in Core](core_lang::syntax::names::Identifier).
pub fn compile_type_params(
    params: &fun::syntax::context::TypeContext,
    max_id: &mut ID,
) -> Vec<Identifier> {
    params
        .bindings
        .iter()
        .map(|param| fresh_identifier(max_id, param))
        .collect()
}
