use core_lang::syntax::declaration::{CodataDeclaration, DataDeclaration};
use core_lang::syntax::def::FsDef;

use crate::context::translate_context;
use crate::traits::{Shrinking, ShrinkingState};

pub fn translate_def(
    mut def: FsDef,
    data_types: &[DataDeclaration],
    codata_types: &[CodataDeclaration],
) -> axcut::syntax::Def {
    axcut::syntax::Def {
        name: def.name,
        context: translate_context(def.context, codata_types),
        body: def.body.shrink(&mut ShrinkingState {
            used_vars: &mut def.used_vars,
            data: data_types,
            codata: codata_types,
        }),
        used_vars: def.used_vars,
    }
}
