//! Applies a finished [`SplitTable`](crate::splitting::split_table::SplitTable) to a labeled
//! program, rewriting every label into its split-copy name and physically duplicating each
//! data/codata declaration once per equivalence class. Mirrors `mono::specialize`'s "table built
//! once, then `flat_map` every declaration through a trait that walks and renames" shape.

use std::rc::Rc;

use crate::splitting::labeling::{DeclSignature, DeclSignatures, FieldObservations, Label};
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
    sigs: &DeclSignatures,
    field_observations: &FieldObservations,
    max_id: &mut ID,
) -> Vec<TypeDeclaration<P>> {
    let roots = table.copies_for(&decl.name);
    if roots.is_empty() {
        return vec![build_declaration_copy(
            decl,
            table,
            sigs,
            field_observations,
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
                sigs,
                field_observations,
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
    sigs: &DeclSignatures,
    field_observations: &FieldObservations,
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
            .map(|old| (old.id.clone(), fresh_identifier(max_id, &old.id.name)))
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
            .map(|xtor| {
                split_xtor_sig(
                    xtor,
                    table,
                    sigs,
                    field_observations,
                    root,
                    &decl_subst,
                    max_id,
                )
            })
            .collect(),
        type_params: rename_params(&decl.type_params, &decl_subst),
    }
}

/// Rewrites one xtor's field types and, if `root` is given, renames it to its split copy's name
/// (paired with the same `root` as [`build_declaration_copy`], so e.g. `Cons__1` only ever ends up
/// inside `List__1`). `None` leaves the name unchanged, mirroring the unreferenced-declaration case
/// in [`build_declaration_copy`].
fn split_xtor_sig<P: Polarity + Clone>(
    xtor: &XtorSig<P>,
    table: &SplitTable,
    sigs: &DeclSignatures,
    field_observations: &FieldObservations,
    root: Option<(&Label, bool)>,
    decl_subst: &[(Identifier, Identifier)],
    max_id: &mut ID,
) -> XtorSig<P> {
    let xtor_subst: Vec<(Identifier, Identifier)> = match root {
        Some((_, true)) => xtor
            .type_params
            .iter()
            .map(|old| (old.id.clone(), fresh_identifier(max_id, &old.id.name)))
            .collect(),
        _ => vec![],
    };
    let subst: Vec<(Identifier, Identifier)> = decl_subst
        .iter()
        .chain(xtor_subst.iter())
        .cloned()
        .collect();

    let sig = sigs
        .get(&xtor.name)
        .unwrap_or_else(|| panic!("missing signature for xtor: {}", xtor.name.name));
    let args = build_field_args(xtor, sig, table, field_observations, root);

    XtorSig {
        xtor: xtor.xtor.clone(),
        name: match root {
            Some((root, _)) => table.resolve_xtor_name(&xtor.name, root).clone(),
            None => xtor.name.clone(),
        },
        type_params: rename_params(&xtor.type_params, &xtor_subst),
        args: substitute_args(&args, &subst),
    }
}

/// Builds one physical copy's field types. A self-referential field (`sig.self_referential[i]`,
/// see [`crate::splitting::reachability`]) keeps its declared shape as-is, rewritten through
/// `table`. A genuinely independent field instead resolves to the field observation
/// recorded for this specific copy's equivalence class (`root`), so e.g. two split copies of `Bar`
/// each end up with their own, independently split copy of a nested `Foo` field, falling back to
/// the declared shape if the xtor was never actually constructed/matched anywhere in the program.
fn build_field_args<P: Polarity + Clone>(
    xtor: &XtorSig<P>,
    sig: &DeclSignature,
    table: &SplitTable,
    field_observations: &FieldObservations,
    root: Option<(&Label, bool)>,
) -> TypingContext {
    TypingContext {
        bindings: xtor
            .args
            .bindings
            .iter()
            .enumerate()
            .map(|(i, binding)| {
                let ty = if sig.self_referential[i] {
                    binding.ty.rewrite(table)
                } else {
                    match root.and_then(|(r, _)| field_observations.get(r, &xtor.name, i)) {
                        // observed at some real occurrence: split independently from its owner
                        Some(representative) => representative.rewrite(table),
                        // never constructed/matched anywhere: `binding.ty` was left unlabeled by
                        // `label_xtor_signature`, so there is nothing to rewrite, use it as-is
                        None => binding.ty.clone(),
                    }
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
            let id = subst
                .iter()
                .find(|(old, _)| old == &param.id)
                .map(|(_, new)| new.clone())
                .unwrap_or_else(|| param.id.clone());
            TypeParam {
                id,
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
    use crate::splitting::labeling::{
        FieldObservation, SplitState, label_in, merge_field_observations,
    };
    use crate::splitting::union_find::UnionFind;
    use crate::syntax::{DataDeclaration, Ty};
    extern crate self as core_lang;
    use core_macros::{bind, ctor, ctor_sig, data, id, prd, tparam, tvar, ty};
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

    fn wrap_sigs() -> DeclSignatures {
        DeclSignatures::from([(
            id!("Wrap"),
            DeclSignature {
                decl_type_params: vec![],
                own_type_params: vec![],
                tys: vec![Ty::I64],
                self_referential: vec![true],
            },
        )])
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
            [tparam!(id!("C", 1), "+")]
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
        let sigs = wrap_sigs();
        let field_observations = FieldObservations::default();
        let copies =
            split_declaration(&box_decl(), &table, &sigs, &field_observations, &mut max_id);

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
        let sigs = wrap_sigs();
        let field_observations = FieldObservations::default();
        let copies =
            split_declaration(&box_decl(), &table, &sigs, &field_observations, &mut max_id);

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
        let sigs = wrap_sigs();
        let field_observations = FieldObservations::default();
        let copies = split_declaration(
            &pack_decl(),
            &table,
            &sigs,
            &field_observations,
            &mut max_id,
        );

        assert_eq!(copies.len(), 2);
        assert_eq!(copies[0].type_params.len(), 1);
        assert_eq!(copies[1].type_params.len(), 1);
        // both copies must get fresh, mutually distinct, nonzero ids, otherwise the constraint
        // graph would index both copies' type parameter under the same node
        assert_ne!(copies[0].type_params[0], copies[1].type_params[0]);
        assert_ne!(copies[0].type_params[0].id.id, 0);
        assert_ne!(copies[1].type_params[0].id.id, 0);
    }

    #[test]
    fn split_declaration_renaming_is_consistent_between_decl_head_and_fields() {
        let mut uf = UnionFind::default();
        let a = box_label(1);
        let b = box_label(2);
        let label_origin = HashMap::from([(a, id!("Pack")), (b, id!("Pack"))]);
        let table = SplitTable::build(&mut uf, &label_origin, &[pack_decl()], &[]);

        let mut max_id = 0;
        let sigs = wrap_sigs();
        let field_observations = FieldObservations::default();
        let copies = split_declaration(
            &pack_decl(),
            &table,
            &sigs,
            &field_observations,
            &mut max_id,
        );

        for copy in &copies {
            let field_ty = &copy.xtors[0].args.bindings[0].ty;
            assert_eq!(field_ty, &Ty::Var(copy.type_params[0].id.clone()));
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

    fn bar_sigs() -> DeclSignatures {
        DeclSignatures::from([(
            id!("MkBar"),
            DeclSignature {
                decl_type_params: vec![],
                own_type_params: vec![],
                tys: vec![ty!(id!("Foo"))],
                self_referential: vec![false],
            },
        )])
    }

    /// The central proof that independently constructed, non-self-referential fields no longer
    /// bleed into each other: two `Bar` copies, each fed its own `Foo` field observation, end up
    /// with two different field types instead of one shared one.
    #[test]
    fn split_declaration_gives_each_copy_its_own_field_observation() {
        let mut state = SplitState::default();
        let bar_a = state.label_ty(&ty!(id!("Bar")));
        let bar_b = state.label_ty(&ty!(id!("Bar")));
        let foo_a = state.label_ty(&ty!(id!("Foo")));
        let foo_b = state.label_ty(&ty!(id!("Foo")));
        state.field_observations.push(FieldObservation {
            owner: label_in(&bar_a).clone(),
            xtor: id!("MkBar"),
            field_index: 0,
            ty: foo_a.clone(),
        });
        state.field_observations.push(FieldObservation {
            owner: label_in(&bar_b).clone(),
            xtor: id!("MkBar"),
            field_index: 0,
            ty: foo_b.clone(),
        });
        let field_observations = merge_field_observations(&mut state);

        let label_origin = HashMap::from([
            (label_in(&bar_a).clone(), id!("Bar")),
            (label_in(&bar_b).clone(), id!("Bar")),
            (label_in(&foo_a).clone(), id!("Foo")),
            (label_in(&foo_b).clone(), id!("Foo")),
        ]);
        let decl = bar_decl(foo_a.clone());
        let table = SplitTable::build(&mut state.uf, &label_origin, &[decl.clone()], &[]);

        let sigs = bar_sigs();
        let mut max_id = 0;
        let copies = split_declaration(&decl, &table, &sigs, &field_observations, &mut max_id);

        assert_eq!(copies.len(), 2);
        let field_ty = |copy: &DataDeclaration| copy.xtors[0].args.bindings[0].ty.clone();
        assert_ne!(field_ty(&copies[0]), field_ty(&copies[1]));
    }

    /// An xtor whose declaration is never actually constructed anywhere falls back to its
    /// originally declared field shape, mirroring `SplitTable::copies_for`'s existing
    /// never-referenced fallback. `label_xtor_signature` never labels a non-self-referential
    /// field in the first place, so the fallback shape is the bare, unlabeled original type.
    #[test]
    fn split_declaration_falls_back_to_declared_shape_when_never_constructed() {
        let mut state = SplitState::default();
        let bar = state.label_ty(&ty!(id!("Bar")));

        let label_origin = HashMap::from([(label_in(&bar).clone(), id!("Bar"))]);
        let decl = bar_decl(ty!(id!("Foo")));
        let table = SplitTable::build(&mut state.uf, &label_origin, &[decl.clone()], &[]);

        let sigs = bar_sigs();
        let field_observations = FieldObservations::default();
        let mut max_id = 0;
        let copies = split_declaration(&decl, &table, &sigs, &field_observations, &mut max_id);

        assert_eq!(copies.len(), 1);
        assert_eq!(copies[0].xtors[0].args.bindings[0].ty, ty!(id!("Foo")));
    }
}
