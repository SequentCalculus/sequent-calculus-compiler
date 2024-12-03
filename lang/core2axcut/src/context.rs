use core_lang::syntax::declaration::cont_int;
use core_lang::syntax::{Chirality, Ty};

use crate::chirality::translate_chirality;
use crate::types::translate_ty;

#[must_use]
pub fn translate_binding(
    binding: core_lang::syntax::context::FsContextBinding,
) -> axcut::syntax::ContextBinding {
    if binding.ty == Ty::Int {
        if binding.chi == Chirality::Prd {
            axcut::syntax::ContextBinding {
                var: binding.var,
                chi: axcut::syntax::Chirality::Ext,
                ty: axcut::syntax::Ty::Int,
            }
        } else {
            axcut::syntax::ContextBinding {
                var: binding.var,
                chi: axcut::syntax::Chirality::Cns,
                ty: axcut::syntax::Ty::Decl(cont_int().name),
            }
        }
    } else {
        axcut::syntax::ContextBinding {
            var: binding.var,
            chi: translate_chirality(&binding.chi),
            ty: translate_ty(binding.ty),
        }
    }
}

pub fn translate_context(
    context: core_lang::syntax::context::FsTypingContext,
) -> axcut::syntax::TypingContext {
    context
        .bindings
        .into_iter()
        .map(translate_binding)
        .collect::<Vec<_>>()
        .into()
}
