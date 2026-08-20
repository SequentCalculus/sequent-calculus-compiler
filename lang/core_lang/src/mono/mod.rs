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
