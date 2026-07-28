//! Core typechecking infrastructure.

use printer::{Print, PrintCfg};

use crate::{
    mono::{
        constraint_graph::ConstraintGraph, constraints::ConstraintCollector, errors::MonoError,
        solver::solve_with_erasure, specialize::specialize_program,
    },
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

/// Monomorphizes a program and returns the monomorphized program along with the constraint graph.
pub fn monomorphize_program(
    program: Prog,
    debug: bool,
    viz_path: Option<Option<std::path::PathBuf>>,
) -> Result<Prog, MonoError> {
    let constraints = program
        .collect_constraints(&GlobalEnv::new(
            &program.data_types,
            &program.codata_types,
            &program.defs,
        ))
        .unwrap();

    let forced_set_cfg = PrintCfg {
        width: 0,
        allow_linebreaks: true,
        indent: 4,
        ..PrintCfg::default()
    };

    if debug {
        println!(
            "Flow Constraints: \n{}",
            constraints.print_to_colored_string(Some(&forced_set_cfg))
        );
    }

    let (solution, erased_decls, erased_constraints) = solve_with_erasure(constraints.clone());

    let graph = ConstraintGraph::from(constraints.clone());

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
