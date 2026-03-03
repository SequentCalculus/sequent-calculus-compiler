//! This module defines the translation of top-level functions.

use core_lang::syntax::declaration::{CodataDeclaration, DataDeclaration};
use core_lang::syntax::def::FsDef;
use core_lang::syntax::{ID, Identifier};

use crate::context::shrink_context;
use crate::names::shrink_identifier;
use crate::shrinking::{Shrinking, ShrinkingState};

use std::collections::{HashSet, VecDeque};

/// This function translates a top-level function in focused [Core](core_lang) to a top-level
/// function in non-linearized [AxCut](axcut).
/// - `def` is the top-level function to translate.
/// - `data_types` is the list of data types in the corresponding [Core](core_lang) program.
/// - `codata_types` is the list of codata types in the corresponding [Core](core_lang) program.
/// - `used_labels` is the set of labels of top-level functions in the corresponding
///   [Core](core_lang) program.
pub fn shrink_def(
    def: FsDef,
    data_types: &[DataDeclaration],
    codata_types: &[CodataDeclaration],
    used_labels: &mut HashSet<Identifier>,
    max_id: &mut ID,
) -> VecDeque<axcut::syntax::Def> {
    // we sometimes create new top-level labels during the translation, so we need to collect them
    let mut def_plus_lifted_statements = VecDeque::new();

    let body = def.body.shrink(&mut ShrinkingState {
        max_id,
        data: data_types,
        codata: codata_types,
        used_labels,
        current_label: &def.name.name,
        lifted_statements: &mut def_plus_lifted_statements,
    });

    def_plus_lifted_statements.push_front(axcut::syntax::Def {
        name: shrink_identifier(def.name),
        context: shrink_context(def.context, codata_types),
        body,
    });

    def_plus_lifted_statements
}
