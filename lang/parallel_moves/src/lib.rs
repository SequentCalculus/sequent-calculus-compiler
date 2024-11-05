use std::collections::HashSet;
use std::hash::Hash;

pub enum Tree<T> {
    BackEdge,
    Node(T, Vec<Tree<T>>),
}

pub enum Root<T> {
    StartNode(T, Vec<Tree<T>>),
}

pub fn tree_nodes<T>(tree: &Tree<T>) -> HashSet<T>
where
    T: Eq + Hash + Copy,
{
    let mut visited = HashSet::new();
    match tree {
        Tree::BackEdge => {}
        Tree::Node(temporary, trees) => {
            visited.insert(*temporary);
            for tree in trees {
                visited.extend(tree_nodes(tree));
            }
        }
    }
    visited
}

pub fn refers_back<T>(tree: &Tree<T>) -> bool {
    match tree {
        Tree::BackEdge => true,
        Tree::Node(_, trees) => trees.iter().any(refers_back),
    }
}

pub fn visited_by<T: Eq + Hash + Copy>(root: &Root<T>) -> HashSet<T> {
    let mut visited = HashSet::new();
    match root {
        Root::StartNode(temporary, trees) => {
            if trees.iter().any(refers_back) {
                visited.insert(*temporary);
            };
            for tree in trees {
                visited.extend(tree_nodes(tree));
            }
        }
    }
    visited
}
