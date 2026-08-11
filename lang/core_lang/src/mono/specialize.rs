use std::{rc::Rc, vec};

use crate::{
    mono::{erasure::ErasedDecls, naming_table::NamingTable, solver::Solution},
    syntax::{
        Chi, Clause, Def, Identifier, Prog, Ty,
        declaration::{Polarity, TypeDeclaration, XtorSig},
        statements::Unreachable,
    },
    traits::Typed,
};

/// A context for specializing polymorphic declarations into monomorphic ones.
///
/// `table` is a reference to the naming table that maps polymorphic type parameters to their corresponding concrete types.
/// `subst` is an optional tuple containing a reference to the list of type parameters and their corresponding concrete types for the current specialization context.
#[derive(Clone)]
pub struct SpecializeContext<'a> {
    pub table: &'a NamingTable,
    pub subst: (Vec<Identifier>, Vec<Ty>),
    pub erased_decls: &'a ErasedDecls,
}

impl<'a> SpecializeContext<'a> {
    /// A context for specializing already-ground terms, with no active variable substitution.
    pub fn ground(table: &'a NamingTable, erased_decls: &'a ErasedDecls) -> Self {
        SpecializeContext {
            table,
            subst: (vec![], vec![]),
            erased_decls,
        }
    }

    /// A context for specializing one instantiation of a polymorphic declaration body.
    pub fn with_subst(
        table: &'a NamingTable,
        params: &'a [Identifier],
        args: &'a [Ty],
        erased_decls: &'a ErasedDecls,
    ) -> Self {
        debug_assert_eq!(
            params.len(),
            args.len(),
            "with_subst called with mismatched lengths: params={:?}, args={:?}",
            params,
            args
        );
        SpecializeContext {
            table,
            subst: (params.to_vec(), args.to_vec()),
            erased_decls,
        }
    }

    /// Extends the current specialization context with additional type parameters and their corresponding concrete types, returning a new `SpecializeContext` that combines the existing substitution with the new one.
    fn extend_with_substs(&self, new_params: &[Identifier], new_args: &[Ty]) -> Self {
        debug_assert_eq!(
            new_params.len(),
            new_args.len(),
            "extend_with_substs called with mismatched lengths: new_params={:?}, new_args={:?}",
            new_params,
            new_args
        );
        let mut extended_params = self.subst.0.clone();
        extended_params.extend_from_slice(new_params);

        let mut extended_args = self.subst.1.clone();
        extended_args.extend_from_slice(new_args);

        SpecializeContext {
            table: self.table,
            subst: (extended_params, extended_args),
            erased_decls: self.erased_decls,
        }
    }
}

/// A trait for types that can be specialized from polymorphic to monomorphic forms.
pub trait Specialize {
    /// Specializes the current instance using the provided specialization context, returning a new instance with all polymorphic type parameters replaced by their corresponding concrete types.
    fn specialize(&self, context: &SpecializeContext) -> Self;
}

impl<X: Specialize> Specialize for Vec<X> {
    fn specialize(&self, ctx: &SpecializeContext) -> Self {
        self.iter().map(|x| x.specialize(ctx)).collect()
    }
}

impl<X: Specialize> Specialize for Option<X> {
    fn specialize(&self, ctx: &SpecializeContext) -> Self {
        self.as_ref().map(|x| x.specialize(ctx))
    }
}

impl<X: Specialize> Specialize for std::rc::Rc<X> {
    fn specialize(&self, ctx: &SpecializeContext) -> Self {
        std::rc::Rc::new(self.as_ref().specialize(ctx))
    }
}

/// This function is the entry point for specializing a program from polymorphic to monomorphic form. It takes a reference to a [`Solution`] produced by the constraint solving process, and returns a new program where all polymorphic type parameters have been replaced with their corresponding concrete types according to the solution.
pub fn specialize_program(prog: &Prog, solution: &Solution, erased_decls: &ErasedDecls) -> Prog {
    let table = NamingTable::build(
        solution,
        &prog.data_types,
        &prog.codata_types,
        &prog.defs,
        erased_decls,
    );

    let data_types = prog
        .data_types
        .iter()
        .flat_map(|data_decl| specialize_declaration(data_decl, &table, erased_decls))
        .collect::<Vec<_>>();

    let codata_types = prog
        .codata_types
        .iter()
        .flat_map(|codata_decl| specialize_declaration(codata_decl, &table, erased_decls))
        .collect::<Vec<_>>();

    let defs: Vec<_> = prog
        .defs
        .iter()
        .flat_map(|def| specialize_def(def, &table, erased_decls))
        .collect();

    Prog {
        defs,
        data_types,
        codata_types,
        max_id: prog.max_id,
    }
}

/// Specialization of polymorphic type declarations into monomorphic ones
fn specialize_declaration<P: Polarity + Clone>(
    decl: &TypeDeclaration<P>,
    table: &NamingTable,
    erased_decls: &ErasedDecls,
) -> Vec<TypeDeclaration<P>> {
    let params = &decl.type_params;
    let is_erased = erased_decls.is_erased(&decl.name);

    if params.is_empty() || is_erased {
        if !is_erased && decl.xtors.iter().all(|xtor| xtor.type_params.is_empty()) {
            // This is already a monomorphic declaration, so we can return it as-is
            return vec![decl.clone()];
        }

        let ctx = SpecializeContext::ground(table, erased_decls);
        let extra_params: &[Identifier] = if is_erased { params } else { &[] };

        // This is already a monomorphic declaration, so we only need to specialize its xtors.
        return vec![TypeDeclaration {
            dat: decl.dat.clone(),
            name: decl.name.clone(),
            xtors: decl
                .xtors
                .iter()
                .flat_map(|xtor| specialize_xtor_sig(xtor, extra_params, &ctx))
                .collect(),
            type_params: vec![],
        }];
    }

    table
        .instantiations_for(&decl.name)
        .iter()
        .map(|tuple| {
            let ctx = SpecializeContext::with_subst(table, params, tuple, erased_decls);
            TypeDeclaration {
                dat: decl.dat.clone(),
                name: table.lookup(&decl.name, tuple).clone(),
                xtors: decl
                    .xtors
                    .iter()
                    .flat_map(|xtor| specialize_xtor_sig(xtor, &[], &ctx))
                    .collect(),
                type_params: vec![],
            }
        })
        .collect()
}

/// Specialization of polymorphic constructor/destructor signatures into monomorphic ones
fn specialize_xtor_sig<P: Polarity + Clone>(
    xtor_sig: &XtorSig<P>,
    extra_params: &[Identifier],
    ctx: &SpecializeContext,
) -> Vec<XtorSig<P>> {
    let mut params = xtor_sig.type_params.clone();
    params.extend_from_slice(extra_params);

    // This is already a monomorphic declaration, so we can return it as-is
    if params.is_empty() {
        return vec![XtorSig {
            xtor: xtor_sig.xtor.clone(),
            name: xtor_sig.name.clone(),
            type_params: vec![],
            args: xtor_sig.args.specialize(ctx),
        }];
    }

    ctx.table
        .instantiations_for(&xtor_sig.name)
        .iter()
        .map(|tuple| {
            let extended_ctx = ctx.extend_with_substs(&params, tuple);
            XtorSig {
                xtor: xtor_sig.xtor.clone(),
                name: extended_ctx.table.lookup(&xtor_sig.name, tuple).clone(),
                type_params: vec![],
                args: xtor_sig.args.specialize(&extended_ctx),
            }
        })
        .collect()
}

/// Specialization of polymorphic clauses into monomorphic ones.
pub fn specialize_clause<C: Chi>(
    clause: &Clause<C>,
    ctx: &SpecializeContext,
    scrutinee_extra_args: Option<&[Ty]>,
) -> Vec<Clause<C>> {
    let extra_params = ctx.table.extra_params_for(&clause.xtor);
    let mut full_params = clause.type_params.clone();
    full_params.extend_from_slice(extra_params);

    if full_params.is_empty() {
        // No own type parameters bound by this clause and also no extra parameters from erased declarations.
        // The xtor name is looked up under the already-established decl-level substitution only.
        return vec![Clause {
            prdcns: clause.prdcns.clone(),
            xtor: clause.xtor.clone(),
            type_params: vec![],
            context: clause.context.specialize(ctx),
            body: clause.body.specialize(ctx),
        }];
    }

    ctx.table
        .instantiations_for(&clause.xtor)
        .iter()
        .map(|tuple| {
            let (_own_args, extra_args) = tuple.split_at(clause.type_params.len());
            let extended_ctx = ctx.extend_with_substs(&full_params, tuple);
            let xtor_name = extended_ctx.table.lookup(&clause.xtor, tuple).clone();

            let context = clause.context.specialize(&extended_ctx);

            let reachable = match scrutinee_extra_args {
                Some(active) => extra_args == active,
                None => true,
            };

            let body = if reachable {
                clause.body.specialize(&extended_ctx)
            } else {
                Rc::new(
                    Unreachable {
                        ty: clause.body.get_type(),
                    }
                    .into(),
                )
            };

            Clause {
                prdcns: clause.prdcns.clone(),
                xtor: xtor_name,
                type_params: vec![],
                context: context,
                body: body,
            }
        })
        .collect()
}

/// Specialization of polymorphic function definitions into monomorphic ones
pub fn specialize_def(def: &Def, table: &NamingTable, erased_decls: &ErasedDecls) -> Vec<Def> {
    let params = &def.type_params;

    if params.is_empty() {
        // This function has no type parameters of its own, so it produces
        // exactly one monomorphic copy. However, the body still
        // needs to be traversed with a ground context, because it may
        // contain calls to polymorphic functions or constructors that must
        // be rewritten to their specialized names.
        let ctx = SpecializeContext::ground(table, erased_decls);
        return vec![Def {
            name: def.name.clone(),
            type_params: vec![],
            context: def.context.specialize(&ctx),
            body: def.body.specialize(&ctx),
        }];
    }

    table
        .instantiations_for(&def.name)
        .iter()
        .map(|tuple| {
            let ctx = SpecializeContext::with_subst(table, params, tuple, erased_decls);
            Def {
                name: table.lookup(&def.name, tuple).clone(),
                type_params: vec![],
                context: def.context.specialize(&ctx),
                body: def.body.specialize(&ctx),
            }
        })
        .collect()
}

#[cfg(test)]
mod specialize_tests {
    use std::collections::{HashMap, HashSet};

    use crate::{
        mono::{
            erasure::ErasedDecls,
            naming_table::NamingTable,
            solver::Solution,
            specialize::{
                Specialize, SpecializeContext, specialize_clause, specialize_declaration,
                specialize_def, specialize_program,
            },
        },
        syntax::{
            Clause, Cns, CodataDeclaration, DataDeclaration, Def, Prog, Statement, Ty,
            types::TypeArgs,
        },
        traits::Typed,
    };
    extern crate self as core_lang;
    use core_macros::{
        bind, call, clause, cns, codata, covar, ctor, ctor_sig, cut, data, def, dtor_sig, exit, id,
        lit, prd, tvar, ty, var,
    };

    fn list_decl() -> DataDeclaration {
        return data!(
            id!("List"),
            [
                ctor_sig!(id!("Nil"), [], []),
                ctor_sig!(
                    id!("Cons"),
                    [],
                    [
                        bind!(id!("x"), prd!(), tvar!(id!("A", 1))),
                        bind!(id!("xs"), prd!(), ty!(id!("List"), [tvar!(id!("A", 1))]))
                    ]
                )
            ],
            [id!("A", 1)]
        );
    }

    fn bool_decl() -> DataDeclaration {
        return data!(
            id!("Bool"),
            [
                ctor_sig!(id!("True"), [], []),
                ctor_sig!(id!("False"), [], [])
            ],
            []
        );
    }

    fn pair_decl() -> DataDeclaration {
        return data!(
            id!("Pair"),
            [ctor_sig!(
                id!("mkPair"),
                [],
                [
                    bind!(id!("x"), prd!(), tvar!(id!("A", 2))),
                    bind!(id!("y"), prd!(), tvar!(id!("B", 3)))
                ]
            )],
            [id!("A", 2), id!("B", 3)]
        );
    }

    fn identity_def() -> Def {
        return def!(
            id!("identity"),
            [id!("A", 1)],
            [
                bind!(id!("x"), prd!(), tvar!(id!("A", 1))),
                bind!(id!("ret"), cns!(), tvar!(id!("A", 1)))
            ],
            cut!(
                var!(id!("x"), tvar!(id!("A", 1))),
                covar!(id!("ret"), tvar!(id!("A", 1))),
                tvar!(id!("A", 1))
            )
        );
    }

    fn box_decl() -> DataDeclaration {
        return data!(
            id!("Box"),
            [ctor_sig!(
                id!("Pack"),
                [id!("E", 2)],
                [bind!(id!("x"), prd!(), tvar!(id!("E", 2)))]
            )],
            []
        );
    }

    fn container_decl() -> CodataDeclaration {
        return codata!(
            id!("Container"),
            [dtor_sig!(
                id!("wrap"),
                [id!("S", 2)],
                [
                    bind!(id!("x"), prd!(), tvar!(id!("S", 2))),
                    bind!(id!("tag"), prd!(), tvar!(id!("T", 1)))
                ]
            )],
            [id!("T", 1)]
        );
    }

    fn nil_clause() -> Clause<Cns> {
        return clause!(Cns, id!("Nil"), [], [], exit!(lit!(0)));
    }

    fn pack_clause() -> Clause<Cns> {
        return clause!(
            Cns,
            id!("Pack"),
            [id!("G", 4)],
            [bind!(id!("x"), prd!(), tvar!(id!("G", 4)))],
            exit!(lit!(0))
        );
    }

    #[test]
    fn specialize_data_declaration_produces_one_copy_per_instantiation() {
        // data List[A] { Nil, Cons(x: A, xs: List[A]) }
        // instantiated at both i64 and Bool. Expect two monomorphic
        // copies, each with A correctly substituted throughout.

        let node = vec![id!("A", 1)];
        let solution = Solution::from(HashMap::from([(
            node.clone(),
            HashSet::from([vec![ty!("int")], vec![ty!(id!("Bool"))]]),
        )]));

        let table = NamingTable::build(
            &solution,
            &[list_decl(), bool_decl()],
            &[],
            &[],
            &ErasedDecls::default(),
        );
        let copies = specialize_declaration(&list_decl(), &table, &ErasedDecls::default());

        assert_eq!(
            copies.len(),
            2,
            "expected one monomorphic copy per instantiation"
        );

        // Every copy must have no remaining type parameters and a Cons
        // signature whose field type matches its own instantiation.
        for copy in &copies {
            assert!(copy.type_params.is_empty());
            let cons = copy.xtors.iter().find(|x| x.name == id!("Cons")).unwrap();
            let x_binding = &cons.args.bindings[0];
            // x_binding.ty must be one of the two resolved ground types,
            // and must match the mangled name used for `xs`'s List[...] field.
            assert!(matches!(x_binding.ty, Ty::I64) || matches!(&x_binding.ty, Ty::Decl { .. }));
        }
    }

    #[test]
    fn specialize_multi_param_declaration_keeps_correlated_tuples() {
        // data Pair[A, B] { mkPair(x: A, y: B) }
        // Only the correlated tuple [i64, Bool] was ever observed -- not
        // the full cross product. Specialization must produce exactly one
        // monomorphic copy, not four.

        let node = vec![id!("A", 2), id!("B", 3)];
        let solution = Solution::from(HashMap::from([(
            node.clone(),
            HashSet::from([vec![ty!("int"), ty!(id!("Bool"))]]),
        )]));

        let table = NamingTable::build(
            &solution,
            &[pair_decl(), bool_decl()],
            &[],
            &[],
            &ErasedDecls::default(),
        );
        let copies = specialize_declaration(&pair_decl(), &table, &ErasedDecls::default());

        assert_eq!(
            copies.len(),
            1,
            "expected exactly one correlated copy, not a cross product"
        );

        let mk_pair = &copies[0].xtors[0];
        assert_eq!(mk_pair.args.bindings[0].ty, Ty::I64);
        assert_eq!(
            mk_pair.args.bindings[1].ty,
            copies[0].xtors[0].args.bindings[1].ty
        );
    }

    #[test]
    fn specialize_ground_ctor_term_at_call_site() {
        // Cons(1, Nil) : List[i64], fully ground as it would appear after
        // type checking. No substitution is active; the naming table alone
        // resolves List[i64] to its mangled name.

        let node = vec![id!("A", 1)];
        let solution = Solution::from(HashMap::from([(
            node.clone(),
            HashSet::from([vec![ty!("int")]]),
        )]));
        let table =
            NamingTable::build(&solution, &[list_decl()], &[], &[], &ErasedDecls::default());
        let erased = ErasedDecls::default();
        let ctx = &SpecializeContext::ground(&table, &erased);

        let term = ctor!(
            id!("Cons"),
            [],
            [
                lit!(1),
                ctor!(id!("Nil"), [], [], ty!(id!("List"), [ty!("int")]))
            ],
            ty!(id!("List"), [ty!("int")])
        );

        let result = term.specialize(ctx);

        let expected_name = table.lookup(&list_decl().name, &[ty!("int")]).clone();
        assert_eq!(
            result.ty,
            Ty::Decl {
                name: expected_name,
                type_args: TypeArgs::default(),
            }
        );
        assert_eq!(result.name, id!("Cons")); // xtor name unchanged for now
    }

    #[test]
    fn specialize_end_to_end_pair_inside_list() {
        // data Pair[A, B] { mkPair(x: A, y: B) }
        // data List[C] { Nil, Cons(x: C, xs: List[C]) }
        //
        // Verifies that the Pair node and the List node are specialized
        // independently and consistently, with the inner Pair instantiation
        // correctly nested inside the outer List instantiation's lookup.

        let pair_node = vec![id!("A", 2), id!("B", 3)];
        let list_node = vec![id!("A", 1)];
        let pair_ty = ty!(id!("Pair"), [ty!("int"), ty!("int")]);

        let solution = Solution::from(HashMap::from([
            (
                pair_node.clone(),
                HashSet::from([vec![ty!("int"), ty!("int")]]),
            ),
            (list_node.clone(), HashSet::from([vec![pair_ty.clone()]])),
        ]));

        let table = NamingTable::build(
            &solution,
            &[pair_decl(), list_decl()],
            &[],
            &[],
            &ErasedDecls::default(),
        );

        let pair_copies = specialize_declaration(&pair_decl(), &table, &ErasedDecls::default());
        let list_copies = specialize_declaration(&list_decl(), &table, &ErasedDecls::default());

        assert_eq!(pair_copies.len(), 1);
        assert_eq!(list_copies.len(), 1);

        // The List copy's Cons.xs field must reference the *same* mangled
        // List name as the declaration's own specialized name, and its x
        // field must reference the *same* mangled Pair name produced for
        // the Pair declaration above -- consistency across two independent
        // top-level specializations.
        let list_name = table.lookup(&list_decl().name, &[pair_ty.clone()]).clone();
        let pair_name = table
            .lookup(&pair_decl().name, &[ty!("int"), ty!("int")])
            .clone();

        assert_eq!(list_copies[0].name, list_name);
        assert_eq!(pair_copies[0].name, pair_name);

        let cons = list_copies[0]
            .xtors
            .iter()
            .find(|x| x.name == id!("Cons"))
            .unwrap();
        assert_eq!(
            cons.args.bindings[0].ty,
            Ty::Decl {
                name: pair_name.clone(),
                type_args: TypeArgs { args: vec![] }
            }
        );
        assert_eq!(
            cons.args.bindings[1].ty,
            Ty::Decl {
                name: list_name,
                type_args: TypeArgs { args: vec![] }
            }
        );
    }

    #[test]
    fn test_specialize_program_end_to_end() {
        // data List[A] { Nil, Cons(x: A, xs: List[A]) }
        // instantiated at i64. Expect one monomorphic copy, with A correctly substituted throughout.

        let node = vec![id!("A", 1)];
        let solution = Solution::from(HashMap::from([(
            node.clone(),
            HashSet::from([vec![ty!("int")]]),
        )]));

        let prog = Prog {
            defs: vec![],
            data_types: vec![list_decl()],
            codata_types: vec![],
            max_id: 0,
        };

        let specialized_prog = specialize_program(&prog, &solution, &ErasedDecls::default());

        assert_eq!(
            specialized_prog.data_types.len(),
            1,
            "Expected exactly one specialized copy"
        );
        assert_eq!(specialized_prog.codata_types.len(), 0);

        let table =
            NamingTable::build(&solution, &[list_decl()], &[], &[], &ErasedDecls::default());
        let expected_name = table.lookup(&list_decl().name, &[ty!("int")]).clone();

        let specialized_list = &specialized_prog.data_types[0];

        assert_eq!(specialized_list.name, expected_name);

        assert!(
            specialized_list.type_params.is_empty(),
            "Expected the specialized list to have no type parameters"
        );

        let cons = specialized_list
            .xtors
            .iter()
            .find(|x| x.name == id!("Cons"))
            .unwrap();
        assert_eq!(cons.args.bindings[0].ty, Ty::I64);
    }

    #[test]
    fn specialize_monomorphic_def_traverses_body() {
        // def main() { ⟨ Cons(1, Nil) | a ⟩ }
        //
        // main has no type parameters of its own, so it produces exactly one
        // copy. However, its body contains List[i64] at the call site and that
        // type must be rewritten to the mangled name -- the body traversal must
        // happen even though no substitution is active.

        let main_def = def!(
            id!("main"),
            [],
            [bind!(id!("ret"), cns!(), ty!("int"))],
            cut!(
                ctor!(
                    id!("Cons"),
                    [],
                    [
                        lit!(1),
                        ctor!(id!("Nil"), [], [], ty!(id!("List"), [ty!("int")]))
                    ],
                    ty!(id!("List"), [ty!("int")])
                ),
                covar!(id!("ret"), ty!("int")),
                ty!("int")
            )
        );

        let node = vec![id!("A", 1)];
        let solution = Solution::from(HashMap::from([(
            node.clone(),
            HashSet::from([vec![ty!("int")]]),
        )]));

        let table = NamingTable::build(
            &solution,
            &[list_decl()],
            &[],
            &[main_def.clone()],
            &ErasedDecls::default(),
        );
        let copies = specialize_def(&main_def, &table, &ErasedDecls::default());

        // Exactly one copy of main, no multiplication.
        assert_eq!(copies.len(), 1);
        assert_eq!(copies[0].name, id!("main"));
        assert!(copies[0].type_params.is_empty());

        // The body's Xtor type must have been rewritten to the mangled List name.
        let expected_list_name = table.lookup(&list_decl().name, &[ty!("int")]).clone();
        let Statement::Cut(cut) = &copies[0].body else {
            panic!("expected a cut statement in main's body");
        };
        assert_eq!(
            cut.producer.get_type(),
            Ty::Decl {
                name: expected_list_name,
                type_args: TypeArgs { args: vec![] }
            }
        );
    }

    #[test]
    fn specialize_polymorphic_def_produces_one_copy_per_instantiation() {
        // def identity[A](x: A, ret: cns A) { ⟨ x | ret ⟩ }
        //
        // Instantiated at i64 and Bool. Expect two monomorphic copies, each
        // with A substituted, an empty type_params list, and a name that
        // encodes the instantiation.

        let node = vec![id!("A", 1)];
        let solution = Solution::from(HashMap::from([(
            node.clone(),
            HashSet::from([vec![ty!("int")], vec![ty!(id!("Bool"))]]),
        )]));

        let table = NamingTable::build(
            &solution,
            &[bool_decl()],
            &[],
            &[identity_def()],
            &ErasedDecls::default(),
        );
        let copies = specialize_def(&identity_def(), &table, &ErasedDecls::default());

        assert_eq!(copies.len(), 2, "expected one copy per instantiation");

        for copy in &copies {
            assert!(copy.type_params.is_empty());

            // Context bindings must be ground, no Ty::Var remaining.
            for binding in &copy.context.bindings {
                assert!(
                    !matches!(binding.ty, Ty::Var(_)),
                    "expected ground type in context, got: {:?}",
                    binding.ty
                );
            }

            // The cut's type in the body must be ground as well.
            let Statement::Cut(cut) = &copy.body else {
                panic!("expected a cut statement in identity's body");
            };
            assert!(!matches!(cut.ty, Ty::Var(_)));
        }

        // Both expected instantiations must be present.
        let name_int = table.lookup(&identity_def().name, &[ty!("int")]).clone();
        let name_bool = table
            .lookup(&identity_def().name, &[ty!(id!("Bool"))])
            .clone();
        let copy_names: Vec<_> = copies.iter().map(|d| d.name.clone()).collect();
        assert!(copy_names.contains(&name_int));
        assert!(copy_names.contains(&name_bool));
    }

    #[test]
    fn specialize_unused_polymorphic_def_is_dropped() {
        // A polymorphic function that appears in the program but is never
        // called (and therefore never appears in the solution) must be
        // silently dropped from monomorphic Core rather than producing a copy
        // that retains unresolved type variables.

        let unused = def!(
            id!("unused"),
            [id!("A", 1)],
            [bind!(id!("x"), prd!(), tvar!(id!("A", 1)))],
            cut!(
                var!(id!("x"), tvar!(id!("A", 1))),
                covar!(id!("ret", 2), tvar!(id!("A", 1))),
                tvar!(id!("A", 1))
            )
        );

        // Empty solution: no instantiation was ever observed for this def.
        let solution = Solution::from(HashMap::new());
        let table = NamingTable::build(
            &solution,
            &[],
            &[],
            &[unused.clone()],
            &ErasedDecls::default(),
        );
        let copies = specialize_def(&unused, &table, &ErasedDecls::default());

        assert!(
            copies.is_empty(),
            "expected an unused polymorphic def to be dropped, got: {:?}",
            copies
        );
    }

    #[test]
    fn specialize_def_using_polymorphic_data_type_consistent_names() {
        // data List[A] { Nil, Cons(x: A, xs: List[A]) }
        // def singleton[A](x: A, ret: cns List[A]) { ⟨ Cons(x, Nil) | ret ⟩ }
        //
        // The List[i64] reference inside singleton's body must resolve to the
        // *same* mangled name that specialize_declaration produces for List
        // when instantiated at i64. Consistency between declaration and
        // call-site specialization is the core invariant being tested here.

        let singleton = def!(
            id!("singleton"),
            [id!("A", 1)],
            [
                bind!(id!("x"), prd!(), tvar!(id!("A", 1))),
                bind!(id!("ret"), cns!(), ty!(id!("List"), [tvar!(id!("A", 1))]))
            ],
            cut!(
                ctor!(
                    id!("Cons"),
                    [],
                    [
                        var!(id!("x"), tvar!(id!("A", 1))),
                        ctor!(id!("Nil"), [], [], ty!(id!("List"), [tvar!(id!("A", 1))]))
                    ],
                    ty!(id!("List"), [tvar!(id!("A", 1))])
                ),
                covar!(id!("ret"), ty!(id!("List"), [tvar!(id!("A", 1))])),
                ty!(id!("List"), [tvar!(id!("A", 1))])
            )
        );

        let node = vec![id!("A", 1)];
        let solution = Solution::from(HashMap::from([(
            node.clone(),
            HashSet::from([vec![ty!("int")]]),
        )]));

        let table = NamingTable::build(
            &solution,
            &[list_decl()],
            &[],
            &[singleton.clone()],
            &ErasedDecls::default(),
        );

        let list_copies = specialize_declaration(&list_decl(), &table, &ErasedDecls::default());
        let def_copies = specialize_def(&singleton, &table, &ErasedDecls::default());

        assert_eq!(list_copies.len(), 1);
        assert_eq!(def_copies.len(), 1);

        // The mangled List name used inside the def's body must match the
        // declaration's own specialized name exactly.
        let expected_list_name = table.lookup(&list_decl().name, &[ty!("int")]).clone();

        // Check the context binding: ret: List[i64] -> ret: List_i64 (or equivalent)
        let ret_binding = def_copies[0]
            .context
            .bindings
            .iter()
            .find(|b| b.var == id!("ret"))
            .unwrap();
        assert_eq!(
            ret_binding.ty,
            Ty::Decl {
                name: expected_list_name.clone(),
                type_args: TypeArgs { args: vec![] }
            }
        );

        // Check the cut's type in the body
        let Statement::Cut(cut) = &def_copies[0].body else {
            panic!("expected a cut statement in singleton's body");
        };
        assert_eq!(
            cut.ty,
            Ty::Decl {
                name: expected_list_name.clone(),
                type_args: TypeArgs { args: vec![] }
            }
        );

        // The specialized declaration's name must match what the def sees.
        assert_eq!(list_copies[0].name, expected_list_name);
    }

    #[test]
    fn specialize_def_with_multiple_type_params_correlated() {
        // def swap[A, B](x: A, y: B, ret_a: cns A, ret_b: cns B) { ⟨ x | ret_a ⟩ }
        //
        // Instantiated only at the correlated tuple [i64, Bool] -- not at all
        // four combinations of {i64, Bool} × {i64, Bool}. Verifies that
        // multi-parameter defs are handled like multi-parameter data
        // declarations: one copy per correlated tuple in the solution, not
        // a cross product of independent single-variable solution sets.

        let swap = def!(
            id!("swap"),
            [id!("A", 1), id!("B", 2)],
            [
                bind!(id!("x"), prd!(), tvar!(id!("A", 1))),
                bind!(id!("y"), prd!(), tvar!(id!("B", 2))),
                bind!(id!("ret_a"), cns!(), tvar!(id!("A", 1))),
                bind!(id!("ret_b"), cns!(), tvar!(id!("B", 2)))
            ],
            cut!(
                var!(id!("x"), tvar!(id!("A", 1))),
                covar!(id!("ret_a"), tvar!(id!("A", 1))),
                tvar!(id!("A", 1))
            )
        );

        // Only the single correlated instantiation [i64, Bool] was observed.
        let node = vec![id!("A", 1), id!("B", 2)];
        let solution = Solution::from(HashMap::from([(
            node.clone(),
            HashSet::from([vec![ty!("int"), ty!(id!("Bool"))]]),
        )]));

        let table = NamingTable::build(
            &solution,
            &[bool_decl()],
            &[],
            &[swap.clone()],
            &ErasedDecls::default(),
        );
        let copies = specialize_def(&swap, &table, &ErasedDecls::default());

        assert_eq!(
            copies.len(),
            1,
            "expected exactly one correlated copy, not a cross product: got {:?}",
            copies.iter().map(|d| &d.name).collect::<Vec<_>>()
        );

        let copy = &copies[0];
        assert!(copy.type_params.is_empty());

        // x must have been resolved to i64, y to Bool.
        let x_ty = &copy
            .context
            .bindings
            .iter()
            .find(|b| b.var == id!("x"))
            .unwrap()
            .ty;
        let y_ty = &copy
            .context
            .bindings
            .iter()
            .find(|b| b.var == id!("y"))
            .unwrap()
            .ty;
        assert_eq!(*x_ty, Ty::I64, "expected x: i64 after substitution");
        assert_eq!(
            *y_ty,
            Ty::Decl {
                name: id!("Bool"),
                type_args: TypeArgs { args: vec![] }
            },
            "expected y: Bool after substitution"
        );

        // The mangled name must encode both instantiated positions.
        let expected_name = table
            .lookup(&swap.name, &[ty!("int"), ty!(id!("Bool"))])
            .clone();
        assert_eq!(copy.name, expected_name);
    }

    #[test]
    fn specialize_program_with_data_and_def_end_to_end() {
        // Full program round-trip through specialize_program:
        //
        //   data List[A] { Nil, Cons(x: A, xs: List[A]) }
        //   def main(ret: cns i64) { ⟨ Cons(1, Nil) | a ⟩ }   -- uses List[i64]
        //
        // After specialization the program must contain exactly one data
        // declaration (List_i64 or equivalent), exactly one def (main,
        // unchanged name), and no Ty::Var anywhere in either.

        let main_def = def!(
            id!("main"),
            [],
            [bind!(id!("ret"), cns!(), ty!("int"))],
            cut!(
                ctor!(
                    id!("Cons"),
                    [],
                    [
                        lit!(1),
                        ctor!(id!("Nil"), [], [], ty!(id!("List"), [ty!("int")]))
                    ],
                    ty!(id!("List"), [ty!("int")])
                ),
                covar!(id!("ret"), ty!("int")),
                ty!("int")
            )
        );

        let node = vec![id!("A", 1)];
        let solution = Solution::from(HashMap::from([(
            node.clone(),
            HashSet::from([vec![ty!("int")]]),
        )]));

        let prog = Prog {
            defs: vec![main_def],
            data_types: vec![list_decl().clone()],
            codata_types: vec![],
            max_id: 0,
        };

        let result = specialize_program(&prog, &solution, &ErasedDecls::default());

        // One monomorphic List copy, one main def.
        assert_eq!(result.data_types.len(), 1);
        assert_eq!(result.defs.len(), 1);
        assert!(result.data_types[0].type_params.is_empty());
        assert!(result.defs[0].type_params.is_empty());
        assert_eq!(result.defs[0].name, id!("main"));

        // No Ty::Var must survive in the output data declaration.
        for xtor in &result.data_types[0].xtors {
            for binding in &xtor.args.bindings {
                assert!(
                    !matches!(binding.ty, Ty::Var(_)),
                    "Ty::Var survived specialization in data declaration: {:?}",
                    binding.ty
                );
            }
        }

        // No Ty::Var must survive in main's context or body type.
        for binding in &result.defs[0].context.bindings {
            assert!(!matches!(binding.ty, Ty::Var(_)));
        }
        let Statement::Cut(cut) = &result.defs[0].body else {
            panic!("expected a cut in main's body");
        };
        assert!(!matches!(cut.ty, Ty::Var(_)));
        assert!(!matches!(cut.producer.get_type(), Ty::Var(_)));
    }

    #[test]
    fn specialize_def_that_calls_another_polymorphic_def() {
        // def identity[A](x: A, ret: cns A) { ⟨ x | ret ⟩ }
        // def wrap[A](x: A, ret: cns A) { identity[A](x, ret) }
        //
        // wrap calls identity, forwarding its own type parameter A as the
        // type argument. After specialization at i64, the call site inside
        // wrap's body must reference the specialized identity name (not the
        // polymorphic one), and wrap itself must have an empty type_params.
        // This is the key test for call-site rewriting when a def's own type
        // variable flows into another def's type argument.

        let wrap = def!(
            id!("wrap"),
            [id!("A", 1)],
            [
                bind!(id!("x"), prd!(), tvar!(id!("A", 1))),
                bind!(id!("ret"), cns!(), tvar!(id!("A", 1)))
            ],
            // wrap's body wird jetzt elegant über das call!-Macro erzeugt
            call!(
                id!("identity"),
                [tvar!(id!("A", 1))],
                [
                    var!(id!("x"), tvar!(id!("A", 1))),
                    covar!(id!("ret"), tvar!(id!("A", 1)))
                ]
            )
        );

        let node = vec![id!("A", 1)];
        let solution = Solution::from(HashMap::from([(
            node.clone(),
            HashSet::from([vec![ty!("int")]]),
        )]));

        let table = NamingTable::build(
            &solution,
            &[],
            &[],
            &[identity_def(), wrap.clone()],
            &ErasedDecls::default(),
        );

        let identity_copies = specialize_def(&identity_def(), &table, &ErasedDecls::default());
        let wrap_copies = specialize_def(&wrap, &table, &ErasedDecls::default());

        assert_eq!(identity_copies.len(), 1);
        assert_eq!(wrap_copies.len(), 1);

        let identity_copy = &identity_copies[0];
        let wrap_copy = &wrap_copies[0];

        // Both must be monomorphic after specialization.
        assert!(identity_copy.type_params.is_empty());
        assert!(wrap_copy.type_params.is_empty());

        // The call inside wrap's body must now reference the specialized
        // identity name, not the original polymorphic "identity".
        let expected_identity_name = table.lookup(&identity_def().name, &[ty!("int")]).clone();

        let Statement::Call(call) = &wrap_copy.body else {
            panic!(
                "expected a Call statement in wrap's body, got: {:?}",
                wrap_copy.body
            );
        };

        assert_eq!(
            call.name, expected_identity_name,
            "wrap's body must call the specialized identity, not the polymorphic one"
        );

        // The call's type argument must be fully ground -- no Ty::Var remaining.
        for arg_ty in &call.type_args.args {
            assert!(
                !matches!(arg_ty, Ty::Var(_)),
                "Ty::Var survived in call type args: {:?}",
                arg_ty
            );
        }
        assert!(call.type_args.args.is_empty());

        // The wrap copy's own name must be the specialized one.
        let expected_wrap_name = table.lookup(&wrap.name, &[ty!("int")]).clone();
        assert_eq!(wrap_copy.name, expected_wrap_name);
    }

    #[test]
    fn specialize_xtor_with_own_type_params_produces_one_signature_per_instantiation() {
        // Box itself has no type parameters, but its constructor Pack has its
        // own existential type parameter E, instantiated at both i64 and Bool.
        // Expect exactly one Box declaration (unparameterized) whose xtors list
        // contains two monomorphic Pack signatures, one per instantiation.

        let node = vec![id!("E", 2)];
        let solution = Solution::from(HashMap::from([(
            node.clone(),
            HashSet::from([vec![ty!("int")], vec![ty!(id!("Bool"))]]),
        )]));

        let table = NamingTable::build(
            &solution,
            &[box_decl(), bool_decl()],
            &[],
            &[],
            &ErasedDecls::default(),
        );
        let copies = specialize_declaration(&box_decl(), &table, &ErasedDecls::default());

        assert_eq!(
            copies.len(),
            1,
            "Box itself has no type parameters, so it must not be duplicated"
        );
        assert!(copies[0].type_params.is_empty());
        assert_eq!(
            copies[0].xtors.len(),
            2,
            "expected one monomorphic Pack signature per instantiation of E"
        );

        for pack in &copies[0].xtors {
            assert!(pack.type_params.is_empty());
            let x_ty = &pack.args.bindings[0].ty;
            assert!(matches!(x_ty, Ty::I64) || matches!(x_ty, Ty::Decl { .. }));
        }

        // Names must be distinct and match the naming table's own mangling.
        let name_int = table.lookup(&id!("Pack"), &[ty!("int")]).clone();
        let name_bool = table.lookup(&id!("Pack"), &[ty!(id!("Bool"))]).clone();
        let names: Vec<_> = copies[0].xtors.iter().map(|p| p.name.clone()).collect();
        assert!(names.contains(&name_int));
        assert!(names.contains(&name_bool));
    }

    #[test]
    fn specialize_declaration_and_xtor_both_with_own_type_params() {
        // Container[T] { wrap[S](x: S, tag: T) }
        // Both the declaration (T) and its destructor (S) have their own type
        // parameter. Container is instantiated only at i64, and wrap's own S
        // is only ever observed instantiated at Bool. Expect one monomorphic
        // Container copy whose single wrap signature has both x: Bool and
        // tag: i64 correctly substituted.

        let decl_node = vec![id!("T", 1)];
        let xtor_node = vec![id!("S", 2)];
        let solution = Solution::from(HashMap::from([
            (decl_node.clone(), HashSet::from([vec![ty!("int")]])),
            (xtor_node.clone(), HashSet::from([vec![ty!(id!("Bool"))]])),
        ]));

        let table = NamingTable::build(
            &solution,
            &[bool_decl()],
            &[container_decl()],
            &[],
            &ErasedDecls::default(),
        );
        let copies = specialize_declaration(&container_decl(), &table, &ErasedDecls::default());

        assert_eq!(copies.len(), 1, "Container[T] instantiated only at i64");
        assert!(copies[0].type_params.is_empty());
        assert_eq!(
            copies[0].xtors.len(),
            1,
            "wrap's S was only ever observed instantiated at Bool"
        );

        let wrap = &copies[0].xtors[0];
        assert!(wrap.type_params.is_empty());

        assert_eq!(
            wrap.args.bindings[0].ty, // x: S -> Bool
            Ty::Decl {
                name: id!("Bool"),
                type_args: TypeArgs { args: vec![] }
            }
        );
        assert_eq!(wrap.args.bindings[1].ty, Ty::I64); // tag: T -> i64
    }

    #[test]
    fn specialize_clause_without_own_type_params_produces_single_copy() {
        let node = vec![id!("A", 1)];
        let solution = Solution::from(HashMap::from([(
            node.clone(),
            HashSet::from([vec![ty!("int")]]),
        )]));
        let table =
            NamingTable::build(&solution, &[list_decl()], &[], &[], &ErasedDecls::default());
        let erased = ErasedDecls::default();
        let ctx = SpecializeContext::ground(&table, &erased);

        let copies = specialize_clause(&nil_clause(), &ctx, None);

        assert_eq!(copies.len(), 1);
        assert_eq!(copies[0].xtor, id!("Nil"));
        assert!(copies[0].type_params.is_empty());
    }

    #[test]
    fn specialize_clause_with_own_type_params_produces_one_copy_per_instantiation() {
        // "Pack[G](x) => 0"
        //
        // Pack's own type parameter is instantiated at both i64 and Bool,
        // mirroring the constructor declaration. Expect two clauses, each with
        // the binder's type fully resolved and the xtor name mangled to match
        // exactly what specialize_declaration produces for Box's Pack
        // signatures -- consistency between clause- and declaration-level
        // specialization is the core invariant being tested here.

        let node = vec![id!("E", 2)];
        let solution = Solution::from(HashMap::from([(
            node.clone(),
            HashSet::from([vec![ty!("int")], vec![ty!(id!("Bool"))]]),
        )]));

        let table = NamingTable::build(
            &solution,
            &[box_decl(), bool_decl()],
            &[],
            &[],
            &ErasedDecls::default(),
        );
        let erased = ErasedDecls::default();
        let ctx = SpecializeContext::ground(&table, &erased);

        let copies = specialize_clause(&pack_clause(), &ctx, None);
        assert_eq!(copies.len(), 2);

        let decl_copies = specialize_declaration(&box_decl(), &table, &ErasedDecls::default());
        let expected_names: Vec<_> = decl_copies[0]
            .xtors
            .iter()
            .map(|x| x.name.clone())
            .collect();

        for clause in &copies {
            assert!(clause.type_params.is_empty());
            assert!(
                expected_names.contains(&clause.xtor),
                "clause's mangled xtor name must match one produced for the Pack declaration"
            );
            assert!(
                !matches!(clause.context.bindings[0].ty, Ty::Var(_)),
                "binder type must be fully ground after specialization"
            );
        }

        // The two clauses must specialize to two *different* concrete types.
        assert_ne!(
            copies[0].context.bindings[0].ty,
            copies[1].context.bindings[0].ty
        );
    }
}

#[cfg(test)]
mod erasure_tests {
    use super::*;
    use crate::{
        mono::erasure::ErasedDecls,
        syntax::{DataDeclaration, types::TypeArgs},
    };
    use std::collections::{HashMap, HashSet};
    extern crate self as core_lang;
    use core_macros::{
        bind, call, covar, ctor, ctor_sig, cut, data, def, id, lit, prd, tvar, ty, var,
    };

    fn box_decl() -> DataDeclaration {
        return data!(
            id!("Box"),
            [ctor_sig!(
                id!("Wrap"),
                [],
                [bind!(id!("x"), prd!(), tvar!(id!("A", 1)))]
            )],
            [id!("A", 1)]
        );
    }

    #[test]
    fn specialize_erased_declaration_keeps_one_copy_with_two_xtor_variants() {
        let erased = ErasedDecls(HashSet::from([id!("Box")]));
        let solution = Solution::from(HashMap::from([(
            vec![id!("A", 1)],
            HashSet::from([vec![ty!("int")], vec![ty!(id!("Box"))]]),
        )]));

        let table = NamingTable::build(&solution, &[box_decl()], &[], &[], &erased);
        let copies = specialize_declaration(&box_decl(), &table, &erased);

        assert_eq!(copies.len(), 1, "erased declaration must not be duplicated");
        assert!(copies[0].type_params.is_empty());
        assert_eq!(
            copies[0].xtors.len(),
            2,
            "expected one Wrap variant per instantiation of the erased A"
        );
        for xtor in &copies[0].xtors {
            assert!(xtor.type_params.is_empty());
        }
    }

    #[test]
    fn xtor_specialize_recovers_extra_args_from_ty_when_erased() {
        // Wrap(123) : Box[i64]
        // Box is erased, so `type_args` on the Xtor term itself is
        // empty; the concrete instantiation must be recovere d from `self.ty`.
        let erased = ErasedDecls(HashSet::from([id!("Box")]));
        let solution = Solution::from(HashMap::from([(
            vec![id!("A", 1)],
            HashSet::from([vec![ty!("int")]]),
        )]));
        let table = NamingTable::build(&solution, &[box_decl()], &[], &[], &erased);
        let ctx = SpecializeContext::ground(&table, &erased);

        let term = ctor!(id!("Wrap"), [], [lit!(123)], ty!(id!("Box"), [ty!("int")]));

        let result = term.specialize(&ctx);

        let expected_name = table.lookup(&id!("Wrap"), &[ty!("int")]).clone();
        assert_eq!(result.name, expected_name);
        assert!(result.type_args.args.is_empty());
        assert_eq!(
            result.ty,
            Ty::Decl {
                name: id!("Box"),
                type_args: TypeArgs::default()
            }
        );
    }

    #[test]
    fn call_specialize_erases_nested_recursive_type_argument() {
        // nest[Box[C]](...) while specializing nest's own C := Box: after substitution the
        // call's type argument is Box[Box], which must be erased to bare Box before lookup.
        let erased = ErasedDecls(HashSet::from([id!("Box")]));
        let mut solution_map = HashMap::new();
        solution_map.insert(
            vec![id!("C", 1)],
            HashSet::from([vec![ty!("int")], vec![ty!(id!("Box"))]]),
        );
        let solution = Solution::from(solution_map);

        let nest_def = def!(
            id!("nest"),
            [id!("C", 1)],
            [bind!(id!("x"), prd!(), tvar!(id!("C", 1)))],
            cut!(
                var!(id!("x"), tvar!(id!("C", 1))),
                covar!(id!("ret"), tvar!(id!("C", 1))),
                tvar!(id!("C", 1))
            )
        );

        let table = NamingTable::build(&solution, &[box_decl()], &[], &[nest_def.clone()], &erased);

        let params = vec![id!("C", 1)];
        let args = vec![ty!(id!("Box"))];
        let ctx = SpecializeContext::with_subst(&table, &params, &args, &erased);

        let call = call!(id!("nest"), [ty!(id!("Box"), [tvar!(id!("C", 1))])], [],);

        let result = call.specialize(&ctx);

        // Box[C] with C := Box, erased to bare Box, must resolve to nest's own Box-instance
        // name, the very definition of the recursive call closing the loop.
        let expected_name = table.lookup(&id!("nest"), &[ty!(id!("Box"))]).clone();
        assert_eq!(result.name, expected_name);
        assert!(result.type_args.args.is_empty());
    }
}
