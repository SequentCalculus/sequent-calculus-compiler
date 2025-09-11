//! This module defines the translation of a focused [Core](core_lang) program into a
//! non-linearized [AxCut](axcut) program.

use core_lang::syntax::declaration::cont_int;

use crate::declaration::shrink_declaration;
use crate::def::shrink_def;

/// This function translates a focused [Core](core_lang) program into a non-linearized
/// [AxCut](axcut) program. It assumes all variable bindings in each path through a program to be
/// unique and maintains this invariant.
/// - `program` is the focused [Core](core_lang) program.
///
/// # Panics
///
/// A panic is caused if one of the data or codata types in the [Core](core_lang) program has the
/// same name as the continuation type for integers returned by [`cont_int`] (the name is
/// `"_Cont"`).
pub fn shrink_prog(mut program: core_lang::syntax::program::FsProg) -> axcut::syntax::Prog {
    // check that no name of a user-declared type clashes with the type for integer continuations
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
    // add the type for integer continuations to the program
    program.data_types.push(cont_int);

    // we thread the set of labels of top-level functions through the translation, because we need
    // to generate fresh labels in some places
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
