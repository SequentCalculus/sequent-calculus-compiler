use crate::code::Instructions;

use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::hash::Hash;

/// Spanning trees without a root node, having at most one back edge to the root.
pub enum Tree<Temporary> {
    BackEdge,
    Node(Temporary, Vec<Tree<Temporary>>),
}

impl<Temporary: Eq + Hash + Copy> Tree<Temporary> {
    /// Returns the set of temporaries in a spanning tree.
    fn nodes(&self) -> HashSet<Temporary> {
        let mut visited = HashSet::new();
        match self {
            Tree::BackEdge => {}
            Tree::Node(temporary, trees) => {
                visited.insert(*temporary);
                for tree in trees {
                    visited.extend(tree.nodes());
                }
            }
        }
        visited
    }

    /// Returns whether a spanning tree contains a back edge or not.
    pub fn refers_back(&self) -> bool {
        match self {
            Tree::BackEdge => true,
            Tree::Node(_, trees) => trees.iter().any(Tree::refers_back),
        }
    }
}

/// Root node of a spanning trees.
pub enum Root<Temporary> {
    StartNode(Temporary, Vec<Tree<Temporary>>),
}

impl<Temporary: Eq + Hash + Copy> Root<Temporary> {
    /// Returns the set of temporaries having an edge pointing to them in a rooted spanning tree.
    fn visited_by(&self) -> HashSet<Temporary> {
        let mut visited = HashSet::new();
        match self {
            Root::StartNode(temporary, trees) => {
                if trees.iter().any(Tree::refers_back) {
                    visited.insert(*temporary);
                };
                for tree in trees {
                    visited.extend(tree.nodes());
                }
            }
        }
        visited
    }
}

/// This function deletes the parallel moves that have already been performed.
/// - `to_delete` contains the target temporaries of the moves already performed.
/// - `parallel_moves` maps a temporary to a set of all temporaries it must be moved to.
fn delete_targets<Temporary: Ord + Hash>(
    to_delete: &HashSet<Temporary>,
    parallel_moves: &mut BTreeMap<Temporary, BTreeSet<Temporary>>,
) {
    for targets in parallel_moves.values_mut() {
        targets.retain(|temporary| !(to_delete.contains(temporary)));
    }
}

/// This function creates a spanning tree for a given temporary from a mapping of parallel moves
/// to be performed. There is one edge for each temporary the given temporary must be moved to.
/// - `parallel_moves` maps a temporary to a set of all temporaries it must be moved to.
/// - `root` is the root temporary of the overall spanning tree.
/// - `node` is the node for which the spanning tree is created.
fn spanning_tree<Temporary: Ord + Copy + Clone>(
    parallel_moves: &BTreeMap<Temporary, BTreeSet<Temporary>>,
    root: Temporary,
    node: Temporary,
) -> Tree<Temporary> {
    if root == node {
        Tree::BackEdge
    } else if parallel_moves.contains_key(&node) {
        let targets = parallel_moves[&node].clone();
        Tree::Node(
            node,
            targets
                .into_iter()
                .map(|target| spanning_tree(parallel_moves, root, target))
                .collect(),
        )
    } else {
        Tree::Node(node, Vec::new())
    }
}

/// This function creates a spanning forest, i.e., a list of spanning trees for a given mapping of
/// parallel moves to be performed. Each move in contained in exactly one tree.
/// - `parallel_moves` maps a temporary to a set of all temporaries it must be moved to.
fn spanning_forest<Temporary: Ord + Hash + Copy>(
    mut parallel_moves: BTreeMap<Temporary, BTreeSet<Temporary>>,
) -> Vec<Root<Temporary>> {
    let mut root_list = Vec::with_capacity(parallel_moves.len());
    let mappings = parallel_moves.clone();
    for temporary in mappings.keys() {
        let mut targets = parallel_moves[temporary].clone();
        let _ = targets.remove(temporary);
        let root = Root::StartNode(
            *temporary,
            targets
                .into_iter()
                .map(|target| spanning_tree(&parallel_moves, *temporary, target))
                .collect(),
        );
        delete_targets(&root.visited_by(), &mut parallel_moves);
        root_list.push(root);
    }
    root_list
}

pub type SpillMove = bool;

/// This trait abstracts how the temporary moves are performed in the backend platform.
pub trait ParallelMoves<Code, Temporary> {
    /// This method returns whether one of the edges in a rooted spanning tree represents a move
    /// between two spill positions in memory. Some platforms (e.g., x86_64) need this information.
    /// - `root` is the rooted spanning tree.
    fn contains_spill_edge(root: &Root<Temporary>) -> bool;
    /// This method generates code for a move between two temporaries.
    fn move_to_temporary(
        target_temporary: Temporary,
        source_temporary: Temporary,
        instructions: &mut Vec<Code>,
    );
    /// This method generates code for storing a temporary to a scratch spot.
    /// - `temporary` is the temporary to store.
    /// - `contains_spill_move` indicates whether there will be a move between two spill positions
    ///   in memory. Some platforms (e.g., x86_64) need this information.
    /// - `instructions` is the list of instructions to which the new instructions are appended.
    fn store_temporary(
        temporary: Temporary,
        contains_spill_move: SpillMove,
        instructions: &mut Vec<Code>,
    );
    /// This method generates code for restoring a temporary from a scratch spot.
    /// - `temporary` is the temporary to restore.
    /// - `contains_spill_move` indicates whether there will be a move between two spill positions
    ///   in memory. Some platforms (e.g., x86_64) need this information.
    /// - `instructions` is the list of instructions to which the new instructions are appended.
    fn restore_temporary(
        temporary: Temporary,
        contains_spill_move: SpillMove,
        instructions: &mut Vec<Code>,
    );
}

/// This function generates the instructions for performing parallel moves for a spanning tree,
/// depth-first. If there is a back edge, the temporary which is to be moved to the root temporary
/// is stored in a scratch spot at the beginning and will be restored in the end in [`root_moves`].
/// - `temporary` is the temporary of th current node
/// - `tree` is the the spanning tree.
/// - `contains_spill_move` indicates whether one of the performed moves is between two spill
///   positions in memory. Some platforms (e.g., x86_64) need this information.
/// - `instructions` is the list of instructions to which the new instructions are appended.
fn tree_moves<Backend, Code, Temporary: Copy>(
    temporary: Temporary,
    tree: &Tree<Temporary>,
    contains_spill_move: SpillMove,
    instructions: &mut Vec<Code>,
) where
    Backend: ParallelMoves<Code, Temporary>,
{
    match tree {
        Tree::BackEdge => Backend::store_temporary(temporary, contains_spill_move, instructions),
        Tree::Node(target_temporary, trees) => {
            for tree in trees {
                tree_moves::<Backend, _, _>(
                    *target_temporary,
                    tree,
                    contains_spill_move,
                    instructions,
                );
            }
            Backend::move_to_temporary(*target_temporary, temporary, instructions);
        }
    }
}

/// This function generates the instructions for performing parallel moves for a rooted spanning
/// tree. If there is a back edge, one temporary will be stored in a scratch spot at the beginning
/// and thus has be restored in the end.
/// - `root` is the rooted spanning tree.
/// - `instructions` is the list of instructions to which the new instructions are appended.
fn root_moves<Backend, Code, Temporary: Ord + Hash + Copy>(
    root: Root<Temporary>,
    instructions: &mut Vec<Code>,
) where
    Backend: ParallelMoves<Code, Temporary>,
{
    let contains_spill_move = Backend::contains_spill_edge(&root);
    match root {
        Root::StartNode(temporary, trees) => {
            for tree in &trees {
                tree_moves::<Backend, _, _>(temporary, tree, contains_spill_move, instructions);
            }
            if trees.iter().any(Tree::refers_back) {
                Backend::restore_temporary(temporary, contains_spill_move, instructions);
            };
        }
    }
}

/// This function generates the instructions for performing parallel moves.
/// - `assignments` maps a temporary to a set of all temporaries it must be moved to. This assumes
///   that the `BTreeSet`s in `assignments` are pairwise disjoint.
/// - `instructions` is the list of instructions to which the new instructions are appended.
pub fn parallel_moves<Backend, Code, Temporary: Ord + Hash + Copy, Immediate>(
    assignments: BTreeMap<Temporary, BTreeSet<Temporary>>,
    instructions: &mut Vec<Code>,
) where
    Backend: ParallelMoves<Code, Temporary> + Instructions<Code, Temporary, Immediate>,
{
    let spanning_forest = spanning_forest(assignments);

    // if there are no moves, we do not generate a comment either
    if !spanning_forest
        .iter()
        .all(|Root::StartNode(_, targets)| targets.is_empty())
    {
        instructions.push(Backend::comment("#move variables".to_string()));
    }

    for root in spanning_forest {
        root_moves::<Backend, _, _>(root, instructions);
    }
}
