pub mod labeling;
pub mod rewrite;
pub mod split_table;
pub mod union_find;

use crate::splitting::labeling::{
    LabelAndUnify, SplitState, build_decl_signatures, finalize_used_xtors, merge_field_observations,
};
use crate::splitting::rewrite::{Rewrite, split_declaration};
use crate::splitting::split_table::SplitTable;
use crate::syntax::{Def, Prog, TypingContext};

/// Type-splitting preprocessing pass, run before constraint collection (see
/// `mono::monomorphize_program`). Labels every declared-type occurrence and eagerly unifies label
/// pairs wherever the type system already requires equality (Phase 1, [`labeling`]), then produces
/// one physical copy of each data/codata declaration per resulting equivalence class and rewrites
/// every reference to point at the correct copy (Phase 2, [`rewrite`]). Each copy keeps only the
/// xtors its own equivalence class actually uses -- constructed or observed at an `Xtor` node, or
/// matched or defined by a `case`/`new` clause (see [`rewrite::keeps_xtor`]). Splitting is a pure
/// bookkeeping refinement: it never changes program semantics, only how many physical
/// declarations later phases see, and how many xtors each of them still carries. It just runs the existing, unmodified monomorphization pipeline over a more finely
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
    let used_xtors = finalize_used_xtors(&mut state);

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
        .flat_map(|decl| {
            split_declaration(
                decl,
                &table,
                &sigs,
                &field_observations,
                &used_xtors,
                &mut max_id,
            )
        })
        .collect();
    let codata_types = labeled_codata
        .iter()
        .flat_map(|decl| {
            split_declaration(
                decl,
                &table,
                &sigs,
                &field_observations,
                &used_xtors,
                &mut max_id,
            )
        })
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
    use crate::mono::constraints::ConstraintCollector;
    use crate::syntax::*;
    use crate::traits::Typed;
    use crate::typing::env::GlobalEnv;
    extern crate self as core_lang;
    use core_macros::{
        bind, call, case, clause, cns, cocase, codata, covar, ctor, ctor_sig, cut, data, def, dtor,
        dtor_sig, exit, id, lit, prd, tparam, tvar, ty, var,
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

    /// Runs the exact machinery `mono::constraints_of` runs on a split program next (Core-level
    /// `Checked::check`, then constraint collection) and asserts it succeeds. Splitting renames and
    /// duplicates declarations and drops xtors from individual copies -- easy to get subtly wrong
    /// in a way that only a real re-check against a `GlobalEnv` built from the split program's own
    /// (renamed) declarations catches; asserting on e.g. `data_types.len()` alone would not.
    fn assert_split_program_typechecks(result: &Prog) {
        let env = GlobalEnv::new(&result.data_types, &result.codata_types, &result.defs);
        if let Err(err) = result.collect_constraints(&env) {
            panic!("split program failed to typecheck/collect constraints: {err:?}");
        }
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
        assert_split_program_typechecks(&result);

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
        assert_split_program_typechecks(&result);

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
        assert_split_program_typechecks(&result);

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
        let shallow_result = split_program(&list_prog(nested_list(1)));
        let deep_result = split_program(&list_prog(nested_list(5)));
        assert_split_program_typechecks(&shallow_result);
        assert_split_program_typechecks(&deep_result);
        let shallow = shallow_result.data_types.len();
        let deep = deep_result.data_types.len();

        assert!(
            deep > shallow,
            "expected the manually unrolled chain to split into more copies as it gets deeper, got shallow={shallow}, deep={deep}"
        );
    }

    #[test]
    fn split_program_lets_a_self_referential_field_point_at_a_different_copy() {
        let result = split_program(&list_prog(nested_list(2)));
        assert_split_program_typechecks(&result);

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
        assert_split_program_typechecks(&result);

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
        assert_split_program_typechecks(&result);

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
        let shallow_result = split_program(&ab_prog(nested_a(0)));
        let deep_result = split_program(&ab_prog(nested_a(5)));
        assert_split_program_typechecks(&shallow_result);
        assert_split_program_typechecks(&deep_result);
        let shallow = shallow_result.data_types.len();
        let deep = deep_result.data_types.len();

        assert!(
            deep > shallow,
            "expected the manually unrolled mutually recursive chain to split into more copies as it gets deeper, got shallow={shallow}, deep={deep}"
        );
    }

    #[test]
    fn split_program_keeps_a_declarations_own_type_parameter_in_a_field_typed_by_it() {
        let box_ty = || ty!(id!("Box"), [tvar!(id!("C", 2))]);
        let generic_box = data!(
            id!("Box"),
            [ctor_sig!(
                id!("Wrap"),
                [],
                [bind!(id!("x"), prd!(), tvar!(id!("A", 1)))]
            )],
            [tparam!(id!("A", 1), "+")]
        );
        let wrap = def!(
            id!("wrap"),
            [tparam!(id!("C", 2), "+")],
            [
                bind!(id!("x"), prd!(), tvar!(id!("C", 2))),
                bind!(id!("ret"), cns!(), box_ty())
            ],
            cut!(
                ctor!(
                    id!("Wrap"),
                    [],
                    [var!(id!("x"), tvar!(id!("C", 2)))],
                    box_ty()
                ),
                covar!(id!("ret"), box_ty()),
                box_ty()
            )
        );
        let prog = Prog {
            defs: vec![wrap],
            data_types: vec![generic_box],
            codata_types: vec![],
            max_id: 10,
        };

        let result = split_program(&prog);
        assert_split_program_typechecks(&result);

        for copy in &result.data_types {
            let field = &copy.xtors[0].args.bindings[0].ty;
            assert_eq!(
                field,
                &tvar!(copy.type_params[0].id.clone()),
                "expected the field to stay generic in the copy's own type parameter, got {field:?}"
            );
        }
    }

    /// An observation at a concrete occurrence (`Cons(1, Nil)`, so
    /// `i64`) must not freeze the copy's field to that one instantiation either. Only the *head* of
    /// `xs: List[B]` is a splitting decision; `B` stays `B`.
    #[test]
    fn split_program_does_not_freeze_a_field_to_the_instantiation_seen_at_one_occurrence() {
        let list_ty = || ty!(id!("List"), [ty!("int")]);
        let generic_list = data!(
            id!("List"),
            [
                ctor_sig!(id!("Nil"), [], []),
                ctor_sig!(
                    id!("Cons"),
                    [],
                    [
                        bind!(id!("x"), prd!(), tvar!(id!("B", 1))),
                        bind!(id!("xs"), prd!(), ty!(id!("List"), [tvar!(id!("B", 1))]))
                    ]
                )
            ],
            [tparam!(id!("B", 1), "+")]
        );
        let build = def!(
            id!("build"),
            [bind!(id!("ret"), cns!(), list_ty())],
            cut!(
                ctor!(
                    id!("Cons"),
                    [],
                    [lit!(1), ctor!(id!("Nil"), [], [], list_ty())],
                    list_ty()
                ),
                covar!(id!("ret"), list_ty()),
                list_ty()
            )
        );
        let prog = Prog {
            defs: vec![build],
            data_types: vec![generic_list],
            codata_types: vec![],
            max_id: 10,
        };

        let result = split_program(&prog);
        assert_split_program_typechecks(&result);

        let cons_copy = result
            .data_types
            .iter()
            .find(|d| d.xtors.iter().any(|x| x.name.name.starts_with("Cons")))
            .expect("expected a copy that still has Cons");
        let param = tvar!(cons_copy.type_params[0].id.clone());
        let cons = &cons_copy.xtors[0];
        assert_eq!(cons.args.bindings[0].ty, param, "x must stay generic");
        let Ty::Decl {
            name: tail_head,
            type_args,
        } = &cons.args.bindings[1].ty
        else {
            panic!("expected the tail field to be a declared type");
        };
        // only the head is a splitting decision, the type argument stays the copy's own parameter
        assert!(tail_head.name.starts_with("List"));
        assert_eq!(type_args.args, vec![param]);
    }

    #[test]
    fn split_program_ties_a_type_variable_field_to_the_owners_type_argument() {
        let box_ty = || ty!(id!("Box"), [ty!("int")]);
        let list_ty = || ty!(id!("List"), [box_ty()]);
        let generic_box = data!(
            id!("Box"),
            [ctor_sig!(
                id!("Wrap"),
                [],
                [bind!(id!("x"), prd!(), tvar!(id!("A", 1)))]
            )],
            [tparam!(id!("A", 1), "+")]
        );
        let generic_list = data!(
            id!("List"),
            [
                ctor_sig!(id!("Nil"), [], []),
                ctor_sig!(
                    id!("Cons"),
                    [],
                    [
                        bind!(id!("x"), prd!(), tvar!(id!("B", 2))),
                        bind!(id!("xs"), prd!(), ty!(id!("List"), [tvar!(id!("B", 2))]))
                    ]
                )
            ],
            [tparam!(id!("B", 2), "+")]
        );
        let build = def!(
            id!("build"),
            [bind!(id!("ret"), cns!(), list_ty())],
            cut!(
                ctor!(
                    id!("Cons"),
                    [],
                    [
                        ctor!(id!("Wrap"), [], [lit!(1)], box_ty()),
                        ctor!(id!("Nil"), [], [], list_ty())
                    ],
                    list_ty()
                ),
                covar!(id!("ret"), list_ty()),
                list_ty()
            )
        );
        let prog = Prog {
            defs: vec![build],
            data_types: vec![generic_box, generic_list],
            codata_types: vec![],
            max_id: 10,
        };

        let result = split_program(&prog);
        assert_split_program_typechecks(&result);

        let Statement::Cut(cut) = &result.defs[0].body else {
            panic!("expected build's body to be a Cut");
        };
        let Term::Xtor(cons) = cut.producer.as_ref() else {
            panic!("expected the producer to be the Cons occurrence");
        };
        let Ty::Decl { type_args, .. } = &cons.ty else {
            panic!("expected Cons's type to be a declared type");
        };
        let annotated_box = &type_args.args[0];
        let wrapped_box = cons.args.entries[0].get_type();
        assert_eq!(
            annotated_box, &wrapped_box,
            "the Box in `List[Box[i64]]` and the Box of the `Wrap` value stored in it must be the              same copy"
        );
        assert_eq!(
            result
                .data_types
                .iter()
                .filter(|d| d.name.name.starts_with("Box"))
                .count(),
            1,
            "expected a single Box copy, got: {:#?}",
            result.data_types
        );
    }

    // -- Dead-ctor/dtor dropping --------------------------------------------------------------

    /// Neither `List` class here is ever matched, so each keeps exactly the ctors it constructs.
    #[test]
    fn split_program_drops_an_xtor_that_is_never_used_for_a_copy() {
        let def_a = def!(
            id!("a"),
            [bind!(id!("ret"), cns!(), ty!(id!("List")))],
            cut!(
                ctor!(id!("Nil"), [], [], ty!(id!("List"))),
                covar!(id!("ret"), ty!(id!("List"))),
                ty!(id!("List"))
            )
        );
        let def_b = def!(
            id!("b"),
            [bind!(id!("ret"), cns!(), ty!(id!("List")))],
            cut!(
                ctor!(
                    id!("Cons"),
                    [],
                    [lit!(1), ctor!(id!("Nil"), [], [], ty!(id!("List")))],
                    ty!(id!("List"))
                ),
                covar!(id!("ret"), ty!(id!("List"))),
                ty!(id!("List"))
            )
        );
        let prog = Prog {
            defs: vec![def_a, def_b],
            data_types: vec![list_decl()],
            codata_types: vec![],
            max_id: 0,
        };

        let result = split_program(&prog);
        assert_split_program_typechecks(&result);

        let cons_only_copy = result
            .data_types
            .iter()
            .find(|d| d.xtors.iter().any(|x| x.name.name.starts_with("Cons")));
        let cons_only_copy =
            cons_only_copy.expect("expected a split copy whose Cons was actually constructed");
        assert_eq!(
            cons_only_copy.xtors.len(),
            1,
            "expected Nil to be dropped from the Cons-value's own class, it is never used there,              got: {:#?}",
            cons_only_copy.xtors
        );
    }

    /// `f` matches a `List` it is handed by `a`, which only ever builds `Cons(1, Nil)`. Two
    /// classes result: `f`s own (the outer `Cons` plus both clauses) and the one its `Cons` field
    /// points at (only ever the inner `Nil`).
    fn matched_list_prog() -> Prog {
        let f = def!(
            id!("f"),
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
                            cut!(var!(id!("x"), ty!("int")), covar!(id!("ret")), ty!("int"))
                        )
                    ],
                    ty!(id!("List"))
                ),
                ty!(id!("List"))
            )
        );
        let caller = def!(
            id!("a"),
            [bind!(id!("ret"), cns!(), ty!("int"))],
            call!(
                id!("f"),
                [
                    ctor!(
                        id!("Cons"),
                        [],
                        [lit!(1), ctor!(id!("Nil"), [], [], ty!(id!("List")))],
                        ty!(id!("List"))
                    ),
                    covar!(id!("ret"))
                ]
            )
        );
        Prog {
            defs: vec![f, caller],
            data_types: vec![list_decl()],
            codata_types: vec![],
            max_id: 0,
        }
    }

    /// Returns the `XCase` `f`s body cuts against, in an already split program.
    fn f_case(result: &Prog) -> &XCase<Cns> {
        let f_def = result
            .defs
            .iter()
            .find(|d| d.name == id!("f"))
            .expect("expected f to survive rewriting");
        let Statement::Cut(cut) = &f_def.body else {
            panic!("expected f's body to be a Cut, got {:?}", f_def.body);
        };
        let Term::XCase(xcase) = cut.consumer.as_ref() else {
            panic!(
                "expected f's body to cut against an XCase, got {:?}",
                cut.consumer
            );
        };
        xcase
    }

    #[test]
    fn split_program_keeps_an_xtor_that_is_only_matched_never_constructed() {
        let result = split_program(&matched_list_prog());
        assert_split_program_typechecks(&result);

        let clauses = &f_case(&result).clauses;
        assert_eq!(
            clauses.len(),
            2,
            "expected both clauses to survive, matching an xtor is a use of it, got: {clauses:#?}"
        );
        assert!(clauses.iter().any(|c| c.xtor.name.starts_with("Nil")));

        let Ty::Decl {
            name: scrutinee, ..
        } = &f_case(&result).ty
        else {
            panic!("expected the scrutinee's type to be a declared type");
        };
        let matched_copy = result
            .data_types
            .iter()
            .find(|d| d.name == *scrutinee)
            .expect("expected the copy f matches on to exist");
        assert_eq!(
            matched_copy.xtors.len(),
            2,
            "expected the matched copy to keep both xtors, got: {:#?}",
            matched_copy.xtors
        );
    }

    #[test]
    fn split_program_drops_an_unused_xtor_from_a_field_class_while_the_matched_class_keeps_both() {
        let result = split_program(&matched_list_prog());
        assert_split_program_typechecks(&result);

        assert_eq!(result.data_types.len(), 2, "got: {:#?}", result.data_types);
        let tail_copy = result
            .data_types
            .iter()
            .find(|d| d.xtors.len() == 1)
            .expect("expected one copy to have dropped an xtor");
        assert!(
            tail_copy.xtors[0].name.name.starts_with("Nil"),
            "expected Cons to be the dropped one, got: {:#?}",
            tail_copy.xtors
        );

        // and the surviving `Cons` field really points at that shrunk copy
        let matched_copy = result
            .data_types
            .iter()
            .find(|d| d.xtors.len() == 2)
            .expect("expected the matched copy to keep both xtors");
        let cons = matched_copy
            .xtors
            .iter()
            .find(|x| x.name.name.starts_with("Cons"))
            .expect("expected Cons to survive in the matched copy");
        assert_eq!(
            cons.args.bindings[1].ty,
            ty!(tail_copy.name.clone()),
            "expected Cons's tail field to point at the shrunk copy"
        );
    }

    #[test]
    fn split_program_keeps_every_xtor_of_a_completely_unreferenced_declaration() {
        let prog = Prog {
            defs: vec![def!(
                id!("a"),
                [bind!(id!("ret"), cns!(), ty!("int"))],
                cut!(lit!(0), covar!(id!("ret")), ty!("int"))
            )],
            data_types: vec![list_decl()],
            codata_types: vec![],
            max_id: 0,
        };

        let result = split_program(&prog);
        assert_split_program_typechecks(&result);

        assert_eq!(result.data_types.len(), 1);
        assert_eq!(result.data_types[0].name, id!("List"));
        assert_eq!(result.data_types[0].xtors.len(), 2);
    }

    fn pair_decl() -> CodataDeclaration {
        codata!(
            id!("Pair"),
            [dtor_sig!(id!("Fst"), [], []), dtor_sig!(id!("Snd"), [], [])],
            []
        )
    }

    #[test]
    fn split_program_keeps_all_dtors_of_a_codata_value_defined_via_new() {
        let def_a = def!(
            id!("a"),
            [bind!(id!("ret"), cns!(), ty!(id!("Pair")))],
            cut!(
                cocase!(
                    [
                        clause!(Prd, id!("Fst"), [], [], exit!(lit!(1))),
                        clause!(Prd, id!("Snd"), [], [], exit!(lit!(2)))
                    ],
                    ty!(id!("Pair"))
                ),
                covar!(id!("ret"), ty!(id!("Pair"))),
                ty!(id!("Pair"))
            )
        );
        let prog = Prog {
            defs: vec![def_a],
            data_types: vec![],
            codata_types: vec![pair_decl()],
            max_id: 0,
        };

        let result = split_program(&prog);
        assert_split_program_typechecks(&result);

        assert_eq!(result.codata_types.len(), 1);
        assert_eq!(result.codata_types[0].xtors.len(), 2);
    }

    #[test]
    fn split_program_keeps_an_observed_dtor_and_drops_the_unobserved_one() {
        let observe = def!(
            id!("observe"),
            [bind!(id!("p"), prd!(), ty!(id!("Pair")))],
            cut!(
                var!(id!("p"), ty!(id!("Pair"))),
                dtor!(id!("Fst"), [], [], ty!(id!("Pair"))),
                ty!(id!("Pair"))
            )
        );
        let prog = Prog {
            defs: vec![observe],
            data_types: vec![],
            codata_types: vec![pair_decl()],
            max_id: 0,
        };

        let result = split_program(&prog);
        assert_split_program_typechecks(&result);

        assert_eq!(result.codata_types.len(), 1);
        assert_eq!(
            result.codata_types[0].xtors.len(),
            1,
            "expected the never-observed Snd to be dropped, got: {:#?}",
            result.codata_types[0].xtors
        );
        assert!(result.codata_types[0].xtors[0].name.name.starts_with("Fst"));
    }

    #[test]
    fn split_program_keeps_everything_for_a_case_whose_scrutinee_is_never_constructed_anywhere() {
        let dead = def!(
            id!("dead"),
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
                            cut!(var!(id!("x"), ty!("int")), covar!(id!("ret")), ty!("int"))
                        )
                    ],
                    ty!(id!("List"))
                ),
                ty!(id!("List"))
            )
        );
        // No caller anywhere in the program, `dead` is never actually invoked.
        let prog = Prog {
            defs: vec![dead],
            data_types: vec![list_decl()],
            codata_types: vec![],
            max_id: 0,
        };

        let result = split_program(&prog);
        assert_split_program_typechecks(&result);

        assert!(!result.data_types.is_empty());
        for copy in &result.data_types {
            assert_eq!(
                copy.xtors.len(),
                2,
                "expected both Nil and Cons to survive since neither was ever constructed for \
                 this class, got: {:#?}",
                copy.xtors
            );
        }

        let dead_def = result
            .defs
            .iter()
            .find(|d| d.name == id!("dead"))
            .expect("expected dead to survive rewriting");
        let Statement::Cut(cut) = &dead_def.body else {
            panic!("expected dead's body to be a Cut, got {:?}", dead_def.body);
        };
        let Term::XCase(xcase) = cut.consumer.as_ref() else {
            panic!(
                "expected dead's body to cut against an XCase, got {:?}",
                cut.consumer
            );
        };
        assert_eq!(
            xcase.clauses.len(),
            2,
            "expected both clauses to survive, got: {:#?}",
            xcase.clauses
        );
    }
}
