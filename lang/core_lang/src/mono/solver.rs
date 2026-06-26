use std::{
    collections::{HashMap, HashSet, VecDeque},
    ops::Deref,
};

use printer::{Alloc, Builder, DocAllocator, Print, PrintCfg};

use crate::{
    mono::{
        constraint_graph::{ConstraintGraph, Edge, Node, VarLocations},
        errors::MonoError,
        growing_cycle::find_growing_cycle,
        position::Position,
    },
    syntax::{Identifier, Ty, types::TypeArgs},
};

/// Maps each [`Node`] to the set of concrete ground vectors it may be instantiated with.
///
/// Each element of the set is a full vector, e.g. `[i64, Bool]` for a `Pair`
/// node `[A, B]`, preserving the correlation between positions. This is the
/// output of the solving phase and the direct input to specialization.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Solution {
    pub map: HashMap<Node, HashSet<Vec<Ty>>>,
}

impl Deref for Solution {
    type Target = HashMap<Node, HashSet<Vec<Ty>>>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl From<HashMap<Node, HashSet<Vec<Ty>>>> for Solution {
    fn from(map: HashMap<Node, HashSet<Vec<Ty>>>) -> Self {
        Solution { map }
    }
}

impl Print for Solution {
    fn print<'a>(&'a self, cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        if self.map.is_empty() {
            return alloc.text("{}");
        }

        let mut sorted_entries: Vec<(&Node, &HashSet<Vec<Ty>>)> = self.map.iter().collect();
        sorted_entries.sort_by(|a, b| a.0.cmp(b.0));

        let body = alloc.intersperse(
            sorted_entries.into_iter().map(|(node, ty_sets)| {
                let mut sorted_solutions: Vec<&Vec<Ty>> = ty_sets.iter().collect();
                sorted_solutions.sort();

                let solutions_disp = alloc.intersperse(
                    sorted_solutions.into_iter().map(|ty_vec| {
                        let tys = alloc.intersperse(
                            ty_vec.iter().map(|ty| ty.print(cfg, alloc)),
                            alloc.text(", "),
                        );
                        alloc.text("[").append(tys).append(alloc.text("]"))
                    }),
                    alloc.text(", "),
                );

                let mut inline_cfg = cfg.clone();
                inline_cfg.allow_linebreaks = false;

                let key_disp = alloc
                    .text("[")
                    .append(node.print(&inline_cfg, alloc))
                    .append(alloc.text("]"));

                key_disp.append(alloc.text(" -> ")).append(solutions_disp)
            }),
            alloc.line(),
        );

        alloc
            .text("{")
            .append(alloc.line().append(body).nest(cfg.indent))
            .append(alloc.line())
            .append(alloc.text("}"))
            .group()
    }
}
/// Runs the worklist fixpoint solver over the constraint graph.
///
/// Starts from the seed vectors and repeatedly propagates vectors through
/// edges until no node's solution changes.
pub fn solve(graph: &ConstraintGraph) -> Result<Solution, MonoError> {
    if let Some(growing_cycle) = find_growing_cycle(graph) {
        return Err(MonoError::PolymorphicRecursion {
            cycle: growing_cycle,
        });
    }

    let mut solution: HashMap<Node, HashSet<Vec<Ty>>> = graph
        .nodes
        .iter()
        .map(|n| (n.clone(), HashSet::new()))
        .collect();

    for (node, seed_tuples) in &graph.seeds {
        solution
            .entry(node.clone())
            .or_default()
            .extend(seed_tuples.iter().cloned());
    }

    let mut in_worklist: HashSet<Node> = graph.seeds.keys().cloned().collect();
    let mut worklist: VecDeque<Node> = in_worklist.iter().cloned().collect();

    while let Some(changed) = worklist.pop_front() {
        in_worklist.remove(&changed);

        for edge in graph.outgoing(&changed) {
            let new_tuples = propagate(edge, &graph.locations, &solution);
            if new_tuples.is_empty() {
                continue;
            }

            let target = solution.entry(edge.into.clone()).or_default();
            let before = target.len();
            target.extend(new_tuples);

            if target.len() > before && in_worklist.insert(edge.into.clone()) {
                worklist.push_back(edge.into.clone());
            }
        }
    }

    Ok(solution.into())
}

/// Computes the new vector that flow through a single edge given the
/// current solution.
///
/// Builds the Cartesian product of the vector sets of all distinct source
/// nodes the edge depends on, then resolves each position of the edge
/// against the chosen combination. When the edge depends on exactly one
/// source node - including when several positions reference different
/// indices of that *same* node, as in `Pair[A, B]` - the "product" is just
/// an iteration over that node's own vectors: no spurious combinations are
/// introduced, because the correlation between positions is already baked
/// into each vector stored for that node.
pub fn propagate(
    edge: &Edge,
    locations: &VarLocations,
    solution: &HashMap<Node, HashSet<Vec<Ty>>>,
) -> HashSet<Vec<Ty>> {
    let source_nodes = edge.source_nodes(locations);
    if source_nodes.is_empty() {
        // An edge with no variable-dependent positions would have been
        // classified as a seed during graph construction, so this should
        // not occur in practice.
        return HashSet::new();
    }

    // Build the Cartesian product of the current vector sets of all distinct
    // source nodes. `combinations[k]` is a vector with one chosen vector per
    // source node, in the same order as `source_nodes`.
    let mut combinations: Vec<Vec<Vec<Ty>>> = vec![vec![]];
    for node in &source_nodes {
        let vectors = solution.get(node).cloned().unwrap_or_default();
        let mut next = Vec::with_capacity(combinations.len() * vectors.len().max(1));
        for partial in &combinations {
            for vector in &vectors {
                let mut extended = partial.clone();
                extended.push(vector.clone());
                next.push(extended);
            }
        }
        combinations = next;
    }

    let mut results = HashSet::new();
    for combo in &combinations {
        let resolved: Vec<Ty> = edge
            .positions
            .iter()
            .map(|pos| resolve_position(pos, &source_nodes, combo, locations))
            .collect();
        results.insert(resolved);
    }
    results
}

/// Resolves a single position to a concrete type, given one chosen
/// combination of source vectors.
///
/// `combo[k]` is the vector chosen for `source_nodes[k]`; this function finds
/// which node a variable position belongs to, looks up the corresponding
/// vector in `combo`, and reads off the component at the variable's index.
fn resolve_position(
    pos: &Position,
    source_nodes: &[Node],
    combo: &[Vec<Ty>],
    locations: &VarLocations,
) -> Ty {
    match pos {
        Position::Ground(ty) => ty.clone(),
        Position::Variable { template, vars } => {
            let resolved: HashMap<&Identifier, &Ty> = vars
                .iter()
                .map(|v| (v, lookup_value(v, source_nodes, combo, locations)))
                .collect();
            substitute(template, &resolved)
        }
    }
}

/// Substitutes every occurrence of a variable in `ty` with its resolved
/// value from `resolved`. Variables not present in `resolved` are left
/// unchanged (should not occur in practice once all positions are classified
/// and their variables located).
fn substitute(ty: &Ty, resolved: &HashMap<&Identifier, &Ty>) -> Ty {
    match ty {
        Ty::I64 => Ty::I64,
        Ty::Var(id) => resolved
            .get(id)
            .map(|t| (*t).clone())
            .unwrap_or_else(|| ty.clone()),
        Ty::Decl { name, type_args } => Ty::Decl {
            name: name.clone(),
            type_args: TypeArgs {
                args: type_args
                    .args
                    .iter()
                    .map(|a| substitute(a, resolved))
                    .collect(),
            },
        },
    }
}

/// Looks up the resolved value of a variable within the chosen combination
/// of source vectors.
fn lookup_value<'a>(
    var: &Identifier,
    source_nodes: &[Node],
    combo: &'a [Vec<Ty>],
    locations: &VarLocations,
) -> &'a Ty {
    let owning_node = locations.node_of(var);
    let node_position = source_nodes
        .iter()
        .position(|n| n == &owning_node)
        .expect("owning node must be among the edge's source nodes");
    let index = locations.index_of(var);
    &combo[node_position][index]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mono::{
        constraint_graph::ConstraintGraph,
        constraints::{FlowConstraint, FlowConstraintSet},
        errors::MonoError,
    };
    extern crate self as core_lang;
    use core_macros::{id, tvar, ty};

    #[test]
    fn test_solve_basic_propagation() {
        let mut set = FlowConstraintSet::new();

        set.insert(FlowConstraint {
            from: vec![ty!("int")],
            to: vec![id!("A", 1)],
        });

        set.insert(FlowConstraint {
            from: vec![tvar!(id!("A", 1))],
            to: vec![id!("B", 2)],
        });

        let graph = ConstraintGraph::from(set);
        let result = solve(&graph);

        assert!(result.is_ok(), "Solver failed with error: {:?}", result);
        let solution = result.unwrap();

        let node_a = vec![id!("A", 1)];
        let node_b = vec![id!("B", 2)];

        let expected_ty_vector = vec![ty!("int")];

        assert!(solution[&node_a].contains(&expected_ty_vector));
        assert!(solution[&node_b].contains(&expected_ty_vector));
    }

    #[test]
    fn test_solve_cartesian_product_and_constructor() {
        let mut set = FlowConstraintSet::new();

        // Provide multiple types for A (Node 1)
        set.insert(FlowConstraint {
            from: vec![ty!("int")],
            to: vec![id!("A", 1)],
        });
        set.insert(FlowConstraint {
            from: vec![ty!(id!("float"))],
            to: vec![id!("A", 1)],
        });

        // Provide multiple types for B (Node 2)
        set.insert(FlowConstraint {
            from: vec![ty!(id!("bool"))],
            to: vec![id!("B", 2)],
        });
        set.insert(FlowConstraint {
            from: vec![ty!(id!("string"))],
            to: vec![id!("B", 2)],
        });

        // Combine them into Pair[A, B] -> C
        set.insert(FlowConstraint {
            from: vec![ty!(id!("Pair"), [tvar!(id!("A", 1)), tvar!(id!("B", 2))])],
            to: vec![id!("C", 3)],
        });

        let graph = ConstraintGraph::from(set);
        let result = solve(&graph);

        assert!(result.is_ok());
        let solution = result.unwrap();

        let node_c = vec![id!("C", 3)];

        // We expect exactly 4 combinations (2 * 2) due to the Cartesian product
        let expected_1 = vec![ty!(id!("Pair"), [ty!("int"), ty!(id!("bool"))])];
        let expected_2 = vec![ty!(id!("Pair"), [ty!("int"), ty!(id!("string"))])];
        let expected_3 = vec![ty!(id!("Pair"), [ty!(id!("float")), ty!(id!("bool"))])];
        let expected_4 = vec![ty!(id!("Pair"), [ty!(id!("float")), ty!(id!("string"))])];

        let solver_solutions_for_c = &solution[&node_c];

        assert_eq!(solver_solutions_for_c.len(), 4);
        assert!(solver_solutions_for_c.contains(&expected_1));
        assert!(solver_solutions_for_c.contains(&expected_2));
        assert!(solver_solutions_for_c.contains(&expected_3));
        assert!(solver_solutions_for_c.contains(&expected_4));
    }

    #[test]
    fn test_solve_detects_growing_cycle() {
        let mut set = FlowConstraintSet::new();

        set.insert(FlowConstraint {
            from: vec![ty!(id!("List"), [tvar!(id!("A", 1))])],
            to: vec![id!("A", 1)],
        });

        let graph = ConstraintGraph::from(set);
        let result = solve(&graph);

        assert!(result.is_err(),);

        let err = result.unwrap_err();
        assert!(
            matches!(err, MonoError::PolymorphicRecursion { .. }),
            "Expected MonoError::PolymorphicRecursion, got: {:?}",
            err
        );

        if let MonoError::PolymorphicRecursion { cycle } = err {
            let node_a = vec![id!("A", 1)];
            assert_eq!(
                cycle,
                vec![node_a],
                "Expected cycle to contain only node A, got: {:?}",
                cycle
            );
        }
    }
}
