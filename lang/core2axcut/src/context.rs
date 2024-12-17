use core_lang::syntax::declaration::{cont_int, CodataDeclaration};
use core_lang::syntax::Ty;

use crate::types::translate_ty;

#[must_use]
pub fn translate_binding(
    binding: core_lang::syntax::context::ContextBinding,
    codata_types: &[CodataDeclaration],
) -> axcut::syntax::ContextBinding {
    match binding {
        core_lang::syntax::context::ContextBinding::VarBinding { var, ty } => {
            if ty == Ty::I64 {
                axcut::syntax::ContextBinding {
                    var,
                    chi: axcut::syntax::Chirality::Ext,
                    ty: axcut::syntax::Ty::I64,
                }
            } else if ty.is_codata(codata_types) {
                axcut::syntax::ContextBinding {
                    var,
                    chi: axcut::syntax::Chirality::Cns,
                    ty: translate_ty(ty),
                }
            } else {
                axcut::syntax::ContextBinding {
                    var,
                    chi: axcut::syntax::Chirality::Prd,
                    ty: translate_ty(ty),
                }
            }
        }
        core_lang::syntax::context::ContextBinding::CovarBinding { covar, ty } => {
            if ty == Ty::I64 {
                axcut::syntax::ContextBinding {
                    var: covar,
                    chi: axcut::syntax::Chirality::Cns,
                    ty: axcut::syntax::Ty::Decl(cont_int().name),
                }
            } else if ty.is_codata(codata_types) {
                axcut::syntax::ContextBinding {
                    var: covar,
                    chi: axcut::syntax::Chirality::Prd,
                    ty: translate_ty(ty),
                }
            } else {
                axcut::syntax::ContextBinding {
                    var: covar,
                    chi: axcut::syntax::Chirality::Cns,
                    ty: translate_ty(ty),
                }
            }
        }
    }
}

pub fn translate_context(
    context: core_lang::syntax::context::TypingContext,
    codata_types: &[CodataDeclaration],
) -> axcut::syntax::TypingContext {
    context
        .bindings
        .into_iter()
        .map(|binding| translate_binding(binding, codata_types))
        .collect::<Vec<_>>()
        .into()
}
