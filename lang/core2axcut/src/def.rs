use core_lang::syntax::Name;
use core_lang::syntax::declaration::{CodataDeclaration, DataDeclaration};
use core_lang::syntax::def::FsDef;

use crate::context::shrink_context;
use crate::shrinking::{Shrinking, ShrinkingState};

use std::collections::{HashSet, VecDeque};

pub fn shrink_def(
    mut def: FsDef,
    data_types: &[DataDeclaration],
    codata_types: &[CodataDeclaration],
    used_labels: &mut HashSet<Name>,
) -> VecDeque<axcut::syntax::Def> {
    let mut def_plus_lifted_statements = VecDeque::new();

    let body = def.body.shrink(&mut ShrinkingState {
        used_vars: &mut def.used_vars,
        data: data_types,
        codata: codata_types,
        used_labels,
        current_label: &def.name,
        lifted_statements: &mut def_plus_lifted_statements,
    });

    def_plus_lifted_statements.push_front(axcut::syntax::Def {
        name: def.name,
        context: shrink_context(def.context, codata_types),
        body,
        used_vars: def.used_vars,
    });

    def_plus_lifted_statements
}
