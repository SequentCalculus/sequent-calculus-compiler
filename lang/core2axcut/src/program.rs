use core_lang::syntax::declaration::cont_int;

use crate::declaration::shrink_declaration;
use crate::def::shrink_def;

pub fn shrink_prog(mut program: core_lang::syntax::program::FsProg) -> axcut::syntax::Prog {
    let cont_int = cont_int();
    for typ in &program.data_types {
        assert!(
            typ.name != cont_int.name,
            "{} cannot be used as a type name",
            cont_int.name
        );
    }
    for typ in &program.codata_types {
        assert!(
            typ.name != cont_int.name,
            "{} cannot be used as a type name",
            cont_int.name
        );
    }
    program.data_types.push(cont_int);

    let mut used_labels = program.defs.iter().map(|def| def.name.clone()).collect();

    axcut::syntax::Prog {
        defs: program
            .defs
            .into_iter()
            .flat_map(|def| {
                shrink_def(
                    def,
                    &program.data_types,
                    &program.codata_types,
                    &mut used_labels,
                )
            })
            .collect(),
        types: [
            program
                .data_types
                .into_iter()
                .map(|declaration| shrink_declaration(declaration, &program.codata_types))
                .collect::<Vec<_>>(),
            program
                .codata_types
                .clone()
                .into_iter()
                .map(|declaration| shrink_declaration(declaration, &program.codata_types))
                .collect(),
        ]
        .concat(),
    }
}
