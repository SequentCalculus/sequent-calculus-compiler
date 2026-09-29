//! Specializes a polymorphic program into its monomorphic form, given the solver's solution
//! and the set of declarations erasure widened.

use std::{rc::Rc, vec};

use crate::{
    mono::{erasure::ErasedDecls, erasure::erase_ty, naming_table::NamingTable, solver::Solution},
    syntax::{
        Chi, Clause, Def, Identifier, Prog, Ty, TypeParam,
        declaration::{Polarity, TypeDeclaration, XtorSig},
        statements::Unreachable,
    },
    traits::Typed,
};

/// A substitution mapping declaration-site type parameters to their concrete instantiation, e.g.
/// `[A, B] -> [i64, Bool]` for `Pair[i64, Bool]`. The two lists are always the same length
/// which is enforced once here, at construction, rather than by convention at every call site
/// that used to build the pair by hand.
#[derive(Clone, Debug, Default)]
pub struct Substitution {
    params: Vec<Identifier>,
    args: Vec<Ty>,
}

impl Substitution {
    /// The empty substitution: no type parameters bound to anything.
    pub fn empty() -> Self {
        Self::default()
    }

    /// Builds a substitution from a parameter list and its concrete instantiation.
    pub fn new(params: Vec<Identifier>, args: Vec<Ty>) -> Self {
        debug_assert_eq!(
            params.len(),
            args.len(),
            "Substitution::new called with mismatched lengths: params={:?}, args={:?}",
            params,
            args
        );
        Substitution { params, args }
    }

    /// Returns a new substitution extending this one with additional parameter/argument pairs.
    pub fn extended(&self, new_params: &[Identifier], new_args: &[Ty]) -> Self {
        debug_assert_eq!(
            new_params.len(),
            new_args.len(),
            "extended called with mismatched lengths: new_params={:?}, new_args={:?}",
            new_params,
            new_args
        );
        let mut params = self.params.clone();
        params.extend_from_slice(new_params);
        let mut args = self.args.clone();
        args.extend_from_slice(new_args);
        Substitution { params, args }
    }

    /// Returns the substitution as the `(params, args)` slice pair [`Ty::substitute`] expects.
    pub fn as_slices(&self) -> (&[Identifier], &[Ty]) {
        (&self.params, &self.args)
    }
}

/// A context for specializing polymorphic declarations into monomorphic ones.
///
/// `table` is a reference to the naming table that maps polymorphic type parameters to their
/// corresponding concrete types.
/// `subst` carries the type parameters and their corresponding concrete types for the current
/// specialization context.
#[derive(Clone)]
pub struct SpecializeContext<'a> {
    pub table: &'a NamingTable,
    pub subst: Substitution,
    pub erased_decls: &'a ErasedDecls,
}

impl<'a> SpecializeContext<'a> {
    /// A context for specializing already-ground terms, with no active variable substitution.
    pub fn ground(table: &'a NamingTable, erased_decls: &'a ErasedDecls) -> Self {
        SpecializeContext {
            table,
            subst: Substitution::empty(),
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
        SpecializeContext {
            table,
            subst: Substitution::new(params.to_vec(), args.to_vec()),
            erased_decls,
        }
    }

    /// Extends the current specialization context with additional type parameters and their
    /// corresponding concrete types, returning a new `SpecializeContext` that combines the
    /// existing substitution with the new one.
    fn extend_with_substs(&self, new_params: &[Identifier], new_args: &[Ty]) -> Self {
        SpecializeContext {
            table: self.table,
            subst: self.subst.extended(new_params, new_args),
            erased_decls: self.erased_decls,
        }
    }
}

/// A trait for types that can be specialized from polymorphic to monomorphic forms.
pub trait Specialize {
    /// Specializes the current instance using the provided specialization context, returning a
    /// new instance with all polymorphic type parameters replaced by their corresponding
    /// concrete types.
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

/// Erases `ty` and then grounds it under the context's substitution.
pub fn erase_and_substitute(ty: &Ty, ctx: &SpecializeContext) -> Ty {
    erase_ty(ty, ctx.erased_decls).substitute(ctx.subst.as_slices())
}

/// Recovers the arguments an erased declaration's erased type parameters were instantiated
/// with, from a concrete occurrence's type, in erased form. Returns `None` if `ty` is not a
/// declaration type, or if it is but none of its parameters was erased.
pub fn recover_extra_args(ty: &Ty, ctx: &SpecializeContext) -> Option<Vec<Ty>> {
    let Ty::Decl { name, type_args } = ty else {
        return None;
    };
    if !ctx.erased_decls.is_erased(name) {
        return None;
    }
    Some(
        ctx.erased_decls
            .erased_args(name, &type_args.args)
            .iter()
            .map(|a| erase_and_substitute(a, ctx))
            .collect(),
    )
}

/// What specializing a `XCase` knows about its scrutinee when the scrutinee's type is a
/// declaration with erased type parameters.
pub struct ErasedScrutinee {
    /// The erased declaration the scrutinee belongs to.
    pub decl: Identifier,
    /// The instantiation of the declaration's kept parameters, selecting the copy of the
    /// declaration the scrutinee has, and with it the xtor variants its clauses must cover.
    pub kept_args: Vec<Ty>,
    /// The instantiation of the declaration's erased parameters, if it can be read off the
    /// scrutinee's type (see [`recover_extra_args`]); used to mark the clauses of every other
    /// variant unreachable.
    pub erased_args: Option<Vec<Ty>>,
}

impl ErasedScrutinee {
    /// Inspects the scrutinee type `ty` of a `XCase`. Returns `None` if it is not a declaration
    /// with erased type parameters. The kept arguments are always available, even if `ty` is a
    /// bare type variable, since the value it is substituted with is already in erased form and
    /// thus carries exactly the kept arguments.
    pub fn of(ty: &Ty, ctx: &SpecializeContext) -> Option<Self> {
        let Ty::Decl { name, type_args } = erase_and_substitute(ty, ctx) else {
            return None;
        };
        if !ctx.erased_decls.is_erased(&name) {
            return None;
        }
        Some(ErasedScrutinee {
            decl: name,
            kept_args: type_args.args,
            erased_args: recover_extra_args(ty, ctx),
        })
    }
}

/// This function is the entry point for specializing a program from polymorphic to monomorphic
/// form. It takes a reference to a [`Solution`] produced by the constraint solving process, and
/// returns a new program where all polymorphic type parameters have been replaced with their
/// corresponding concrete types according to the solution.
///
/// Fails with [`MonoError::NameCollision`] if two distinct declarations, xtors, or defs would
/// mangle to the same monomorphic name (see [`NamingTable::build`]).
pub fn specialize_program(
    prog: &Prog,
    solution: &Solution,
    erased_decls: &ErasedDecls,
) -> Result<Prog, crate::mono::errors::MonoError> {
    let table = NamingTable::build(
        solution,
        &prog.data_types,
        &prog.codata_types,
        &prog.defs,
        erased_decls,
    )?;

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

    Ok(Prog {
        defs,
        data_types,
        codata_types,
        max_id: prog.max_id,
    })
}

/// Specialization of polymorphic type declarations into monomorphic ones
fn specialize_declaration<P: Polarity + Clone>(
    decl: &TypeDeclaration<P>,
    table: &NamingTable,
    erased_decls: &ErasedDecls,
) -> Vec<TypeDeclaration<P>> {
    let params: Vec<Identifier> = TypeParam::names(&decl.type_params);

    if erased_decls.is_erased(&decl.name) {
        return specialize_erased_declaration(decl, &params, table, erased_decls);
    }

    if params.is_empty() {
        if decl.xtors.iter().all(|xtor| xtor.type_params.is_empty()) {
            // This is already a monomorphic declaration, so we can return it as-is
            return vec![decl.clone()];
        }

        let ctx = SpecializeContext::ground(table, erased_decls);

        // This is already a monomorphic declaration, so we only need to specialize its xtors.
        return vec![TypeDeclaration {
            dat: decl.dat.clone(),
            name: decl.name.clone(),
            xtors: decl
                .xtors
                .iter()
                .flat_map(|xtor| specialize_xtor_sig(xtor, &[], None, &ctx))
                .collect(),
            type_params: vec![],
        }];
    }

    table
        .instantiations_for(&decl.name)
        .iter()
        .map(|tuple| {
            let ctx = SpecializeContext::with_subst(table, &params, tuple, erased_decls);
            TypeDeclaration {
                dat: decl.dat.clone(),
                name: table.lookup(&decl.name, tuple).clone(),
                xtors: decl
                    .xtors
                    .iter()
                    .flat_map(|xtor| specialize_xtor_sig(xtor, &[], None, &ctx))
                    .collect(),
                type_params: vec![],
            }
        })
        .collect()
}

/// Specialization of a type declaration with erased type parameters: one copy per
/// instantiation of its kept parameters (a single, unmangled copy if every parameter is
/// erased), each containing one variant of every xtor per erased instantiation that occurs
/// together with that copy's kept instantiation in the solution.
fn specialize_erased_declaration<P: Polarity + Clone>(
    decl: &TypeDeclaration<P>,
    params: &[Identifier],
    table: &NamingTable,
    erased_decls: &ErasedDecls,
) -> Vec<TypeDeclaration<P>> {
    let kept_params = erased_decls.kept_args(&decl.name, params);
    let erased_params = erased_decls.erased_args(&decl.name, params);

    table
        .instantiations_for(&decl.name)
        .iter()
        .map(|kept| {
            let ctx = SpecializeContext::with_subst(table, &kept_params, kept, erased_decls);
            TypeDeclaration {
                dat: decl.dat.clone(),
                name: table.lookup(&decl.name, kept).clone(),
                xtors: decl
                    .xtors
                    .iter()
                    .flat_map(|xtor| {
                        specialize_xtor_sig(xtor, &erased_params, Some((&decl.name, kept)), &ctx)
                    })
                    .collect(),
                type_params: vec![],
            }
        })
        .collect()
}

/// Specialization of polymorphic constructor/destructor signatures into monomorphic ones.
///
/// `extra_params` are the erased type parameters of the surrounding declaration, if any. In that
/// case `erased_copy` names the declaration and the kept instantiation of the copy being built,
/// and only the variants whose erased instantiation belongs to that copy are generated.
fn specialize_xtor_sig<P: Polarity + Clone>(
    xtor_sig: &XtorSig<P>,
    extra_params: &[Identifier],
    erased_copy: Option<(&Identifier, &[Ty])>,
    ctx: &SpecializeContext,
) -> Vec<XtorSig<P>> {
    let own_len = xtor_sig.type_params.len();
    let mut params: Vec<Identifier> = TypeParam::names(&xtor_sig.type_params);
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
        .filter(|tuple| match erased_copy {
            Some((decl, kept)) => ctx.table.has_erased_variant(decl, kept, &tuple[own_len..]),
            None => true,
        })
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
///
/// If the scrutinee belongs to a declaration with erased type parameters, `scrutinee` describes
/// it: only the variants of the scrutinee's own copy of the declaration get a clause, and among
/// those, every variant whose erased instantiation differs from the scrutinee's is unreachable.
pub fn specialize_clause<C: Chi>(
    clause: &Clause<C>,
    ctx: &SpecializeContext,
    scrutinee: Option<&ErasedScrutinee>,
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
        // variants of every other copy of the declaration do not belong in this case at all
        .filter(|tuple| match scrutinee {
            Some(s) => ctx.table.has_erased_variant(
                &s.decl,
                &s.kept_args,
                &tuple[clause.type_params.len()..],
            ),
            None => true,
        })
        .map(|tuple| {
            let (_own_args, extra_args) = tuple.split_at(clause.type_params.len());
            let extended_ctx = ctx.extend_with_substs(&full_params, tuple);
            let xtor_name = extended_ctx.table.lookup(&clause.xtor, tuple).clone();

            let context = clause.context.specialize(&extended_ctx);

            let reachable = match scrutinee.and_then(|s| s.erased_args.as_deref()) {
                Some(active) => extra_args == active,
                None => true,
            };

            let body = if reachable {
                clause.body.specialize(&extended_ctx)
            } else {
                Rc::new(
                    Unreachable {
                        ty: clause.body.get_type().specialize(&extended_ctx),
                    }
                    .into(),
                )
            };

            Clause {
                prdcns: clause.prdcns.clone(),
                xtor: xtor_name,
                type_params: vec![],
                context,
                body,
            }
        })
        .collect()
}

/// Specialization of polymorphic function definitions into monomorphic ones
pub fn specialize_def(def: &Def, table: &NamingTable, erased_decls: &ErasedDecls) -> Vec<Def> {
    let params: Vec<Identifier> = TypeParam::names(&def.type_params);

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
            let ctx = SpecializeContext::with_subst(table, &params, tuple, erased_decls);
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
        lit, prd, tparam, tvar, ty, var,
    };

    fn list_decl() -> DataDeclaration {
        data!(
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
            [tparam!(id!("A", 1), "+")]
        )
    }

    fn bool_decl() -> DataDeclaration {
        data!(
            id!("Bool"),
            [
                ctor_sig!(id!("True"), [], []),
                ctor_sig!(id!("False"), [], [])
            ],
            []
        )
    }

    fn pair_decl() -> DataDeclaration {
        data!(
            id!("Pair"),
            [ctor_sig!(
                id!("mkPair"),
                [],
                [
                    bind!(id!("x"), prd!(), tvar!(id!("A", 2))),
                    bind!(id!("y"), prd!(), tvar!(id!("B", 3)))
                ]
            )],
            [tparam!(id!("A", 2), "+"), tparam!(id!("B", 3), "+")]
        )
    }

    fn identity_def() -> Def {
        def!(
            id!("identity"),
            [tparam!(id!("A", 1), "+")],
            [
                bind!(id!("x"), prd!(), tvar!(id!("A", 1))),
                bind!(id!("ret"), cns!(), tvar!(id!("A", 1)))
            ],
            cut!(
                var!(id!("x"), tvar!(id!("A", 1))),
                covar!(id!("ret"), tvar!(id!("A", 1))),
                tvar!(id!("A", 1))
            )
        )
    }

    fn box_decl() -> DataDeclaration {
        data!(
            id!("Box"),
            [ctor_sig!(
                id!("Pack"),
                [tparam!(id!("E", 2), "+")],
                [bind!(id!("x"), prd!(), tvar!(id!("E", 2)))]
            )],
            []
        )
    }

    fn container_decl() -> CodataDeclaration {
        codata!(
            id!("Container"),
            [dtor_sig!(
                id!("wrap"),
                [tparam!(id!("S", 2), "+")],
                [
                    bind!(id!("x"), prd!(), tvar!(id!("S", 2))),
                    bind!(id!("tag"), prd!(), tvar!(id!("T", 1)))
                ]
            )],
            [tparam!(id!("T", 1), "+")]
        )
    }

    fn nil_clause() -> Clause<Cns> {
        clause!(Cns, id!("Nil"), [], [], exit!(lit!(0)))
    }

    fn pack_clause() -> Clause<Cns> {
        clause!(
            Cns,
            id!("Pack"),
            [id!("G", 4)],
            [bind!(id!("x"), prd!(), tvar!(id!("G", 4)))],
            exit!(lit!(0))
        )
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
        )
        .expect("test fixture must not collide");
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
        )
        .expect("test fixture must not collide");
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
            NamingTable::build(&solution, &[list_decl()], &[], &[], &ErasedDecls::default())
                .expect("test fixture must not collide");
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
        )
        .expect("test fixture must not collide");

        let pair_copies = specialize_declaration(&pair_decl(), &table, &ErasedDecls::default());
        let list_copies = specialize_declaration(&list_decl(), &table, &ErasedDecls::default());

        assert_eq!(pair_copies.len(), 1);
        assert_eq!(list_copies.len(), 1);

        // The List copy's Cons.xs field must reference the *same* mangled
        // List name as the declaration's own specialized name, and its x
        // field must reference the *same* mangled Pair name produced for
        // the Pair declaration above -- consistency across two independent
        // top-level specializations.
        let list_name = table
            .lookup(&list_decl().name, std::slice::from_ref(&pair_ty))
            .clone();
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

        let specialized_prog = specialize_program(&prog, &solution, &ErasedDecls::default())
            .expect("test fixture must not collide");

        assert_eq!(
            specialized_prog.data_types.len(),
            1,
            "Expected exactly one specialized copy"
        );
        assert_eq!(specialized_prog.codata_types.len(), 0);

        let table =
            NamingTable::build(&solution, &[list_decl()], &[], &[], &ErasedDecls::default())
                .expect("test fixture must not collide");
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
            std::slice::from_ref(&main_def),
            &ErasedDecls::default(),
        )
        .expect("test fixture must not collide");
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
        )
        .expect("test fixture must not collide");
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
            [tparam!(id!("A", 1), "+")],
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
            std::slice::from_ref(&unused),
            &ErasedDecls::default(),
        )
        .expect("test fixture must not collide");
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
            [tparam!(id!("A", 1), "+")],
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
            std::slice::from_ref(&singleton),
            &ErasedDecls::default(),
        )
        .expect("test fixture must not collide");

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
            [tparam!(id!("A", 1), "+"), tparam!(id!("B", 2), "+")],
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
            std::slice::from_ref(&swap),
            &ErasedDecls::default(),
        )
        .expect("test fixture must not collide");
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

        let result = specialize_program(&prog, &solution, &ErasedDecls::default())
            .expect("test fixture must not collide");

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
            [tparam!(id!("A", 1), "+")],
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
        )
        .expect("test fixture must not collide");

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
        )
        .expect("test fixture must not collide");
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
        )
        .expect("test fixture must not collide");
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
            NamingTable::build(&solution, &[list_decl()], &[], &[], &ErasedDecls::default())
                .expect("test fixture must not collide");
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
        )
        .expect("test fixture must not collide");
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
        syntax::{Cns, DataDeclaration, Statement, types::TypeArgs},
    };
    use std::collections::{HashMap, HashSet};
    extern crate self as core_lang;
    use core_macros::{
        bind, call, clause, covar, ctor, ctor_sig, cut, data, def, id, lit, prd, tparam, tvar, ty,
        var,
    };

    fn box_decl() -> DataDeclaration {
        data!(
            id!("Box"),
            [ctor_sig!(
                id!("Wrap"),
                [],
                [bind!(id!("x"), prd!(), tvar!(id!("A", 1)))]
            )],
            [tparam!(id!("A", 1), "+")]
        )
    }

    #[test]
    fn specialize_erased_declaration_keeps_one_copy_with_two_xtor_variants() {
        let erased = ErasedDecls::from(HashSet::from([(id!("Box"), 0)]));
        let solution = Solution::from(HashMap::from([(
            vec![id!("A", 1)],
            HashSet::from([vec![ty!("int")], vec![ty!(id!("Box"))]]),
        )]));

        let table = NamingTable::build(&solution, &[box_decl()], &[], &[], &erased)
            .expect("test fixture must not collide");
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
        let erased = ErasedDecls::from(HashSet::from([(id!("Box"), 0)]));
        let solution = Solution::from(HashMap::from([(
            vec![id!("A", 1)],
            HashSet::from([vec![ty!("int")]]),
        )]));
        let table = NamingTable::build(&solution, &[box_decl()], &[], &[], &erased)
            .expect("test fixture must not collide");
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

    fn wrap_clause() -> Clause<Cns> {
        clause!(
            Cns,
            id!("Wrap"),
            [],
            [bind!(id!("x"), prd!(), tvar!(id!("A", 1)))],
            cut!(
                var!(id!("x"), tvar!(id!("A", 1))),
                covar!(id!("ret"), ty!("int")),
                ty!("int")
            )
        )
    }

    #[test]
    fn specialize_clause_marks_the_non_matching_erased_instantiation_unreachable() {
        let erased = ErasedDecls::from(HashSet::from([(id!("Box"), 0)]));
        let solution = Solution::from(HashMap::from([(
            vec![id!("A", 1)],
            HashSet::from([vec![ty!("int")], vec![ty!(id!("Box"))]]),
        )]));
        let table = NamingTable::build(&solution, &[box_decl()], &[], &[], &erased)
            .expect("test fixture must not collide");
        let ctx = SpecializeContext::ground(&table, &erased);

        // mirrors a scrutinee whose own active instantiation of the erased Box is `int`
        let scrutinee = ErasedScrutinee {
            decl: id!("Box"),
            kept_args: vec![],
            erased_args: Some(vec![ty!("int")]),
        };
        let copies = specialize_clause(&wrap_clause(), &ctx, Some(&scrutinee));
        assert_eq!(copies.len(), 2);

        let reachable_name = table.lookup(&id!("Wrap"), &[ty!("int")]);
        let unreachable_name = table.lookup(&id!("Wrap"), &[ty!(id!("Box"))]);

        let reachable_clause = copies
            .iter()
            .find(|c| &c.xtor == reachable_name)
            .expect("expected a clause for the reachable instantiation");
        let unreachable_clause = copies
            .iter()
            .find(|c| &c.xtor == unreachable_name)
            .expect("expected a clause for the unreachable instantiation");

        assert!(
            !matches!(reachable_clause.body.as_ref(), Statement::Unreachable(_)),
            "expected the matching instantiation to keep its real body, got {:?}",
            reachable_clause.body
        );
        assert!(
            matches!(unreachable_clause.body.as_ref(), Statement::Unreachable(_)),
            "expected the non-matching instantiation's body to become Unreachable, got {:?}",
            unreachable_clause.body
        );
    }

    #[test]
    fn specialize_clause_keeps_every_instantiation_reachable_without_a_scrutinee_type() {
        let erased = ErasedDecls::from(HashSet::from([(id!("Box"), 0)]));
        let solution = Solution::from(HashMap::from([(
            vec![id!("A", 1)],
            HashSet::from([vec![ty!("int")], vec![ty!(id!("Box"))]]),
        )]));
        let table = NamingTable::build(&solution, &[box_decl()], &[], &[], &erased)
            .expect("test fixture must not collide");
        let ctx = SpecializeContext::ground(&table, &erased);

        let copies = specialize_clause(&wrap_clause(), &ctx, None);

        assert_eq!(copies.len(), 2);
        assert!(
            copies
                .iter()
                .all(|c| !matches!(c.body.as_ref(), Statement::Unreachable(_))),
            "expected every instantiation to keep its real body when no active scrutinee \
             instantiation is known, got {:#?}",
            copies
        );
    }

    #[test]
    fn call_specialize_erases_nested_recursive_type_argument() {
        // nest[Box[C]](...) while specializing nest's own C := Box: after substitution the
        // call's type argument is Box[Box], which must be erased to bare Box before lookup.
        let erased = ErasedDecls::from(HashSet::from([(id!("Box"), 0)]));
        let mut solution_map = HashMap::new();
        solution_map.insert(
            vec![id!("C", 1)],
            HashSet::from([vec![ty!("int")], vec![ty!(id!("Box"))]]),
        );
        let solution = Solution::from(solution_map);

        let nest_def = def!(
            id!("nest"),
            [tparam!(id!("C", 1), "+")],
            [bind!(id!("x"), prd!(), tvar!(id!("C", 1)))],
            cut!(
                var!(id!("x"), tvar!(id!("C", 1))),
                covar!(id!("ret"), tvar!(id!("C", 1))),
                tvar!(id!("C", 1))
            )
        );

        let table = NamingTable::build(
            &solution,
            &[box_decl()],
            &[],
            std::slice::from_ref(&nest_def),
            &erased,
        )
        .expect("test fixture must not collide");

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

    /// `data Tag[V+, W+] { MkTag(val: V, label: W) }`
    fn tag_decl() -> DataDeclaration {
        data!(
            id!("Tag"),
            [ctor_sig!(
                id!("MkTag"),
                [],
                [
                    bind!(id!("val"), prd!(), tvar!(id!("V", 1))),
                    bind!(id!("label"), prd!(), tvar!(id!("W", 2)))
                ]
            )],
            [tparam!(id!("V", 1), "+"), tparam!(id!("W", 2), "+")]
        )
    }

    /// [`tag_decl`] plus a monomorphic `Bool` its instantiations refer to.
    fn tag_decls() -> Vec<DataDeclaration> {
        vec![tag_decl(), data!(id!("Bool"), [], [])]
    }

    /// `Tag` with only `V` erased, and a solution in which `V` grew once (`Tag[i64]`, already
    /// in erased form) together with `W = i64`, while `W = Bool` only occurs with `V = i64`.
    fn partially_erased_tag() -> (ErasedDecls, Solution) {
        let erased = ErasedDecls::from(HashSet::from([(id!("Tag"), 0)]));
        let solution = Solution::from(HashMap::from([(
            vec![id!("V", 1), id!("W", 2)],
            HashSet::from([
                vec![ty!("int"), ty!("int")],
                vec![ty!(id!("Tag"), [ty!("int")]), ty!("int")],
                vec![ty!("int"), ty!(id!("Bool"))],
            ]),
        )]));
        (erased, solution)
    }

    #[test]
    fn partially_erased_declaration_gets_one_copy_per_kept_instantiation() {
        let (erased, solution) = partially_erased_tag();
        let table = NamingTable::build(&solution, &tag_decls(), &[], &[], &erased)
            .expect("test fixture must not collide");
        let copies = specialize_declaration(&tag_decl(), &table, &erased);

        let variants_of = |copy: &str| -> Vec<Vec<Ty>> {
            let copy = copies
                .iter()
                .find(|d| d.name.name == copy)
                .unwrap_or_else(|| panic!("expected a copy named {copy}"));
            copy.xtors
                .iter()
                .map(|x| x.args.bindings.iter().map(|b| b.ty.clone()).collect())
                .collect()
        };

        assert_eq!(copies.len(), 2, "one copy per instantiation of the kept W");

        // Tag[i64] holds both variants of the erased V that occur with W = i64; the grown V is
        // the copy Tag[i64] itself, referenced by its mangled name without being erased twice
        let mut int_copy = variants_of("Tag[i64]");
        int_copy.sort();
        assert_eq!(
            int_copy,
            vec![vec![Ty::I64, Ty::I64], vec![ty!(id!("Tag[i64]")), Ty::I64],]
        );

        // Tag[Bool] only holds the variant its solution tuple correlates it with
        assert_eq!(
            variants_of("Tag[Bool]"),
            vec![vec![Ty::I64, ty!(id!("Bool"))]]
        );
    }

    #[test]
    fn specialize_clause_only_covers_the_scrutinees_own_copy() {
        let (erased, solution) = partially_erased_tag();
        let table = NamingTable::build(&solution, &tag_decls(), &[], &[], &erased)
            .expect("test fixture must not collide");
        let ctx = SpecializeContext::ground(&table, &erased);

        let clause = clause!(
            Cns,
            id!("MkTag"),
            [],
            [
                bind!(id!("val"), prd!(), ty!("int")),
                bind!(id!("label"), prd!(), ty!(id!("Bool")))
            ],
            cut!(lit!(0), covar!(id!("ret"), ty!("int")), ty!("int"))
        );
        let scrutinee = ErasedScrutinee {
            decl: id!("Tag"),
            kept_args: vec![ty!(id!("Bool"))],
            erased_args: Some(vec![ty!("int")]),
        };

        let copies = specialize_clause(&clause, &ctx, Some(&scrutinee));

        // MkTag[Tag[i64]] exists, but only in the copy Tag[i64], so it gets no clause here
        assert_eq!(copies.len(), 1);
        assert_eq!(&copies[0].xtor, table.lookup(&id!("MkTag"), &[ty!("int")]));
        assert!(!matches!(
            copies[0].body.as_ref(),
            Statement::Unreachable(_)
        ));
    }

    #[test]
    fn scrutinee_kept_args_are_recovered_from_a_type_variable() {
        // case on x: C with C := Tag[Bool] (erased form): the copy is known even though the
        // erased argument itself is not
        let (erased, solution) = partially_erased_tag();
        let table = NamingTable::build(&solution, &tag_decls(), &[], &[], &erased)
            .expect("test fixture must not collide");
        let params = vec![id!("C", 5)];
        let args = vec![ty!(id!("Tag"), [ty!(id!("Bool"))])];
        let ctx = SpecializeContext::with_subst(&table, &params, &args, &erased);

        let scrutinee =
            ErasedScrutinee::of(&tvar!(id!("C", 5)), &ctx).expect("Tag has an erased parameter");
        assert_eq!(scrutinee.decl, id!("Tag"));
        assert_eq!(scrutinee.kept_args, vec![ty!(id!("Bool"))]);
        assert_eq!(scrutinee.erased_args, None);
    }
}
