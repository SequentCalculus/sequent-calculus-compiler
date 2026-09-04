//! Core monomorphization infrastructure.

use printer::{Print, PrintCfg};

use crate::{
    mono::{
        constraint_graph::ConstraintGraph,
        constraints::{ConstraintCollector, FlowConstraintSet},
        errors::MonoError,
        growing_cycle::find_all_growing_cycles,
        solver::solve_with_erasure,
        specialize::specialize_program,
    },
    splitting::split_program,
    syntax::program::Prog,
    typing::env::GlobalEnv,
};
pub mod constraint_graph;
pub mod constraints;
pub mod erasure;
pub mod errors;
pub mod graph_viz;
pub mod growing_cycle;
pub mod naming_table;
pub mod position;
pub mod solver;
pub mod specialize;

/// Type-checks and collects the flow constraints of `program`. The `GlobalEnv` only borrows
/// `program`'s own declaration vectors, so it must be rebuilt for every distinct `Prog` value.
/// In particular after type splitting, which renames declarations and xtors.
fn constraints_of(program: &Prog) -> FlowConstraintSet {
    let env = GlobalEnv::new(&program.data_types, &program.codata_types, &program.defs);
    program.collect_constraints(&env).unwrap()
}

/// Monomorphizes a program and returns the monomorphized program.
pub fn monomorphize_program(
    program: Prog,
    debug: bool,
    viz_path: Option<Option<std::path::PathBuf>>,
) -> Result<Prog, MonoError> {
    let constraints = constraints_of(&program);
    let graph = ConstraintGraph::from(constraints.clone());

    let growing_cycle_found = !find_all_growing_cycles(&graph).is_empty();
    let (program, constraints, graph) = if growing_cycle_found {
        let program = split_program(&program);
        let constraints = constraints_of(&program);
        let graph = ConstraintGraph::from(constraints.clone());
        (program, constraints, graph)
    } else {
        (program, constraints, graph)
    };

    let forced_set_cfg = PrintCfg {
        width: 0,
        allow_linebreaks: true,
        indent: 4,
        ..PrintCfg::default()
    };

    if debug {
        if growing_cycle_found {
            println!(
                "Growing cycle found -- ran type splitting:\n{}",
                program.print_to_colored_string(Some(&forced_set_cfg))
            );
        } else {
            println!(
                "No growing cycle found -- skipped type splitting:\n{}",
                program.print_to_colored_string(Some(&forced_set_cfg))
            );
        }

        println!(
            "Flow Constraints: \n{}",
            constraints.print_to_colored_string(Some(&forced_set_cfg))
        );
    }

    let (solution, erased_decls, erased_constraints) = solve_with_erasure(constraints.clone());

    if let Some(path) = viz_path {
        graph.render_as(graph_viz::OutputFormat::Png, path).unwrap();
    }

    if debug {
        if !erased_constraints.constraints.is_empty() {
            println!(
                "Erased Constraints: \n{}",
                erased_constraints.print_to_string(Some(&forced_set_cfg))
            );
        }
        if !erased_decls.0.is_empty() {
            println!(
                "Erased Declarations: {}",
                erased_decls
                    .0
                    .iter()
                    .map(|id| id.print_to_string(Some(&forced_set_cfg)))
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }
        println!(
            "Solution: \n{}",
            solution.print_to_string(Some(&forced_set_cfg))
        );
    }

    let mono_prog = specialize_program(&program, &solution, &erased_decls);

    Ok(mono_prog)
}

#[cfg(test)]
mod determinism_tests {
    use super::*;
    use crate::syntax::{Cns, DataDeclaration, Def, Identifier, Ty, terms::Literal};
    extern crate self as core_lang;
    use core_macros::{
        bind, call, case, clause, cns, covar, ctor_sig, cut, data, def, id, prd, tparam, tvar, ty,
        var,
    };

    fn growing(arg: Ty) -> Ty {
        ty!(id!("Growing"), [arg])
    }

    /// `data Growing[B+] { Base(x: B), Grow(next: Growing[Growing[B]]) }` -- the `Grow` field grows
    /// by one level per step, which is what makes the constraint graph cyclic in the first place.
    fn growing_decl() -> DataDeclaration {
        data!(
            id!("Growing"),
            [
                ctor_sig!(
                    id!("Base"),
                    [],
                    [bind!(id!("x"), prd!(), tvar!(id!("B", 2)))]
                ),
                ctor_sig!(
                    id!("Grow"),
                    [],
                    [bind!(
                        id!("next"),
                        prd!(),
                        growing(growing(tvar!(id!("B", 2))))
                    )]
                )
            ],
            [tparam!(id!("B", 2), "+")]
        )
    }

    /// One half of a mutually recursive pair, both matching directly on `Growing[A]` and calling
    /// the other one at `Growing[A]`. Two *separate* defs recursing into each other do not converge
    /// under Type Splitting, so the growing cycle survives into the solver and erasure has to break
    /// it -- exactly the situation in which the choice of erasure target was observed to vary.
    fn level_def(name: &str, other: &str, result: i64) -> Def {
        let a = || tvar!(id!("A", 1));
        def!(
            Identifier::new(name.to_string()),
            [tparam!(id!("A", 1), "+")],
            [
                bind!(id!("g"), prd!(), growing(a())),
                bind!(id!("a0"), cns!(), ty!("int"))
            ],
            cut!(
                var!(id!("g"), growing(a())),
                case!(
                    [
                        clause!(
                            Cns,
                            id!("Base"),
                            [],
                            [bind!(id!("x"), prd!(), a())],
                            cut!(
                                Literal { lit: result },
                                covar!(id!("a0"), ty!("int")),
                                ty!("int")
                            )
                        ),
                        clause!(
                            Cns,
                            id!("Grow"),
                            [],
                            [bind!(id!("next"), prd!(), growing(growing(a())))],
                            call!(
                                Identifier::new(other.to_string()),
                                [growing(a())],
                                [
                                    var!(id!("next"), growing(growing(a()))),
                                    covar!(id!("a0"), ty!("int"))
                                ]
                            )
                        )
                    ],
                    growing(a())
                ),
                growing(a())
            )
        )
    }

    /// Seeds the solver with a ground instantiation (`Growing[i64]`); without it there is nothing
    /// for the fixpoint to propagate.
    fn start_def() -> Def {
        def!(
            id!("start"),
            [],
            [
                bind!(id!("g"), prd!(), growing(ty!("int"))),
                bind!(id!("a0"), cns!(), ty!("int"))
            ],
            call!(
                id!("evenLevel"),
                [ty!("int")],
                [
                    var!(id!("g"), growing(ty!("int"))),
                    covar!(id!("a0"), ty!("int"))
                ]
            )
        )
    }

    fn mutually_recursive_prog() -> Prog {
        Prog {
            defs: vec![
                level_def("evenLevel", "oddLevel", 0),
                level_def("oddLevel", "evenLevel", 1),
                start_def(),
            ],
            data_types: vec![growing_decl()],
            codata_types: vec![],
            max_id: 100,
        }
    }

    /// Monomorphizing the same program twice must yield the very same program.
    #[test]
    fn monomorphizing_the_same_program_twice_yields_the_same_program() {
        let first = monomorphize_program(mutually_recursive_prog(), false, None)
            .expect("fixture must monomorphize");

        for run in 1..20 {
            let again = monomorphize_program(mutually_recursive_prog(), false, None)
                .expect("fixture must monomorphize");
            assert_eq!(
                first, again,
                "monomorphization is not deterministic -- run {run} differs from the first"
            );
        }
    }
}
