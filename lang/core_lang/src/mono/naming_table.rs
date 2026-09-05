//! Builds the table mapping every polymorphic declaration, xtor, and def, together with its
//! concrete instantiation, to its mangled monomorphic name.

use std::collections::{BTreeMap, HashMap};

use crate::{
    mono::{erasure::ErasedDecls, solver::Solution},
    syntax::{
        CodataDeclaration, DataDeclaration, Def, Identifier, Ty, TypeParam,
        declaration::{Polarity, TypeDeclaration},
    },
};

/// A mapping from polymorphic type parameters to their corresponding concrete types as string representations after monomorphization.
///
/// This is the single source of truth for specialization: once built, it is
/// the only structure the specialization passes need. The [`Solution`] used
/// to build it is not required afterwards.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamingTable {
    names: BTreeMap<(Identifier, Vec<Ty>), Identifier>,
    xtor_extra_params: HashMap<Identifier, Vec<Identifier>>,
}

impl NamingTable {
    /// Builds the naming table from the solver's output.
    ///
    /// For each node and each ground vector in its solution, generates a
    /// fresh, deterministically mangled identifier derived from the concrete types,
    /// e.g. `Pair[A,B]` instantiated with `[i64, Bool]` becomes `Pair[i64, Bool]`.
    pub fn build(
        solution: &Solution,
        data_decls: &[DataDeclaration],
        codata_decls: &[CodataDeclaration],
        defs: &[Def],
        erased_decls: &ErasedDecls,
    ) -> Self {
        let mut table = NamingTable {
            names: BTreeMap::new(),
            xtor_extra_params: HashMap::new(),
        };

        for decl in data_decls {
            table.register_decl(decl, solution, erased_decls, mangle_ty_declaration);
        }

        for decl in codata_decls {
            table.register_decl(decl, solution, erased_decls, mangle_ty_declaration);
        }

        for def in defs {
            let def_type_param_ids: Vec<Identifier> = TypeParam::ids(&def.type_params);
            table.register(
                &def.name,
                &def_type_param_ids,
                solution,
                mangle_def_declaration,
            );
        }

        table
    }

    /// Registers a single data or codata declaration and its xtors.
    ///
    /// If the declaration is erased (widened to a single recursive type), it keeps its name
    /// unchanged and unduplicated, and each of its xtors is registered under the combination of
    /// its own type parameters plus the declaration's
    /// own (now-erased) type parameters, exactly as if the latter had been declared on the xtor
    /// itself. Otherwise, behavior matches the ordinary (non-erased) path used so far.
    fn register_decl<P: Polarity>(
        &mut self,
        decl: &TypeDeclaration<P>,
        solution: &Solution,
        erased_decls: &ErasedDecls,
        mangle: fn(&Identifier, &[Ty]) -> String,
    ) {
        let decl_type_param_ids: Vec<Identifier> = TypeParam::ids(&decl.type_params);
        if erased_decls.is_erased(&decl.name) {
            self.names
                .insert((decl.name.clone(), vec![]), decl.name.clone());
            for xtor in &decl.xtors {
                // Record the declaration's own type parameters as extra parameters for the xtor.
                self.xtor_extra_params
                    .insert(xtor.name.clone(), decl_type_param_ids.clone());

                let xtor_type_param_ids: Vec<Identifier> = TypeParam::ids(&xtor.type_params);
                self.register_combined(
                    &xtor.name,
                    &xtor_type_param_ids,
                    &decl_type_param_ids,
                    solution,
                    mangle,
                );
            }
        } else {
            self.register(&decl.name, &decl_type_param_ids, solution, mangle);
            for xtor in &decl.xtors {
                let xtor_type_param_ids: Vec<Identifier> = TypeParam::ids(&xtor.type_params);
                self.register(&xtor.name, &xtor_type_param_ids, solution, mangle);
            }
        }
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
    ) {
        self.register_combined(name, type_params, &[], solution, mangle);
    }

    /// Registers every instantiation of `name` under the combination of `own_params` (the
    /// name's own declared parameters) and `extra_params` (any additional parameters pushed down
    /// from an erased surrounding declaration). If both are non-empty, the solver currently
    /// tracks them as two independent nodes rather than one correlated one, so we take their
    /// cartesian product.
    fn register_combined(
        &mut self,
        name: &Identifier,
        own_params: &[Identifier],
        extra_params: &[Identifier],
        solution: &Solution,
        mangle: fn(&Identifier, &[Ty]) -> String,
    ) {
        if own_params.is_empty() && extra_params.is_empty() {
            self.names.insert((name.clone(), vec![]), name.clone());
            return;
        }

        let tuples: Vec<Vec<Ty>> = if extra_params.is_empty() {
            solution
                .get(own_params)
                .cloned()
                .unwrap_or_default()
                .into_iter()
                .collect()
        } else if own_params.is_empty() {
            solution
                .get(extra_params)
                .cloned()
                .unwrap_or_default()
                .into_iter()
                .collect()
        } else {
            let own_tuples = solution.get(own_params).cloned().unwrap_or_default();
            let extra_tuples = solution.get(extra_params).cloned().unwrap_or_default();

            own_tuples
                .into_iter()
                .flat_map(|o| {
                    extra_tuples.iter().map(move |e| {
                        let mut combined = o.clone();
                        combined.extend(e.clone());
                        combined
                    })
                })
                .collect()
        };

        for tuple in tuples {
            let mangeled = mangle(name, &tuple);
            self.names
                .insert((name.clone(), tuple.clone()), Identifier::new(mangeled));
        }
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

    /// Returns any extra type parameters pushed down to this xtor from an erased declaration.
    /// Returns an empty slice if the xtor belongs to a non-erased declaration.
    pub fn extra_params_for(&self, xtor: &Identifier) -> &[Identifier] {
        self.xtor_extra_params
            .get(xtor)
            .map(|params| params.as_slice())
            .unwrap_or(&[])
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
        let erased = ErasedDecls(HashSet::from([id!("Box")]));
        let solution = Solution::from(HashMap::from([(
            vec![id!("A", 1)],
            HashSet::from([vec![ty!("int")], vec![ty!(id!("Box"))]]),
        )]));

        let table = NamingTable::build(&solution, &[box_decl()], &[], &[], &erased);

        // Box itself must remain registered under its own, unchanged name.
        assert_eq!(table.lookup(&id!("Box"), &[]), &id!("Box"));
    }

    #[test]
    fn erased_decl_registers_two_xtor_instantiations() {
        let erased = ErasedDecls(HashSet::from([id!("Box")]));
        let solution = Solution::from(HashMap::from([(
            vec![id!("A", 1)],
            HashSet::from([vec![ty!("int")], vec![ty!(id!("Box"))]]),
        )]));

        let table = NamingTable::build(&solution, &[box_decl()], &[], &[], &erased);

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
        let erased = ErasedDecls(HashSet::from([id!("Box")]));
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

        let table = NamingTable::build(&solution, &[box_decl(), list_decl], &[], &[], &erased);

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
        let erased = ErasedDecls(HashSet::from([id!("Box")]));
        let solution = Solution::from(HashMap::from([(
            vec![id!("A", 1)],
            HashSet::from([vec![ty!("int")]]),
        )]));

        let table = NamingTable::build(&solution, &[box_decl()], &[], &[], &erased);

        assert_eq!(table.extra_params_for(&id!("Wrap")), &[id!("A", 1)]);
    }

    #[test]
    fn extra_params_for_is_empty_for_non_erased_xtor() {
        let erased = ErasedDecls::default();
        let solution = Solution::from(HashMap::from([(
            vec![id!("A", 1)],
            HashSet::from([vec![ty!("int")]]),
        )]));

        let table = NamingTable::build(&solution, &[box_decl()], &[], &[], &erased);

        assert!(table.extra_params_for(&id!("Wrap")).is_empty());
    }
}
