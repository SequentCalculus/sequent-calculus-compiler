pub mod labeling;
pub mod reachability;
pub mod rewrite;
pub mod split_table;
pub mod union_find;

use crate::splitting::labeling::{LabelAndUnify, SplitState, build_decl_signatures};
use crate::splitting::rewrite::{Rewrite, split_declaration};
use crate::splitting::split_table::SplitTable;
use crate::syntax::{Def, Prog, TypingContext};

/// Type-splitting preprocessing pass, run before constraint collection (see
/// `mono::monomorphize_program`). Labels every declared-type occurrence and eagerly unifies label
/// pairs wherever the type system already requires equality (Phase 1, [`labeling`]), then produces
/// one physical copy of each data/codata declaration per resulting equivalence class and rewrites
/// every reference to point at the correct copy (Phase 2, [`rewrite`]). Splitting is a pure
/// bookkeeping refinement: it never changes program semantics, only how many physical
/// declarations later phases see. It just runs the existing, unmodified monomorphization pipeline over a more finely
/// divided program, so it finds no more, but possibly fewer, growing cycles.
pub fn split_program(prog: &Prog) -> Prog {
    let mut state = SplitState::default();
    let (sigs, labeled_data, labeled_codata) = build_decl_signatures(prog, &mut state);

    let labeled_defs: Vec<Def> = prog
        .defs
        .iter()
        .map(|def| def.label_and_unify(&mut state, &sigs, &TypingContext::default()))
        .collect();

    let table = SplitTable::build(
        &mut state.uf,
        &state.label_origin,
        &labeled_data,
        &labeled_codata,
    );

    let mut max_id = prog.max_id;

    // Physical copies of a split declaration are alpha-renamed here (fresh `type_params` ids),
    // since the constraint graph indexes its nodes directly by these identifiers, unrenamed
    // copies would collapse onto the same node, leaving splitting unable to ever separate a
    // growing cycle (see `splitting::rewrite::build_declaration_copy`).
    let data_types = labeled_data
        .iter()
        .flat_map(|decl| split_declaration(decl, &table, &mut max_id))
        .collect();
    let codata_types = labeled_codata
        .iter()
        .flat_map(|decl| split_declaration(decl, &table, &mut max_id))
        .collect();
    let defs = labeled_defs.iter().map(|def| def.rewrite(&table)).collect();

    Prog {
        defs,
        data_types,
        codata_types,
        max_id,
    }
}

#[cfg(test)]
mod split_program_tests {
    use super::*;
    use crate::syntax::*;
    extern crate self as core_lang;
    use core_macros::{bind, call, cns, covar, ctor, ctor_sig, cut, data, def, id, lit, prd, ty};

    fn box_decl() -> DataDeclaration {
        data!(
            id!("Box"),
            [ctor_sig!(
                id!("Wrap"),
                [],
                [bind!(id!("x"), prd!(), ty!("int"))]
            )],
            []
        )
    }

    /// Two defs that each independently construct their own `Box` value and hand it straight to
    /// their own return continuation: nothing ever unifies the two occurrences, so `Box` splits
    /// into two physically distinct, correctly and consistently renamed declarations.
    #[test]
    fn split_program_splits_two_independently_used_boxes() {
        let def_a = def!(
            id!("a"),
            [bind!(id!("ret"), cns!(), ty!(id!("Box")))],
            cut!(
                ctor!(id!("Wrap"), [], [lit!(1)], ty!(id!("Box"))),
                covar!(id!("ret"), ty!(id!("Box"))),
                ty!(id!("Box"))
            )
        );
        let def_b = def!(
            id!("b"),
            [bind!(id!("ret"), cns!(), ty!(id!("Box")))],
            cut!(
                ctor!(id!("Wrap"), [], [lit!(2)], ty!(id!("Box"))),
                covar!(id!("ret"), ty!(id!("Box"))),
                ty!(id!("Box"))
            )
        );
        let prog = Prog {
            defs: vec![def_a, def_b],
            data_types: vec![box_decl()],
            codata_types: vec![],
            max_id: 0,
        };

        let result = split_program(&prog);

        assert_eq!(result.data_types.len(), 2);
        assert_ne!(result.data_types[0].name, result.data_types[1].name);

        // each def's constructor call must reference the xtor name of the matching split copy,
        // and each split copy must have exactly the one xtor it started with
        let used_xtor_names: Vec<Identifier> = result
            .defs
            .iter()
            .map(|def| {
                let Statement::Cut(cut) = &def.body else {
                    panic!("expected a Cut");
                };
                let Term::Xtor(xtor) = cut.producer.as_ref() else {
                    panic!("expected an Xtor producer");
                };
                xtor.name.clone()
            })
            .collect();
        assert_ne!(used_xtor_names[0], used_xtor_names[1]);
        for decl in &result.data_types {
            assert_eq!(decl.xtors.len(), 1);
            assert!(used_xtor_names.contains(&decl.xtors[0].name));
        }
    }

    /// Two defs that both construct a `Box` but pass it through the same shared call site
    /// (`use_box`'s one canonical parameter label unifies every call site's argument against
    /// itself) end up in a single equivalence class, `Box` stays as one, unrenamed declaration.
    /// Splitting only ever separates occurrences the type system doesn't already force together.
    #[test]
    fn split_program_keeps_boxes_merged_through_a_shared_call_site() {
        let use_box = def!(
            id!("use_box"),
            [
                bind!(id!("b"), prd!(), ty!(id!("Box"))),
                bind!(id!("ret"), cns!(), ty!("int"))
            ],
            cut!(lit!(0), covar!(id!("ret")), ty!("int"))
        );
        let def_a = def!(
            id!("a"),
            [bind!(id!("ret"), cns!(), ty!("int"))],
            call!(
                id!("use_box"),
                [
                    ctor!(id!("Wrap"), [], [lit!(1)], ty!(id!("Box"))),
                    covar!(id!("ret"))
                ]
            )
        );
        let def_b = def!(
            id!("b"),
            [bind!(id!("ret"), cns!(), ty!("int"))],
            call!(
                id!("use_box"),
                [
                    ctor!(id!("Wrap"), [], [lit!(2)], ty!(id!("Box"))),
                    covar!(id!("ret"))
                ]
            )
        );
        let prog = Prog {
            defs: vec![use_box, def_a, def_b],
            data_types: vec![box_decl()],
            codata_types: vec![],
            max_id: 0,
        };

        let result = split_program(&prog);

        assert_eq!(result.data_types.len(), 1);
        assert_eq!(result.data_types[0].name, id!("Box"));
        assert_eq!(result.data_types[0].xtors[0].name, id!("Wrap"));
    }
}
