use core::syntax_var::cont_int;
use core::traits::used_binders::UsedBinders;

use crate::declaration::translate_declaration;
use crate::traits::Shrinking;

use std::collections::HashSet;

#[must_use]
pub fn translate_prog(mut program: core::syntax_var::Prog) -> axcut::syntax::Prog {
    let cont_int = cont_int();
    for typ in &program.types {
        assert!(
            typ.name != cont_int.name,
            "{} cannot be used as a type name",
            cont_int.name
        );
    }
    program.types.push(cont_int);
    axcut::syntax::Prog {
        defs: program
            .defs
            .into_iter()
            .map(|def| {
                let mut used_vars = HashSet::new();
                def.body.used_binders(&mut used_vars);
                for binding in &def.context {
                    used_vars.insert(binding.var.clone());
                }
                def.shrink(&mut used_vars, &program.types)
            })
            .collect(),
        types: program
            .types
            .into_iter()
            .map(translate_declaration)
            .collect(),
    }
}
