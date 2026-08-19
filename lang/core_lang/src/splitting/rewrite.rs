//! Applies a finished [`SplitTable`](crate::splitting::split_table::SplitTable) to a labeled
//! program, rewriting every label into its split-copy name and physically duplicating each
//! data/codata declaration once per equivalence class. Mirrors `mono::specialize`'s "table built
//! once, then `flat_map` every declaration through a trait that walks and renames" shape.

use std::rc::Rc;

use crate::splitting::labeling::Label;
use crate::splitting::split_table::SplitTable;
use crate::syntax::declaration::{Polarity, TypeDeclaration, XtorSig};
use crate::syntax::{Chi, Clause, Ty};

/// This trait rewrites every label produced by [`crate::splitting::labeling::LabelAndUnify`] into
/// the name of the split-copy its equivalence class was assigned. The tree shape never changes,
/// only `Identifier`s inside `Ty::Decl` (and `Xtor`/`Clause` names, which are handled specially,
/// see below) are substituted, so this always returns `Self`.
pub trait Rewrite {
    fn rewrite(&self, table: &SplitTable) -> Self;
}

impl<X: Rewrite> Rewrite for Vec<X> {
    fn rewrite(&self, table: &SplitTable) -> Self {
        self.iter().map(|x| x.rewrite(table)).collect()
    }
}

impl<X: Rewrite> Rewrite for Rc<X> {
    fn rewrite(&self, table: &SplitTable) -> Self {
        Rc::new(self.as_ref().rewrite(table))
    }
}

impl<X: Rewrite> Rewrite for Option<X> {
    fn rewrite(&self, table: &SplitTable) -> Self {
        self.as_ref().map(|x| x.rewrite(table))
    }
}

/// Rewrites one clause of a match/comatch. Not a `Rewrite` impl: unlike every other node, a
/// `Clause` carries no `.ty` of its own, which split-copy it belongs to is determined by the
/// owning `XCase`'s `.ty`, so the caller must pass that label in. The `owner_label` is used to
/// rename the `Clause`'s `xtor` to the split-copy of the owning declaration.
pub fn rewrite_clause<C: Chi>(
    clause: &Clause<C>,
    table: &SplitTable,
    owner_label: &Label,
) -> Clause<C> {
    Clause {
        prdcns: clause.prdcns.clone(),
        xtor: table.resolve_xtor_name(&clause.xtor, owner_label).clone(),
        type_params: clause.type_params.clone(),
        context: clause.context.rewrite(table),
        body: clause.body.rewrite(table),
    }
}

/// Unwraps the label embedded in a `Ty::Decl`. Every type this is called on (an `Xtor`'s or
/// `XCase`'s own `.ty`) is guaranteed by the type system (see `Checked`) to be a declared type, so
/// anything else indicates a labeling bug.
pub fn label_in(ty: &Ty) -> &Label {
    match ty {
        Ty::Decl { name, .. } => name,
        _ => panic!("expected a labeled Ty::Decl, got {ty:?}"),
    }
}

/// Splits one data/codata declaration into one physical copy per equivalence class recorded for
/// it. A declaration with no recorded equivalence classes (never referenced anywhere in the
/// program) is kept as a single, unrenamed copy.
pub fn split_declaration<P: Polarity + Clone>(
    decl: &TypeDeclaration<P>,
    table: &SplitTable,
) -> Vec<TypeDeclaration<P>> {
    let roots = table.copies_for(&decl.name);
    if roots.is_empty() {
        return vec![build_declaration_copy(decl, table, None)];
    }
    roots
        .iter()
        .map(|root| build_declaration_copy(decl, table, Some(root)))
        .collect()
}

/// Builds one physical copy of a declaration for a given equivalence class. `root` identifies
/// which class this copy is for (renaming the declaration and its xtors accordingly via `table`);
/// `None` means the declaration has no recorded equivalence classes at all, so it's emitted
/// unrenamed as-is (see [`split_declaration`]).
fn build_declaration_copy<P: Polarity + Clone>(
    decl: &TypeDeclaration<P>,
    table: &SplitTable,
    root: Option<&Label>,
) -> TypeDeclaration<P> {
    TypeDeclaration {
        dat: decl.dat.clone(),
        name: match root {
            Some(root) => table.resolve_ty_name(root).clone(),
            None => decl.name.clone(),
        },
        xtors: decl
            .xtors
            .iter()
            .map(|xtor| split_xtor_sig(xtor, table, root))
            .collect(),
        type_params: decl.type_params.clone(),
    }
}

/// Rewrites one xtor's field types and, if `root` is given, renames it to its split copy's name
/// (paired with the same `root` as [`build_declaration_copy`], so e.g. `Cons__1` only ever ends up
/// inside `List__1`). `None` leaves the name unchanged, mirroring the unreferenced-declaration case
/// in `build_declaration_copy`.
fn split_xtor_sig<P: Polarity + Clone>(
    xtor: &XtorSig<P>,
    table: &SplitTable,
    root: Option<&Label>,
) -> XtorSig<P> {
    XtorSig {
        xtor: xtor.xtor.clone(),
        name: match root {
            Some(root) => table.resolve_xtor_name(&xtor.name, root).clone(),
            None => xtor.name.clone(),
        },
        type_params: xtor.type_params.clone(),
        args: xtor.args.rewrite(table),
    }
}

#[cfg(test)]
mod rewrite_tests {
    use super::*;
    use crate::splitting::union_find::UnionFind;
    use crate::syntax::{DataDeclaration, Ty};
    extern crate self as core_lang;
    use core_macros::{bind, ctor, ctor_sig, data, id, prd, ty};
    use std::collections::HashMap;

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
        crate::syntax::Identifier {
            name: format!("Box#{n}"),
            id: 0,
        }
    }

    #[test]
    fn split_declaration_produces_one_physical_copy_per_equivalence_class() {
        let mut uf = UnionFind::default();
        let a = box_label(1);
        let b = box_label(2);
        let label_origin = HashMap::from([(a.clone(), id!("Box")), (b.clone(), id!("Box"))]);
        let table = SplitTable::build(&mut uf, &label_origin, &[box_decl()], &[]);

        let copies = split_declaration(&box_decl(), &table);

        assert_eq!(copies.len(), 2);
        assert_ne!(copies[0].name, copies[1].name);
        for copy in &copies {
            assert_eq!(copy.xtors.len(), 1);
            // no leftover label Identifier (e.g. "Box#1") anywhere in the output
            assert!(!copy.name.name.contains('#'));
            assert!(!copy.xtors[0].name.name.contains('#'));
        }
    }

    #[test]
    fn split_declaration_keeps_an_unreferenced_declaration_as_a_single_unchanged_copy() {
        let mut uf = UnionFind::default();
        let table = SplitTable::build(&mut uf, &HashMap::new(), &[box_decl()], &[]);

        let copies = split_declaration(&box_decl(), &table);

        assert_eq!(copies.len(), 1);
        assert_eq!(copies[0].name, id!("Box"));
        assert_eq!(copies[0].xtors[0].name, id!("Wrap"));
    }

    #[test]
    fn xtor_rewrite_renames_type_and_constructor_name_from_the_same_index() {
        let mut uf = UnionFind::default();
        let a = box_label(1);
        let b = box_label(2);
        let label_origin = HashMap::from([(a.clone(), id!("Box")), (b.clone(), id!("Box"))]);
        let table = SplitTable::build(&mut uf, &label_origin, &[box_decl()], &[]);

        let occurrence = ctor!(id!("Wrap"), [], [], ty!(id!("Box")));
        let labeled = crate::syntax::Xtor {
            ty: Ty::Decl {
                name: a.clone(),
                type_args: Default::default(),
            },
            ..occurrence
        };

        let rewritten = labeled.rewrite(&table);

        let Ty::Decl { name: ty_name, .. } = &rewritten.ty else {
            panic!("expected Ty::Decl");
        };
        assert_eq!(
            ty_name.name.split("__").nth(1),
            rewritten.name.name.split("__").nth(1)
        );
    }
}
