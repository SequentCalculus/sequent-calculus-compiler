//! This module defines the translation of typing contexts.

use core_lang::syntax::Ty;
use core_lang::syntax::declaration::{CodataDeclaration, cont_int};

use crate::{names::shrink_var, types::shrink_ty};

/// This function translates a context binding in [Core](core_lang) to one in [AxCut](axcut). It
/// essentially consists of mapping producers of data types and consumers of codata types to the
/// same representation (having chirality `Prd`) and mapping consumers of data types and producers
/// of codata types to the same representation (having chirality `Cns`). It moreover maps consumers
/// of integers to integer continuations (having chirality `Cns`) and uses chirality `Ext` for
/// integer producers.
/// - `binding` is the context binding to translate.
/// - `codata_types` is the list of codata types in the corresponding [Core](core_lang) program.
pub fn shrink_binding(
    binding: core_lang::syntax::context::ContextBinding,
    codata_types: &[CodataDeclaration],
) -> axcut::syntax::ContextBinding {
    if binding.ty == Ty::I64 {
        if binding.chi == core_lang::syntax::context::Chirality::Cns {
            axcut::syntax::ContextBinding {
                var: shrink_var(binding.var),
                chi: axcut::syntax::Chirality::Cns,
                ty: axcut::syntax::Ty::Decl(cont_int().name),
            }
        } else {
            axcut::syntax::ContextBinding {
                var: shrink_var(binding.var),
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
            var: shrink_var(binding.var),
            chi: axcut::syntax::Chirality::Prd,
            ty: shrink_ty(binding.ty),
        }
    } else {
        axcut::syntax::ContextBinding {
            var: shrink_var(binding.var),
            chi: axcut::syntax::Chirality::Cns,
            ty: shrink_ty(binding.ty),
        }
    }
}

/// This function translates a typing context in [Core](core_lang) to one in [AxCut](axcut). It
/// essentially consists of translating each binding.
/// - `context` is the typing context to translate.
/// - `codata_types` is the list of codata types in the corresponding [Core](core_lang) program.
pub fn shrink_context(
    context: core_lang::syntax::context::TypingContext,
    codata_types: &[CodataDeclaration],
) -> axcut::syntax::TypingContext {
    context
        .bindings
        .into_iter()
        .map(|binding| shrink_binding(binding, codata_types))
        .collect::<Vec<_>>()
        .into()
}
