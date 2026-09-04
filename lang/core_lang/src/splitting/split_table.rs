//! Builds the table the rewrite phase consults to turn each label into the name of the physical
//! declaration copy its equivalence class was assigned. Mirrors `mono::naming_table::NamingTable`:
//! resolved once, up front, from the finished union-find, so the rewrite phase itself never needs
//! live access to the union-find.

use std::collections::HashMap;

use crate::splitting::labeling::Label;
use crate::splitting::union_find::UnionFind;
use crate::syntax::declaration::{Polarity, TypeDeclaration};
use crate::syntax::types::TypeArgs;
use crate::syntax::{CodataDeclaration, DataDeclaration, Identifier, Ty};

/// Maps every label minted during labeling to the name of the physical declaration copy its
/// equivalence class was assigned, and every original xtor name (paired with a label from its
/// owning declaration's equivalence class) to that xtor's name in the matching copy.
#[derive(Debug, Default)]
pub struct SplitTable {
    ty_names: HashMap<Label, Identifier>,
    xtor_names: HashMap<(Identifier, Label), Identifier>,
    roots_by_origin: HashMap<Identifier, Vec<Label>>,
}

impl SplitTable {
    /// Builds the table from the finished union-find. `label_origin` must be the same map
    /// `SplitState` accumulated during labeling (label -> original, unlabeled declaration name).
    pub fn build(
        uf: &mut UnionFind,
        label_origin: &[(Label, Identifier)],
        data_types: &[DataDeclaration],
        codata_types: &[CodataDeclaration],
    ) -> Self {
        let mut labels_by_origin: HashMap<Identifier, Vec<Label>> = HashMap::new();
        for (label, origin) in label_origin {
            labels_by_origin
                .entry(origin.clone())
                .or_default()
                .push(label.clone());
        }

        let mut ty_names: HashMap<Label, Identifier> = HashMap::new();
        let mut roots_by_origin: HashMap<Identifier, Vec<Label>> = HashMap::new();
        for (origin, labels) in &labels_by_origin {
            // distinct roots, in first-seen order, so the same run always assigns the same index
            let mut roots: Vec<Label> = vec![];
            for label in labels {
                let root = uf.find(label);
                let idx = roots.iter().position(|r| *r == root).unwrap_or_else(|| {
                    roots.push(root.clone());
                    roots.len() - 1
                });
                ty_names.insert(
                    label.clone(),
                    Identifier {
                        name: format!("{}__{}", origin.name, idx + 1),
                        id: origin.id,
                    },
                );
            }

            // only one equivalence class: keep the original name instead, so a declaration that
            // never actually gets split stays identical up to alpha-renaming
            if roots.len() == 1 {
                for label in labels {
                    ty_names.insert(label.clone(), origin.clone());
                }
            }
            roots_by_origin.insert(origin.clone(), roots);
        }

        let mut xtor_names: HashMap<(Identifier, Label), Identifier> = HashMap::new();
        for decl in data_types {
            register_xtor_names(
                decl,
                &labels_by_origin,
                &roots_by_origin,
                uf,
                &mut xtor_names,
            );
        }
        for decl in codata_types {
            register_xtor_names(
                decl,
                &labels_by_origin,
                &roots_by_origin,
                uf,
                &mut xtor_names,
            );
        }

        SplitTable {
            ty_names,
            xtor_names,
            roots_by_origin,
        }
    }

    /// Looks up the split-copy name for a given label.
    pub fn resolve_ty_name(&self, label: &Label) -> &Identifier {
        self.ty_names.get(label).unwrap_or_else(|| {
            panic!(
                "no split name recorded for label {} -- this indicates a bug in labeling or \
                 split-table construction",
                label.name
            )
        })
    }

    /// Looks up the split-copy name for an xtor, given a label belonging to its owning
    /// declaration's equivalence class (typically the label embedded in the specific `Xtor`
    /// occurrence's own `.ty`, not a label taken from `sigs`). Every xtor of every declaration is
    /// registered for every one of its labels, so this is total for any label minted during
    /// labeling -- dead-xtor dropping happens later, when the physical copies are built (see
    /// [`crate::splitting::rewrite::keeps_xtor`]), and never removes a name a term still uses.
    pub fn resolve_xtor_name(&self, original: &Identifier, owner_label: &Label) -> &Identifier {
        self.xtor_names
            .get(&(original.clone(), owner_label.clone()))
            .unwrap_or_else(|| {
                panic!(
                    "no split name recorded for xtor {} under label {} -- this indicates a \
                     bug in labeling or split-table construction",
                    original.name, owner_label.name
                )
            })
    }

    /// Returns every root label of every equivalence class recorded for `origin`, driving how
    /// many physical copies `split_declaration` emits. Empty if `origin` was never referenced
    /// anywhere in the program (a genuinely unused declaration), callers must treat that as "one
    /// unchanged copy", not "drop the declaration".
    pub fn copies_for(&self, origin: &Identifier) -> &[Label] {
        self.roots_by_origin
            .get(origin)
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }

    /// Resolves a type that was never labeled in the first place. The declared shape of a field
    /// that was never actually observed at any real occurrence (see
    /// [`crate::splitting::rewrite::build_field_args`]'s fallback for a never-constructed xtor).
    /// Such a type carries only an *origin* name (e.g. `List`, never `List#3`), which is only
    /// valid as-is when that origin was never split or
    /// never referenced at all. When the origin *was* split into several physical copies, that bare name
    /// belongs to none of them, since nothing was ever observed, any one of them is an equally
    /// valid (if arbitrary) choice, so this deterministically resolves to the first-assigned one.
    pub fn resolve_unobserved_ty(&self, ty: &Ty) -> Ty {
        match ty {
            Ty::I64 => Ty::I64,
            Ty::Var(v) => Ty::Var(v.clone()),
            Ty::Decl { name, type_args } => {
                let resolved_name = match self.roots_by_origin.get(name).and_then(|r| r.first()) {
                    Some(root) => self.resolve_ty_name(root).clone(),
                    None => name.clone(),
                };
                Ty::Decl {
                    name: resolved_name,
                    type_args: TypeArgs {
                        args: type_args
                            .args
                            .iter()
                            .map(|arg| self.resolve_unobserved_ty(arg))
                            .collect(),
                    },
                }
            }
        }
    }
}

/// Registers the split-copy name of every xtor of `decl`, for every label in `decl.name`'s
/// equivalence classes, sharing the same per-class index assigned to the declaration itself so
/// e.g. `Cons__1` is always paired with `List__1`. Purely a naming step: which of these xtors a
/// given physical copy actually keeps is decided separately, when the copy is built (see
/// [`crate::splitting::rewrite::keeps_xtor`]).
fn register_xtor_names<P: Polarity>(
    decl: &TypeDeclaration<P>,
    labels_by_origin: &HashMap<Identifier, Vec<Label>>,
    roots_by_origin: &HashMap<Identifier, Vec<Label>>,
    uf: &mut UnionFind,
    xtor_names: &mut HashMap<(Identifier, Label), Identifier>,
) {
    let Some(roots) = roots_by_origin.get(&decl.name) else {
        return;
    };
    let Some(labels) = labels_by_origin.get(&decl.name) else {
        return;
    };

    for xtor in &decl.xtors {
        for label in labels {
            let root = uf.find(label);
            let idx = roots.iter().position(|r| *r == root).unwrap();
            let name = if roots.len() == 1 {
                xtor.name.clone()
            } else {
                // Preserve the xtor's own unique `id` (assigned by `fresh_identifier` in
                // `fun2core::declaration::compile_ctor`/`compile_dtor` specifically to keep an
                // xtor's name distinct from its owning declaration's, e.g. `Pair { Pair(...) }`)
                Identifier {
                    name: format!("{}__{}", xtor.name.name, idx + 1),
                    id: xtor.name.id,
                }
            };
            xtor_names.insert((xtor.name.clone(), label.clone()), name);
        }
    }
}

#[cfg(test)]
mod split_table_tests {
    use super::*;
    use crate::syntax::DataDeclaration;
    extern crate self as core_lang;
    use core_macros::{bind, ctor_sig, data, id, prd, ty};

    fn box_decl() -> DataDeclaration {
        data!(
            id!("Box"),
            [ctor_sig!(
                id!("Wrap"),
                [],
                [bind!(id!("x"), prd!(), ty!("int"))]
            )],
            []
        )
    }

    fn box_label(n: usize) -> Label {
        Identifier {
            name: format!("Box#{n}"),
            id: 0,
        }
    }

    #[test]
    fn build_assigns_copy_indices_in_the_order_label_origin_lists_them() {
        let mut uf = UnionFind::default();
        // three distinct, never-unified occurrences, deliberately listed out of numeric order
        let c = box_label(3);
        let a = box_label(1);
        let b = box_label(2);
        let label_origin = vec![
            (c.clone(), id!("Box")),
            (a.clone(), id!("Box")),
            (b.clone(), id!("Box")),
        ];

        let table = SplitTable::build(&mut uf, &label_origin, &[box_decl()], &[]);

        assert_eq!(table.resolve_ty_name(&c).name, "Box__1");
        assert_eq!(table.resolve_ty_name(&a).name, "Box__2");
        assert_eq!(table.resolve_ty_name(&b).name, "Box__3");
    }

    #[test]
    fn build_assigns_two_distinct_names_for_two_never_unified_occurrences() {
        let mut uf = UnionFind::default();
        let a = box_label(1);
        let b = box_label(2);
        let label_origin = vec![(a.clone(), id!("Box")), (b.clone(), id!("Box"))];

        let table = SplitTable::build(&mut uf, &label_origin, &[box_decl()], &[]);

        assert_eq!(table.copies_for(&id!("Box")).len(), 2);
        assert_ne!(table.resolve_ty_name(&a), table.resolve_ty_name(&b));
    }

    #[test]
    fn build_keeps_bare_name_when_only_one_equivalence_class_exists() {
        let mut uf = UnionFind::default();
        let a = box_label(1);
        let b = box_label(2);
        uf.union(&a, &b);
        let label_origin = vec![(a.clone(), id!("Box")), (b.clone(), id!("Box"))];

        let table = SplitTable::build(&mut uf, &label_origin, &[box_decl()], &[]);

        assert_eq!(table.copies_for(&id!("Box")).len(), 1);
        assert_eq!(table.resolve_ty_name(&a), &id!("Box"));
        assert_eq!(table.resolve_ty_name(&b), &id!("Box"));
    }

    #[test]
    fn resolve_xtor_name_shares_index_with_owning_decl_copy() {
        let mut uf = UnionFind::default();
        let a = box_label(1);
        let b = box_label(2);
        let label_origin = vec![(a.clone(), id!("Box")), (b.clone(), id!("Box"))];

        let table = SplitTable::build(&mut uf, &label_origin, &[box_decl()], &[]);

        let ty_name_a = table.resolve_ty_name(&a).clone();
        let xtor_name_a = table.resolve_xtor_name(&id!("Wrap"), &a);
        // whichever suffix `Box` got for label `a`, `Wrap` must carry the identical suffix
        assert_eq!(
            ty_name_a.name.split("__").nth(1),
            xtor_name_a.name.split("__").nth(1)
        );
    }

    #[test]
    fn copies_for_is_empty_for_a_never_referenced_declaration() {
        let mut uf = UnionFind::default();
        let table = SplitTable::build(&mut uf, &[], &[box_decl()], &[]);
        assert!(table.copies_for(&id!("Box")).is_empty());
    }

    /// A field type that was never labeled (no real occurrence ever observed it, see
    /// `crate::splitting::rewrite::build_field_args`'s fallback) only names its *origin*
    /// declaration, not any specific split copy. If that origin was itself split into several
    /// physical copies, the bare origin name belongs to none of them -- `resolve_unobserved_ty`
    /// must resolve it to one of the actual copies instead of leaving a dangling reference.
    #[test]
    fn resolve_unobserved_ty_picks_an_actual_copy_when_the_origin_was_split() {
        let mut uf = UnionFind::default();
        let a = box_label(1);
        let b = box_label(2);
        let label_origin = vec![(a.clone(), id!("Box")), (b.clone(), id!("Box"))];

        let table = SplitTable::build(&mut uf, &label_origin, &[box_decl()], &[]);

        let bare_box = crate::syntax::Ty::Decl {
            name: id!("Box"),
            type_args: Default::default(),
        };
        let resolved = table.resolve_unobserved_ty(&bare_box);
        let crate::syntax::Ty::Decl { name, .. } = &resolved else {
            panic!("expected Ty::Decl");
        };
        // must be one of the two actual split copies, never the bare, now-nonexistent "Box"
        assert_ne!(name, &id!("Box"));
        assert!(name == table.resolve_ty_name(&a) || name == table.resolve_ty_name(&b));
    }

    #[test]
    fn resolve_unobserved_ty_keeps_the_bare_name_when_the_origin_was_never_split() {
        let mut uf = UnionFind::default();
        let a = box_label(1);
        let label_origin = vec![(a.clone(), id!("Box"))];

        let table = SplitTable::build(&mut uf, &label_origin, &[box_decl()], &[]);

        let bare_box = crate::syntax::Ty::Decl {
            name: id!("Box"),
            type_args: Default::default(),
        };
        assert_eq!(table.resolve_unobserved_ty(&bare_box), bare_box);
    }
}
