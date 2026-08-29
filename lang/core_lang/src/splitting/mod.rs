pub mod labeling;
pub mod rewrite;
pub mod split_table;
pub mod union_find;

use crate::splitting::labeling::{
    LabelAndUnify, SplitState, build_decl_signatures, merge_field_observations,
};
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

    // Must run after every occurrence has been walked (`state.uf`'s roots need to be final) and
    // before `SplitTable::build`, which is what turns those roots into split-copy names.
    let field_observations = merge_field_observations(&mut state);

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
        .flat_map(|decl| split_declaration(decl, &table, &sigs, &field_observations, &mut max_id))
        .collect();
    let codata_types = labeled_codata
        .iter()
        .flat_map(|decl| split_declaration(decl, &table, &sigs, &field_observations, &mut max_id))
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
    use core_macros::{
        bind, call, case, clause, cns, covar, ctor, ctor_sig, cut, data, def, id, lit, prd, ty, var,
    };

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

    fn foo_decl() -> DataDeclaration {
        data!(id!("Foo"), [ctor_sig!(id!("MkFoo"), [], [])], [])
    }

    fn bar_decl() -> DataDeclaration {
        data!(
            id!("Bar"),
            [ctor_sig!(
                id!("MkBar"),
                [],
                [bind!(id!("f"), prd!(), ty!(id!("Foo")))]
            )],
            []
        )
    }

    /// `Foo` is nested (non-self-referentially) inside `Bar`'s field. Two independent `Bar`
    /// constructions must split `Foo` apart too, not just `Bar`.
    #[test]
    fn split_program_splits_an_independent_nested_declaration_too() {
        let def_a = def!(
            id!("a"),
            [bind!(id!("ret"), cns!(), ty!(id!("Bar")))],
            cut!(
                ctor!(
                    id!("MkBar"),
                    [],
                    [ctor!(id!("MkFoo"), [], [], ty!(id!("Foo")))],
                    ty!(id!("Bar"))
                ),
                covar!(id!("ret"), ty!(id!("Bar"))),
                ty!(id!("Bar"))
            )
        );
        let def_b = def!(
            id!("b"),
            [bind!(id!("ret"), cns!(), ty!(id!("Bar")))],
            cut!(
                ctor!(
                    id!("MkBar"),
                    [],
                    [ctor!(id!("MkFoo"), [], [], ty!(id!("Foo")))],
                    ty!(id!("Bar"))
                ),
                covar!(id!("ret"), ty!(id!("Bar"))),
                ty!(id!("Bar"))
            )
        );
        let prog = Prog {
            defs: vec![def_a, def_b],
            data_types: vec![foo_decl(), bar_decl()],
            codata_types: vec![],
            max_id: 0,
        };

        let result = split_program(&prog);

        // 2 independent `Bar` copies, each with its own independent `Foo` copy.
        assert_eq!(result.data_types.len(), 4);
        let bars: Vec<_> = result
            .data_types
            .iter()
            .filter(|d| d.name.name.starts_with("Bar"))
            .collect();
        let foos: Vec<_> = result
            .data_types
            .iter()
            .filter(|d| d.name.name.starts_with("Foo"))
            .collect();
        assert_eq!(bars.len(), 2);
        assert_eq!(foos.len(), 2);
        assert_ne!(
            bars[0].xtors[0].args.bindings[0].ty,
            bars[1].xtors[0].args.bindings[0].ty
        );
    }

    fn list_decl() -> DataDeclaration {
        data!(
            id!("List"),
            [
                ctor_sig!(id!("Nil"), [], []),
                ctor_sig!(
                    id!("Cons"),
                    [],
                    [
                        bind!(id!("x"), prd!(), ty!("int")),
                        bind!(id!("xs"), prd!(), ty!(id!("List")))
                    ]
                )
            ],
            []
        )
    }

    /// Builds a `Cons`-chain of the given `depth` (0 = a bare `Nil`).
    fn nested_list(depth: usize) -> Term<Prd> {
        if depth == 0 {
            ctor!(id!("Nil"), [], [], ty!(id!("List"))).into()
        } else {
            ctor!(
                id!("Cons"),
                [],
                [lit!(depth as i64), nested_list(depth - 1)],
                ty!(id!("List"))
            )
            .into()
        }
    }

    fn list_prog(list: Term<Prd>) -> Prog {
        Prog {
            defs: vec![def!(
                id!("a"),
                [bind!(id!("ret"), cns!(), ty!(id!("List")))],
                cut!(list, covar!(id!("ret"), ty!(id!("List"))), ty!(id!("List")))
            )],
            data_types: vec![list_decl()],
            codata_types: vec![],
            max_id: 0,
        }
    }

    #[test]
    fn split_program_splits_a_manually_unrolled_chain_into_one_copy_per_depth_level() {
        let shallow = split_program(&list_prog(nested_list(1))).data_types.len();
        let deep = split_program(&list_prog(nested_list(5))).data_types.len();

        assert!(
            deep > shallow,
            "expected the manually unrolled chain to split into more copies as it gets deeper, got shallow={shallow}, deep={deep}"
        );
    }

    #[test]
    fn split_program_lets_a_self_referential_field_point_at_a_different_copy() {
        let result = split_program(&list_prog(nested_list(2)));

        assert!(result.data_types.len() > 1);
        let cross_references_a_different_copy = result.data_types.iter().any(|copy| {
            copy.xtors.iter().any(|xtor| {
                xtor.name.name.starts_with("Cons")
                    && match &xtor.args.bindings[1].ty {
                        Ty::Decl { name, .. } => name != &copy.name,
                        _ => false,
                    }
            })
        });
        assert!(
            cross_references_a_different_copy,
            "expected at least one split copy whose Cons.xs field points at a different copy \
             than its own declaration, got: {:#?}",
            result.data_types
        );
    }

    #[test]
    fn split_program_keeps_a_def_call_recursive_function_from_exploding_with_depth() {
        let len = def!(
            id!("len"),
            [
                bind!(id!("xs"), prd!(), ty!(id!("List"))),
                bind!(id!("ret"), cns!(), ty!("int"))
            ],
            cut!(
                var!(id!("xs"), ty!(id!("List"))),
                case!(
                    [
                        clause!(
                            Cns,
                            id!("Nil"),
                            [],
                            [],
                            cut!(lit!(0), covar!(id!("ret")), ty!("int"))
                        ),
                        clause!(
                            Cns,
                            id!("Cons"),
                            [],
                            [
                                bind!(id!("x"), prd!(), ty!("int")),
                                bind!(id!("tail"), prd!(), ty!(id!("List")))
                            ],
                            call!(
                                id!("len"),
                                [var!(id!("tail"), ty!(id!("List"))), covar!(id!("ret"))]
                            )
                        )
                    ],
                    ty!(id!("List"))
                ),
                ty!(id!("List"))
            )
        );
        // Two unrelated call sites, feeding manually unrolled chains of very different depths
        // into the very same recursive function.
        let def_a = def!(
            id!("a"),
            [bind!(id!("ret"), cns!(), ty!("int"))],
            call!(id!("len"), [nested_list(1), covar!(id!("ret"))])
        );
        let def_b = def!(
            id!("b"),
            [bind!(id!("ret"), cns!(), ty!("int"))],
            call!(id!("len"), [nested_list(5), covar!(id!("ret"))])
        );
        let prog = Prog {
            defs: vec![len, def_a, def_b],
            data_types: vec![list_decl()],
            codata_types: vec![],
            max_id: 0,
        };

        let result = split_program(&prog);

        assert_eq!(result.data_types.len(), 1);
        assert_eq!(result.data_types[0].name, id!("List"));
    }

    #[test]
    fn split_program_does_not_merge_two_separate_mutually_recursive_defs_canonical_classes() {
        let is_even = def!(
            id!("is_even"),
            [
                bind!(id!("xs"), prd!(), ty!(id!("List"))),
                bind!(id!("ret"), cns!(), ty!("int"))
            ],
            cut!(
                var!(id!("xs"), ty!(id!("List"))),
                case!(
                    [
                        clause!(
                            Cns,
                            id!("Nil"),
                            [],
                            [],
                            cut!(lit!(1), covar!(id!("ret")), ty!("int"))
                        ),
                        clause!(
                            Cns,
                            id!("Cons"),
                            [],
                            [
                                bind!(id!("x"), prd!(), ty!("int")),
                                bind!(id!("tail"), prd!(), ty!(id!("List")))
                            ],
                            call!(
                                id!("is_odd"),
                                [var!(id!("tail"), ty!(id!("List"))), covar!(id!("ret"))]
                            )
                        )
                    ],
                    ty!(id!("List"))
                ),
                ty!(id!("List"))
            )
        );
        let is_odd = def!(
            id!("is_odd"),
            [
                bind!(id!("xs"), prd!(), ty!(id!("List"))),
                bind!(id!("ret"), cns!(), ty!("int"))
            ],
            cut!(
                var!(id!("xs"), ty!(id!("List"))),
                case!(
                    [
                        clause!(
                            Cns,
                            id!("Nil"),
                            [],
                            [],
                            cut!(lit!(0), covar!(id!("ret")), ty!("int"))
                        ),
                        clause!(
                            Cns,
                            id!("Cons"),
                            [],
                            [
                                bind!(id!("x"), prd!(), ty!("int")),
                                bind!(id!("tail"), prd!(), ty!(id!("List")))
                            ],
                            call!(
                                id!("is_even"),
                                [var!(id!("tail"), ty!(id!("List"))), covar!(id!("ret"))]
                            )
                        )
                    ],
                    ty!(id!("List"))
                ),
                ty!(id!("List"))
            )
        );
        let def_a = def!(
            id!("a"),
            [bind!(id!("ret"), cns!(), ty!("int"))],
            call!(id!("is_even"), [nested_list(1), covar!(id!("ret"))])
        );
        let def_b = def!(
            id!("b"),
            [bind!(id!("ret"), cns!(), ty!("int"))],
            call!(id!("is_odd"), [nested_list(5), covar!(id!("ret"))])
        );
        let prog = Prog {
            defs: vec![is_even, is_odd, def_a, def_b],
            data_types: vec![list_decl()],
            codata_types: vec![],
            max_id: 0,
        };

        let result = split_program(&prog);

        assert_eq!(result.data_types.len(), 2);
    }

    fn a_decl() -> DataDeclaration {
        data!(
            id!("A"),
            [ctor_sig!(
                id!("MkA"),
                [],
                [bind!(id!("b"), prd!(), ty!(id!("B")))]
            )],
            []
        )
    }

    fn b_decl() -> DataDeclaration {
        data!(
            id!("B"),
            [
                ctor_sig!(id!("MkB"), [], [bind!(id!("a"), prd!(), ty!(id!("A")))]),
                ctor_sig!(id!("Leaf"), [], [])
            ],
            []
        )
    }

    /// Builds an `A` value nested `depth` levels deep, alternating `MkA`/`MkB`, terminated by
    /// `Leaf` (`MkA(MkB(MkA(...Leaf))))`).
    fn nested_a(depth: usize) -> Term<Prd> {
        ctor!(id!("MkA"), [], [nested_b(depth)], ty!(id!("A"))).into()
    }

    fn nested_b(depth: usize) -> Term<Prd> {
        if depth == 0 {
            ctor!(id!("Leaf"), [], [], ty!(id!("B"))).into()
        } else {
            ctor!(id!("MkB"), [], [nested_a(depth - 1)], ty!(id!("B"))).into()
        }
    }

    fn ab_prog(a: Term<Prd>) -> Prog {
        Prog {
            defs: vec![def!(
                id!("a"),
                [bind!(id!("ret"), cns!(), ty!(id!("A")))],
                cut!(a, covar!(id!("ret"), ty!(id!("A"))), ty!(id!("A")))
            )],
            data_types: vec![a_decl(), b_decl()],
            codata_types: vec![],
            max_id: 0,
        }
    }
    #[test]
    fn split_program_splits_a_manually_unrolled_mutually_recursive_chain_into_more_copies_with_depth()
     {
        let shallow = split_program(&ab_prog(nested_a(0))).data_types.len();
        let deep = split_program(&ab_prog(nested_a(5))).data_types.len();

        assert!(
            deep > shallow,
            "expected the manually unrolled mutually recursive chain to split into more copies as it gets deeper, got shallow={shallow}, deep={deep}"
        );
    }
}
