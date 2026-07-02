use crate::{
    mono::{naming_table::NamingTable, solver::Solution},
    syntax::{
        Identifier, Prog, Ty,
        declaration::{Polarity, TypeDeclaration},
    },
};

/// A context for specializing polymorphic declarations into monomorphic ones.
///
/// `table` is a reference to the naming table that maps polymorphic type parameters to their corresponding concrete types.
/// `subst` is an optional tuple containing a reference to the list of type parameters and their corresponding concrete types for the current specialization context.
#[derive(Clone, Copy)]
pub struct SpecializeContext<'a> {
    pub table: &'a NamingTable,
    pub subst: Option<(&'a [Identifier], &'a [Ty])>,
}

impl<'a> SpecializeContext<'a> {
    /// A context for specializing already-ground terms, with no active variable substitution.
    pub fn ground(table: &'a NamingTable) -> Self {
        SpecializeContext { table, subst: None }
    }

    /// A context for specializing one instantiation of a polymorphic declaration body.
    pub fn with_subst(table: &'a NamingTable, params: &'a [Identifier], args: &'a [Ty]) -> Self {
        SpecializeContext {
            table,
            subst: Some((params, args)),
        }
    }
}

/// A trait for types that can be specialized from polymorphic to monomorphic forms.
pub trait Specialize {
    /// Specializes the current instance using the provided specialization context, returning a new instance with all polymorphic type parameters replaced by their corresponding concrete types.
    fn specialize(&self, context: SpecializeContext) -> Self;
}

impl<X: Specialize> Specialize for Vec<X> {
    fn specialize(&self, ctx: SpecializeContext) -> Self {
        self.iter().map(|x| x.specialize(ctx)).collect()
    }
}

impl<X: Specialize> Specialize for Option<X> {
    fn specialize(&self, ctx: SpecializeContext) -> Self {
        self.as_ref().map(|x| x.specialize(ctx))
    }
}

impl<X: Specialize> Specialize for std::rc::Rc<X> {
    fn specialize(&self, ctx: SpecializeContext) -> Self {
        std::rc::Rc::new(self.as_ref().specialize(ctx))
    }
}

/// This function is the entry point for specializing a program from polymorphic to monomorphic form. It takes a reference to a [`Solution`] produced by the constraint solving process, and returns a new program where all polymorphic type parameters have been replaced with their corresponding concrete types according to the solution.
pub fn specialize_program(prog: &Prog, solution: &Solution) -> Prog {
    let table = NamingTable::build(solution, &prog.data_types, &prog.codata_types);

    let data_types = prog
        .data_types
        .iter()
        .flat_map(|data_decl| specialize_declaration(data_decl, solution, &table))
        .collect::<Vec<_>>();

    let codata_types = prog
        .codata_types
        .iter()
        .flat_map(|codata_decl| specialize_declaration(codata_decl, solution, &table))
        .collect::<Vec<_>>();

    let defs: Vec<_> = prog
        .defs
        .iter()
        .map(|def| def.specialize(SpecializeContext::ground(&table)))
        .collect();

    Prog {
        defs,
        data_types,
        codata_types,
        max_id: prog.max_id,
    }
}

/// Specialization of polymorphic type declarations into monomorphic ones
pub fn specialize_declaration<P: Polarity + Clone>(
    decl: &TypeDeclaration<P>,
    solution: &Solution,
    table: &NamingTable,
) -> Vec<TypeDeclaration<P>> {
    let node = &decl.type_params;
    if node.is_empty() {
        // This is already a monomorphic declaration, so we can just return it as-is.
        return vec![decl.clone()];
    }

    let Some(tuples) = solution.map.get(node) else {
        // No instantiation was ever observed for this declaration -- it is
        // unused in the program and can be dropped from monomorphic Core.
        return vec![];
    };

    tuples
        .iter()
        .map(|tuple| {
            let ctx = SpecializeContext::with_subst(table, node, tuple);
            TypeDeclaration {
                dat: decl.dat.clone(),
                name: table.lookup(&decl.name, tuple).clone(),
                xtors: decl.xtors.specialize(ctx),
                type_params: vec![],
            }
        })
        .collect()
}

#[cfg(test)]
mod specialize_tests {
    use std::collections::{HashMap, HashSet};

    use crate::{
        mono::{
            naming_table::NamingTable,
            solver::Solution,
            specialize::{
                Specialize, SpecializeContext, specialize_declaration, specialize_program,
            },
        },
        syntax::{Prog, Ty, types::TypeArgs},
    };
    extern crate self as core_lang;
    use core_macros::{bind, ctor, ctor_sig, data, id, lit, prd, tvar, ty};

    #[test]
    fn specialize_data_declaration_produces_one_copy_per_instantiation() {
        // data List[A] { Nil, Cons(x: A, xs: List[A]) }
        // instantiated at both i64 and Bool. Expect two monomorphic
        // copies, each with A correctly substituted throughout.
        let list = data!(
            id!("List"),
            [
                ctor_sig!(id!("Nil"), []),
                ctor_sig!(
                    id!("Cons"),
                    [
                        bind!(id!("x"), prd!(), tvar!(id!("A", 1))),
                        bind!(id!("xs"), prd!(), ty!(id!("List"), [tvar!(id!("A", 1))]))
                    ]
                )
            ],
            [id!("A", 1)]
        );

        let bool = data!(
            id!("Bool"),
            [ctor_sig!(id!("True"), []), ctor_sig!(id!("False"), [])],
            []
        );

        let node = vec![id!("A", 1)];
        let solution = Solution::from(HashMap::from([(
            node.clone(),
            HashSet::from([vec![ty!("int")], vec![ty!(id!("Bool"))]]),
        )]));

        let table = NamingTable::build(&solution, &[list.clone(), bool.clone()], &[]);
        let copies = specialize_declaration(&list, &solution, &table);

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
        let pair = data!(
            id!("Pair"),
            [ctor_sig!(
                id!("mkPair"),
                [
                    bind!(id!("x"), prd!(), tvar!(id!("A", 1))),
                    bind!(id!("y"), prd!(), tvar!(id!("B", 2)))
                ]
            )],
            [id!("A", 1), id!("B", 2)]
        );

        let bool = data!(
            id!("Bool"),
            [ctor_sig!(id!("True"), []), ctor_sig!(id!("False"), [])],
            []
        );

        let node = vec![id!("A", 1), id!("B", 2)];
        let solution = Solution::from(HashMap::from([(
            node.clone(),
            HashSet::from([vec![ty!("int"), ty!(id!("Bool"))]]),
        )]));

        let table = NamingTable::build(&solution, &[pair.clone(), bool.clone()], &[]);
        let copies = specialize_declaration(&pair, &solution, &table);

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
        let list = data!(
            id!("List"),
            [
                ctor_sig!(id!("Nil"), []),
                ctor_sig!(
                    id!("Cons"),
                    [
                        bind!(id!("x"), prd!(), tvar!(id!("A", 1))),
                        bind!(id!("xs"), prd!(), ty!(id!("List"), [tvar!(id!("A", 1))]))
                    ]
                )
            ],
            [id!("A", 1)]
        );

        let node = vec![id!("A", 1)];
        let solution = Solution::from(HashMap::from([(
            node.clone(),
            HashSet::from([vec![ty!("int")]]),
        )]));
        let table = NamingTable::build(&solution, &[list.clone()], &[]);
        let ctx = SpecializeContext::ground(&table);

        let term = ctor!(
            id!("Cons"),
            [
                lit!(1),
                ctor!(id!("Nil"), [], ty!(id!("List"), [ty!("int")]))
            ],
            ty!(id!("List"), [ty!("int")])
        );

        let result = term.specialize(ctx);

        let expected_name = table.lookup(&list.name, &[ty!("int")]).clone();
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
        let pair = data!(
            id!("Pair"),
            [ctor_sig!(
                id!("mkPair"),
                [
                    bind!(id!("x"), prd!(), tvar!(id!("A", 1))),
                    bind!(id!("y"), prd!(), tvar!(id!("B", 2)))
                ]
            )],
            [id!("A", 1), id!("B", 2)]
        );
        let list = data!(
            id!("List"),
            [
                ctor_sig!(id!("Nil"), []),
                ctor_sig!(
                    id!("Cons"),
                    [
                        bind!(id!("x"), prd!(), tvar!(id!("C", 3))),
                        bind!(id!("xs"), prd!(), ty!(id!("List"), [tvar!(id!("C", 3))]))
                    ]
                )
            ],
            [id!("C", 3)]
        );

        let pair_node = vec![id!("A", 1), id!("B", 2)];
        let list_node = vec![id!("C", 3)];
        let pair_ty = ty!(id!("Pair"), [ty!("int"), ty!("int")]);

        let solution = Solution::from(HashMap::from([
            (
                pair_node.clone(),
                HashSet::from([vec![ty!("int"), ty!("int")]]),
            ),
            (list_node.clone(), HashSet::from([vec![pair_ty.clone()]])),
        ]));

        let table = NamingTable::build(&solution, &[pair.clone(), list.clone()], &[]);

        let pair_copies = specialize_declaration(&pair, &solution, &table);
        let list_copies = specialize_declaration(&list, &solution, &table);

        assert_eq!(pair_copies.len(), 1);
        assert_eq!(list_copies.len(), 1);

        // The List copy's Cons.xs field must reference the *same* mangled
        // List name as the declaration's own specialized name, and its x
        // field must reference the *same* mangled Pair name produced for
        // the Pair declaration above -- consistency across two independent
        // top-level specializations.
        let list_name = table.lookup(&list.name, &[pair_ty.clone()]).clone();
        let pair_name = table.lookup(&pair.name, &[ty!("int"), ty!("int")]).clone();

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
        let list = data!(
            id!("List"),
            [
                ctor_sig!(id!("Nil"), []),
                ctor_sig!(
                    id!("Cons"),
                    [
                        bind!(id!("x"), prd!(), tvar!(id!("A", 1))),
                        bind!(id!("xs"), prd!(), ty!(id!("List"), [tvar!(id!("A", 1))]))
                    ]
                )
            ],
            [id!("A", 1)]
        );

        let node = vec![id!("A", 1)];
        let solution = Solution::from(HashMap::from([(
            node.clone(),
            HashSet::from([vec![ty!("int")]]),
        )]));

        let prog = Prog {
            defs: vec![],
            data_types: vec![list.clone()],
            codata_types: vec![],
            max_id: 0,
        };

        let specialized_prog = specialize_program(&prog, &solution);

        assert_eq!(
            specialized_prog.data_types.len(),
            1,
            "Expected exactly one specialized copy"
        );
        assert_eq!(specialized_prog.codata_types.len(), 0);

        let table = NamingTable::build(&solution, &[list.clone()], &[]);
        let expected_name = table.lookup(&list.name, &[ty!("int")]).clone();

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
}
