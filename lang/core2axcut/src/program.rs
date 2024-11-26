use core::syntax_var::cont_int;

use crate::declaration::translate_declaration;

#[must_use]
pub fn translate_prog(mut program: core::syntax_var::FsProg) -> axcut::syntax::Prog {
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
            .map(|def| super::def::shrink(def, &program.types))
            .collect(),
        types: program
            .types
            .into_iter()
            .map(translate_declaration)
            .collect(),
    }
}
