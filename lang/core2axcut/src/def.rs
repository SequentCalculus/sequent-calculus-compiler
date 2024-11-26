use core::syntax::declaration::FsTypeDeclaration;
use core::syntax::def::FsDef;

use crate::context::translate_context;
use crate::traits::Shrinking;

pub fn shrink(mut def: FsDef, types: &[FsTypeDeclaration]) -> axcut::syntax::Def {
    axcut::syntax::Def {
        name: def.name,
        context: translate_context(def.context),
        body: def.body.shrink(&mut def.used_vars, types),
        used_vars: def.used_vars,
    }
}
