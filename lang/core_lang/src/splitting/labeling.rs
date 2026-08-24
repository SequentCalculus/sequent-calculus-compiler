use std::{collections::HashMap, mem::take, rc::Rc};

use crate::{
    splitting::{reachability::compute_field_reachability, union_find::UnionFind},
    syntax::{
        Chi, Clause, CodataDeclaration, ContextBinding, DataDeclaration, Def, Identifier, Prog, Ty,
        TypingContext,
        declaration::{Polarity, TypeDeclaration, XtorSig},
        types::TypeArgs,
    },
};

/// A label is a fresh `Identifier` sharing the declared type's name but carrying a unique `id` prefixed with `#`, e.g. `Box#1`
pub type Label = Identifier;

/// The canonical, labeled signature of one `Def` or `XtorSig`, alongside its labeled
/// parameter/field types. Both `type_params` fields are needed at every call/construction site to
/// substitute the site's own concrete instantiation into `tys` *before* unifying against the
/// actual argument, otherwise a field typed `Ty::Var(...)` would never unify with anything,
/// since `unify_ty` only ever matches `Ty::Decl` pairs. There are two independent levels of
/// generics an xtor's field type can reference (mirroring the double substitution in
/// `check_xcase_against_decl`):
pub struct DeclSignature {
    /// The *enclosing* data/codata declaration's own type parameters, e.g. `Fun`'s `A, B` for its
    /// `apply` dtor's `x: A`. Always empty for a `Def` entry, a `Def` has no enclosing
    /// declaration, its own `type_params` is the only level.
    pub decl_type_params: Vec<Identifier>,
    /// This entry's own type parameters: a `Def`'s own generics, or an xtor's own
    /// existential/universal parameters (distinct from the enclosing declaration's, e.g. `Pack`'s
    /// own `B` in `Pack[B](val: B)`).
    pub own_type_params: Vec<Identifier>,
    pub tys: Vec<Ty>,
    /// Parallel to `tys`: whether each field's raw declared type structurally loops back to this
    /// xtor's own enclosing declaration (see [`crate::splitting::reachability`]), e.g. `true` for
    /// `List.Cons.xs: List[A]`, `false` for `Bar.MkBar.f: Foo`. Always empty for a `Def` entry.
    pub self_referential: Vec<bool>,
}

pub type DeclSignatures = HashMap<Identifier, DeclSignature>;

/// Unwraps the label embedded in a `Ty::Decl`. Every type this is called on (an `Xtor`'s or
/// `XCase`'s own `.ty`) is guaranteed by the type system (see `Checked`) to be a declared type, so
/// anything else indicates a labeling bug.
pub fn label_in(ty: &Ty) -> &Label {
    match ty {
        Ty::Decl { name, .. } => name,
        _ => panic!("expected a labeled Ty::Decl, got {ty:?}"),
    }
}

/// One field position's actual type at one specific `Xtor`/`Clause` occurrence, recorded instead
/// of unified immediately: whether two occurrences' observations should later merge depends on
/// whether their *enclosing* declaration occurrences end up in the same equivalence class, which
/// is only known once the whole walk (and its union-find) is finished.
pub struct FieldObservation {
    /// The label embedded in the enclosing `Xtor`/`XCase` occurrence's own `.ty`.
    pub owner: Label,
    /// The xtor this field belongs to (original, pre-split name).
    pub xtor: Identifier,
    pub field_index: usize,
    pub ty: Ty,
}

/// The result of reconciling every [`FieldObservation`] recorded during the walk, keyed by (the
/// owner's union-find root, xtor, field index).
#[derive(Default)]
pub struct FieldObservations(HashMap<(Label, Identifier, usize), Ty>);

impl FieldObservations {
    /// Looks up the reconciled field type observed for `xtor`'s `field_index`-th field, among
    /// occurrences belonging to the equivalence class rooted at `root`. `None` means the xtor was
    /// never actually constructed/matched anywhere in the program.
    pub fn get(&self, root: &Label, xtor: &Identifier, field_index: usize) -> Option<&Ty> {
        self.0.get(&(root.clone(), xtor.clone(), field_index))
    }
}

/// Carries all mutable state through the single label+unify walk: the fresh-id counter, the
/// union-find, and a record of each label's origin.
#[derive(Default)]
pub struct SplitState {
    pub uf: UnionFind,
    per_name_counters: HashMap<String, usize>,
    /// Maps every minted label back to the original (unlabeled) declaration identifier it was
    /// derived from, e.g. `Box#3` -> `Box`.
    pub label_origin: HashMap<Label, Identifier>,
    /// Field observations recorded for non-self-referential fields, reconciled once the walk
    /// finishes. 
    pub field_observations: Vec<FieldObservation>,
}

impl SplitState {
    /// Generates a fresh, globally unique label for `base_name`. The uniqueness is baked
    /// directly into the label's `name` (e.g. `Box#1`, `Box#2`), rather than relying on the
    /// `id` field, so that printing a label via `Identifier`'s existing `Print` implementation
    /// (which only appends a suffix for `id != 0`) shows the label's identity plainly, without
    /// any additional formatting logic.
    fn fresh(&mut self, base_name: &str) -> Label {
        let counter = self
            .per_name_counters
            .entry(base_name.to_string())
            .or_insert(0);
        *counter += 1;
        let label = Identifier {
            name: format!("{base_name}#{counter}"),
            id: 0,
        };
        self.label_origin
            .insert(label.clone(), Identifier::new(base_name.to_string()));
        label
    }

    /// Labels every `Ty::Decl` occurrence with a fresh label, recursively into type arguments.
    pub fn label_ty(&mut self, ty: &Ty) -> Ty {
        match ty {
            Ty::I64 => Ty::I64,
            Ty::Var(v) => Ty::Var(v.clone()),
            Ty::Decl { name, type_args } => Ty::Decl {
                name: self.fresh(&name.name),
                type_args: TypeArgs {
                    args: type_args.args.iter().map(|a| self.label_ty(a)).collect(),
                },
            },
        }
    }

    /// Unifies two already-labeled types that the type system requires to be equal at this position.
    pub fn unify_ty(&mut self, a: &Ty, b: &Ty) {
        if let (
            Ty::Decl {
                name: n1,
                type_args: t1,
            },
            Ty::Decl {
                name: n2,
                type_args: t2,
            },
        ) = (a, b)
        {
            self.uf.union(n1, n2);
            for (x, y) in t1.args.iter().zip(t2.args.iter()) {
                self.unify_ty(x, y);
            }
        }
    }
}

/// Reconciles every recorded [`FieldObservation`]: groups them by their owner's *final*
/// union-find root, then unifies every observation within a group together, so occurrences that
/// end up sharing one physical copy of the enclosing declaration also end up with one consistent
/// field type. Must run after every other `unify_ty` call in the walk, since it relies on
/// `state.uf`'s roots being final.
pub fn merge_field_observations(state: &mut SplitState) -> FieldObservations {
    let observations = take(&mut state.field_observations);

    let mut groups: HashMap<(Label, Identifier, usize), Vec<Ty>> = HashMap::new();
    for obs in &observations {
        let root = state.uf.find(&obs.owner);
        groups
            .entry((root, obs.xtor.clone(), obs.field_index))
            .or_default()
            .push(obs.ty.clone());
    }

    let mut representatives = HashMap::new();
    for (key, tys) in groups {
        let mut tys = tys.into_iter();
        let first = tys.next().expect("group is never empty by construction");
        for ty in tys {
            state.unify_ty(&first, &ty);
        }
        representatives.insert(key, first);
    }

    FieldObservations(representatives)
}

/// Labels a `Def`'s own signature, so that every future call site can unify against these fixed
/// labels (looked up via the returned `DeclSignatures`) rather than creating a fresh, unrelated
/// one per call.
fn label_def_signature(def: &Def, state: &mut SplitState) -> Vec<Ty> {
    def.context
        .bindings
        .iter()
        .map(|b| state.label_ty(&b.ty))
        .collect()
}

/// Labels one `XtorSig`'s argument types and rebuilds the full labeled tree in the same pass.
fn label_xtor_signature<P: Polarity + Clone>(
    xtor: &XtorSig<P>,
    state: &mut SplitState,
) -> XtorSig<P> {
    let bindings = xtor
        .args
        .bindings
        .iter()
        .map(|binding| ContextBinding {
            var: binding.var.clone(),
            chi: binding.chi.clone(),
            ty: state.label_ty(&binding.ty),
        })
        .collect();
    XtorSig {
        xtor: xtor.xtor.clone(),
        name: xtor.name.clone(),
        type_params: xtor.type_params.clone(),
        args: TypingContext { bindings },
    }
}

/// Labels every xtor of one data/codata declaration (see [`label_xtor_sig`]).
fn label_typedeclaration_signature<P: Polarity + Clone>(
    decl: &TypeDeclaration<P>,
    state: &mut SplitState,
) -> TypeDeclaration<P> {
    TypeDeclaration {
        dat: decl.dat.clone(),
        name: decl.name.clone(),
        xtors: decl
            .xtors
            .iter()
            .map(|xtor| label_xtor_signature(xtor, state))
            .collect(),
        type_params: decl.type_params.clone(),
    }
}

/// Labels every `Def`'s parameter types and every data/codata declaration's xtor field types
/// exactly once, so that every future call/use site can unify against these fixed labels (looked
/// up via the returned `DeclSignatures`) rather than creating a fresh, unrelated one per call.
/// Also returns the fully labeled declaration trees (needed by the rewrite phase to walk and
/// split them).
pub fn build_decl_signatures(
    prog: &Prog,
    state: &mut SplitState,
) -> (DeclSignatures, Vec<DataDeclaration>, Vec<CodataDeclaration>) {
    let field_reachability = compute_field_reachability(prog);
    let mut sigs = DeclSignatures::new();
    for def in &prog.defs {
        let tys = label_def_signature(def, state);
        sigs.insert(
            def.name.clone(),
            DeclSignature {
                decl_type_params: vec![],
                own_type_params: def.type_params.clone(),
                tys,
                self_referential: vec![],
            },
        );
    }

    let data_types: Vec<DataDeclaration> = prog
        .data_types
        .iter()
        .map(|decl| label_typedeclaration_signature(decl, state))
        .collect();
    for decl in &data_types {
        for xtor in &decl.xtors {
            let tys = xtor.args.bindings.iter().map(|b| b.ty.clone()).collect();
            sigs.insert(
                xtor.name.clone(),
                DeclSignature {
                    decl_type_params: decl.type_params.clone(),
                    own_type_params: xtor.type_params.clone(),
                    tys,
                    self_referential: field_reachability[&xtor.name].clone(),
                },
            );
        }
    }

    let codata_types: Vec<CodataDeclaration> = prog
        .codata_types
        .iter()
        .map(|decl| label_typedeclaration_signature(decl, state))
        .collect();
    for decl in &codata_types {
        for xtor in &decl.xtors {
            let tys = xtor.args.bindings.iter().map(|b| b.ty.clone()).collect();
            sigs.insert(
                xtor.name.clone(),
                DeclSignature {
                    decl_type_params: decl.type_params.clone(),
                    own_type_params: xtor.type_params.clone(),
                    tys,
                    self_referential: field_reachability[&xtor.name].clone(),
                },
            );
        }
    }

    (sigs, data_types, codata_types)
}

/// This trait assigns fresh labels to every declared-type occurrence within a syntax element and
/// eagerly unifies label pairs wherever the existing type system already requires
/// two types to be equal at that position, e.g. a `Cut`'s producer/consumer type, a `Call`'s
/// argument against the callee's declared parameter type, an `Xtor`'s argument against its
/// declared field type, or an `XCase` clause's binder against the matched `Xtor`'s declared field.
///
/// This is the combined labeling-and-unification pass of type splitting;
/// running it eagerly during a single tree walk avoids a separate constraint-collection pass, at
/// the cost of requiring declaration signatures to be labeled once upfront (see
/// [`build_def_signatures`]) so that multiple call/use sites unify against one shared, stable
/// label rather than against each other pairwise.
///
/// `scope` maps every locally bound (co)variable (`Mu`'s variable, a `Clause`'s context bindings)
/// to its already-labeled type, mirroring [`crate::typing::check::Checked::check`]'s
/// `context: &TypingContext` parameter: it is extended at each binder and looked up at each
/// `XVar`, so that a variable's binding site and every one of its use sites end up carrying the
/// identical label.
pub trait LabelAndUnify {
    fn label_and_unify(
        &self,
        state: &mut SplitState,
        sigs: &DeclSignatures,
        scope: &TypingContext,
    ) -> Self;
}

impl<X: LabelAndUnify> LabelAndUnify for Vec<X> {
    fn label_and_unify(
        &self,
        state: &mut SplitState,
        sigs: &DeclSignatures,
        scope: &TypingContext,
    ) -> Self {
        self.iter()
            .map(|x| x.label_and_unify(state, sigs, scope))
            .collect()
    }
}

impl<X: LabelAndUnify> LabelAndUnify for Rc<X> {
    fn label_and_unify(
        &self,
        state: &mut SplitState,
        sigs: &DeclSignatures,
        scope: &TypingContext,
    ) -> Self {
        Rc::new(self.as_ref().label_and_unify(state, sigs, scope))
    }
}

impl<X: LabelAndUnify> LabelAndUnify for Option<X> {
    fn label_and_unify(
        &self,
        state: &mut SplitState,
        sigs: &DeclSignatures,
        scope: &TypingContext,
    ) -> Self {
        self.as_ref().map(|x| x.label_and_unify(state, sigs, scope))
    }
}

/// Labels and unifies one clause of a match/comatch. Not a `LabelAndUnify` impl: unlike every
/// other node, a `Clause` carries no `.ty` of its own, the enclosing declaration's own
/// concrete type arguments (e.g. `Fun`'s `A`, `B`) and the scrutinee's own label are only known
/// from the owning `XCase`, so the caller must pass them in (mirrors
/// [`crate::splitting::rewrite::rewrite_clause`], which needs the analogous `owner_label` for the
/// same structural reason).
pub fn label_and_unify_clause<C: Chi>(
    clause: &Clause<C>,
    state: &mut SplitState,
    sigs: &DeclSignatures,
    scope: &TypingContext,
    decl_type_args: &[Ty],
    owner: &Label,
) -> Clause<C> {
    let sig = sigs
        .get(&clause.xtor)
        .unwrap_or_else(|| panic!("missing signature for xtor: {}", clause.xtor.name));

    // Substitute the enclosing declaration's own type parameters (e.g. `Fun`'s `A`, `B`) using
    // the concrete arguments from the matched/constructed value's own type, mirrors the
    // identical substitution in `check_xcase_against_decl`. The xtor's own existential/universal
    // parameters are deliberately not substituted here (unlike Xtor/Call): a clause introduces
    // fresh, abstract names for those instead, it doesn't know a concrete instantiation for them.
    let labeled_bindings: Vec<ContextBinding> = clause
        .context
        .bindings
        .iter()
        .zip(&sig.tys)
        .enumerate()
        .map(|(i, (binding, field_ty))| {
            let ty = state.label_ty(&binding.ty);
            if sig.self_referential[i] {
                let expected = field_ty.substitute((&sig.decl_type_params, decl_type_args));
                state.unify_ty(&ty, &expected);
            } else {
                state.field_observations.push(FieldObservation {
                    owner: owner.clone(),
                    xtor: clause.xtor.clone(),
                    field_index: i,
                    ty: ty.clone(),
                });
            }
            ContextBinding {
                var: binding.var.clone(),
                chi: binding.chi.clone(),
                ty,
            }
        })
        .collect();

    let mut extended_scope = scope.clone();
    extended_scope.bindings.extend(labeled_bindings.clone());

    Clause {
        prdcns: clause.prdcns.clone(),
        xtor: clause.xtor.clone(),
        type_params: clause.type_params.clone(),
        context: TypingContext {
            bindings: labeled_bindings,
        },
        body: clause.body.label_and_unify(state, sigs, &extended_scope),
    }
}

#[cfg(test)]
mod split_state_tests {
    use crate::syntax::statements::Unreachable;

    use super::*;
    extern crate self as core_lang;
    use core_macros::{bind, codata, ctor_sig, data, def, dtor_sig, id, prd, tvar, ty};

    fn fresh_state() -> SplitState {
        SplitState::default()
    }

    #[test]
    fn fresh_encodes_uniqueness_in_the_name_not_the_id() {
        let mut state = fresh_state();
        let l1 = state.fresh("Box");
        let l2 = state.fresh("Box");

        assert_eq!(l1.id, 0);
        assert_eq!(l2.id, 0);
        assert_ne!(l1.name, l2.name);
        assert_eq!(l1.name, "Box#1");
        assert_eq!(l2.name, "Box#2");
    }

    #[test]
    fn fresh_labels_for_different_base_names_are_still_distinct() {
        let mut state = fresh_state();
        let a = state.fresh("Box");
        let b = state.fresh("List");
        assert_ne!(a, b);
    }

    #[test]
    fn label_ty_leaves_i64_and_var_unchanged() {
        let mut state = fresh_state();
        assert_eq!(state.label_ty(&Ty::I64), Ty::I64);

        let var = tvar!(id!("A", 1));
        assert_eq!(state.label_ty(&var), var);
    }

    #[test]
    fn label_ty_assigns_a_fresh_label_to_a_bare_decl() {
        let mut state = fresh_state();
        let result = state.label_ty(&ty!(id!("Box")));

        let Ty::Decl { name, type_args } = &result else {
            panic!("expected a Ty::Decl");
        };
        assert!(type_args.args.is_empty());
        assert_eq!(name.name, "Box#1");
    }

    #[test]
    fn label_ty_recurses_into_nested_type_arguments() {
        let mut state = fresh_state();
        // Box[List[i64]]
        let input = ty!(id!("Box"), [ty!(id!("List"), [ty!("int")])]);
        let result = state.label_ty(&input);

        let Ty::Decl {
            name: outer_name,
            type_args,
        } = &result
        else {
            panic!("expected a Ty::Decl");
        };
        assert_eq!(outer_name.name, "Box#1");

        let Ty::Decl {
            name: inner_name,
            type_args: inner_args,
        } = &type_args.args[0]
        else {
            panic!("expected a nested Ty::Decl");
        };
        assert_eq!(inner_name.name, "List#1");
        assert_eq!(inner_args.args[0], Ty::I64);
    }

    #[test]
    fn label_ty_gives_two_separate_occurrences_two_distinct_labels() {
        // Two independent occurrences of the same declared type must never accidentally share a
        // label -- that is the entire point of type splitting.
        let mut state = fresh_state();
        let a = state.label_ty(&ty!(id!("Box")));
        let b = state.label_ty(&ty!(id!("Box")));
        assert_ne!(a, b);
    }

    #[test]
    fn unify_ty_merges_the_two_labels_of_decl_types() {
        let mut state = fresh_state();
        let a = state.label_ty(&ty!(id!("Box")));
        let b = state.label_ty(&ty!(id!("Box")));

        let (Ty::Decl { name: n1, .. }, Ty::Decl { name: n2, .. }) = (&a, &b) else {
            panic!("expected Ty::Decl on both sides");
        };
        assert_ne!(state.uf.find(n1), state.uf.find(n2));

        state.unify_ty(&a, &b);

        assert_eq!(state.uf.find(n1), state.uf.find(n2));
    }

    #[test]
    fn unify_ty_recursively_unifies_nested_type_arguments() {
        let mut state = fresh_state();
        // Box[List[i64]] unified with a second, independently labeled Box[List[i64]] must merge
        // both the outer Box labels *and* the inner List labels.
        let a = state.label_ty(&ty!(id!("Box"), [ty!(id!("List"), [ty!("int")])]));
        let b = state.label_ty(&ty!(id!("Box"), [ty!(id!("List"), [ty!("int")])]));

        state.unify_ty(&a, &b);

        let (
            Ty::Decl {
                name: outer_a,
                type_args: args_a,
            },
            Ty::Decl {
                name: outer_b,
                type_args: args_b,
            },
        ) = (&a, &b)
        else {
            panic!("expected Ty::Decl on both sides");
        };
        assert_eq!(state.uf.find(outer_a), state.uf.find(outer_b));

        let (Ty::Decl { name: inner_a, .. }, Ty::Decl { name: inner_b, .. }) =
            (&args_a.args[0], &args_b.args[0])
        else {
            panic!("expected nested Ty::Decl on both sides");
        };
        assert_eq!(state.uf.find(inner_a), state.uf.find(inner_b));
    }

    #[test]
    fn unify_ty_is_transitive_via_repeated_calls() {
        // unify(a,b) then unify(b,c) must put a and c in the same class too, exercising the
        // union-find underneath through the higher-level unify_ty entry point.
        let mut state = fresh_state();
        let a = state.label_ty(&ty!(id!("Box")));
        let b = state.label_ty(&ty!(id!("Box")));
        let c = state.label_ty(&ty!(id!("Box")));

        state.unify_ty(&a, &b);
        state.unify_ty(&b, &c);

        let (Ty::Decl { name: na, .. }, Ty::Decl { name: nc, .. }) = (&a, &c) else {
            panic!("expected Ty::Decl");
        };
        assert_eq!(state.uf.find(na), state.uf.find(nc));
    }

    #[test]
    fn label_def_signature_labels_each_parameter() {
        let f = def!(
            id!("f"),
            [],
            [
                bind!(id!("x"), prd!(), ty!(id!("Box"))),
                bind!(id!("y"), prd!(), ty!("int"))
            ],
            // body is irrelevant here; using a placeholder is fine as long as `def!` accepts it
            Unreachable { ty: ty!("int") }
        );

        let mut state = fresh_state();
        let params = label_def_signature(&f, &mut state);

        assert_eq!(params.len(), 2);
        assert!(matches!(params[0], Ty::Decl { .. }));
        assert_eq!(params[1], Ty::I64);
    }

    #[test]
    fn label_xtor_sig_labels_each_field_and_rebuilds_the_tree() {
        let ctor = ctor_sig!(
            id!("Cons"),
            [],
            [
                bind!(id!("x"), prd!(), ty!("int")),
                bind!(id!("xs"), prd!(), ty!(id!("List")))
            ]
        );

        let mut state = fresh_state();
        let labeled = label_xtor_signature(&ctor, &mut state);

        assert_eq!(labeled.name, id!("Cons"));
        assert_eq!(labeled.args.bindings.len(), 2);
        assert_eq!(labeled.args.bindings[0].ty, Ty::I64);
        assert!(matches!(labeled.args.bindings[1].ty, Ty::Decl { .. }));
    }

    #[test]
    fn build_def_signatures_covers_defs_ctors_and_dtors() {
        let f = def!(
            id!("f"),
            [],
            [bind!(id!("x"), prd!(), ty!("int"))],
            Unreachable { ty: ty!("int") }
        );

        let list = data!(
            id!("List"),
            [ctor_sig!(
                id!("Cons"),
                [],
                [bind!(id!("x"), prd!(), ty!("int"))]
            )],
            []
        );

        let stream = codata!(
            id!("Stream"),
            [dtor_sig!(
                id!("head"),
                [],
                [bind!(id!("h"), prd!(), ty!("int"))]
            )],
            []
        );

        let prog = Prog {
            defs: vec![f],
            data_types: vec![list],
            codata_types: vec![stream],
            max_id: 0,
        };

        let mut state = fresh_state();
        let (sigs, data_types, codata_types) = build_decl_signatures(&prog, &mut state);

        assert!(sigs.contains_key(&id!("f")));
        assert!(sigs.contains_key(&id!("Cons")));
        assert!(sigs.contains_key(&id!("head")));
        assert_eq!(sigs[&id!("f")].tys.len(), 1);
        assert_eq!(sigs[&id!("Cons")].tys.len(), 1);
        assert_eq!(sigs[&id!("head")].tys.len(), 1);

        // the labeled declaration trees carry the same field types as `sigs`, not a separate copy
        assert_eq!(data_types.len(), 1);
        assert_eq!(
            data_types[0].xtors[0].args.bindings[0].ty,
            sigs[&id!("Cons")].tys[0]
        );
        assert_eq!(codata_types.len(), 1);
        assert_eq!(
            codata_types[0].xtors[0].args.bindings[0].ty,
            sigs[&id!("head")].tys[0]
        );
    }

    #[test]
    fn merge_field_observations_keeps_distinct_owner_roots_separate() {
        let mut state = fresh_state();
        let owner_a = state.label_ty(&ty!(id!("Bar")));
        let owner_b = state.label_ty(&ty!(id!("Bar")));
        let field_a = state.label_ty(&ty!(id!("Foo")));
        let field_b = state.label_ty(&ty!(id!("Foo")));
        state.field_observations.push(FieldObservation {
            owner: label_in(&owner_a).clone(),
            xtor: id!("MkBar"),
            field_index: 0,
            ty: field_a.clone(),
        });
        state.field_observations.push(FieldObservation {
            owner: label_in(&owner_b).clone(),
            xtor: id!("MkBar"),
            field_index: 0,
            ty: field_b.clone(),
        });

        let observations = merge_field_observations(&mut state);

        let root_a = state.uf.find(label_in(&owner_a));
        let root_b = state.uf.find(label_in(&owner_b));
        assert_eq!(observations.get(&root_a, &id!("MkBar"), 0), Some(&field_a));
        assert_eq!(observations.get(&root_b, &id!("MkBar"), 0), Some(&field_b));
        assert_ne!(
            state.uf.find(label_in(&field_a)),
            state.uf.find(label_in(&field_b))
        );
    }

    #[test]
    fn merge_field_observations_unifies_observations_sharing_an_owner_root() {
        let mut state = fresh_state();
        let owner_a = state.label_ty(&ty!(id!("Bar")));
        let owner_b = state.label_ty(&ty!(id!("Bar")));
        state.unify_ty(&owner_a, &owner_b);
        let field_a = state.label_ty(&ty!(id!("Foo")));
        let field_b = state.label_ty(&ty!(id!("Foo")));
        state.field_observations.push(FieldObservation {
            owner: label_in(&owner_a).clone(),
            xtor: id!("MkBar"),
            field_index: 0,
            ty: field_a.clone(),
        });
        state.field_observations.push(FieldObservation {
            owner: label_in(&owner_b).clone(),
            xtor: id!("MkBar"),
            field_index: 0,
            ty: field_b.clone(),
        });

        merge_field_observations(&mut state);

        assert_eq!(
            state.uf.find(label_in(&field_a)),
            state.uf.find(label_in(&field_b))
        );
    }

    #[test]
    fn merge_field_observations_returns_none_for_a_field_never_observed() {
        let mut state = fresh_state();
        let never_observed = Identifier {
            name: "Bar#1".to_string(),
            id: 0,
        };
        let observations = merge_field_observations(&mut state);
        assert_eq!(observations.get(&never_observed, &id!("MkBar"), 0), None);
    }
}
