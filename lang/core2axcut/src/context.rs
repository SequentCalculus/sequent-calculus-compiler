use core_lang::syntax::declaration::{cont_int, CodataDeclaration};
use core_lang::syntax::Ty;

use crate::types::translate_ty;

#[must_use]
pub fn translate_binding(
    binding: core_lang::syntax::context::ContextBinding,
    codata_types: &[CodataDeclaration],
) -> axcut::syntax::ContextBinding {
    if binding.ty == Ty::I64 {
        if binding.chi == core_lang::syntax::context::Chirality::Cns {
            axcut::syntax::ContextBinding {
                var: binding.var,
                chi: axcut::syntax::Chirality::Cns,
                ty: axcut::syntax::Ty::Decl(cont_int().name),
            }
        } else {
            axcut::syntax::ContextBinding {
                var: binding.var,
                chi: axcut::syntax::Chirality::Ext,
                ty: axcut::syntax::Ty::I64,
            }
        }
    } else if !binding.ty.is_codata(codata_types)
        && binding.chi == core_lang::syntax::context::Chirality::Prd
        || binding.ty.is_codata(codata_types)
            && binding.chi == core_lang::syntax::context::Chirality::Cns
    {
        axcut::syntax::ContextBinding {
            var: binding.var,
            chi: axcut::syntax::Chirality::Prd,
            ty: translate_ty(binding.ty),
        }
    } else {
        axcut::syntax::ContextBinding {
            var: binding.var,
            chi: axcut::syntax::Chirality::Cns,
            ty: translate_ty(binding.ty),
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
