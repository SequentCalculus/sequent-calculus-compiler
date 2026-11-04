//! Builds the table mapping every polymorphic declaration, xtor, and def, together with its
//! concrete instantiation, to its mangled monomorphic name.

use std::collections::{BTreeMap, BTreeSet, HashMap};

use crate::{
    mono::{erasure::ErasedDecls, errors::MonoError, solver::Solution},
    syntax::{
        CodataDeclaration, DataDeclaration, Def, Identifier, Ty, TypeParam,
        declaration::{Polarity, TypeDeclaration},
    },
};

/// A mapping from a polymorphic declaration, xtor, or def, paired with one concrete
/// instantiation of its type parameters, to the mangled [`Identifier`] its monomorphic copy is
/// given.
///
/// This is the single source of truth for specialization: once built, it is
/// the only structure the specialization passes need. The [`Solution`] used
/// to build it is not required afterwards.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamingTable {
    names: BTreeMap<(Identifier, Vec<Ty>), Identifier>,
    xtor_extra_params: HashMap<Identifier, Vec<Identifier>>,
    /// For every copy of a declaration with erased type parameters, keyed by the declaration's
    /// name and the instantiation of its kept parameters, the instantiations of its erased
    /// parameters that occur together with it in the solution. These are exactly the xtor
    /// variants that copy contains.
    erased_variants: BTreeMap<(Identifier, Vec<Ty>), BTreeSet<Vec<Ty>>>,
    /// Tracks which (name, instantiation) pair first claimed each mangled name, so a second,
    /// distinct pair mangling to the same name is reported as a [`MonoError::NameCollision`]
    /// instead of silently overwriting the first one.
    mangled_owners: HashMap<Identifier, (Identifier, Vec<Ty>)>,
}

impl NamingTable {
    /// Builds the naming table from the solver's output.
    ///
    /// For each node and each ground vector in its solution, generates a
    /// fresh, deterministically mangled identifier derived from the concrete types,
    /// e.g. `Pair[A,B]` instantiated with `[i64, Bool]` becomes `Pair[i64, Bool]`.
    ///
    /// Fails with [`MonoError::NameCollision`] if two distinct declarations, xtors, or defs
    /// mangle to the same monomorphic name.
    pub fn build(
        solution: &Solution,
        data_decls: &[DataDeclaration],
        codata_decls: &[CodataDeclaration],
        defs: &[Def],
        erased_decls: &ErasedDecls,
    ) -> Result<Self, MonoError> {
        let mut table = NamingTable {
            names: BTreeMap::new(),
            xtor_extra_params: HashMap::new(),
            erased_variants: BTreeMap::new(),
            mangled_owners: HashMap::new(),
        };

        for decl in data_decls {
            table.register_decl(decl, solution, erased_decls, mangle_ty_declaration)?;
        }

        for decl in codata_decls {
            table.register_decl(decl, solution, erased_decls, mangle_ty_declaration)?;
        }

        for def in defs {
            let def_type_param_ids: Vec<Identifier> = TypeParam::names(&def.type_params);
            table.register(
                &def.name,
                &def_type_param_ids,
                solution,
                mangle_def_declaration,
            )?;
        }

        Ok(table)
    }

    /// Registers a single data or codata declaration and its xtors.
    ///
    /// If some of the declaration's type parameters are erased (see [`ErasedDecls`]), the
    /// declaration is registered once per instantiation of its *kept* parameters only, e.g.
    /// `Pair[i64]` for `Pair[A, B]` with `A` erased, or just `Box` if every parameter is erased.
    /// Each of its xtors is registered under the combination of its own type parameters plus
    /// the declaration's *erased* parameters, exactly as if the latter had been declared on the
    /// xtor itself. Which erased instantiations belong to which copy is recorded in
    /// `erased_variants`, since the solver tracks all of the declaration's parameters as one
    /// correlated node. Otherwise, behavior matches the ordinary (non-erased) path.
    fn register_decl<P: Polarity>(
        &mut self,
        decl: &TypeDeclaration<P>,
        solution: &Solution,
        erased_decls: &ErasedDecls,
        mangle: fn(&Identifier, &[Ty]) -> String,
    ) -> Result<(), MonoError> {
        let decl_type_param_ids: Vec<Identifier> = TypeParam::names(&decl.type_params);
        if !erased_decls.is_erased(&decl.name) {
            self.register(&decl.name, &decl_type_param_ids, solution, mangle)?;
            for xtor in &decl.xtors {
                let xtor_type_param_ids: Vec<Identifier> = TypeParam::names(&xtor.type_params);
                self.register(&xtor.name, &xtor_type_param_ids, solution, mangle)?;
            }
            return Ok(());
        }

        let erased_param_ids = erased_decls.erased_args(&decl.name, &decl_type_param_ids);

        // Split every correlated solution tuple of the declaration into its kept part (which
        // selects the copy) and its erased part (which selects the xtor variant in that copy).
        let mut variants: BTreeMap<Vec<Ty>, BTreeSet<Vec<Ty>>> = BTreeMap::new();
        if erased_param_ids.len() == decl_type_param_ids.len() {
            // A fully erased declaration always keeps its single, unmangled copy.
            variants.entry(vec![]).or_default();
        }
        for tuple in solution.get(&decl_type_param_ids).into_iter().flatten() {
            variants
                .entry(erased_decls.kept_args(&decl.name, tuple))
                .or_default()
                .insert(erased_decls.erased_args(&decl.name, tuple));
        }

        let all_erased_tuples: BTreeSet<Vec<Ty>> = variants.values().flatten().cloned().collect();
        for (kept, erased_tuples) in variants {
            let mangled = Identifier::new(mangle(&decl.name, &kept));
            self.insert_name((decl.name.clone(), kept.clone()), mangled)?;
            self.erased_variants
                .insert((decl.name.clone(), kept), erased_tuples);
        }

        for xtor in &decl.xtors {
            self.xtor_extra_params
                .insert(xtor.name.clone(), erased_param_ids.clone());

            // The erased parameters are not a solver node of their own, so their instantiations
            // come from the projections collected above, combined with every instantiation of
            // the xtor's own parameters.
            let xtor_type_param_ids: Vec<Identifier> = TypeParam::names(&xtor.type_params);
            let own_tuples: Vec<Vec<Ty>> = if xtor_type_param_ids.is_empty() {
                vec![vec![]]
            } else {
                solution
                    .get(&xtor_type_param_ids)
                    .into_iter()
                    .flatten()
                    .cloned()
                    .collect()
            };
            for own in &own_tuples {
                for erased in &all_erased_tuples {
                    let mut combined = own.clone();
                    combined.extend(erased.iter().cloned());
                    let mangled = Identifier::new(mangle(&xtor.name, &combined));
                    self.insert_name((xtor.name.clone(), combined), mangled)?;
                }
            }
        }
        Ok(())
    }

    /// Registers a single declaration (data, codata, or def) and its type parameters.
    ///
    /// If the declaration has no type parameters, it is registered under its own name.
    /// Otherwise, for each ground instantiation tuple in the solution, a fresh mangled name
    /// is generated and registered under that name.
    fn register(
        &mut self,
        name: &Identifier,
        type_params: &[Identifier],
        solution: &Solution,
        mangle: fn(&Identifier, &[Ty]) -> String,
    ) -> Result<(), MonoError> {
        if type_params.is_empty() {
            return self.insert_name((name.clone(), vec![]), name.clone());
        }

        for tuple in solution.get(type_params).into_iter().flatten() {
            let mangled = Identifier::new(mangle(name, tuple));
            self.insert_name((name.clone(), tuple.clone()), mangled)?;
        }
        Ok(())
    }

    /// Registers `key` (a declaration/xtor/def name paired with its concrete instantiation) as
    /// mangling to `mangled`. This is the single choke point every insertion into `names` goes
    /// through, so every mangled name gets checked for collisions regardless of which caller
    /// produced it.
    ///
    /// Returns [`MonoError::NameCollision`] if some other, distinct key already claimed the same
    /// mangled name.
    fn insert_name(
        &mut self,
        key: (Identifier, Vec<Ty>),
        mangled: Identifier,
    ) -> Result<(), MonoError> {
        if let Some(previous) = self.mangled_owners.get(&mangled)
            && previous != &key
        {
            return Err(MonoError::NameCollision {
                mangled: mangled.name.clone(),
                first: describe_owner(&previous.0, &previous.1),
                second: describe_owner(&key.0, &key.1),
            });
        }
        self.mangled_owners.insert(mangled.clone(), key.clone());
        self.names.insert(key, mangled);
        Ok(())
    }

    /// Looks up the mangled name for a given type and its instantiation.
    pub fn lookup(&self, name: &Identifier, tuple: &[Ty]) -> &Identifier {
        self.names
            .get(&(name.clone(), tuple.to_vec()))
            .unwrap_or_else(|| {
                panic!(
                    "no specialized name recorded for {} with instantiation {:?} -- \
                     this indicates a bug in constraint collection or solving",
                    name.name, tuple
                )
            })
    }

    /// Returns all concrete type instantiation tuples recorded for a given identifier, in
    /// canonical order.
    pub fn instantiations_for(&self, name: &Identifier) -> Vec<Vec<Ty>> {
        self.names
            .keys()
            .filter(|(id, _)| id == name)
            .map(|(_, tuple)| tuple.clone())
            .collect()
    }

    /// Returns the extra type parameters pushed down to this xtor from its declaration.
    /// Returns an empty slice if the xtor belongs to a non-erased declaration.
    pub fn extra_params_for(&self, xtor: &Identifier) -> &[Identifier] {
        self.xtor_extra_params
            .get(xtor)
            .map(|params| params.as_slice())
            .unwrap_or(&[])
    }

    /// True iff the copy of the erased declaration `decl` for the kept instantiation `kept`
    /// contains the xtor variants for the erased instantiation `erased`, i.e. iff both occur
    /// together in one solution tuple of the declaration.
    pub fn has_erased_variant(&self, decl: &Identifier, kept: &[Ty], erased: &[Ty]) -> bool {
        self.erased_variants
            .get(&(decl.clone(), kept.to_vec()))
            .is_some_and(|tuples| tuples.contains(erased))
    }
}

/// Renders one side of a [`MonoError::NameCollision`]: the source name paired with its concrete
/// instantiation, for a human-readable error message.
fn describe_owner(name: &Identifier, tuple: &[Ty]) -> String {
    if tuple.is_empty() {
        name.name.clone()
    } else {
        format!("{}{:?}", name.name, tuple)
    }
}

/// Generates a mangled name for a type declaration given its base name and the concrete types it is instantiated with.
fn mangle_ty_declaration(base_name: &Identifier, tuple: &[Ty]) -> String {
    if tuple.is_empty() {
        base_name.name.clone()
    } else {
        let args: Vec<String> = tuple.iter().map(mangle_ty).collect();
        format!("{}[{}]", base_name.name, args.join(", "))
    }
}

/// Generates a mangled name for a function declaration given its base name and the concrete types it is instantiated with.
fn mangle_def_declaration(base_name: &Identifier, tuple: &[Ty]) -> String {
    if tuple.is_empty() {
        base_name.name.clone()
    } else {
        let args: Vec<String> = tuple.iter().map(mangle_ty).collect();
        format!("{}_{}", base_name.name, args.join("_"))
    }
}

/// Generates a mangled name for a type given its concrete instantiation.
fn mangle_ty(ty: &Ty) -> String {
    match ty {
        Ty::I64 => "i64".to_owned(),
        Ty::Decl { name, type_args } => {
            if type_args.args.is_empty() {
                name.name.clone()
            } else {
                let inner: Vec<String> = type_args.args.iter().map(mangle_ty).collect();
                format!("{}[{}]", name.name, inner.join(", "))
            }
        }
        Ty::Var(_) => unreachable!("mangle_ty called on a non-ground type"),
    }
}

#[cfg(test)]
mod erasure_tests {
    use super::*;
    use crate::mono::erasure::ErasedDecls;
    use std::collections::{HashMap, HashSet};
    extern crate self as core_lang;
    use core_macros::{bind, ctor_sig, data, id, prd, tparam, tvar, ty};

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
    fn erased_decl_keeps_single_unmangled_name() {
        let erased = ErasedDecls::from(HashSet::from([(id!("Box"), 0)]));
        let solution = Solution::from(HashMap::from([(
            vec![id!("A", 1)],
            HashSet::from([vec![ty!("int")], vec![ty!(id!("Box"))]]),
        )]));

        let table = NamingTable::build(&solution, &[box_decl()], &[], &[], &erased)
            .expect("test fixture must not collide");

        // Box itself must remain registered under its own, unchanged name.
        assert_eq!(table.lookup(&id!("Box"), &[]), &id!("Box"));
    }

    #[test]
    fn erased_decl_registers_two_xtor_instantiations() {
        let erased = ErasedDecls::from(HashSet::from([(id!("Box"), 0)]));
        let solution = Solution::from(HashMap::from([(
            vec![id!("A", 1)],
            HashSet::from([vec![ty!("int")], vec![ty!(id!("Box"))]]),
        )]));

        let table = NamingTable::build(&solution, &[box_decl()], &[], &[], &erased)
            .expect("test fixture must not collide");

        let mut tuples = table.instantiations_for(&id!("Wrap"));
        tuples.sort();
        assert_eq!(tuples, vec![vec![ty!("int")], vec![ty!(id!("Box"))]]);

        let name_int = table.lookup(&id!("Wrap"), &[ty!("int")]);
        let name_box = table.lookup(&id!("Wrap"), &[ty!(id!("Box"))]);
        assert_ne!(name_int, name_box);
    }

    /// With several distinct names in the table, `instantiations_for` must return exactly the
    /// queried identifier's own tuples, never another one's -- the whole point of filtering by
    /// `id == name` rather than assuming the table holds only one name's entries.
    #[test]
    fn instantiations_for_returns_only_the_requested_identifiers_tuples() {
        // `Box` is erased, so its ctor `Wrap` gets one variant per instantiation of `A`; `Cons`
        // carries its own existential parameter and lands in the same table with its own tuples.
        let list_decl = data!(
            id!("List"),
            [ctor_sig!(
                id!("Cons"),
                [tparam!(id!("E", 3), "+")],
                [bind!(id!("y"), prd!(), tvar!(id!("E", 3)))]
            )],
            [tparam!(id!("L", 2), "+")]
        );
        let erased = ErasedDecls::from(HashSet::from([(id!("Box"), 0)]));
        let solution = Solution::from(HashMap::from([
            (
                vec![id!("A", 1)],
                HashSet::from([vec![ty!("int")], vec![ty!(id!("Box"))]]),
            ),
            (vec![id!("L", 2)], HashSet::from([vec![ty!("int")]])),
            (
                vec![id!("E", 3)],
                HashSet::from([vec![ty!("int")], vec![ty!(id!("List"), [ty!("int")])]]),
            ),
        ]));

        let table = NamingTable::build(&solution, &[box_decl(), list_decl], &[], &[], &erased)
            .expect("test fixture must not collide");

        // exactly Cons's own tuples, in canonical order (`Ty::I64` sorts before `Ty::Decl`) --
        // neither Box's/Wrap's nor List's entries may leak in
        assert_eq!(
            table.instantiations_for(&id!("Cons")),
            vec![vec![ty!("int")], vec![ty!(id!("List"), [ty!("int")])]]
        );
        assert_eq!(
            table.instantiations_for(&id!("Wrap")),
            vec![vec![ty!("int")], vec![ty!(id!("Box"))]]
        );
        // an identifier that was never registered yields nothing rather than the next one's tuples
        assert!(table.instantiations_for(&id!("Absent")).is_empty());
    }

    #[test]
    fn extra_params_for_returns_declarations_own_params_when_erased() {
        let erased = ErasedDecls::from(HashSet::from([(id!("Box"), 0)]));
        let solution = Solution::from(HashMap::from([(
            vec![id!("A", 1)],
            HashSet::from([vec![ty!("int")]]),
        )]));

        let table = NamingTable::build(&solution, &[box_decl()], &[], &[], &erased)
            .expect("test fixture must not collide");

        assert_eq!(table.extra_params_for(&id!("Wrap")), &[id!("A", 1)]);
    }

    #[test]
    fn extra_params_for_is_empty_for_non_erased_xtor() {
        let erased = ErasedDecls::default();
        let solution = Solution::from(HashMap::from([(
            vec![id!("A", 1)],
            HashSet::from([vec![ty!("int")]]),
        )]));

        let table = NamingTable::build(&solution, &[box_decl()], &[], &[], &erased)
            .expect("test fixture must not collide");

        assert!(table.extra_params_for(&id!("Wrap")).is_empty());
    }

    #[test]
    fn partially_erased_decl_pushes_only_erased_params_onto_its_xtors() {
        // Pair[A, B] with only A erased: copies per B, MkPair variants per A, and each copy only
        // owns the A instantiations its solution tuples correlate it with
        let pair_decl = data!(
            id!("Pair"),
            [ctor_sig!(
                id!("MkPair"),
                [],
                [
                    bind!(id!("x"), prd!(), tvar!(id!("A", 1))),
                    bind!(id!("y"), prd!(), tvar!(id!("B", 2)))
                ]
            )],
            [tparam!(id!("A", 1), "+"), tparam!(id!("B", 2), "+")]
        );
        let erased = ErasedDecls::from(HashSet::from([(id!("Pair"), 0)]));
        let solution = Solution::from(HashMap::from([(
            vec![id!("A", 1), id!("B", 2)],
            HashSet::from([
                vec![ty!("int"), ty!("int")],
                vec![ty!(id!("Pair"), [ty!("int")]), ty!("int")],
                vec![ty!("int"), ty!(id!("Bool"))],
            ]),
        )]));

        let table = NamingTable::build(&solution, &[pair_decl], &[], &[], &erased)
            .expect("test fixture must not collide");

        assert_eq!(
            table.instantiations_for(&id!("Pair")),
            vec![vec![ty!("int")], vec![ty!(id!("Bool"))]]
        );
        assert_eq!(table.extra_params_for(&id!("MkPair")), &[id!("A", 1)]);
        assert_eq!(
            table.instantiations_for(&id!("MkPair")),
            vec![vec![ty!("int")], vec![ty!(id!("Pair"), [ty!("int")])]]
        );

        let grown = [ty!(id!("Pair"), [ty!("int")])];
        assert!(table.has_erased_variant(&id!("Pair"), &[ty!("int")], &[ty!("int")]));
        assert!(table.has_erased_variant(&id!("Pair"), &[ty!("int")], &grown));
        assert!(table.has_erased_variant(&id!("Pair"), &[ty!(id!("Bool"))], &[ty!("int")]));
        assert!(!table.has_erased_variant(&id!("Pair"), &[ty!(id!("Bool"))], &grown));
    }
}
