use core_lang::syntax::declaration::cont_int;
use core_lang::syntax::Ty;

use crate::types::translate_ty;

#[must_use]
pub fn translate_binding(
    binding: core_lang::syntax::context::ContextBinding,
) -> axcut::syntax::ContextBinding {
    match binding {
        core_lang::syntax::context::ContextBinding::VarBinding { var, ty } => {
            if ty == Ty::Int {
                axcut::syntax::ContextBinding {
                    var,
                    chi: axcut::syntax::Chirality::Ext,
                    ty: axcut::syntax::Ty::Int,
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
            if ty == Ty::Int {
                axcut::syntax::ContextBinding {
                    var: covar,
                    chi: axcut::syntax::Chirality::Cns,
                    ty: axcut::syntax::Ty::Decl(cont_int().name),
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
) -> axcut::syntax::TypingContext {
    context
        .bindings
        .into_iter()
        .map(translate_binding)
        .collect::<Vec<_>>()
        .into()
}
