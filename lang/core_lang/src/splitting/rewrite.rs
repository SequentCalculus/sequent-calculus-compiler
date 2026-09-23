//! Applies a finished [`SplitTable`](crate::splitting::split_table::SplitTable) to a labeled
//! program, rewriting every label into its split-copy name and physically duplicating each
//! data/codata declaration once per equivalence class. Mirrors `mono::specialize`'s "table built
//! once, then `flat_map` every declaration through a trait that walks and renames" shape.

use std::rc::Rc;

use crate::splitting::labeling::{ClassFields, Label, UsedXtors};
use crate::splitting::split_table::SplitTable;
use crate::syntax::declaration::{Polarity, TypeDeclaration, XtorSig};
use crate::syntax::{
    Chi, Clause, ContextBinding, ID, Identifier, Ty, TypeParam, TypingContext, fresh_identifier,
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
///
/// A clause is never dropped: matching an xtor counts as using it (see [`keeps_xtor`]), so the xtor
/// it names is still present in the owning declaration's copy and always resolves.
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

/// Splits one data/codata declaration into one physical copy per equivalence class recorded for
/// it. A declaration with no recorded equivalence classes (never referenced anywhere in the
/// program) is kept as a single, unrenamed copy.
pub fn split_declaration<P: Polarity + Clone>(
    decl: &TypeDeclaration<P>,
    table: &SplitTable,
    class_fields: &ClassFields,
    used: &UsedXtors,
    max_id: &mut ID,
) -> Vec<TypeDeclaration<P>> {
    let roots = table.copies_for(&decl.name);
    if roots.is_empty() {
        return vec![build_declaration_copy(
            decl,
            table,
            class_fields,
            used,
            None,
            max_id,
        )];
    }
    // only alpha-rename `type_params` when the declaration actually gets split into several
    // physical copies
    let alpha_rename = roots.len() > 1;
    roots
        .iter()
        .map(|root| {
            build_declaration_copy(
                decl,
                table,
                class_fields,
                used,
                Some((root, alpha_rename)),
                max_id,
            )
        })
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
    class_fields: &ClassFields,
    used: &UsedXtors,
    root: Option<(&Label, bool)>,
    max_id: &mut ID,
) -> TypeDeclaration<P> {
    // Every physical copy of a split declaration shares the same `type_params` `Identifier`s
    // (`decl.type_params.clone()`) unless renamed here. Since the constraint graph indexes its
    // nodes directly by these `Identifier`s (see `mono::constraint_graph::Node`), unrenamed copies
    // would collapse onto the very same graph node, making splitting unable to ever separate a
    // growing cycle. Minting fresh `id`s keeps
    // each copy's node distinct.
    let decl_subst: Vec<(Identifier, Identifier)> = match root {
        Some((_, true)) => decl
            .type_params
            .iter()
            .map(|old| (old.name.clone(), fresh_identifier(max_id, &old.name.name)))
            .collect(),
        _ => vec![],
    };

    // Whether *no* xtor of `decl` is used for this equivalence class at all. Computed once
    // here, over all of `decl.xtors`, rather than inside `keeps_xtor` itself, which would redo
    // this same full scan for every one of `decl.xtors`, making the filter below quadratic in
    // their count.
    let class_unused = root.is_some_and(|(root, _)| {
        decl.xtors
            .iter()
            .all(|xtor| !used.contains(root, &xtor.name))
    });

    TypeDeclaration {
        dat: decl.dat.clone(),
        name: match root {
            Some((root, _)) => table.resolve_ty_name(root).clone(),
            None => decl.name.clone(),
        },
        xtors: decl
            .xtors
            .iter()
            .filter(|xtor| keeps_xtor(xtor, used, root.map(|(r, _)| r), class_unused))
            .map(|xtor| split_xtor_sig(xtor, table, class_fields, root, &decl_subst, max_id))
            .collect(),
        type_params: rename_params(&decl.type_params, &decl_subst),
    }
}

/// Decides whether `xtor` survives in the physical copy `root` identifies, the one and only place
/// dead-xtor dropping happens.
///
/// An xtor is dropped exactly when it is *unused* for that equivalence class: neither constructed
/// or observed at any `Xtor` node, nor matched or defined by any `case`/`new` clause (see
/// [`crate::splitting::labeling::SplitState::record_xtor_use`]). Merely matching an xtor keeps it,
/// so no term ever loses a clause and every name a term uses still resolves. `class_unused` is
/// whether *no* xtor of the owning declaration is used for this equivalence class at all.
/// The caller computes it once, over every xtor, rather than this function recomputing it on
/// every call.
pub fn keeps_xtor<P: Polarity>(
    xtor: &XtorSig<P>,
    used: &UsedXtors,
    root: Option<&Label>,
    class_unused: bool,
) -> bool {
    let Some(root) = root else {
        return true;
    };
    used.contains(root, &xtor.name) || class_unused
}

/// Rewrites one xtor's field types and, if `root` is given, renames it to its split copy's name
/// (paired with the same `root` as [`build_declaration_copy`], so e.g. `Cons#1` only ever ends up
/// inside `List#1`). `None` leaves the name unchanged, mirroring the unreferenced-declaration case
/// in [`build_declaration_copy`]. Only ever called for xtors [`keeps_xtor`] let through, so the
/// name always resolves.
fn split_xtor_sig<P: Polarity + Clone>(
    xtor: &XtorSig<P>,
    table: &SplitTable,
    class_fields: &ClassFields,
    root: Option<(&Label, bool)>,
    decl_subst: &[(Identifier, Identifier)],
    max_id: &mut ID,
) -> XtorSig<P> {
    let name = match root {
        Some((root, _)) => table.resolve_xtor_name(&xtor.name, root).clone(),
        None => xtor.name.clone(),
    };

    let xtor_subst: Vec<(Identifier, Identifier)> = match root {
        Some((_, true)) => xtor
            .type_params
            .iter()
            .map(|old| (old.name.clone(), fresh_identifier(max_id, &old.name.name)))
            .collect(),
        _ => vec![],
    };
    let subst: Vec<(Identifier, Identifier)> = decl_subst
        .iter()
        .chain(xtor_subst.iter())
        .cloned()
        .collect();

    let args = build_field_args(xtor, table, class_fields, root);

    XtorSig {
        xtor: xtor.xtor.clone(),
        name,
        type_params: rename_params(&xtor.type_params, &xtor_subst),
        args: substitute_args(&args, &subst),
    }
}

/// Builds one physical copy's field types from the copy of each field its equivalence class
/// (`root`) holds (see [`crate::splitting::union_find::ClassData`]), so e.g. two split copies of
/// `Bar` each end up pointing at their own, independently split copy of a nested or
/// self-referential field. A class's field copy already has exactly the shape the physical copy
/// declares: the declared type with every head labeled for this class and every type parameter
/// left as a variable, so rewriting its labels is all that is left to do.
///
/// A field no occurrence of the class ever touched falls back to its declared shape, resolved via
/// `resolve_unlabeled_ty`. That shape only names the *origin* type, not a specific split copy of
/// it, so if the origin itself was split into several physical copies, one of them is picked
/// explicitly, rather than leaving a name that belongs to none of the copies dangling.
fn build_field_args<P: Polarity + Clone>(
    xtor: &XtorSig<P>,
    table: &SplitTable,
    class_fields: &ClassFields,
    root: Option<(&Label, bool)>,
) -> TypingContext {
    TypingContext {
        bindings: xtor
            .args
            .bindings
            .iter()
            .enumerate()
            .map(|(i, binding)| {
                let ty = match root.and_then(|(r, _)| class_fields.get(r, &xtor.name, i)) {
                    Some(class_field) => class_field.rewrite(table),
                    // no occurrence of this class touched the field: `binding.ty` is unlabeled
                    None => table.resolve_unlabeled_ty(&binding.ty),
                };
                ContextBinding {
                    var: binding.var.clone(),
                    chi: binding.chi.clone(),
                    ty,
                }
            })
            .collect(),
    }
}

/// Applies an `old -> new` `Identifier` renaming to a `type_params` list, leaving it unchanged if
/// `subst` is empty. Each parameter's declared `ParamPolarity` is preserved across the rename,
/// only the `Identifier` changes, since renaming is purely an alpha-renaming for constraint-graph
/// node freshness, never a change in what polarity was actually declared.
fn rename_params(params: &[TypeParam], subst: &[(Identifier, Identifier)]) -> Vec<TypeParam> {
    if subst.is_empty() {
        return params.to_vec();
    }
    params
        .iter()
        .map(|param| {
            let name = subst
                .iter()
                .find(|(old, _)| old == &param.name)
                .map(|(_, new)| new.clone())
                .unwrap_or_else(|| param.name.clone());
            TypeParam {
                name,
                polarity: param.polarity,
            }
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
    use crate::splitting::labeling::{SplitState, finish_classes, label_in};
    use crate::splitting::union_find::UnionFind;
    use crate::syntax::{DataDeclaration, Ty};
    extern crate self as core_lang;
    use core_macros::{bind, ctor, ctor_sig, data, id, prd, tparam, tvar, ty};

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

    fn used_for(xtor: Identifier, labels: &[Label]) -> UsedXtors {
        labels
            .iter()
            .map(|label| (label.clone(), xtor.clone()))
            .collect()
    }

    /// A two-ctor declaration, so a class can use one of its xtors without the other.
    fn choice_decl() -> DataDeclaration {
        data!(
            id!("Choice"),
            [
                ctor_sig!(id!("Left"), [], []),
                ctor_sig!(id!("Right"), [], [])
            ],
            []
        )
    }

    #[test]
    fn keeps_xtor_drops_the_unused_xtor_of_a_class_that_uses_another() {
        let decl = choice_decl();
        let root = box_label(1);
        let used = used_for(id!("Left"), std::slice::from_ref(&root));

        assert!(keeps_xtor(&decl.xtors[0], &used, Some(&root), false));
        assert!(!keeps_xtor(&decl.xtors[1], &used, Some(&root), false));
    }

    #[test]
    fn keeps_xtor_keeps_everything_for_a_class_that_uses_no_xtor_at_all() {
        let decl = choice_decl();
        let root = box_label(1);
        let used = UsedXtors::default();

        assert!(
            decl.xtors
                .iter()
                .all(|xtor| keeps_xtor(xtor, &used, Some(&root), true))
        );
    }

    /// A declaration nothing in the program ever references is emitted as one unchanged copy, so
    /// nothing may be dropped from it either.
    #[test]
    fn keeps_xtor_keeps_everything_for_an_unreferenced_declaration() {
        let decl = choice_decl();
        let used = UsedXtors::default();

        assert!(
            decl.xtors
                .iter()
                .all(|xtor| keeps_xtor(xtor, &used, None, false))
        );
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
            [tparam!(id!("C", 1), "+")]
        )
    }

    #[test]
    fn split_declaration_produces_one_physical_copy_per_equivalence_class() {
        let mut uf = UnionFind::default();
        let a = box_label(1);
        let b = box_label(2);
        let label_origin = vec![(a.clone(), id!("Box")), (b.clone(), id!("Box"))];
        let used = used_for(id!("Wrap"), &[a.clone(), b.clone()]);
        let table = SplitTable::build(&mut uf, &label_origin, &[box_decl()], &[]);

        let mut max_id = 0;
        let class_fields = ClassFields::default();
        let copies = split_declaration(&box_decl(), &table, &class_fields, &used, &mut max_id);

        assert_eq!(copies.len(), 2);
        assert_ne!(copies[0].name, copies[1].name);
        for copy in &copies {
            assert_eq!(copy.xtors.len(), 1);
        }
    }

    #[test]
    fn split_declaration_keeps_an_unreferenced_declaration_as_a_single_unchanged_copy() {
        let mut uf = UnionFind::default();
        let table = SplitTable::build(&mut uf, &[], &[box_decl()], &[]);

        let mut max_id = 0;
        let class_fields = ClassFields::default();
        let used = UsedXtors::default();
        let copies = split_declaration(&box_decl(), &table, &class_fields, &used, &mut max_id);

        assert_eq!(copies.len(), 1);
        assert_eq!(copies[0].name, id!("Box"));
        assert_eq!(copies[0].xtors[0].name, id!("Wrap"));
    }

    #[test]
    fn split_declaration_gives_each_copy_disjoint_type_param_ids() {
        let mut uf = UnionFind::default();
        let a = box_label(1);
        let b = box_label(2);
        let label_origin = vec![(a.clone(), id!("Pack")), (b.clone(), id!("Pack"))];
        let used = used_for(id!("Wrap"), &[a, b]);
        let table = SplitTable::build(&mut uf, &label_origin, &[pack_decl()], &[]);

        let mut max_id = 0;
        let class_fields = ClassFields::default();
        let copies = split_declaration(&pack_decl(), &table, &class_fields, &used, &mut max_id);

        assert_eq!(copies.len(), 2);
        assert_eq!(copies[0].type_params.len(), 1);
        assert_eq!(copies[1].type_params.len(), 1);
        // both copies must get fresh, mutually distinct, nonzero ids, otherwise the constraint
        // graph would index both copies' type parameter under the same node
        assert_ne!(copies[0].type_params[0], copies[1].type_params[0]);
        assert_ne!(copies[0].type_params[0].name.id, 0);
        assert_ne!(copies[1].type_params[0].name.id, 0);
    }

    #[test]
    fn split_declaration_renaming_is_consistent_between_decl_head_and_fields() {
        let mut uf = UnionFind::default();
        let a = box_label(1);
        let b = box_label(2);
        let label_origin = vec![(a.clone(), id!("Pack")), (b.clone(), id!("Pack"))];
        let used = used_for(id!("Wrap"), &[a, b]);
        let table = SplitTable::build(&mut uf, &label_origin, &[pack_decl()], &[]);

        let mut max_id = 0;
        let class_fields = ClassFields::default();
        let copies = split_declaration(&pack_decl(), &table, &class_fields, &used, &mut max_id);

        for copy in &copies {
            let field_ty = &copy.xtors[0].args.bindings[0].ty;
            assert_eq!(field_ty, &Ty::Var(copy.type_params[0].name.clone()));
        }
    }

    #[test]
    fn xtor_rewrite_renames_type_and_constructor_name_from_the_same_index() {
        let mut uf = UnionFind::default();
        let a = box_label(1);
        let b = box_label(2);
        let label_origin = vec![(a.clone(), id!("Box")), (b.clone(), id!("Box"))];
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
            ty_name.name.split('#').nth(1),
            rewritten.name.name.split('#').nth(1)
        );
    }

    fn bar_decl(foo_field: Ty) -> DataDeclaration {
        data!(
            id!("Bar"),
            [ctor_sig!(
                id!("MkBar"),
                [],
                [bind!(id!("f"), prd!(), foo_field)]
            )],
            []
        )
    }

    /// The central proof that independently constructed, non-self-referential fields no longer
    /// bleed into each other: two `Bar` copies, each holding its own copy of the `Foo` field, end
    /// up with two different field types instead of one shared one.
    #[test]
    fn split_declaration_gives_each_copy_its_own_field() {
        let mut state = SplitState::default();
        let bar_a = state.label_ty(&ty!(id!("Bar")));
        let bar_b = state.label_ty(&ty!(id!("Bar")));
        let foo_a = state.label_ty(&ty!(id!("Foo")));
        let foo_b = state.label_ty(&ty!(id!("Foo")));
        for (bar, foo) in [(&bar_a, &foo_a), (&bar_b, &foo_b)] {
            let field = state.class_field(label_in(bar), &id!("MkBar"), 0, &ty!(id!("Foo")));
            state.unify_ty(foo, &field);
        }
        let (class_fields, _) = finish_classes(&state);

        // taken from the state rather than written out by hand, since each class's copy of the
        // field minted its own `Foo` label
        let label_origin = state.label_origin.clone();
        let decl = bar_decl(foo_a.clone());
        let used = used_for(
            id!("MkBar"),
            &[label_in(&bar_a).clone(), label_in(&bar_b).clone()],
        );
        let table = SplitTable::build(
            &mut state.uf,
            &label_origin,
            std::slice::from_ref(&decl),
            &[],
        );

        let mut max_id = 0;
        let copies = split_declaration(&decl, &table, &class_fields, &used, &mut max_id);

        assert_eq!(copies.len(), 2);
        let field_ty = |copy: &DataDeclaration| copy.xtors[0].args.bindings[0].ty.clone();
        assert_ne!(field_ty(&copies[0]), field_ty(&copies[1]));
    }

    #[test]
    fn split_declaration_falls_back_to_declared_shape_when_never_constructed() {
        let mut state = SplitState::default();
        let bar = state.label_ty(&ty!(id!("Bar")));

        let label_origin = vec![(label_in(&bar).clone(), id!("Bar"))];
        let decl = bar_decl(ty!(id!("Foo")));
        // `MkBar` itself was constructed (so it isn't dropped), only its field was never
        // touched, so the class holds no copy of it.
        let used = used_for(id!("MkBar"), &[label_in(&bar).clone()]);
        let table = SplitTable::build(
            &mut state.uf,
            &label_origin,
            std::slice::from_ref(&decl),
            &[],
        );

        let class_fields = ClassFields::default();
        let mut max_id = 0;
        let copies = split_declaration(&decl, &table, &class_fields, &used, &mut max_id);

        assert_eq!(copies.len(), 1);
        assert_eq!(copies[0].xtors[0].args.bindings[0].ty, ty!(id!("Foo")));
    }
}
