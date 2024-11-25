use core::syntax_var::{Def, TypeDeclaration};

use crate::context::translate_context;
use crate::traits::Shrinking;

pub fn shrink(mut def: Def, types: &[TypeDeclaration]) -> axcut::syntax::Def {
    axcut::syntax::Def {
        name: def.name,
        context: translate_context(def.context),
        body: def.body.shrink(&mut def.used_vars, types),
        used_vars: def.used_vars,
    }
}
