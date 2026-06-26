use crate::mono::constraints::FlowConstraintSet;
use crate::mono::position::Position;
use crate::syntax::{Identifier, Ty};
use std::collections::{HashMap, HashSet};

/// A node in the constraint graph: the vector of type parameters
/// belonging to one declaration site, in declared order.
///
/// For an ordinary single-parameter declaration like `List[A]`, a node is a
/// singleton vector `[A]`. For a multi-parameter declaration like
/// `Pair[A, B]`, the node is the vector `[A, B]`. Treating the whole parameter
/// list as one node.
pub type Node = Vec<Identifier>;

/// Tracks, for each type variable, which node it belongs to and
/// at which index within that node's vector.
///
/// This registry is what allows a bare reference like `Ty::Var(A)` appearing
/// in some `from` position to be resolved back to "index 0 of the `Pair`
/// node `[A, B]`", so that propagation can pull the correct component out
/// of a correlated tuple rather than treating `A` as an isolated variable.
#[derive(Debug, Default, Clone)]
pub struct VarLocations {
    location: HashMap<Identifier, (Node, usize)>,
}

impl VarLocations {
    /// Registers a [`Node`], recording the location of each of its members.
    ///
    /// Calling this multiple times with the same node is harmless. Calling
    /// it with an identifier that was previously registered under a
    /// *different* node is a bug in constraint collection: identifiers are
    /// minted uniquely per type parameter declaration and should always be
    /// grouped the same way.
    pub fn register(&mut self, node: &Node) {
        for (index, id) in node.iter().enumerate() {
            match self.location.get(id) {
                Some((existing_node, existing_index)) => {
                    debug_assert!(
                        existing_node == node && *existing_index == index,
                        "identifier {:?} registered with inconsistent node groupings: \
                         previously {:?} at {}, now {:?} at {}",
                        id,
                        existing_node,
                        existing_index,
                        node,
                        index
                    );
                }
                None => {
                    self.location.insert(id.clone(), (node.clone(), index));
                }
            }
        }
    }

    /// Returns the node that the given [`Identifier`] belongs to.
    ///
    /// If the identifier was never registered as part of any node, it is
    /// treated as its own singleton node. This covers ordinary
    /// single-parameter declarations that never appear bundled with others.
    pub fn node_of(&self, id: &Identifier) -> Node {
        self.location
            .get(id)
            .map(|(node, _)| node.clone())
            .unwrap_or_else(|| vec![id.clone()])
    }

    /// Returns the index of the given [`Identifier`] within its node's vector.
    pub fn index_of(&self, id: &Identifier) -> usize {
        self.location.get(id).map(|(_, index)| *index).unwrap_or(0)
    }
}

/// A directed edge between two type variables in the constraint graph.
///
/// The shape of `from` encodes both the source variable and any constructor
/// wrapping that is applied to types as they flow through this edge:
///
/// - `Ty::Var(α)` is a **flat edge**: every type in `S(α)` flows unchanged into `into`.
/// - `Ty::Decl { name: T, args: [Ty::Var(α)] }` is a **constructor edge**: for each
///   `ρ ∈ S(α)`, the wrapped type `T[ρ]` flows into `into`.
#[derive(Debug, Clone)]
pub struct Edge {
    /// Classified positions, one per element of the original `from` vector, in order.
    pub positions: Vec<Position>,
    /// The target node receiving propagated vectors.
    pub into: Node,
}

impl Edge {
    /// Returns the distinct source nodes this edge depends on, deduplicated.
    pub fn source_nodes(&self, locations: &VarLocations) -> Vec<Node> {
        let mut nodes: Vec<Node> = self
            .positions
            .iter()
            .flat_map(Position::vars)
            .map(|id| locations.node_of(id))
            .collect();
        nodes.sort();
        nodes.dedup();
        nodes
    }

    /// Returns `true` if at least one position applies a type constructor.
    ///
    /// Such edges are the only ones that can form growing cycles, since
    /// only constructor application causes a solution to grow in structural
    /// size as it travels around a cycle.
    pub fn has_constructor_position(&self) -> bool {
        self.positions.iter().any(|p| match p {
            Position::Ground(_) => false,
            Position::Variable { template, .. } => !matches!(template, Ty::Var(_)),
        })
    }

    /// Returns the original `from` types for this edge, one per position,
    /// reconstructed from the internal position classification. Used for
    /// display purposes such as graph visualization.
    pub fn from_types(&self) -> Vec<Ty> {
        self.positions.iter().map(|p| p.as_ty().clone()).collect()
    }
}

/// The constraint graph built from a [`FlowConstraintSet`].
///
/// Constraints are split into two disjoint categories:
///
/// - **Seeds** hold ground types that enter a variable directly without passing
///   through any intermediate variable first. They correspond to constraints
///   whose source is already fully concrete, such as `i64 ⊑ γ` or `List[i64] ⊑ γ`.
/// - **Edges** hold variable-to-variable flows, either flat (`γ₁ ⊑ γ₂`) or
///   constructor-wrapped (`T[γ₁] ⊑ γ₂`). Edges are indexed by source variable
///   so the fixpoint solver can quickly find all edges to re-evaluate when a
///   solution set grows.
#[derive(Debug, Default)]
pub struct ConstraintGraph {
    /// All known nodes (vectors of type parameters).
    pub nodes: HashSet<Node>,
    /// Ground vectors seeding each node directly, preserving the correlation between positions within each vector.
    pub seeds: HashMap<Node, HashSet<Vec<Ty>>>,
    /// Outgoing edges for each node, indexed by source node. An edge whose
    /// positions reference several distinct source nodes appears once
    /// under each of those nodes, so the solver can find it regardless of
    /// which contributing node last changed.
    pub edges: HashMap<Node, Vec<Edge>>,
    /// Registry mapping each identifier to its owning node and index.
    pub locations: VarLocations,
}

impl From<FlowConstraintSet> for ConstraintGraph {
    /// Builds a constraint graph from the given constraint set.
    ///
    /// Construction happens in two passes:
    ///
    /// 1. Every constraint's `to` vector is registered as a node. This fixes
    ///    the canonical grouping and index of each identifier *before* any
    ///    edge is classified, so that a bare variable referenced in some
    ///    other constraint's `from` position is correctly recognized as
    ///    belonging to, say, index `1` of the `Pair` node `[A, B]`, rather
    ///    than being mistaken for an unrelated singleton node.
    /// 2. Each constraint is classified as either a seed (every position
    ///    ground) or an edge (at least one position depends on a variable).
    ///
    /// # Panics
    ///
    /// Panics if a constraint's `from` and `to` vectors have different
    /// lengths, or if any position of `to` is not a [`Ty::Var`].
    fn from(constraints: FlowConstraintSet) -> Self {
        let mut graph = ConstraintGraph::default();

        // Pass 1: register every target vector as a node.
        for constraint in &constraints.constraints {
            let node = constraint.to.clone();
            graph.locations.register(&node);
            graph.nodes.insert(node);
        }

        // Pass 2: classify each constraint as a seed or an edge.
        for constraint in &constraints.constraints {
            assert_eq!(
                constraint.from.len(),
                constraint.to.len(),
                "FlowConstraint from/to vectors must have matching lengths, got: {:?}",
                constraint
            );

            let into = constraint.to.clone();
            let positions: Vec<Position> = constraint.from.iter().map(Position::classify).collect();

            if positions.iter().all(|p| matches!(p, Position::Ground(_))) {
                // Every position is concrete: record the whole vector as one
                // correlated ground instantiation of `into`.
                let tuple: Vec<Ty> = positions
                    .iter()
                    .map(|p| match p {
                        Position::Ground(ty) => ty.clone(),
                        _ => unreachable!("checked by the all(...) guard above"),
                    })
                    .collect();
                graph.seeds.entry(into).or_default().insert(tuple);
                continue;
            }

            let edge = Edge {
                positions,
                into: into.clone(),
            };
            let source_nodes = edge.source_nodes(&graph.locations);
            for source in source_nodes {
                graph.nodes.insert(source.clone());
                graph.edges.entry(source).or_default().push(edge.clone());
            }
        }

        graph
    }
}

impl ConstraintGraph {
    /// Returns all outgoing edges from the given node as a slice.
    pub fn outgoing(&self, node: &Node) -> &[Edge] {
        self.edges.get(node).map(Vec::as_slice).unwrap_or(&[])
    }
}

/// Returns `true` if the type contains no type variables anywhere in its structure.
pub fn is_ground(ty: &Ty) -> bool {
    match ty {
        Ty::I64 => true,
        Ty::Var(_) => false,
        Ty::Decl { type_args, .. } => type_args.args.iter().all(is_ground),
    }
}

#[cfg(test)]
mod tests {
    use super::{VarLocations, is_ground};
    use crate::mono::{
        constraint_graph::ConstraintGraph,
        constraints::{FlowConstraint, FlowConstraintSet},
    };
    extern crate self as core_lang;
    use core_macros::{id, tvar, ty};

    #[test]
    fn non_growing_cycle() {
        let mut set = FlowConstraintSet::new();
        set.insert(FlowConstraint {
            from: vec![ty!("int")],
            to: vec![id!("A", 1)],
        });
        set.insert(FlowConstraint {
            from: vec![tvar!(id!("A", 1))],
            to: vec![id!("B", 2)],
        });
        set.insert(FlowConstraint {
            from: vec![tvar!(id!("B", 2))],
            to: vec![id!("A", 1)],
        });

        let graph = ConstraintGraph::from(set);

        let node_a = vec![id!("A", 1)];
        let node_b = vec![id!("B", 2)];

        // validate registry nodes
        assert!(graph.nodes.contains(&node_a));
        assert!(graph.nodes.contains(&node_b));
        assert_eq!(graph.nodes.len(), 2);

        // validate seeds
        assert!(graph.seeds.contains_key(&node_a));
        assert!(!graph.seeds.contains_key(&node_b));
        assert_eq!(graph.seeds[&node_a].len(), 1);

        // validate seed is ground
        let seed_vector = graph.seeds[&node_a].iter().next().unwrap();
        assert!(is_ground(&seed_vector[0]));

        // validate flat edges
        let outgoing_a = graph.outgoing(&node_a);
        assert_eq!(outgoing_a.len(), 1);
        assert_eq!(outgoing_a[0].into, node_b);
        assert!(
            !outgoing_a[0].has_constructor_position(),
            "A -> B sollte flach sein"
        );

        let outgoing_b = graph.outgoing(&node_b);
        assert_eq!(outgoing_b.len(), 1);
        assert_eq!(outgoing_b[0].into, node_a);
        assert!(
            !outgoing_b[0].has_constructor_position(),
            "B -> A sollte flach sein"
        );
    }

    #[test]
    fn transitive_flow() {
        let mut set = FlowConstraintSet::new();
        set.insert(FlowConstraint {
            from: vec![ty!("int")],
            to: vec![id!("A", 1)],
        });
        set.insert(FlowConstraint {
            from: vec![ty!(id!("bool"))],
            to: vec![id!("A", 1)],
        });
        set.insert(FlowConstraint {
            from: vec![ty!(id!("Pair"), [tvar!(id!("A", 1)), tvar!(id!("A", 1))])],
            to: vec![id!("B", 2)],
        });

        let graph = ConstraintGraph::from(set);

        let node_a = vec![id!("A", 1)];
        let node_b = vec![id!("B", 2)];

        assert_eq!(graph.seeds[&node_a].len(), 2);

        let outgoing_a = graph.outgoing(&node_a);
        assert_eq!(outgoing_a.len(), 1);
        assert_eq!(outgoing_a[0].into, node_b);
        assert!(
            outgoing_a[0].has_constructor_position(),
            "Kante sollte Constructor-Position enthalten"
        );

        assert!(graph.outgoing(&node_b).is_empty());
    }

    #[test]
    fn test_var_locations_registry() {
        let mut locations = VarLocations::default();
        let node_pair = vec![id!("A", 1), id!("B", 2)];

        locations.register(&node_pair);

        assert_eq!(locations.node_of(&id!("A", 1)), node_pair);
        assert_eq!(locations.index_of(&id!("A", 1)), 0);

        assert_eq!(locations.node_of(&id!("B", 2)), node_pair);
        assert_eq!(locations.index_of(&id!("B", 2)), 1);

        let unregistered = id!("C", 3);
        assert_eq!(locations.node_of(&unregistered), vec![unregistered.clone()]);
        assert_eq!(locations.index_of(&unregistered), 0);
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic(expected = "registered with inconsistent node groupings")]
    fn test_var_locations_inconsistent_grouping_panic() {
        let mut locations = VarLocations::default();
        locations.register(&vec![id!("A", 1), id!("B", 2)]);
        locations.register(&vec![id!("A", 1), id!("C", 3)]);
    }

    #[test]
    fn test_edge_multiple_source_dependencies() {
        let mut set = FlowConstraintSet::new();
        set.insert(FlowConstraint {
            from: vec![ty!("int")],
            to: vec![id!("A", 1)],
        });
        set.insert(FlowConstraint {
            from: vec![ty!("int")],
            to: vec![id!("B", 2)],
        });

        set.insert(FlowConstraint {
            from: vec![ty!(id!("Pair"), [tvar!(id!("A", 1)), tvar!(id!("B", 2))])],
            to: vec![id!("C", 3)],
        });

        let graph = ConstraintGraph::from(set);

        let node_a = vec![id!("A", 1)];
        let node_b = vec![id!("B", 2)];
        let node_c = vec![id!("C", 3)];

        let outgoing_a = graph.outgoing(&node_a);
        let outgoing_b = graph.outgoing(&node_b);

        assert_eq!(outgoing_a.len(), 1);
        assert_eq!(outgoing_a[0].into, node_c);

        assert_eq!(outgoing_b.len(), 1);
        assert_eq!(outgoing_b[0].into, node_c);

        assert!(outgoing_a[0].has_constructor_position());
        assert!(outgoing_b[0].has_constructor_position());
    }
}
