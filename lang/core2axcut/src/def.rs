use core_lang::syntax::declaration::{CodataDeclaration, DataDeclaration};
use core_lang::syntax::def::FsDef;
use core_lang::syntax::{Name, TypingContext};

use crate::context::shrink_context;
use crate::shrinking::{Shrinking, ShrinkingState};

use std::collections::{HashMap, VecDeque};

pub fn shrink_def(
    mut def: FsDef,
    data_types: &[DataDeclaration],
    codata_types: &[CodataDeclaration],
    def_signatures: &mut HashMap<Name, TypingContext>,
) -> VecDeque<axcut::syntax::Def> {
    let mut def_plus_lifted_statements = VecDeque::new();

    let body = def.body.shrink(&mut ShrinkingState {
        used_vars: &mut def.used_vars,
        data: data_types,
        codata: codata_types,
        used_labels: &mut def_signatures.keys().cloned().collect(),
        current_label: &def.name,
        def_signatures,
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
