//! Core typechecking infrastructure.

use printer::{Print, PrintCfg};

use crate::{
    mono::{
        constraint_graph::ConstraintGraph, constraints::ConstraintCollector, solver::solve,
        specialize::specialize_program,
    },
    syntax::program::Prog,
};
pub mod constraint_graph;
pub mod constraints;
pub mod errors;
pub mod graph_viz;
pub mod growing_cycle;
pub mod naming_table;
pub mod position;
pub mod solver;
pub mod specialize;

/// Monomorphizes a program and returns the monomorphized program along with the constraint graph.
pub fn monomorphize_program(program: Prog) -> (Prog, ConstraintGraph) {
    let constraints = program
        .collect_constraints(&program.data_types, &program.codata_types)
        .unwrap();

    let forced_set_cfg = PrintCfg {
        width: 0,
        allow_linebreaks: true,
        indent: 4,
        ..PrintCfg::default()
    };

    println!(
        "{}",
        constraints.print_to_colored_string(Some(&forced_set_cfg))
    );

    let graph = ConstraintGraph::from(constraints);
    let solution = solve(&graph).unwrap();
    println!("{}", solution.print_to_string(Some(&forced_set_cfg)));

    let mono_prog = specialize_program(&program, &solution);

    (mono_prog, graph)
}
