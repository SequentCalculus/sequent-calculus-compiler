use std::collections::HashMap;

use crate::{
    mono::solver::Solution,
    syntax::{CodataDeclaration, DataDeclaration, Identifier, Ty},
};

/// A mapping from polymorphic type parameters to their corresponding concrete types as string representations after monomorphization.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamingTable {
    names: HashMap<(Identifier, Vec<Ty>), Identifier>,
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
    ) -> Self {
        let mut names = HashMap::new();

        for decl in data_decls {
            if decl.type_params.is_empty() {
                // If there are no type parameters, we can just use the original name
                names.insert((decl.name.clone(), vec![]), decl.name.clone());
            } else if let Some(tuples) = solution.map.get(&decl.type_params) {
                for tuple in tuples {
                    let mangled = mangle(&decl.name, tuple);
                    names.insert((decl.name.clone(), tuple.clone()), Identifier::new(mangled));
                }
            }
        }

        for decl in codata_decls {
            if decl.type_params.is_empty() {
                // If there are no type parameters, we can just use the original name
                names.insert((decl.name.clone(), vec![]), decl.name.clone());
            } else if let Some(tuples) = solution.map.get(&decl.type_params) {
                for tuple in tuples {
                    let mangled = mangle(&decl.name, tuple);
                    names.insert((decl.name.clone(), tuple.clone()), Identifier::new(mangled));
                }
            }
        }

        Self { names }
    }

    /// Looks up the mangled name for a given type and its instantiation.
    pub fn lookup(&self, name: &Identifier, tuple: &[Ty]) -> &Identifier {
        self.names
            .get(&(name.clone(), tuple.to_vec()))
            .unwrap_or_else(|| {
                panic!(
                    "no specialized name recorded for type {} with instantiation {:?} -- \
                     this indicates a bug in constraint collection or solving",
                    name.name, tuple
                )
            })
    }
}

/// Generates a mangled name for a type declaration given its base name and the concrete types it is instantiated with.
fn mangle(base_name: &Identifier, tuple: &[Ty]) -> String {
    if tuple.is_empty() {
        base_name.name.clone()
    } else {
        let args: Vec<String> = tuple.iter().map(mangle_ty).collect();
        format!("{}[{}]", base_name.name, args.join(", "))
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
