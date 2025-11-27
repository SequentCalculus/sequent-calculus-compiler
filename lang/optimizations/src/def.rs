use crate::{
    errors::Error,
    rewrite::{Rewrite, RewriteState},
};
use axcut::syntax::{Def, Name};

use std::collections::{HashMap, HashSet, VecDeque};

pub fn rewrite_def(
    mut def: Def,
    used_labels: &mut HashSet<Name>,
    new_changes: &mut bool,
) -> Result<VecDeque<Def>, Error> {
    let mut def_plus_lifted_statements = VecDeque::new();

    def.body = def.body.rewrite(&mut RewriteState {
        used_vars: &def.used_vars,
        used_labels,
        current_label: &def.name,
        lifted_statements: &mut def_plus_lifted_statements,
        let_bindings: HashMap::new(),
        create_bindings: HashMap::new(),
        new_changes,
    })?;
    def_plus_lifted_statements.push_front(def);

    Ok(def_plus_lifted_statements)
}
