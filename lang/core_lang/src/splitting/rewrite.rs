//! Applies a finished [`SplitTable`](crate::splitting::split_table::SplitTable) to a labeled
//! program, rewriting every label into its split-copy name and physically duplicating each
//! data/codata declaration once per equivalence class. The table is built once, then every
//! declaration is `flat_map`ped through a trait that walks and renames.

use std::rc::Rc;

use crate::splitting::labeling::{Label, UsedXtors};
use crate::splitting::split_table::{SplitPlan, SplitTable};
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

/// Which physical copy of a declaration is currently being built.
#[derive(Debug, Clone, Copy)]
pub enum CopyOf<'a> {
    /// Nothing in the program ever referenced the declaration, so it is emitted once, unrenamed
    /// and complete.
    Unreferenced,
    /// One copy for the equivalence class rooted at `root`. `alpha_rename` is set only when the
    /// declaration is actually split into several copies, see `build_declaration_copy`.
    Class { root: &'a Label, alpha_rename: bool },
}

/// Splits one data/codata declaration into one physical copy per equivalence class recorded for
/// it. A declaration with no recorded equivalence classes (never referenced anywhere in the
/// program) is kept as a single, unrenamed copy.
pub fn split_declaration<P: Polarity + Clone>(
    decl: &TypeDeclaration<P>,
    plan: &SplitPlan,
    max_id: &mut ID,
) -> Vec<TypeDeclaration<P>> {
    let roots = plan.table.copies_for(&decl.name);
    if roots.is_empty() {
        return vec![build_declaration_copy(
            decl,
            plan,
            CopyOf::Unreferenced,
            max_id,
        )];
    }
    // only alpha-rename `type_params` when the declaration actually gets split into several
    // physical copies
    let alpha_rename = roots.len() > 1;
    roots
        .iter()
        .map(|root| {
            build_declaration_copy(decl, plan, CopyOf::Class { root, alpha_rename }, max_id)
        })
        .collect()
}

/// Builds one physical copy of a declaration, for the class `copy` identifies.
fn build_declaration_copy<P: Polarity + Clone>(
    decl: &TypeDeclaration<P>,
    plan: &SplitPlan,
    copy: CopyOf,
    max_id: &mut ID,
) -> TypeDeclaration<P> {
    // Every physical copy of a split declaration shares the same `type_params` `Identifier`s
    // (`decl.type_params.clone()`) unless renamed here. A type parameter is identified by its
    // `Identifier` (name and id), so unrenamed copies would share their parameters with each
    // other and could no longer be told apart. Minting fresh `id`s gives every copy parameters of
    // its own.
    let decl_subst: Vec<(Identifier, Identifier)> = match copy {
        CopyOf::Class {
            alpha_rename: true, ..
        } => decl
            .type_params
            .iter()
            .map(|old| (old.name.clone(), fresh_identifier(max_id, &old.name.name)))
            .collect(),
        _ => vec![],
    };

    TypeDeclaration {
        dat: decl.dat.clone(),
        name: match copy {
            CopyOf::Class { root, .. } => plan.table.resolve_ty_name(root).clone(),
            CopyOf::Unreferenced => decl.name.clone(),
        },
        xtors: decl
            .xtors
            .iter()
            .filter(|xtor| keeps_xtor(xtor, &plan.used_xtors, copy))
            .map(|xtor| split_xtor_sig(xtor, plan, copy, &decl_subst, max_id))
            .collect(),
        type_params: rename_params(&decl.type_params, &decl_subst),
    }
}

/// Decides whether `xtor` survives in the physical copy `copy` identifies, the one and only place
/// dead-xtor dropping happens.
///
/// An xtor is dropped exactly when it is *unused* for that equivalence class: neither constructed
/// or observed at any `Xtor` node, nor matched or defined by any `case`/`new` clause (see
/// [`crate::splitting::labeling::SplitState::record_xtor_use`]). Merely matching an xtor keeps it,
/// so no term ever loses a clause and every name a term uses still resolves.
///
/// A class that uses none of them keeps none, so its copy is emitted with an empty xtor list. That
/// is sound for the same reason the partial case is: a value of the copy can only come from an
/// occurrence in its own class, so a class that never constructs one has no values at all, and
/// every position typed by that copy is dead. `core2axcut`s `shrink_unknown_cuts` turns the one
/// place that would otherwise need a clause, the eta-expansion of a cut of a variable against a
/// covariable, into `unreachable`.
pub fn keeps_xtor<P: Polarity>(xtor: &XtorSig<P>, used: &UsedXtors, copy: CopyOf) -> bool {
    match copy {
        CopyOf::Unreferenced => true,
        CopyOf::Class { root, .. } => used.contains(root, &xtor.name),
    }
}

/// Rewrites one xtor's field types and, for a class copy, renames it to that copy's name (paired
/// with the same class as [`build_declaration_copy`], so e.g. `Cons#1` only ever ends up inside
/// `List#1`). Only ever called for xtors [`keeps_xtor`] let through, so the name always resolves.
fn split_xtor_sig<P: Polarity + Clone>(
    xtor: &XtorSig<P>,
    plan: &SplitPlan,
    copy: CopyOf,
    decl_subst: &[(Identifier, Identifier)],
    max_id: &mut ID,
) -> XtorSig<P> {
    let name = match copy {
        CopyOf::Class { root, .. } => plan.table.resolve_xtor_name(&xtor.name, root).clone(),
        CopyOf::Unreferenced => xtor.name.clone(),
    };

    let xtor_subst: Vec<(Identifier, Identifier)> = match copy {
        CopyOf::Class {
            alpha_rename: true, ..
        } => xtor
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

    let args = build_field_args(xtor, plan, copy);

    XtorSig {
        xtor: xtor.xtor.clone(),
        name,
        type_params: rename_params(&xtor.type_params, &xtor_subst),
        args: substitute_args(&args, &subst),
    }
}

/// Builds one physical copy's field types.
///
/// For a class copy they are the field copies that class holds (see
/// [`crate::splitting::union_find::ClassData`]), so e.g. two split copies of `Bar` each end up
/// pointing at their own, independently split copy of a nested or self-referential field. A
/// class's field copy already has exactly the shape the physical copy declares: the declared type
/// with every head labeled for this class and every type parameter left as a variable, so
/// rewriting its labels is all that is left to do.
///
/// For an unreferenced declaration there is no class and hence no field copy, so the declared
/// shape is all there is to go on. It names only the *origin* type, not a specific split copy of
/// it, so if the origin itself was split, `resolve_unlabeled_ty` picks one of its actual copies
/// rather than leaving a name that belongs to none of them.
fn build_field_args<P: Polarity + Clone>(
    xtor: &XtorSig<P>,
    plan: &SplitPlan,
    copy: CopyOf,
) -> TypingContext {
    TypingContext {
        bindings: xtor
            .args
            .bindings
            .iter()
            .enumerate()
            .map(|(i, binding)| ContextBinding {
                var: binding.var.clone(),
                chi: binding.chi.clone(),
                ty: match copy {
                    CopyOf::Class { root, .. } => {
                        class_field(plan, root, xtor, i).rewrite(&plan.table)
                    }
                    CopyOf::Unreferenced => plan.table.resolve_unlabeled_ty(&binding.ty),
                },
            })
            .collect(),
    }
}

/// Looks up the copy of `xtor`'s `index`-th field that the class rooted at `root` holds.
///
/// That copy always exists for an xtor the class actually uses, and `build_declaration_copy` only
/// ever builds a field list for such an xtor, since `keeps_xtor` drops every other one: recording
/// a use and creating the class's copy of every one of that xtor's fields happen in the same step,
/// in `constrain_xtor_occurrence`. A missing copy therefore means the two have drifted apart.
fn class_field<'a, P: Polarity>(
    plan: &'a SplitPlan,
    root: &Label,
    xtor: &XtorSig<P>,
    index: usize,
) -> &'a Ty {
    plan.class_fields
        .get(root, &xtor.name, index)
        .unwrap_or_else(|| {
            panic!(
                "class {} uses {} but holds no copy of its field {index} -- this indicates a bug \
                 in labeling or in dead-xtor dropping",
                root.name, xtor.name.name
            )
        })
}

/// Applies an `old -> new` `Identifier` renaming to a `type_params` list, leaving it unchanged if
/// `subst` is empty. Each parameter's declared `ParamPolarity` is preserved across the rename,
/// only the `Identifier` changes, since renaming is purely an alpha-renaming that makes the
/// parameters of each copy distinct, never a change in what polarity was actually declared.
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
    use crate::splitting::labeling::{ClassFields, SplitState, finish_classes, label_in};
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

    /// Bundles a hand-built table with hand-built class data, the way `SplitPlan::build` would
    /// from a finished walk.
    fn plan_of(table: SplitTable, class_fields: ClassFields, used_xtors: UsedXtors) -> SplitPlan {
        SplitPlan {
            table,
            class_fields,
            used_xtors,
        }
    }

    fn class_copy(root: &Label) -> CopyOf<'_> {
        CopyOf::Class {
            root,
            alpha_rename: false,
        }
    }

    /// The class data a real walk leaves behind for `decl`: every root holds a copy of every
    /// field of every xtor. A class's field copy keeps the declared shape (type parameters stay
    /// variables), so for these tests the declared type is exactly that copy.
    fn fields_for(decl: &DataDeclaration, roots: &[Label]) -> ClassFields {
        roots
            .iter()
            .flat_map(|root| {
                decl.xtors.iter().flat_map(move |xtor| {
                    xtor.args
                        .bindings
                        .iter()
                        .enumerate()
                        .map(move |(i, binding)| {
                            ((root.clone(), xtor.name.clone(), i), binding.ty.clone())
                        })
                })
            })
            .collect()
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

        assert!(keeps_xtor(&decl.xtors[0], &used, class_copy(&root)));
        assert!(!keeps_xtor(&decl.xtors[1], &used, class_copy(&root)));
    }

    /// A class that uses no xtor at all has no values, so its copy keeps nothing. The resulting
    /// empty declaration is only ever referenced from dead positions.
    #[test]
    fn keeps_xtor_drops_every_xtor_of_a_class_that_uses_none() {
        let decl = choice_decl();
        let root = box_label(1);
        let used = UsedXtors::default();

        assert!(
            decl.xtors
                .iter()
                .all(|xtor| !keeps_xtor(xtor, &used, class_copy(&root)))
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
                .all(|xtor| keeps_xtor(xtor, &used, CopyOf::Unreferenced))
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
        let class_fields = fields_for(&box_decl(), &[a.clone(), b.clone()]);
        let table = SplitTable::build(&mut uf, &label_origin, &[box_decl()], &[]);

        let mut max_id = 0;
        let copies = split_declaration(
            &box_decl(),
            &plan_of(table, class_fields, used),
            &mut max_id,
        );

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
        let copies = split_declaration(
            &box_decl(),
            &plan_of(table, class_fields, used),
            &mut max_id,
        );

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
        let class_fields = fields_for(&pack_decl(), &[a.clone(), b.clone()]);
        let used = used_for(id!("Wrap"), &[a, b]);
        let table = SplitTable::build(&mut uf, &label_origin, &[pack_decl()], &[]);

        let mut max_id = 0;
        let copies = split_declaration(
            &pack_decl(),
            &plan_of(table, class_fields, used),
            &mut max_id,
        );

        assert_eq!(copies.len(), 2);
        assert_eq!(copies[0].type_params.len(), 1);
        assert_eq!(copies[1].type_params.len(), 1);
        // both copies must get fresh, mutually distinct, nonzero ids, otherwise both copies'
        // type parameter would be the very same identifier
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
        let class_fields = fields_for(&pack_decl(), &[a.clone(), b.clone()]);
        let used = used_for(id!("Wrap"), &[a, b]);
        let table = SplitTable::build(&mut uf, &label_origin, &[pack_decl()], &[]);

        let mut max_id = 0;
        let copies = split_declaration(
            &pack_decl(),
            &plan_of(table, class_fields, used),
            &mut max_id,
        );

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
        let copies = split_declaration(&decl, &plan_of(table, class_fields, used), &mut max_id);

        assert_eq!(copies.len(), 2);
        let field_ty = |copy: &DataDeclaration| copy.xtors[0].args.bindings[0].ty.clone();
        assert_ne!(field_ty(&copies[0]), field_ty(&copies[1]));
    }

    #[test]
    fn split_declaration_falls_back_to_the_declared_field_shape_when_unreferenced() {
        // Nothing references `Bar`, so it has no equivalence class and no class copy of its
        // field: the declared shape is all `build_field_args` has to go on. `Foo` was never split
        // either, so it keeps its bare name.
        let mut uf = UnionFind::default();
        let decl = bar_decl(ty!(id!("Foo")));
        let table = SplitTable::build(&mut uf, &[], std::slice::from_ref(&decl), &[]);

        let mut max_id = 0;
        let plan = plan_of(table, ClassFields::default(), UsedXtors::default());
        let copies = split_declaration(&decl, &plan, &mut max_id);

        assert_eq!(copies.len(), 1);
        assert_eq!(copies[0].xtors[0].args.bindings[0].ty, ty!(id!("Foo")));
    }
}
