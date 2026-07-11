use std::collections::HashMap;

use crate::{
    mono::solver::Solution,
    syntax::{CodataDeclaration, DataDeclaration, Def, Identifier, Ty},
};

/// A mapping from polymorphic type parameters to their corresponding concrete types as string representations after monomorphization.
///
/// This is the single source of truth for specialization: once built, it is
/// the only structure the specialization passes need. The [`Solution`] used
/// to build it is not required afterwards.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamingTable {
    names: HashMap<(Identifier, Vec<Ty>), Identifier>,
    /// All ground instantiation tuples recorded for a given base identifier,
    /// in the same order the solution produced them.
    instantiations: HashMap<Identifier, Vec<Vec<Ty>>>,
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
    ) -> Self {
        let mut table = NamingTable {
            names: HashMap::new(),
            instantiations: HashMap::new(),
        };

        for decl in data_decls {
            table.register(
                &decl.name,
                &decl.type_params,
                solution,
                mangle_ty_declaration,
            );
        }
        for decl in codata_decls {
            table.register(
                &decl.name,
                &decl.type_params,
                solution,
                mangle_ty_declaration,
            );
        }
        for def in defs {
            table.register(
                &def.name,
                &def.type_params,
                solution,
                mangle_def_declaration,
            );
        }

        table
    }

    /// Registers every instantiation of a single declaration (data, codata,
    /// or def) into both `names` and `instantiations` at once.
    ///
    /// If `type_params` is empty, the declaration is already monomorphic and
    /// keeps its original name under the empty tuple. Otherwise, every
    /// ground tuple recorded in `solution` for this declaration's node is
    /// mangled via `mangle` and inserted into both maps.
    fn register(
        &mut self,
        name: &Identifier,
        type_params: &[Identifier],
        solution: &Solution,
        mangle: fn(&Identifier, &[Ty]) -> String,
    ) {
        if type_params.is_empty() {
            // Monomorphic by construction: keep the original name under the
            // empty tuple, and record that single "instantiation" so
            // instantiations_for still returns something sensible.
            self.names.insert((name.clone(), vec![]), name.clone());
            self.instantiations
                .entry(name.clone())
                .or_default()
                .push(vec![]);
            return;
        }

        let Some(tuples) = solution.map.get(type_params) else {
            // Never instantiated, no entries at all, so instantiations_for
            // will correctly return an empty Vec via unwrap_or_default.
            return;
        };

        for tuple in tuples {
            let mangled = mangle(name, tuple);
            self.names
                .insert((name.clone(), tuple.clone()), Identifier::new(mangled));
            self.instantiations
                .entry(name.clone())
                .or_default()
                .push(tuple.clone());
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

    /// Returns all concrete type instantiation tuples recorded for a given identifier.
    pub fn instantiations_for(&self, name: &Identifier) -> Vec<Vec<Ty>> {
        self.names
            .keys()
            .filter(|(id, _)| id == name)
            .map(|(_, tuple)| tuple.clone())
            .collect()
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
