//! A union-find (disjoint-set) data structure tracking equivalence classes of labels during type
//! splitting, together with the data each class accumulates while the labeling walk runs.

use std::collections::{HashMap, HashSet, hash_map::Entry};

use crate::splitting::labeling::Label;
use crate::syntax::{Identifier, Ty};

/// Everything the labeling walk has learned about one equivalence class of labels.
///
/// Held only at class *roots*: [`UnionFind::union`] migrates an absorbed class's data onto the
/// surviving root, so the keys of `UnionFind::classes` are always exactly the current roots.
#[derive(Debug, Clone, Default)]
pub struct ClassData {
    /// The labeled type observed for one field of one xtor, keyed by `(xtor, field index)`.
    /// Whichever occurrence observed a field first wins; every later observation is unified
    /// against that one rather than replacing it, which keeps the stored representative
    /// independent of hash-map iteration order.
    pub fields: HashMap<(Identifier, usize), Ty>,
    /// Every xtor that literally occurs somewhere in the program for this class. Anything else is
    /// dead for the class and is dropped from its physical copy, see
    /// [`crate::splitting::rewrite::keeps_xtor`].
    pub used_xtors: HashSet<Identifier>,
}

/// A union-find data structure for tracking equivalence classes of labels. Each label is either a
/// root of its own class (in which case it is its own `parent`) or it points to another label that
/// is its `parent`. The `rank` map is used to keep the tree shallow by always attaching the
/// smaller tree to the root of the larger tree during union operations.
///
/// Beyond the plain partition it also carries each class's [`ClassData`], so that the congruence
/// rule of type splitting (equal owners force their equally-named fields to be equal) fires
/// directly inside [`UnionFind::union`] instead of being reconstructed afterwards.
#[derive(Debug, Clone, Default)]
pub struct UnionFind {
    parent: HashMap<Label, Label>,
    rank: HashMap<Label, usize>,
    classes: HashMap<Label, ClassData>,
}

impl UnionFind {
    /// Ensures that a label is present in the union-find structure. If the label is not already
    /// present, it initializes it as its own `parent` (root of its own class) and sets its
    /// `rank` to 0.
    fn make(&mut self, l: &Label) {
        self.parent.entry(l.clone()).or_insert_with(|| l.clone());
        self.rank.entry(l.clone()).or_insert(0);
    }

    /// Finds the root of the equivalence class for the given label, applying path compression to
    /// flatten the structure for future queries. If the label is not present, it is first added
    /// to the structure.
    pub fn find(&mut self, l: &Label) -> Label {
        self.make(l);
        if self.parent[l] != *l {
            let root = self.find(&self.parent[l].clone());
            self.parent.insert(l.clone(), root.clone());
        }
        self.parent[l].clone()
    }

    /// Unions the equivalence classes of two labels and merges their [`ClassData`]. If they are
    /// already in the same class, nothing happens. Otherwise the root of one class is made the
    /// parent of the root of the other, using `rank` to keep the tree shallow.
    ///
    /// Returns the field types that *collided* during the merge: both classes had already
    /// observed the same `(xtor, field index)`, so those two observed types now have to denote the
    /// same thing. That is the congruence rule of type splitting, and it is the caller's job (see
    /// `SplitState::unify_ty`) to unify each returned pair, which may in turn trigger further
    /// unions. Returning them rather than unifying here keeps this type free of any knowledge
    /// about how types are structurally decomposed.
    pub fn union(&mut self, a: &Label, b: &Label) -> Vec<(Ty, Ty)> {
        let (ra, rb) = (self.find(a), self.find(b));
        if ra == rb {
            return vec![];
        }
        let (winner, loser) = match self.rank[&ra].cmp(&self.rank[&rb]) {
            std::cmp::Ordering::Less => {
                self.parent.insert(ra.clone(), rb.clone());
                (rb, ra)
            }
            std::cmp::Ordering::Greater => {
                self.parent.insert(rb.clone(), ra.clone());
                (ra, rb)
            }
            std::cmp::Ordering::Equal => {
                self.parent.insert(rb.clone(), ra.clone());
                *self.rank.get_mut(&ra).unwrap() += 1;
                (ra, rb)
            }
        };
        self.merge_class_data(&winner, &loser)
    }

    /// Moves `loser`'s class data onto `winner`, keeping `winner`'s entry wherever both observed
    /// the same field and reporting that pair back to the caller.
    fn merge_class_data(&mut self, winner: &Label, loser: &Label) -> Vec<(Ty, Ty)> {
        let Some(loser_data) = self.classes.remove(loser) else {
            return vec![];
        };
        let winner_data = self.classes.entry(winner.clone()).or_default();
        // A used-xtor entry is a plain fact about the class and can never itself force a
        // unification, so unlike the field types it just merges as a set.
        winner_data.used_xtors.extend(loser_data.used_xtors);

        let mut collisions = Vec::new();
        for (key, ty) in loser_data.fields {
            match winner_data.fields.entry(key) {
                Entry::Occupied(kept) => collisions.push((kept.get().clone(), ty)),
                Entry::Vacant(slot) => {
                    slot.insert(ty);
                }
            }
        }
        collisions
    }

    /// Records `ty` as the type observed for `xtor`'s `index`-th field in `owner`'s class.
    /// Returns the type already observed at that position, if any, which the caller must unify
    /// against `ty`; the stored entry itself is left untouched.
    pub fn observe_field(
        &mut self,
        owner: &Label,
        xtor: &Identifier,
        index: usize,
        ty: &Ty,
    ) -> Option<Ty> {
        let root = self.find(owner);
        match self
            .classes
            .entry(root)
            .or_default()
            .fields
            .entry((xtor.clone(), index))
        {
            Entry::Occupied(kept) => Some(kept.get().clone()),
            Entry::Vacant(slot) => {
                slot.insert(ty.clone());
                None
            }
        }
    }

    /// Records that `xtor` occurs somewhere in the program for `owner`'s class.
    pub fn record_xtor_use(&mut self, owner: &Label, xtor: &Identifier) {
        let root = self.find(owner);
        self.classes
            .entry(root)
            .or_default()
            .used_xtors
            .insert(xtor.clone());
    }

    /// Iterates every class root that accumulated data, for building the read-only views the
    /// rewrite phase consults (see `labeling::finish_classes`).
    pub fn classes(&self) -> impl Iterator<Item = (&Label, &ClassData)> {
        self.classes.iter()
    }
}

#[cfg(test)]
mod union_find_tests {
    use super::*;
    use crate::syntax::Identifier;
    extern crate self as core_lang;
    use core_macros::{id, ty};

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

    #[test]
    fn observe_field_keeps_the_first_observation_and_reports_the_later_one() {
        let mut uf = UnionFind::default();
        let owner = label("Bar", 1);
        let first = ty!(id!("Foo#1"));
        let second = ty!(id!("Foo#2"));

        assert_eq!(uf.observe_field(&owner, &id!("MkBar"), 0, &first), None);
        // the second observation does not overwrite, it is handed back for unification
        assert_eq!(
            uf.observe_field(&owner, &id!("MkBar"), 0, &second),
            Some(first.clone())
        );
        assert_eq!(
            uf.observe_field(&owner, &id!("MkBar"), 0, &second),
            Some(first)
        );
    }

    #[test]
    fn observations_under_distinct_owners_do_not_interfere() {
        let mut uf = UnionFind::default();
        let owner_a = label("Bar", 1);
        let owner_b = label("Bar", 2);
        let field_a = ty!(id!("Foo#1"));
        let field_b = ty!(id!("Foo#2"));

        assert_eq!(uf.observe_field(&owner_a, &id!("MkBar"), 0, &field_a), None);
        assert_eq!(uf.observe_field(&owner_b, &id!("MkBar"), 0, &field_b), None);
    }

    #[test]
    fn union_reports_colliding_field_observations_as_pending_pairs() {
        // The congruence rule: two owners that turn out equal force their MkBar.0 fields together.
        let mut uf = UnionFind::default();
        let owner_a = label("Bar", 1);
        let owner_b = label("Bar", 2);
        let field_a = ty!(id!("Foo#1"));
        let field_b = ty!(id!("Foo#2"));
        uf.observe_field(&owner_a, &id!("MkBar"), 0, &field_a);
        uf.observe_field(&owner_b, &id!("MkBar"), 0, &field_b);

        let pending = uf.union(&owner_a, &owner_b);

        assert_eq!(pending.len(), 1);
        let (x, y) = &pending[0];
        assert!(
            (x == &field_a && y == &field_b) || (x == &field_b && y == &field_a),
            "expected the two observed field types as a pending pair, got {pending:?}"
        );
    }

    #[test]
    fn union_reports_nothing_when_only_one_side_observed_the_field() {
        let mut uf = UnionFind::default();
        let owner_a = label("Bar", 1);
        let owner_b = label("Bar", 2);
        uf.observe_field(&owner_a, &id!("MkBar"), 0, &ty!(id!("Foo#1")));

        assert!(uf.union(&owner_a, &owner_b).is_empty());
        // the surviving root carries the observation either way
        let root = uf.find(&owner_a);
        let data = uf.classes().find(|(r, _)| *r == &root).map(|(_, d)| d);
        assert_eq!(
            data.and_then(|d| d.fields.get(&(id!("MkBar"), 0))),
            Some(&ty!(id!("Foo#1")))
        );
    }

    #[test]
    fn union_merges_used_xtors_without_reporting_collisions() {
        let mut uf = UnionFind::default();
        let owner_a = label("Bar", 1);
        let owner_b = label("Bar", 2);
        uf.record_xtor_use(&owner_a, &id!("MkBar"));
        uf.record_xtor_use(&owner_b, &id!("MkBaz"));

        assert!(uf.union(&owner_a, &owner_b).is_empty());

        let root = uf.find(&owner_a);
        let (_, data) = uf
            .classes()
            .find(|(r, _)| *r == &root)
            .expect("merged class");
        assert!(data.used_xtors.contains(&id!("MkBar")));
        assert!(data.used_xtors.contains(&id!("MkBaz")));
    }

    #[test]
    fn class_data_is_only_ever_keyed_by_current_roots() {
        let mut uf = UnionFind::default();
        let a = label("Bar", 1);
        let b = label("Bar", 2);
        uf.record_xtor_use(&a, &id!("MkBar"));
        uf.record_xtor_use(&b, &id!("MkBar"));
        uf.union(&a, &b);

        let root = uf.find(&a);
        let keys: Vec<&Label> = uf.classes().map(|(r, _)| r).collect();
        assert_eq!(keys, vec![&root], "the absorbed class must not linger");
    }
}
