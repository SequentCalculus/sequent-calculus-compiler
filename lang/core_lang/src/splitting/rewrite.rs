//! Applies a finished [`SplitTable`](crate::splitting::split_table::SplitTable) to a labeled
//! program, rewriting every label into its split-copy name and physically duplicating each
//! data/codata declaration once per equivalence class. Mirrors `mono::specialize`'s "table built
//! once, then `flat_map` every declaration through a trait that walks and renames" shape.

use std::rc::Rc;

use crate::splitting::labeling::Label;
use crate::splitting::split_table::SplitTable;
use crate::syntax::declaration::{Polarity, TypeDeclaration, XtorSig};
use crate::syntax::{
    Chi, Clause, ContextBinding, ID, Identifier, Ty, TypingContext, fresh_identifier,
};

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
    max_id: &mut ID,
) -> Vec<TypeDeclaration<P>> {
    let roots = table.copies_for(&decl.name);
    if roots.is_empty() {
        return vec![build_declaration_copy(decl, table, None, max_id)];
    }
    // only alpha-rename `type_params` when the declaration actually gets split into several
    // physical copies
    let alpha_rename = roots.len() > 1;
    roots
        .iter()
        .map(|root| build_declaration_copy(decl, table, Some((root, alpha_rename)), max_id))
        .collect()
}

/// Builds one physical copy of a declaration for a given equivalence class. `root` identifies
/// which class this copy is for (renaming the declaration and its xtors accordingly via `table`)
/// together with whether this copy's `type_params` must be alpha-renamed (only when the
/// declaration was actually split into several copies); `None` means
/// the declaration has no recorded equivalence classes at all, so it's emitted unrenamed as-is.
fn build_declaration_copy<P: Polarity + Clone>(
    decl: &TypeDeclaration<P>,
    table: &SplitTable,
    root: Option<(&Label, bool)>,
    max_id: &mut ID,
) -> TypeDeclaration<P> {
    // Every physical copy of a split declaration shares the same `type_params` `Identifier`s
    // (`decl.type_params.clone()`) unless renamed here. Since the constraint graph indexes its
    // nodes directly by these `Identifier`s (see `mono::constraint_graph::Node`), unrenamed copies
    // would collapse onto the very same graph node, making splitting unable to ever separate a
    // growing cycle. Minting fresh `id`s keeps
    // each copy's node distinct.
    let decl_subst = match root {
        Some((_, true)) => decl
            .type_params
            .iter()
            .map(|old| (old.clone(), fresh_identifier(max_id, &old.name)))
            .collect(),
        _ => vec![],
    };

    TypeDeclaration {
        dat: decl.dat.clone(),
        name: match root {
            Some((root, _)) => table.resolve_ty_name(root).clone(),
            None => decl.name.clone(),
        },
        xtors: decl
            .xtors
            .iter()
            .map(|xtor| split_xtor_sig(xtor, table, root, &decl_subst, max_id))
            .collect(),
        type_params: rename_params(&decl.type_params, &decl_subst),
    }
}

/// Rewrites one xtor's field types and, if `root` is given, renames it to its split copy's name
/// (paired with the same `root` as [`build_declaration_copy`], so e.g. `Cons__1` only ever ends up
/// inside `List__1`). `None` leaves the name unchanged, mirroring the unreferenced-declaration case
/// in `build_declaration_copy`.
fn split_xtor_sig<P: Polarity + Clone>(
    xtor: &XtorSig<P>,
    table: &SplitTable,
    root: Option<(&Label, bool)>,
    decl_subst: &[(Identifier, Identifier)],
    max_id: &mut ID,
) -> XtorSig<P> {
    let xtor_subst: Vec<(Identifier, Identifier)> = match root {
        Some((_, true)) => xtor
            .type_params
            .iter()
            .map(|old| (old.clone(), fresh_identifier(max_id, &old.name)))
            .collect(),
        _ => vec![],
    };
    let subst: Vec<(Identifier, Identifier)> = decl_subst
        .iter()
        .chain(xtor_subst.iter())
        .cloned()
        .collect();

    XtorSig {
        xtor: xtor.xtor.clone(),
        name: match root {
            Some((root, _)) => table.resolve_xtor_name(&xtor.name, root).clone(),
            None => xtor.name.clone(),
        },
        type_params: rename_params(&xtor.type_params, &xtor_subst),
        args: substitute_args(&xtor.args.rewrite(table), &subst),
    }
}

/// Applies an `old -> new` `Identifier` renaming to a `type_params` list, leaving it unchanged if
/// `subst` is empty.
fn rename_params(params: &[Identifier], subst: &[(Identifier, Identifier)]) -> Vec<Identifier> {
    if subst.is_empty() {
        return params.to_vec();
    }
    params
        .iter()
        .map(|param| {
            subst
                .iter()
                .find(|(old, _)| old == param)
                .map(|(_, new)| new.clone())
                .unwrap_or_else(|| param.clone())
        })
        .collect()
}

/// Applies an `old -> new` `Identifier` renaming to every field type in a typing context, via
/// [`Ty::substitute`]. Leaves the context unchanged if `subst` is empty.
fn substitute_args(args: &TypingContext, subst: &[(Identifier, Identifier)]) -> TypingContext {
    if subst.is_empty() {
        return args.clone();
    }
    let old_params: Vec<Identifier> = subst.iter().map(|(old, _)| old.clone()).collect();
    let new_tys: Vec<Ty> = subst.iter().map(|(_, new)| Ty::Var(new.clone())).collect();
    TypingContext {
        bindings: args
            .bindings
            .iter()
            .map(|binding| ContextBinding {
                var: binding.var.clone(),
                chi: binding.chi.clone(),
                ty: binding.ty.substitute((&old_params, &new_tys)),
            })
            .collect(),
    }
}

#[cfg(test)]
mod rewrite_tests {
    use super::*;
    use crate::splitting::union_find::UnionFind;
    use crate::syntax::{DataDeclaration, Ty};
    extern crate self as core_lang;
    use core_macros::{bind, ctor, ctor_sig, data, id, prd, tvar, ty};
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

    /// A generic `Pack[C] { Wrap(x: C) }`, whose field type references its own decl-level type
    /// parameter, used to check that alpha-renaming a split copy's `type_params` is applied
    /// consistently to the copy's own fields.
    fn pack_decl() -> DataDeclaration {
        data!(
            id!("Pack"),
            [ctor_sig!(
                id!("Wrap"),
                [],
                [bind!(id!("x"), prd!(), tvar!(id!("C", 1)))]
            )],
            [id!("C", 1)]
        )
    }

    #[test]
    fn split_declaration_produces_one_physical_copy_per_equivalence_class() {
        let mut uf = UnionFind::default();
        let a = box_label(1);
        let b = box_label(2);
        let label_origin = HashMap::from([(a.clone(), id!("Box")), (b.clone(), id!("Box"))]);
        let table = SplitTable::build(&mut uf, &label_origin, &[box_decl()], &[]);

        let mut max_id = 0;
        let copies = split_declaration(&box_decl(), &table, &mut max_id);

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

        let mut max_id = 0;
        let copies = split_declaration(&box_decl(), &table, &mut max_id);

        assert_eq!(copies.len(), 1);
        assert_eq!(copies[0].name, id!("Box"));
        assert_eq!(copies[0].xtors[0].name, id!("Wrap"));
    }

    #[test]
    fn split_declaration_gives_each_copy_disjoint_type_param_ids() {
        let mut uf = UnionFind::default();
        let a = box_label(1);
        let b = box_label(2);
        let label_origin = HashMap::from([(a, id!("Pack")), (b, id!("Pack"))]);
        let table = SplitTable::build(&mut uf, &label_origin, &[pack_decl()], &[]);

        let mut max_id = 0;
        let copies = split_declaration(&pack_decl(), &table, &mut max_id);

        assert_eq!(copies.len(), 2);
        assert_eq!(copies[0].type_params.len(), 1);
        assert_eq!(copies[1].type_params.len(), 1);
        // both copies must get fresh, mutually distinct, nonzero ids, otherwise the constraint
        // graph would index both copies' type parameter under the same node
        assert_ne!(copies[0].type_params[0], copies[1].type_params[0]);
        assert_ne!(copies[0].type_params[0].id, 0);
        assert_ne!(copies[1].type_params[0].id, 0);
    }

    #[test]
    fn split_declaration_renaming_is_consistent_between_decl_head_and_fields() {
        let mut uf = UnionFind::default();
        let a = box_label(1);
        let b = box_label(2);
        let label_origin = HashMap::from([(a, id!("Pack")), (b, id!("Pack"))]);
        let table = SplitTable::build(&mut uf, &label_origin, &[pack_decl()], &[]);

        let mut max_id = 0;
        let copies = split_declaration(&pack_decl(), &table, &mut max_id);

        for copy in &copies {
            let field_ty = &copy.xtors[0].args.bindings[0].ty;
            assert_eq!(field_ty, &Ty::Var(copy.type_params[0].clone()));
        }
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
