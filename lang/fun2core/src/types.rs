//! Compilation for [fun::syntax::types::Ty]
//! Compiles to [core_lang::syntax::types::Ty]
use printer::Print;

/// Helper function to convert [fun::syntax::types::Ty] to [core_lang::syntax::types::Ty]
pub fn compile_ty(ty: &fun::syntax::types::Ty) -> core_lang::syntax::types::Ty {
    match ty {
        fun::syntax::types::Ty::I64 { .. } => core_lang::syntax::types::Ty::I64,
        fun::syntax::types::Ty::Decl { .. } => {
            core_lang::syntax::types::Ty::Decl(ty.print_to_string(None))
        }
    }
}
