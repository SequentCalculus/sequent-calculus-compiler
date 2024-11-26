use core::syntax::Ty;
use core::syntax_var::{cont_int, Chirality};

use crate::chirality::translate_chirality;
use crate::types::translate_ty;

#[must_use]
pub fn translate_binding(
    binding: core::syntax_var::ContextBinding,
) -> axcut::syntax::ContextBinding {
    if binding.ty == Ty::Int() {
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

pub fn translate_context(context: core::syntax_var::TypingContext) -> axcut::syntax::TypingContext {
    context.into_iter().map(translate_binding).collect()
}
