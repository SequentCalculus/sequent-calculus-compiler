//! Core monomorphization infrastructure.

use printer::{Print, PrintCfg};

use crate::{
    mono::{
        constraint_graph::ConstraintGraph,
        constraints::{ConstraintCollector, FlowConstraintSet},
        erasure::ErasedDecls,
        errors::MonoError,
        graph_viz::VizOutput,
        growing_cycle::find_all_growing_cycles,
        solver::{Solution, solve_with_erasure},
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
fn constraints_of(program: &Prog) -> Result<FlowConstraintSet, MonoError> {
    let env = GlobalEnv::new(&program.data_types, &program.codata_types, &program.defs);
    program.collect_constraints(&env)
}

/// Whether [`split_if_needed`] ran type splitting, and why not if it didn't, both non-splitting
/// cases leave erasure alone to widen every growing declaration directly on the unsplit program,
/// but they mean different things for interpreting `--debug` output or benchmark results: one
/// says splitting had nothing to do, the other says it was explicitly asked not to run.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SplitOutcome {
    /// No growing cycle existed, so there was nothing to split.
    NoCycle,
    /// A growing cycle existed, but splitting was disabled (`--no-split`); erasure runs directly
    /// on the unsplit program instead.
    Disabled,
    /// A growing cycle existed and splitting ran.
    Split,
}

/// Collects `program`'s flow constraints and, if they contain a growing cycle and `split` is
/// enabled, runs type splitting and re-collects them on the split program. Splitting only ever
/// helps, it can only reduce how many growing cycles remain, never introduce one, so a single
/// conditional pass suffices. There is no need to re-check the split program's own constraints
/// for a growing cycle.
///
/// Returns the (possibly split) program alongside its constraints, the constraint graph built
/// from them, and the [`SplitOutcome`] (purely for `--debug` reporting).
fn split_if_needed(
    program: Prog,
    split: bool,
) -> Result<(Prog, FlowConstraintSet, ConstraintGraph, SplitOutcome), MonoError> {
    let constraints = constraints_of(&program)?;
    let graph = ConstraintGraph::from(constraints.clone());

    if find_all_growing_cycles(&graph).is_empty() {
        return Ok((program, constraints, graph, SplitOutcome::NoCycle));
    }
    if !split {
        return Ok((program, constraints, graph, SplitOutcome::Disabled));
    }

    let program = split_program(&program);
    let constraints = constraints_of(&program)?;
    let graph = ConstraintGraph::from(constraints.clone());
    Ok((program, constraints, graph, SplitOutcome::Split))
}

/// The `PrintCfg` every `--debug` report below renders with: unbounded width and forced
/// linebreaks, so the output stays readable regardless of terminal size.
fn debug_print_cfg() -> PrintCfg {
    PrintCfg {
        width: 0,
        allow_linebreaks: true,
        indent: 4,
        ..PrintCfg::default()
    }
}

/// Reports the outcome of `split_if_needed`: whether (and why not) a growing cycle forced type
/// splitting, the (possibly split) program, and the flow constraints collected from it.
fn print_splitting_debug(program: &Prog, constraints: &FlowConstraintSet, outcome: SplitOutcome) {
    let cfg = debug_print_cfg();
    let headline = match outcome {
        SplitOutcome::NoCycle => "No growing cycle found -- skipped type splitting:",
        SplitOutcome::Disabled => {
            "Growing cycle found -- type splitting disabled (--no-split), erasure runs on the
             unsplit program:"
        }
        SplitOutcome::Split => "Growing cycle found -- ran type splitting:",
    };
    println!(
        "{headline}\n{}",
        program.print_to_colored_string(Some(&cfg))
    );
    println!(
        "Flow Constraints: \n{}",
        constraints.print_to_colored_string(Some(&cfg))
    );
}

/// Reports the outcome of `solve_with_erasure`: which constraints and declarations, if any, were
/// erased to break a growing cycle, and the resulting solution.
fn print_solving_debug(
    erased_constraints: &FlowConstraintSet,
    erased_decls: &ErasedDecls,
    solution: &Solution,
) {
    let cfg = debug_print_cfg();
    if !erased_constraints.constraints.is_empty() {
        println!(
            "Erased Constraints: \n{}",
            erased_constraints.print_to_string(Some(&cfg))
        );
    }
    if !erased_decls.is_empty() {
        println!(
            "Erased Declarations: {}",
            erased_decls
                .iter()
                .map(|id| id.print_to_string(Some(&cfg)))
                .collect::<Vec<_>>()
                .join(", ")
        );
    }
    println!("Solution: \n{}", solution.print_to_string(Some(&cfg)));
}

/// Renders `graph` as requested by `viz`, a no-op for [`VizOutput::Disabled`].
fn render_viz(graph: &ConstraintGraph, viz: VizOutput) -> Result<(), MonoError> {
    let result = match viz {
        VizOutput::Disabled => return Ok(()),
        VizOutput::DefaultPath => {
            graph.render_as(graph_viz::OutputFormat::Png, None::<&std::path::Path>)
        }
        VizOutput::Path(path) => graph.render_as(graph_viz::OutputFormat::Png, Some(path)),
    };
    result.map_err(|e| MonoError::Contextual {
        msg: format!("failed to render constraint graph visualization: {e}"),
    })
}

/// Monomorphizes a program and returns the monomorphized program.
///
/// `split` controls whether type splitting runs when a growing cycle is found.
///
/// Fails if constraint collection rejects the program (a bug in an earlier phase, since Fun/Core
/// type checking already ran) or, when `viz` requests rendering, if rendering the constraint
/// graph visualization fails (e.g. the `dot` binary is missing or the output path is not
/// writable).
pub fn monomorphize_program(
    program: Prog,
    debug: bool,
    viz: VizOutput,
    split: bool,
) -> Result<Prog, MonoError> {
    let (program, constraints, graph, outcome) = split_if_needed(program, split)?;
    if debug {
        print_splitting_debug(&program, &constraints, outcome);
    }

    let (solution, erased_decls, erased_constraints) = solve_with_erasure(constraints);

    render_viz(&graph, viz)?;
    if debug {
        print_solving_debug(&erased_constraints, &erased_decls, &solution);
    }

    specialize_program(&program, &solution, &erased_decls)
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
        let first =
            monomorphize_program(mutually_recursive_prog(), false, VizOutput::Disabled, true)
                .expect("fixture must monomorphize");

        for run in 1..20 {
            let again =
                monomorphize_program(mutually_recursive_prog(), false, VizOutput::Disabled, true)
                    .expect("fixture must monomorphize");
            assert_eq!(
                first, again,
                "monomorphization is not deterministic -- run {run} differs from the first"
            );
        }
    }
}
