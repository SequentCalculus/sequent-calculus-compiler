use std::collections::HashMap;

use crate::splitting::labeling::Label;

/// A simple union-find data structure for tracking equivalence classes of labels. Each label
/// is either a root of its own class (in which case it is its own `parent`) or it points to another label that is its `parent`. The `rank` map is used to keep the tree shallow by always attaching the smaller tree to the root of the larger tree during union operations.
#[derive(Debug, Clone, Default)]
pub struct UnionFind {
    parent: HashMap<Label, Label>,
    rank: HashMap<Label, usize>,
}

impl UnionFind {
    /// Ensures that a label is present in the union-find structure. If the label is not already present, it initializes it as its own `parent` (root of its own class) and sets its `rank` to 0.
    fn make(&mut self, l: &Label) {
        self.parent.entry(l.clone()).or_insert_with(|| l.clone());
        self.rank.entry(l.clone()).or_insert(0);
    }

    /// Finds the root of the equivalence class for the given label, applying path compression to flatten the structure for future queries. If the label is not present, it is first added to the structure.
    pub fn find(&mut self, l: &Label) -> Label {
        self.make(l);
        if self.parent[l] != *l {
            let root = self.find(&self.parent[l].clone());
            self.parent.insert(l.clone(), root.clone());
        }
        self.parent[l].clone()
    }

    /// Unions the equivalence classes of two labels. If they are already in the same class, nothing is done. Otherwise, the root of one class is made the parent of the root of the other class, using `rank` to keep the tree shallow.
    pub fn union(&mut self, a: &Label, b: &Label) {
        let (ra, rb) = (self.find(a), self.find(b));
        if ra == rb {
            return;
        }
        match self.rank[&ra].cmp(&self.rank[&rb]) {
            std::cmp::Ordering::Less => {
                self.parent.insert(ra, rb);
            }
            std::cmp::Ordering::Greater => {
                self.parent.insert(rb, ra);
            }
            std::cmp::Ordering::Equal => {
                self.parent.insert(rb.clone(), ra.clone());
                *self.rank.get_mut(&ra).unwrap() += 1;
            }
        }
    }
}

#[cfg(test)]
mod union_find_tests {
    use super::*;
    use crate::syntax::Identifier;

    fn label(name: &str, id: usize) -> Label {
        Identifier {
            name: name.to_string(),
            id,
        }
    }

    #[test]
    fn fresh_label_is_its_own_root() {
        let mut uf = UnionFind::default();
        let a = label("Box", 1);
        assert_eq!(uf.find(&a), a);
    }

    #[test]
    fn two_unrelated_labels_are_not_unified() {
        let mut uf = UnionFind::default();
        let a = label("Box", 1);
        let b = label("Box", 2);
        assert_ne!(uf.find(&a), uf.find(&b));
    }

    #[test]
    fn union_merges_two_labels_into_the_same_class() {
        let mut uf = UnionFind::default();
        let a = label("Box", 1);
        let b = label("Box", 2);
        uf.union(&a, &b);
        assert_eq!(uf.find(&a), uf.find(&b));
    }

    #[test]
    fn union_is_idempotent_for_already_merged_labels() {
        let mut uf = UnionFind::default();
        let a = label("Box", 1);
        let b = label("Box", 2);
        uf.union(&a, &b);
        let root_before = uf.find(&a);
        uf.union(&a, &b);
        let root_after = uf.find(&a);
        assert_eq!(root_before, root_after);
    }

    #[test]
    fn union_is_transitive_across_a_chain() {
        // A-B, then B-C must put A, B, and C all in the same class, even though A and C were
        // never unioned directly.
        let mut uf = UnionFind::default();
        let a = label("Box", 1);
        let b = label("Box", 2);
        let c = label("Box", 3);

        uf.union(&a, &b);
        uf.union(&b, &c);

        let root = uf.find(&a);
        assert_eq!(uf.find(&b), root);
        assert_eq!(uf.find(&c), root);
    }

    #[test]
    fn two_separate_classes_remain_distinct_after_unrelated_unions() {
        // example: Box#1/Box#2/Box#3 {1,2} merge, 3 stays isolated.
        let mut uf = UnionFind::default();
        let one = label("Box", 1);
        let two = label("Box", 2);
        let three = label("Box", 3);

        uf.union(&one, &two);

        assert_eq!(uf.find(&one), uf.find(&two));
        assert_ne!(uf.find(&one), uf.find(&three));
    }

    #[test]
    fn union_of_two_disjoint_pairs_can_be_merged_by_a_bridging_union() {
        // {A,B} and {C,D} start separate; unioning B and C must merge all four into one class.
        let mut uf = UnionFind::default();
        let a = label("Box", 1);
        let b = label("Box", 2);
        let c = label("Box", 3);
        let d = label("Box", 4);

        uf.union(&a, &b);
        uf.union(&c, &d);
        assert_ne!(uf.find(&a), uf.find(&c));

        uf.union(&b, &c);

        let root = uf.find(&a);
        assert_eq!(uf.find(&b), root);
        assert_eq!(uf.find(&c), root);
        assert_eq!(uf.find(&d), root);
    }

    #[test]
    fn find_is_stable_across_repeated_calls_after_path_compression() {
        let mut uf = UnionFind::default();
        let a = label("Box", 1);
        let b = label("Box", 2);
        let c = label("Box", 3);

        uf.union(&a, &b);
        uf.union(&b, &c);

        let first = uf.find(&c);
        // Repeated calls (now hitting the path-compressed shortcut) must return the same root.
        let second = uf.find(&c);
        let third = uf.find(&a);
        assert_eq!(first, second);
        assert_eq!(first, third);
    }

    #[test]
    fn labels_with_different_names_can_still_be_unioned() {
        // UnionFind itself doesn't enforce same-head-name unification, that invariant is the
        // caller's (unify_ty's) responsibility, not the data structure's. This test just pins
        // down that UnionFind is name-agnostic.
        let mut uf = UnionFind::default();
        let a = label("Box", 1);
        let b = label("List", 2);
        uf.union(&a, &b);
        assert_eq!(uf.find(&a), uf.find(&b));
    }

    #[test]
    fn many_sequential_unions_keep_find_consistent() {
        // A slightly larger chain to exercise union-by-rank with more than a handful of nodes.
        let mut uf = UnionFind::default();
        let labels: Vec<Label> = (1..=10).map(|i| label("Box", i)).collect();

        for pair in labels.windows(2) {
            uf.union(&pair[0], &pair[1]);
        }

        let root = uf.find(&labels[0]);
        for l in &labels {
            assert_eq!(uf.find(l), root, "label {:?} not in the merged class", l);
        }
    }
}
