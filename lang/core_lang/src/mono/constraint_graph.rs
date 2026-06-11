use crate::mono::constraints::FlowConstraintSet;
use crate::syntax::{Identifier, Ty};
use std::collections::{HashMap, HashSet};

/// Maps each type variable to the set of concrete ground types it is instantiated with.
///
/// This is the output of the solving phase and the direct input to specialization.
/// Every [`Ty`] value stored in the sets is guaranteed to be ground, meaning it
/// contains no [`Ty::Var`] anywhere in its structure.
pub type Solution = HashMap<Identifier, HashSet<Ty>>;

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
    /// The full source type from the original constraint.
    /// Its structure determines whether this is a flat or constructor edge.
    pub from: Ty,
    /// The target type variable that receives the types flowing through this edge.
    pub into: Identifier,
}

impl Edge {
    /// Returns the source type variable of this edge.
    ///
    /// For flat edges this is the variable itself. For constructor edges it is
    /// the variable nested inside the type constructor.
    pub fn source_vars(&self) -> HashSet<Identifier> {
        find_inner_vars(&self.from)
    }

    /// Returns `true` if this edge applies a type constructor to the flowing types.
    ///
    /// Constructor edges are the only edges that can form growing cycles.
    pub fn is_constructor(&self) -> bool {
        matches!(self.from, Ty::Decl { .. })
    }

    /// Returns the name of the type constructor if this is a constructor edge.
    pub fn constructor_name(&self) -> Option<&Identifier> {
        match &self.from {
            Ty::Decl { name, .. } => Some(name),
            _ => None,
        }
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
    /// All type variable nodes in the graph.
    pub nodes: HashSet<Identifier>,
    /// Ground types seeding each type variable directly.
    /// Key: target variable identifier. Value: set of ground types.
    pub seeds: HashMap<Identifier, HashSet<Ty>>,
    /// Outgoing edges for each type variable, indexed by source variable.
    /// Key: source variable identifier. Value: edges leaving that variable.
    pub edges: HashMap<Identifier, Vec<Edge>>,
}

impl From<FlowConstraintSet> for ConstraintGraph {
    /// Builds a constraint graph from the given constraint set.
    ///
    /// Each constraint `τ ⊑ γ` is classified based on the structure of `τ`:
    ///
    /// - Ground `τ` (no type variables anywhere): seed for `γ`.
    /// - `Ty::Var(α)`: flat edge `α → γ`.
    /// - `Ty::Decl { T, [Ty::Var(α)] }`: constructor edge `α →^T γ`.
    ///
    /// # Panics
    ///
    /// Panics if a constraint has a non-variable target. All targets produced
    /// by the constraint collection phase are guaranteed to be [`Ty::Var`].
    fn from(constraints: FlowConstraintSet) -> Self {
        let mut graph = ConstraintGraph::default();

        for constraint in &constraints.constraints {
            // The target of every constraint must be a type variable.
            let into = match &constraint.to {
                Ty::Var(id) => id.clone(),
                other => panic!(
                    "constraint target must be a type variable, got: {:?}",
                    other
                ),
            };

            graph.nodes.insert(into.clone());

            match &constraint.from {
                // i64 is always ground; seed it directly.
                Ty::I64 => {
                    graph.seeds.entry(into).or_default().insert(Ty::I64);
                }

                // A bare type variable produces a flat edge.
                Ty::Var(source) => {
                    graph.nodes.insert(source.clone());
                    graph.edges.entry(source.clone()).or_default().push(Edge {
                        from: constraint.from.clone(),
                        into,
                    });
                }

                // A declared type is either fully ground (seed) or contains a
                // nested variable (constructor edge).
                Ty::Decl { .. } => {
                    if is_ground(&constraint.from) {
                        graph
                            .seeds
                            .entry(into)
                            .or_default()
                            .insert(constraint.from.clone());
                    } else {
                        let sources = find_inner_vars(&constraint.from);
                        for source in sources {
                            graph.nodes.insert(source.clone());
                            graph.edges.entry(source).or_default().push(Edge {
                                from: constraint.from.clone(),
                                into: into.clone(),
                            });
                        }
                    }
                }
            }
        }

        graph
    }
}

impl ConstraintGraph {
    /// Returns all outgoing edges from the given node as a slice.
    pub fn outgoing(&self, node: &Identifier) -> &[Edge] {
        self.edges.get(node).map(Vec::as_slice).unwrap_or(&[])
    }

    /// Returns the total number of edges in the graph.
    pub fn edge_count(&self) -> usize {
        self.edges.values().map(Vec::len).sum()
    }
}

/// Returns `true` if the type contains no type variables anywhere in its structure.
fn is_ground(ty: &Ty) -> bool {
    match ty {
        Ty::I64 => true,
        Ty::Var(_) => false,
        Ty::Decl { type_args, .. } => type_args.args.iter().all(is_ground),
    }
}

/// Returns the first type variable found anywhere inside the type tree, if any.
///
/// Used to locate the source variable for constructor edges.
fn find_inner_vars(ty: &Ty) -> HashSet<Identifier> {
    match ty {
        Ty::Var(id) => HashSet::from([id.clone()]),
        Ty::Decl { type_args, .. } => type_args.args.iter().flat_map(find_inner_vars).collect(),
        Ty::I64 => HashSet::new(),
    }
}

#[cfg(test)]
mod tests {
    use crate::mono::{
        constraint_graph::ConstraintGraph,
        constraints::{FlowConstraint, FlowConstraintSet},
        graph_viz::OutputFormat,
    };
    extern crate self as core_lang;
    use core_macros::{id, tvar, ty};

    #[test]
    fn non_growing_cycle() {
        let mut set = FlowConstraintSet::new();
        set.insert(FlowConstraint {
            from: ty!("int"),
            to: tvar!(id!("A", 1)),
        });
        set.insert(FlowConstraint {
            from: tvar!(id!("A", 1)),
            to: tvar!(id!("B", 2)),
        });
        set.insert(FlowConstraint {
            from: tvar!(id!("B", 2)),
            to: tvar!(id!("A", 1)),
        });

        let graph = ConstraintGraph::from(set);

        graph
            .render_as(OutputFormat::Png, Some("non_growing_cycle.png"))
            .unwrap();
    }

    #[test]
    fn transitive_flow() {
        let mut set = FlowConstraintSet::new();

        set.insert(FlowConstraint {
            from: ty!("int"),
            to: tvar!(id!("A", 1)),
        });
        set.insert(FlowConstraint {
            from: ty!(id!("bool")),
            to: tvar!(id!("A", 1)),
        });
        set.insert(FlowConstraint {
            from: ty!(id!("Pair"), [tvar!(id!("A", 1)), tvar!(id!("A", 1))]),
            to: tvar!(id!("B", 2)),
        });

        let graph = ConstraintGraph::from(set);
        graph
            .render_as(OutputFormat::Png, Some("transitive_flow.png"))
            .unwrap();
    }

    #[test]
    fn growing_cycle() {
        let mut set = FlowConstraintSet::new();

        set.insert(FlowConstraint {
            from: ty!("int"),
            to: tvar!(id!("A", 1)),
        });
        set.insert(FlowConstraint {
            from: tvar!(id!("A", 1)),
            to: tvar!(id!("B", 2)),
        });
        set.insert(FlowConstraint {
            from: ty!(id!("List"), [tvar!(id!("B", 2))]),
            to: tvar!(id!("A", 1)),
        });

        let graph = ConstraintGraph::from(set);
        graph
            .render_as(OutputFormat::Png, Some("growing_cycle"))
            .unwrap();
    }
}
