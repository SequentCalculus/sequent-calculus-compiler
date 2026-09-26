//! Phase 1 of type splitting: labels every declared-type occurrence and eagerly unifies label
//! pairs wherever the type system already requires equality.

use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use crate::{
    splitting::union_find::UnionFind,
    syntax::{
        Chi, Clause, ContextBinding, Def, Identifier, Prog, Ty, TypeParam, TypingContext,
        types::TypeArgs,
    },
};

/// A label is a fresh `Identifier` whose name is the declared type's name with a unique `#`-suffix,
/// e.g. `Box#1`; its `id` is always 0 (see `SplitState::fresh`).
pub type Label = Identifier;

/// The signature of one `Def` or `XtorSig`: its parameter or field types plus the type parameters
/// they may mention. For a `Def`, `tys` are labeled once up front, the one canonical signature
/// every `Call` unifies against. For an xtor, `tys` are the declaration's unlabeled field types,
/// only a template from which every equivalence class instantiates its own copy (see
/// [`SplitState::class_field`]). Both `type_params` fields are needed at every call site and xtor
/// occurrence to substitute the site's own instantiation *before* unifying against the actual
/// argument, otherwise a field typed `Ty::Var(...)` would never unify with anything, since
/// `unify_ty` only ever matches `Ty::Decl` pairs. There are two independent levels of
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

/// Pairs a signature's type parameters with an occurrence's labeled type arguments, in the
/// `(params, args)` shape [`Ty::substitute`] expects, for [`constrain_xtor_occurrence`]. Mirrors
/// the double substitution in `check_xcase_against_decl`:
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

/// Each equivalence class's own copy of every field it touched, keyed by (the class's final
/// union-find root, xtor, field index): the declared field type with its heads labeled for that
/// class and its type parameters left as variables (see
/// [`crate::splitting::union_find::ClassData`]). A read-only view over the finished union-find's
/// class data, built once by [`finish_classes`] so that the rewrite phase never needs live access
/// to the union-find itself (mirroring how [`crate::splitting::split_table::SplitTable`] is
/// resolved up front).
#[derive(Default)]
pub struct ClassFields(HashMap<(Label, Identifier, usize), Ty>);

impl ClassFields {
    /// Looks up the class rooted at `root`'s copy of `xtor`'s `field_index`-th field. `None` means
    /// no occurrence of that class ever touched the field, i.e. the xtor never occurs for it.
    pub fn get(&self, root: &Label, xtor: &Identifier, field_index: usize) -> Option<&Ty> {
        self.0.get(&(root.clone(), xtor.clone(), field_index))
    }
}

/// Every xtor that occurs for an equivalence class, keyed by (the class's final union-find root,
/// original xtor name). A read-only view over the finished union-find's class data, built once by
/// [`finish_classes`] alongside [`ClassFields`].
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

/// Builds the view directly from `((root, xtor, field index), ty)` pairs, bypassing the
/// union-find. Only for tests that exercise the rewrite phase against a hand-built class layout.
#[cfg(test)]
impl FromIterator<((Label, Identifier, usize), Ty)> for ClassFields {
    fn from_iter<I: IntoIterator<Item = ((Label, Identifier, usize), Ty)>>(iter: I) -> Self {
        ClassFields(iter.into_iter().collect())
    }
}

/// Builds the view directly from `(root, xtor)` pairs, bypassing the union-find. Only for tests
/// that exercise the rewrite phase against a hand-built class layout.
#[cfg(test)]
impl FromIterator<(Label, Identifier)> for UsedXtors {
    fn from_iter<I: IntoIterator<Item = (Label, Identifier)>>(iter: I) -> Self {
        UsedXtors(iter.into_iter().collect())
    }
}

/// Turns the finished union-find's class data into the two read-only views the rewrite phase
/// consults. Purely a change of indexing: every union has happened and every class field has been
/// created during the walk, and the class data is keyed by root throughout (see
/// [`crate::splitting::union_find::ClassData`]), so no reconciliation is left to do here.
pub fn finish_classes(state: &SplitState) -> (ClassFields, UsedXtors) {
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
    (ClassFields(fields), UsedXtors(used))
}

/// Carries all mutable state through the single label+unify walk: the fresh-id counter, the
/// union-find (which also holds each class's own copy of its fields and its used xtors), and a
/// record of each label's origin.
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

    /// Returns `owner`'s class's own copy of `xtor`'s `index`-th field, whose declared type is
    /// `declared`, materializing it the first time the class touches that field.
    ///
    /// The declaration itself only exists as an unlabeled template. A class of labels is what
    /// becomes one physical copy of it, so the class gets its own instance of the field: every
    /// declaration head labeled freshly, every type parameter left as a variable. The heads are
    /// this copy's decisions (which copy of `List` does `tail: List[A]` point at?), shared by
    /// every occurrence of the class; the type parameters are not, since one generic copy can be
    /// instantiated differently at every occurrence (see [`constrain_xtor_occurrence`]).
    pub fn class_field(
        &mut self,
        owner: &Label,
        xtor: &Identifier,
        index: usize,
        declared: &Ty,
    ) -> Ty {
        if let Some(field) = self.uf.field(owner, xtor, index) {
            return field;
        }
        let field = self.label_ty(declared);
        self.uf.insert_field(owner, xtor, index, field.clone());
        field
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
    /// of them hold, which [`UnionFind::union`] reports back as pending pairs. Only the second
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
/// [`DeclSignature`] for every def, constructor, and destructor in `prog`, for later use at every
/// call site (`Call::label_and_unify`) and every xtor occurrence ([`constrain_xtor_occurrence`]).
///
/// Only the defs' signatures are labeled. An xtor's entry carries the declaration's field types
/// unchanged, as the unlabeled template every equivalence class instantiates its own copy from
/// (see [`SplitState::class_field`]).
pub fn build_decl_signatures(prog: &Prog, state: &mut SplitState) -> DeclSignatures {
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

    for decl in &prog.data_types {
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

    for decl in &prog.codata_types {
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

    sigs
}

/// This trait assigns fresh labels to every declared-type occurrence within a syntax element and
/// eagerly unifies label pairs wherever the existing type system already requires
/// two types to be equal at that position, e.g. a `Cut`'s producer/consumer type, a `Call`'s
/// argument against the callee's declared parameter type, or an `Xtor`'s argument and an `XCase`
/// clause's binder against the owner class's copy of that field (see
/// [`constrain_xtor_occurrence`]).
///
/// This is the combined labeling-and-unification pass of type splitting; running it eagerly
/// during a single tree walk avoids collecting the required equalities in a separate pass first,
/// which is safe because the resulting partition does not depend on the order of the unions. `Def` signatures are
/// labeled once up front (see [`build_decl_signatures`]) so that every call site unifies against
/// one shared, stable label rather than against each other pairwise.
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
/// The xtor is recorded as used for the owner's class (see [`SplitState::record_xtor_use`]), and
/// every field's actual type is unified with the owner class's copy of that field (see
/// [`SplitState::class_field`]), instantiated for this occurrence by substituting its type
/// arguments. That one unification does both jobs a field has:
/// - the *heads* are the class's own labels, so every occurrence of the class ends up agreeing on
///   which split copy a field such as `tail: List[A]` points at;
/// - the *type parameters* are replaced per occurrence, so a field such as `head: A` is tied to
///   this occurrence's own type argument, and nothing else. Two occurrences sharing one generic
///   copy (e.g. through a polymorphic `Def` called at two types), or two values of one type hiding
///   different existentials, therefore keep their instantiations apart.
///
/// Take `data List[A] { Nil, Cons(head: A, tail: List[A]) }` and the occurrence
/// `Cons(Wrap(1), Nil) : List[Box]`, labeled as `List#1[Box#1]`, with the arguments
/// `Wrap(1) : Box#2` and `Nil : List#2[Box#3]`. Its class holds the fields `head: A` and
/// `tail: List#3[A]`, which instantiate to `Box#1` and `List#3[Box#1]`. Unifying the first with
/// `Box#2` ties the annotation `List[Box]` to the value stored in it; unifying the second with
/// `Nil`'s type ties its head `List#2` to the class's `List#3`, and its `Box#3` to `Box#1`.
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
    for (i, (actual, declared)) in field_tys.iter().zip(&sig.tys).enumerate() {
        let field = state.class_field(owner, xtor, i, declared);
        let expected = field.substitute((subst.0.as_slice(), subst.1.as_slice()));
        state.unify_ty(actual, &expected);
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

    // Constrained here rather than in `XCase::label_and_unify`, since this is the only place that
    // sees the owner type and the clause's xtor together, for both polarities.
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

    /// Stands in for one occurrence holding `actual` at a field declared as `declared`, without
    /// a full xtor signature: unifies `actual` with the owner class's copy of that field.
    fn touch_field(
        state: &mut SplitState,
        owner: &Ty,
        xtor: &Identifier,
        index: usize,
        declared: &Ty,
        actual: &Ty,
    ) {
        let field = state.class_field(label_in(owner), xtor, index, declared);
        state.unify_ty(actual, &field);
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
        let sigs = build_decl_signatures(&prog, &mut state);

        assert!(sigs.contains_key(&id!("f")));
        assert!(sigs.contains_key(&id!("Cons")));
        assert!(sigs.contains_key(&id!("head")));
        assert_eq!(sigs[&id!("f")].tys.len(), 1);
        assert_eq!(sigs[&id!("Cons")].tys.len(), 1);
        assert_eq!(sigs[&id!("head")].tys.len(), 1);

        // an xtor's entry is the declaration's field type unchanged, not a labeled copy of it
        assert_eq!(
            prog.data_types[0].xtors[0].args.bindings[0].ty,
            sigs[&id!("Cons")].tys[0]
        );
        assert_eq!(
            prog.codata_types[0].xtors[0].args.bindings[0].ty,
            sigs[&id!("head")].tys[0]
        );
    }

    #[test]
    fn fields_under_distinct_owner_roots_stay_separate() {
        let mut state = fresh_state();
        let owner_a = state.label_ty(&ty!(id!("Bar")));
        let owner_b = state.label_ty(&ty!(id!("Bar")));
        let field_a = state.label_ty(&ty!(id!("Foo")));
        let field_b = state.label_ty(&ty!(id!("Foo")));

        touch_field(
            &mut state,
            &owner_a,
            &id!("MkBar"),
            0,
            &ty!(id!("Foo")),
            &field_a,
        );
        touch_field(
            &mut state,
            &owner_b,
            &id!("MkBar"),
            0,
            &ty!(id!("Foo")),
            &field_b,
        );

        let (class_fields, _) = finish_classes(&state);
        let root_a = state.uf.find(label_in(&owner_a));
        let root_b = state.uf.find(label_in(&owner_b));
        let copy_a = class_fields
            .get(&root_a, &id!("MkBar"), 0)
            .expect("class a holds MkBar.0")
            .clone();
        let copy_b = class_fields
            .get(&root_b, &id!("MkBar"), 0)
            .expect("class b holds MkBar.0")
            .clone();
        // each class's own copy of the field points at the value stored there
        assert_eq!(
            state.uf.find(label_in(&copy_a)),
            state.uf.find(label_in(&field_a))
        );
        assert_eq!(
            state.uf.find(label_in(&copy_b)),
            state.uf.find(label_in(&field_b))
        );
        assert_ne!(
            state.uf.find(label_in(&field_a)),
            state.uf.find(label_in(&field_b))
        );
    }

    #[test]
    fn touching_one_field_twice_under_the_same_owner_unifies_both_types() {
        let mut state = fresh_state();
        let owner_a = state.label_ty(&ty!(id!("Bar")));
        let owner_b = state.label_ty(&ty!(id!("Bar")));
        state.unify_ty(&owner_a, &owner_b);
        let field_a = state.label_ty(&ty!(id!("Foo")));
        let field_b = state.label_ty(&ty!(id!("Foo")));

        // the owners are already one class, so both values meet in its one copy of the field
        touch_field(
            &mut state,
            &owner_a,
            &id!("MkBar"),
            0,
            &ty!(id!("Foo")),
            &field_a,
        );
        touch_field(
            &mut state,
            &owner_b,
            &id!("MkBar"),
            0,
            &ty!(id!("Foo")),
            &field_b,
        );

        assert_eq!(
            state.uf.find(label_in(&field_a)),
            state.uf.find(label_in(&field_b))
        );
    }

    #[test]
    fn unifying_two_owners_after_the_fact_still_merges_their_fields() {
        // The other order: both fields are touched while the owners are still separate, and only
        // a later union brings them together. The congruence rule has to fire from inside `union`.
        let mut state = fresh_state();
        let owner_a = state.label_ty(&ty!(id!("Bar")));
        let owner_b = state.label_ty(&ty!(id!("Bar")));
        let field_a = state.label_ty(&ty!(id!("Foo")));
        let field_b = state.label_ty(&ty!(id!("Foo")));
        touch_field(
            &mut state,
            &owner_a,
            &id!("MkBar"),
            0,
            &ty!(id!("Foo")),
            &field_a,
        );
        touch_field(
            &mut state,
            &owner_b,
            &id!("MkBar"),
            0,
            &ty!(id!("Foo")),
            &field_b,
        );
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
        // Unifying the two Bars must propagate two levels down, which is what the worklist in
        // `unify_ty` handles.
        let mut state = fresh_state();
        let bar_a = state.label_ty(&ty!(id!("Bar")));
        let bar_b = state.label_ty(&ty!(id!("Bar")));
        let foo_a = state.label_ty(&ty!(id!("Foo")));
        let foo_b = state.label_ty(&ty!(id!("Foo")));
        let baz_a = state.label_ty(&ty!(id!("Baz")));
        let baz_b = state.label_ty(&ty!(id!("Baz")));
        touch_field(
            &mut state,
            &bar_a,
            &id!("MkBar"),
            0,
            &ty!(id!("Foo")),
            &foo_a,
        );
        touch_field(
            &mut state,
            &bar_b,
            &id!("MkBar"),
            0,
            &ty!(id!("Foo")),
            &foo_b,
        );
        touch_field(
            &mut state,
            &foo_a,
            &id!("MkFoo"),
            0,
            &ty!(id!("Baz")),
            &baz_a,
        );
        touch_field(
            &mut state,
            &foo_b,
            &id!("MkFoo"),
            0,
            &ty!(id!("Baz")),
            &baz_b,
        );

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
    fn an_existential_instantiation_stays_with_its_occurrence_not_the_owners_class() {
        // `data Packed { Pack[B](val: B) }`: `B` is the xtor's own existential, so each value picks
        // its own instantiation. Two values can share one class of `Packed` (e.g. both flow into
        // the same variable) and still hide completely unrelated types, here a `List` and a `Box`.
        // Nothing requires those to be equal, so they must not end up in one class.
        let mut state = fresh_state();
        let mut sigs = DeclSignatures::new();
        sigs.insert(
            id!("Pack"),
            DeclSignature {
                decl_type_params: vec![],
                own_type_params: vec![id!("B", 1)],
                tys: vec![tvar!(id!("B", 1))],
            },
        );

        let owner_a = state.label_ty(&ty!(id!("Packed")));
        let owner_b = state.label_ty(&ty!(id!("Packed")));
        state.unify_ty(&owner_a, &owner_b);
        let hidden_a = state.label_ty(&ty!(id!("List")));
        let hidden_b = state.label_ty(&ty!(id!("Box")));

        constrain_xtor_occurrence(
            &mut state,
            &sigs,
            &id!("Pack"),
            &owner_a,
            std::slice::from_ref(&hidden_a),
            std::slice::from_ref(&hidden_a),
        );
        constrain_xtor_occurrence(
            &mut state,
            &sigs,
            &id!("Pack"),
            &owner_b,
            std::slice::from_ref(&hidden_b),
            std::slice::from_ref(&hidden_b),
        );

        assert_ne!(
            state.uf.find(label_in(&hidden_a)),
            state.uf.find(label_in(&hidden_b)),
            "the two hidden types were merged just because their owners share a class"
        );
    }

    #[test]
    fn an_existential_instantiation_stays_with_its_occurrence_when_owners_merge_later() {
        // Same as above, but the two owners only become one class after both occurrences were
        // constrained, so any merging would have to come from the collisions `union` reports.
        let mut state = fresh_state();
        let mut sigs = DeclSignatures::new();
        sigs.insert(
            id!("Pack"),
            DeclSignature {
                decl_type_params: vec![],
                own_type_params: vec![id!("B", 1)],
                tys: vec![tvar!(id!("B", 1))],
            },
        );

        let owner_a = state.label_ty(&ty!(id!("Packed")));
        let owner_b = state.label_ty(&ty!(id!("Packed")));
        let hidden_a = state.label_ty(&ty!(id!("List")));
        let hidden_b = state.label_ty(&ty!(id!("Box")));
        constrain_xtor_occurrence(
            &mut state,
            &sigs,
            &id!("Pack"),
            &owner_a,
            std::slice::from_ref(&hidden_a),
            std::slice::from_ref(&hidden_a),
        );
        constrain_xtor_occurrence(
            &mut state,
            &sigs,
            &id!("Pack"),
            &owner_b,
            std::slice::from_ref(&hidden_b),
            std::slice::from_ref(&hidden_b),
        );

        state.unify_ty(&owner_a, &owner_b);

        assert_ne!(
            state.uf.find(label_in(&hidden_a)),
            state.uf.find(label_in(&hidden_b)),
            "the two hidden types were merged by the owners' later union"
        );
    }

    #[test]
    fn a_declaration_type_argument_stays_with_its_occurrence_across_a_shared_generic_copy() {
        // `data List[A] { Cons(x: A, ..) }` and `def len[T](xs: List[T])`. The def has one labeled
        // signature `List#s[T]`, and the calls `len[Box](a)` and `len[Bool](b)` substitute `T` per
        // call before unifying, exactly like `Call::label_and_unify`. That puts both lists into one
        // class (one generic copy of `List`) without relating their element types, which is
        // legitimate: the copy is generic. The elements stored in them must stay apart.
        let mut state = fresh_state();
        let mut sigs = DeclSignatures::new();
        sigs.insert(
            id!("Cons"),
            DeclSignature {
                decl_type_params: vec![id!("A", 1)],
                own_type_params: vec![],
                tys: vec![tvar!(id!("A", 1))],
            },
        );

        let t = id!("T", 2);
        let param = state.label_ty(&ty!(id!("List"), [tvar!(t.clone())]));
        let list_a = state.label_ty(&ty!(id!("List"), [ty!(id!("Box"))]));
        let list_b = state.label_ty(&ty!(id!("List"), [ty!(id!("Bool"))]));
        let call_arg_a = state.label_ty(&ty!(id!("Box")));
        let call_arg_b = state.label_ty(&ty!(id!("Bool")));
        state.unify_ty(
            &list_a,
            &param.substitute((std::slice::from_ref(&t), std::slice::from_ref(&call_arg_a))),
        );
        state.unify_ty(
            &list_b,
            &param.substitute((std::slice::from_ref(&t), std::slice::from_ref(&call_arg_b))),
        );
        assert_eq!(
            state.uf.find(label_in(&list_a)),
            state.uf.find(label_in(&list_b)),
            "precondition: both lists share one generic copy"
        );

        let elem_a = state.label_ty(&ty!(id!("Box")));
        let elem_b = state.label_ty(&ty!(id!("Bool")));
        constrain_xtor_occurrence(
            &mut state,
            &sigs,
            &id!("Cons"),
            &list_a,
            &[],
            std::slice::from_ref(&elem_a),
        );
        constrain_xtor_occurrence(
            &mut state,
            &sigs,
            &id!("Cons"),
            &list_b,
            &[],
            std::slice::from_ref(&elem_b),
        );

        assert_ne!(
            state.uf.find(label_in(&elem_a)),
            state.uf.find(label_in(&elem_b)),
            "the two lists' elements were merged just because the lists share a generic copy"
        );
    }

    #[test]
    fn finish_classes_returns_none_for_a_field_never_touched() {
        let state = fresh_state();
        let never_touched = Identifier {
            name: "Bar#1".to_string(),
            id: 0,
        };
        let (class_fields, used) = finish_classes(&state);
        assert_eq!(class_fields.get(&never_touched, &id!("MkBar"), 0), None);
        assert!(!used.contains(&never_touched, &id!("MkBar")));
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
