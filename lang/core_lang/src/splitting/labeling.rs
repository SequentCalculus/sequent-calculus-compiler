use std::{collections::HashMap, rc::Rc};

use crate::{
    splitting::union_find::UnionFind,
    syntax::{
        CodataDeclaration, ContextBinding, DataDeclaration, Def, Identifier, Prog, Ty,
        TypingContext,
        declaration::{Polarity, TypeDeclaration, XtorSig},
        types::TypeArgs,
    },
};

/// A label is a fresh `Identifier` sharing the declared type's name but carrying a unique `id` prefixed with `#`, e.g. `Box#1`
pub type Label = Identifier;

pub type DeclSignatures = HashMap<Identifier, Vec<Ty>>;

/// Carries all mutable state through the single label+unify walk: the fresh-id counter, the
/// union-find, and a record of each label's origin.
#[derive(Default)]
pub struct SplitState {
    pub uf: UnionFind,
    per_name_counters: HashMap<String, usize>,
    /// Maps every minted label back to the original (unlabeled) declaration identifier it was
    /// derived from, e.g. `Box#3` -> `Box`.
    pub label_origin: HashMap<Label, Identifier>,
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
fn label_xtor_signature<P: Polarity + Clone>(xtor: &XtorSig<P>, state: &mut SplitState) -> XtorSig<P> {
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
    let mut sigs = DeclSignatures::new();
    for def in &prog.defs {
        let params = label_def_signature(def, state);
        sigs.insert(def.name.clone(), params);
    }

    let data_types: Vec<DataDeclaration> = prog
        .data_types
        .iter()
        .map(|decl| label_typedeclaration_signature(decl, state))
        .collect();
    for xtor in data_types.iter().flat_map(|decl| &decl.xtors) {
        let field_tys = xtor.args.bindings.iter().map(|b| b.ty.clone()).collect();
        sigs.insert(xtor.name.clone(), field_tys);
    }

    let codata_types: Vec<CodataDeclaration> = prog
        .codata_types
        .iter()
        .map(|decl| label_typedeclaration_signature(decl, state))
        .collect();
    for xtor in codata_types.iter().flat_map(|decl| &decl.xtors) {
        let field_tys = xtor.args.bindings.iter().map(|b| b.ty.clone()).collect();
        sigs.insert(xtor.name.clone(), field_tys);
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
    /// The type of this syntax element after labeling. For a `Term<C>` this is again `Term<C>`;
    /// generic containers like `Vec<X>`/`Rc<X>`/`Option<X>` delegate to `X::Target`.
    type Target;

    fn label_and_unify(
        &self,
        state: &mut SplitState,
        sigs: &DeclSignatures,
        scope: &TypingContext,
    ) -> Self::Target;
}

impl<X: LabelAndUnify> LabelAndUnify for Vec<X> {
    type Target = Vec<X::Target>;
    fn label_and_unify(
        &self,
        state: &mut SplitState,
        sigs: &DeclSignatures,
        scope: &TypingContext,
    ) -> Self::Target {
        self.iter()
            .map(|x| x.label_and_unify(state, sigs, scope))
            .collect()
    }
}

impl<X: LabelAndUnify> LabelAndUnify for Rc<X> {
    type Target = Rc<X::Target>;
    fn label_and_unify(
        &self,
        state: &mut SplitState,
        sigs: &DeclSignatures,
        scope: &TypingContext,
    ) -> Self::Target {
        Rc::new(self.as_ref().label_and_unify(state, sigs, scope))
    }
}

impl<X: LabelAndUnify> LabelAndUnify for Option<X> {
    type Target = Option<X::Target>;
    fn label_and_unify(
        &self,
        state: &mut SplitState,
        sigs: &DeclSignatures,
        scope: &TypingContext,
    ) -> Self::Target {
        self.as_ref().map(|x| x.label_and_unify(state, sigs, scope))
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
        assert_eq!(sigs[&id!("f")].len(), 1);
        assert_eq!(sigs[&id!("Cons")].len(), 1);
        assert_eq!(sigs[&id!("head")].len(), 1);

        // the labeled declaration trees carry the same field types as `sigs`, not a separate copy
        assert_eq!(data_types.len(), 1);
        assert_eq!(
            data_types[0].xtors[0].args.bindings[0].ty,
            sigs[&id!("Cons")][0]
        );
        assert_eq!(codata_types.len(), 1);
        assert_eq!(
            codata_types[0].xtors[0].args.bindings[0].ty,
            sigs[&id!("head")][0]
        );
    }
}
