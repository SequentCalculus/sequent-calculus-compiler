//! Phase 1 of type splitting: labels every declared-type occurrence and eagerly unifies label
//! pairs wherever the type system already requires equality.

use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use crate::{
    splitting::union_find::UnionFind,
    syntax::{
        Chi, Clause, CodataDeclaration, ContextBinding, DataDeclaration, Def, Identifier, Prog, Ty,
        TypeParam, TypingContext, types::TypeArgs,
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

/// Unifies the type an occurrence's argument or binder really has with the field type its xtor
/// declares, instantiated for this occurrence.
///
/// The declaration only exists as an unlabeled template, so it is instantiated here first: every
/// declaration head gets a fresh label, then every type parameter is replaced by the labeled type
/// argument this occurrence supplies for it. The result is an ordinary labeled type, and the
/// actual type is unified with it like anywhere else. This is the same substitution `Call`
/// performs against a `Def` signature, except that a `Def` signature was labeled once up front.
///
/// Take `data List[A] { Nil, Cons(head: A, tail: List[A]) }` and the occurrence
/// `Cons(Wrap(1), Nil) : List[Box]`, labeled as `List#1[Box#1]`, with `subst` mapping `A` to
/// `Box#1`:
/// - `head: A` instantiates to `Box#1` and is unified with the argument's `Box#2`. This ties the
///   annotation `List[Box]` to the value actually stored in it; without it the two could end up in
///   different split copies, and the list's type would no longer match the value it holds.
/// - `tail: List[A]` instantiates to `List#3[Box#1]`, where `List#3` is fresh and occurs nowhere
///   else. Unifying it with the argument's type merely makes it an alias of that head, so the head
///   stays exactly as free as before: which split copy it belongs to is still decided through the
///   owner class's observed field types (see [`SplitState::observe_field`]). Only `Box#1` is tied.
///
/// The template is labeled *before* substituting, since labeling afterwards would give the
/// substituted `Box#1` a fresh label as well and cut it loose from the annotation. A type
/// parameter `subst` has no instantiation for (an xtor's own existential in a clause, e.g. `B` in
/// `Pack[B](val: B)`, is abstract there) stays a type variable, which `unify_ty` skips.
fn unify_with_field_template(
    state: &mut SplitState,
    declared: &Ty,
    actual: &Ty,
    subst: &(Vec<Identifier>, Vec<Ty>),
) {
    let template = state
        .label_ty(declared)
        .substitute((subst.0.as_slice(), subst.1.as_slice()));
    // `actual` first, so that on a rank tie the root stays the occurrence's own label rather
    // than the freshly minted template head
    state.unify_ty(actual, &template);
}

/// Pairs a signature's type parameters with an occurrence's labeled type arguments, in the
/// `(params, args)` shape [`Ty::substitute`] expects, for [`unify_with_field_template`]. Mirrors the double substitution in `check_xcase_against_decl`:
/// `decl_type_args` instantiate the enclosing declaration's own parameters (e.g. `Fun`'s `A`, `B`),
/// `own_type_args` the xtor's own existential/universal ones (e.g. `Pack`'s own `B`). A clause
/// passes an empty `own_type_args`: it binds fresh, abstract names for those rather than knowing a
/// concrete instantiation.
fn type_param_subst(
    sig: &DeclSignature,
    decl_type_args: &[Ty],
    own_type_args: &[Ty],
) -> (Vec<Identifier>, Vec<Ty>) {
    sig.decl_type_params
        .iter()
        .cloned()
        .zip(decl_type_args.iter().cloned())
        .chain(
            sig.own_type_params
                .iter()
                .cloned()
                .zip(own_type_args.iter().cloned()),
        )
        .unzip()
}

/// The field type each equivalence class observed, keyed by (the class's final union-find root,
/// xtor, field index). A read-only view over the finished union-find's class data, built once by
/// [`finish_classes`] so that the rewrite phase never needs live access to the union-find itself
/// (mirroring how [`crate::splitting::split_table::SplitTable`] is resolved up front).
#[derive(Default)]
pub struct FieldObservations(HashMap<(Label, Identifier, usize), Ty>);

impl FieldObservations {
    /// Looks up the field type observed for `xtor`'s `field_index`-th field in the equivalence
    /// class rooted at `root`. `None` means the xtor was never actually constructed/matched
    /// anywhere in the program.
    pub fn get(&self, root: &Label, xtor: &Identifier, field_index: usize) -> Option<&Ty> {
        self.0.get(&(root.clone(), xtor.clone(), field_index))
    }
}

/// Every xtor that occurs for an equivalence class, keyed by (the class's final union-find root,
/// original xtor name). A read-only view over the finished union-find's class data, built once by
/// [`finish_classes`] alongside [`FieldObservations`].
#[derive(Default)]
pub struct UsedXtors(HashSet<(Label, Identifier)>);

impl UsedXtors {
    /// True iff `xtor` occurs anywhere in the program for the equivalence class rooted at `root`.
    /// Anything else is dead for that class and is dropped from its physical copy, see
    /// [`crate::splitting::rewrite::keeps_xtor`].
    pub fn contains(&self, root: &Label, xtor: &Identifier) -> bool {
        self.0.contains(&(root.clone(), xtor.clone()))
    }
}

impl FromIterator<(Label, Identifier)> for UsedXtors {
    /// Builds the view directly from `(root, xtor)` pairs, bypassing the union-find. Only for
    /// tests that exercise the rewrite phase against a hand-built class layout.
    fn from_iter<I: IntoIterator<Item = (Label, Identifier)>>(iter: I) -> Self {
        UsedXtors(iter.into_iter().collect())
    }
}

/// Turns the finished union-find's class data into the two read-only views the rewrite phase
/// consults. Purely a change of indexing, every union and every observation has already happened
/// during the walk; the class data is keyed by root throughout (see
/// [`crate::splitting::union_find::ClassData`]), so no reconciliation is left to do here.
pub fn finish_classes(state: &SplitState) -> (FieldObservations, UsedXtors) {
    let mut fields = HashMap::new();
    let mut used = HashSet::new();
    for (root, data) in state.uf.classes() {
        for ((xtor, index), ty) in &data.fields {
            fields.insert((root.clone(), xtor.clone(), *index), ty.clone());
        }
        for xtor in &data.used_xtors {
            used.insert((root.clone(), xtor.clone()));
        }
    }
    (FieldObservations(fields), UsedXtors(used))
}

/// Carries all mutable state through the single label+unify walk: the fresh-id counter, the
/// union-find (which also holds each class's observed field types and used xtors), and a record
/// of each label's origin.
#[derive(Default)]
pub struct SplitState {
    pub uf: UnionFind,
    per_name_counters: HashMap<String, usize>,
    /// Maps every minted label back to the original (unlabeled) declaration identifier it was
    /// derived from, e.g. `Box#3` -> `Box`.
    pub label_origin: Vec<(Label, Identifier)>,
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
            .push((label.clone(), Identifier::new(base_name.to_string())));
        label
    }

    /// Records that `xtor` occurs here, under an enclosing value whose own type carries `owner`.
    pub fn record_xtor_use(&mut self, owner: &Label, xtor: &Identifier) {
        self.uf.record_xtor_use(owner, xtor);
    }

    /// Records the type this occurrence really has at `xtor`'s `index`-th field, under an
    /// enclosing value whose own type carries `owner`.
    ///
    /// A field type cannot simply be unified against the declaration, because the declaration is
    /// never labeled: at a declaration head there is nothing on the declared side to unify with,
    /// and leaving that head free is exactly what lets two occurrences of the same declaration end
    /// up in different split copies (the positions the declaration left open as a type variable
    /// are handled by [`unify_with_field_template`] instead). So the type is stored on the owner's
    /// class, and only a *second* occurrence observing the same field forces the two to agree.
    pub fn observe_field(&mut self, owner: &Label, xtor: &Identifier, index: usize, ty: &Ty) {
        if let Some(already_observed) = self.uf.observe_field(owner, xtor, index, ty) {
            self.unify_ty(&already_observed, ty);
        }
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

    /// Unifies two already-labeled types that the type system requires to be equal at this
    /// position, and propagates the consequences to a fixpoint.
    ///
    /// Two rules feed the worklist. *Structurally*, two equal declared types have equal type
    /// arguments. *By congruence*, two classes that just merged have to agree on every field both
    /// of them observed, which [`UnionFind::union`] reports back as pending pairs. Only the second
    /// rule makes this more than a plain structural recursion: merging two classes' fields can
    /// merge further classes, whose fields then have to agree in turn.
    ///
    /// Terminates because pairs from `type_args` are structurally smaller than the pair they came
    /// from, and every pair from a union is paid for by a union that strictly reduced the number
    /// of equivalence classes.
    ///
    /// Anything that is not a pair of `Ty::Decl` carries no label and is simply dropped: `Ty::I64`
    /// has no identity to split, and a `Ty::Var` is abstract and deliberately left unlabeled.
    pub fn unify_ty(&mut self, a: &Ty, b: &Ty) {
        let mut work = vec![(a.clone(), b.clone())];
        while let Some((a, b)) = work.pop() {
            let (
                Ty::Decl {
                    name: n1,
                    type_args: t1,
                },
                Ty::Decl {
                    name: n2,
                    type_args: t2,
                },
            ) = (a, b)
            else {
                continue;
            };
            work.extend(self.uf.union(&n1, &n2));
            work.extend(t1.args.into_iter().zip(t2.args));
        }
    }
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

/// Labels every def's own signature (see [`label_def_signature`]) and collects a
/// [`DeclSignature`] for every def, constructor, and destructor in `prog`, for later use by
/// [`unify_with_field_template`] at every call/construction site. Returns the resulting
/// [`DeclSignatures`] table alongside `prog`'s own (still unlabeled) data and codata
/// declarations, since the caller needs both to drive the rest of the labeling walk.
pub fn build_decl_signatures(
    prog: &Prog,
    state: &mut SplitState,
) -> (DeclSignatures, Vec<DataDeclaration>, Vec<CodataDeclaration>) {
    let mut sigs = DeclSignatures::new();
    for def in &prog.defs {
        let tys = label_def_signature(def, state);
        sigs.insert(
            def.name.clone(),
            DeclSignature {
                decl_type_params: vec![],
                own_type_params: TypeParam::names(&def.type_params),
                tys,
            },
        );
    }

    let data_types: Vec<DataDeclaration> = prog.data_types.clone();
    for decl in &data_types {
        for xtor in &decl.xtors {
            let tys = xtor.args.bindings.iter().map(|b| b.ty.clone()).collect();
            sigs.insert(
                xtor.name.clone(),
                DeclSignature {
                    decl_type_params: TypeParam::names(&decl.type_params),
                    own_type_params: TypeParam::names(&xtor.type_params),
                    tys,
                },
            );
        }
    }

    let codata_types: Vec<CodataDeclaration> = prog.codata_types.clone();
    for decl in &codata_types {
        for xtor in &decl.xtors {
            let tys = xtor.args.bindings.iter().map(|b| b.ty.clone()).collect();
            sigs.insert(
                xtor.name.clone(),
                DeclSignature {
                    decl_type_params: TypeParam::names(&decl.type_params),
                    own_type_params: TypeParam::names(&xtor.type_params),
                    tys,
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
/// [`build_decl_signatures`]) so that multiple call/use sites unify against one shared, stable
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

/// Everything one occurrence of an xtor contributes to type splitting, shared by an `Xtor` term
/// and a `case`/`new` clause, which are the two ways an xtor can occur.
///
/// `owner_ty` is the labeled type of the value the xtor constructs, observes, matches or defines
/// (an `Xtor`'s own `.ty`, or the owning `XCase`'s `.ty` for a clause); its label owns everything
/// recorded here, and its type arguments instantiate the declaration's own type parameters.
/// `own_type_args` instantiate the xtor's own existential/universal ones: an `Xtor` term supplies
/// them explicitly, a clause passes none, since it binds them as abstract names. `field_tys` are
/// the labeled types the occurrence actually has at the xtor's fields, i.e. an `Xtor`'s argument
/// types or a clause's binder types.
///
/// Three things happen: the xtor is recorded as used for the owner's class (see
/// [`SplitState::record_xtor_use`]), and every field type is unified with the declared field
/// instantiated for this occurrence (see [`unify_with_field_template`]) as well as recorded on the
/// owner's class (see [`SplitState::observe_field`]). The first ties the type-parameter positions
/// to the owner's type arguments; the second is what decides the split copy of every declaration
/// head.
pub fn constrain_xtor_occurrence(
    state: &mut SplitState,
    sigs: &DeclSignatures,
    xtor: &Identifier,
    owner_ty: &Ty,
    own_type_args: &[Ty],
    field_tys: &[Ty],
) {
    let Some(sig) = sigs.get(xtor) else {
        panic!("missing signature for xtor: {}", xtor.name);
    };
    let Ty::Decl {
        name: owner,
        type_args: decl_type_args,
    } = owner_ty
    else {
        panic!(
            "expected a labeled Ty::Decl as the owner of xtor {}, got {owner_ty:?}",
            xtor.name
        );
    };
    let subst = type_param_subst(sig, &decl_type_args.args, own_type_args);

    state.record_xtor_use(owner, xtor);
    for (i, actual) in field_tys.iter().enumerate() {
        if let Some(declared) = sig.tys.get(i) {
            unify_with_field_template(state, declared, actual, &subst);
        }
        state.observe_field(owner, xtor, i, actual);
    }
}

/// Labels and unifies one clause of a match/comatch. Not a `LabelAndUnify` impl: unlike every
/// other node, a `Clause` carries no `.ty` of its own, the matched/constructed value's type is only
/// known from the owning `XCase`, so the caller must pass it in (mirrors
/// [`crate::splitting::rewrite::rewrite_clause`], which needs the analogous owner label for the
/// same structural reason). A clause is a real occurrence of its xtor, for `case` and `new` alike,
/// so once its binders are labeled it is constrained exactly like an `Xtor` term (see
/// [`constrain_xtor_occurrence`]).
pub fn label_and_unify_clause<C: Chi>(
    clause: &Clause<C>,
    state: &mut SplitState,
    sigs: &DeclSignatures,
    scope: &TypingContext,
    owner_ty: &Ty,
) -> Clause<C> {
    let labeled_bindings: Vec<ContextBinding> = clause
        .context
        .bindings
        .iter()
        .map(|binding| ContextBinding {
            var: binding.var.clone(),
            chi: binding.chi.clone(),
            ty: state.label_ty(&binding.ty),
        })
        .collect();

    // Recorded here rather than in `XCase::label_and_unify`, since this is the only place that sees
    // the owner type and the clause's xtor together, for both polarities.
    let binder_tys: Vec<Ty> = labeled_bindings.iter().map(|b| b.ty.clone()).collect();
    constrain_xtor_occurrence(state, sigs, &clause.xtor, owner_ty, &[], &binder_tys);

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
    fn observations_under_distinct_owner_roots_stay_separate() {
        let mut state = fresh_state();
        let owner_a = state.label_ty(&ty!(id!("Bar")));
        let owner_b = state.label_ty(&ty!(id!("Bar")));
        let field_a = state.label_ty(&ty!(id!("Foo")));
        let field_b = state.label_ty(&ty!(id!("Foo")));

        state.observe_field(label_in(&owner_a), &id!("MkBar"), 0, &field_a);
        state.observe_field(label_in(&owner_b), &id!("MkBar"), 0, &field_b);

        let (observations, _) = finish_classes(&state);
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
    fn observing_one_field_twice_under_the_same_owner_unifies_both_types() {
        let mut state = fresh_state();
        let owner_a = state.label_ty(&ty!(id!("Bar")));
        let owner_b = state.label_ty(&ty!(id!("Bar")));
        state.unify_ty(&owner_a, &owner_b);
        let field_a = state.label_ty(&ty!(id!("Foo")));
        let field_b = state.label_ty(&ty!(id!("Foo")));

        // the owners are already one class, so the second observation meets the first
        state.observe_field(label_in(&owner_a), &id!("MkBar"), 0, &field_a);
        state.observe_field(label_in(&owner_b), &id!("MkBar"), 0, &field_b);

        assert_eq!(
            state.uf.find(label_in(&field_a)),
            state.uf.find(label_in(&field_b))
        );
    }

    #[test]
    fn unifying_two_owners_after_the_fact_still_merges_their_field_observations() {
        // The other order: both fields are observed while the owners are still separate, and only
        // a later union brings them together. The congruence rule has to fire from inside `union`.
        let mut state = fresh_state();
        let owner_a = state.label_ty(&ty!(id!("Bar")));
        let owner_b = state.label_ty(&ty!(id!("Bar")));
        let field_a = state.label_ty(&ty!(id!("Foo")));
        let field_b = state.label_ty(&ty!(id!("Foo")));
        state.observe_field(label_in(&owner_a), &id!("MkBar"), 0, &field_a);
        state.observe_field(label_in(&owner_b), &id!("MkBar"), 0, &field_b);
        assert_ne!(
            state.uf.find(label_in(&field_a)),
            state.uf.find(label_in(&field_b))
        );

        state.unify_ty(&owner_a, &owner_b);

        assert_eq!(
            state.uf.find(label_in(&field_a)),
            state.uf.find(label_in(&field_b))
        );
    }

    #[test]
    fn merging_fields_cascades_into_the_fields_of_those_fields() {
        // Bar#1.MkBar.0 = Foo#1, Bar#2.MkBar.0 = Foo#2, and each Foo in turn holds its own Baz.
        // Unifying the two Bars must propagate two levels down, which is exactly what the
        // the worklist in `unify_ty` now handles.
        let mut state = fresh_state();
        let bar_a = state.label_ty(&ty!(id!("Bar")));
        let bar_b = state.label_ty(&ty!(id!("Bar")));
        let foo_a = state.label_ty(&ty!(id!("Foo")));
        let foo_b = state.label_ty(&ty!(id!("Foo")));
        let baz_a = state.label_ty(&ty!(id!("Baz")));
        let baz_b = state.label_ty(&ty!(id!("Baz")));
        state.observe_field(label_in(&bar_a), &id!("MkBar"), 0, &foo_a);
        state.observe_field(label_in(&bar_b), &id!("MkBar"), 0, &foo_b);
        state.observe_field(label_in(&foo_a), &id!("MkFoo"), 0, &baz_a);
        state.observe_field(label_in(&foo_b), &id!("MkFoo"), 0, &baz_b);

        state.unify_ty(&bar_a, &bar_b);

        assert_eq!(
            state.uf.find(label_in(&foo_a)),
            state.uf.find(label_in(&foo_b)),
            "the directly colliding field must merge"
        );
        assert_eq!(
            state.uf.find(label_in(&baz_a)),
            state.uf.find(label_in(&baz_b)),
            "and that merge must cascade into the next level down"
        );
    }

    #[test]
    fn finish_classes_returns_none_for_a_field_never_observed() {
        let state = fresh_state();
        let never_observed = Identifier {
            name: "Bar#1".to_string(),
            id: 0,
        };
        let (observations, used) = finish_classes(&state);
        assert_eq!(observations.get(&never_observed, &id!("MkBar"), 0), None);
        assert!(!used.contains(&never_observed, &id!("MkBar")));
    }

    #[test]
    fn recorded_xtor_uses_follow_their_owner_into_a_merged_class() {
        let mut state = fresh_state();
        let owner_a = state.label_ty(&ty!(id!("Bar")));
        let owner_b = state.label_ty(&ty!(id!("Bar")));
        state.record_xtor_use(label_in(&owner_a), &id!("MkBar"));
        state.record_xtor_use(label_in(&owner_b), &id!("MkBaz"));

        state.unify_ty(&owner_a, &owner_b);

        let (_, used) = finish_classes(&state);
        let root = state.uf.find(label_in(&owner_a));
        assert!(used.contains(&root, &id!("MkBar")));
        assert!(used.contains(&root, &id!("MkBaz")));
    }
}
