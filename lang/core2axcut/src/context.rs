use core::syntax_var::{cont_int, Chirality, ContextBinding, Ty, TypeDeclaration, Var};

use crate::chirality::translate_chirality;
use crate::traits::Shrinking;
use crate::types::translate_ty;

use std::collections::HashSet;

impl Shrinking for ContextBinding {
    type Target = axcut::syntax::ContextBinding;

    fn shrink(
        self,
        _used_vars: &mut HashSet<Var>,
        _types: &[TypeDeclaration],
    ) -> axcut::syntax::ContextBinding {
        if self.ty == Ty::Int {
            if self.chi == Chirality::Prd {
                axcut::syntax::ContextBinding {
                    var: self.var,
                    chi: axcut::syntax::Chirality::Ext,
                    ty: axcut::syntax::Ty::Int,
                }
            } else {
                axcut::syntax::ContextBinding {
                    var: self.var,
                    chi: axcut::syntax::Chirality::Cns,
                    ty: axcut::syntax::Ty::Decl(cont_int().name),
                }
            }
        } else {
            axcut::syntax::ContextBinding {
                var: self.var,
                chi: translate_chirality(&self.chi),
                ty: translate_ty(self.ty),
            }
        }
    }
}
