use std::collections::HashMap;

use crate::{
    mono::solver::Solution,
    syntax::{Identifier, Ty},
};

/// A mapping from polymorphic type parameters to their corresponding concrete types as string representations after monomorphization.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamingTable {
    names: HashMap<Vec<Identifier>, Vec<String>>,
}

impl NamingTable {
    /// Builds the naming table from the solver's output.
    ///
    /// For each node and each ground vector in its solution, generates a
    /// fresh, deterministically mangled identifier derived from the concrete types,
    /// e.g. `Pair[A,B]` instantiated with `[i64, Bool]` becomes `Pair_i64_Bool`.
    pub fn build(solution: &Solution) -> Self {
        let mut names = HashMap::new();
        for (node, vecs) in solution.iter() {
            for vec in vecs {
                let mangled_name = vec
                    .iter()
                    .map(Self::mangle_ty)
                    .collect::<Vec<_>>()
                    .join("_");
                names
                    .entry(node.clone())
                    .or_insert_with(Vec::new)
                    .push(mangled_name);
            }
        }
        Self { names }
    }

    fn mangle_ty(ty: &Ty) -> String {
        match ty {
            Ty::I64 => "i64".to_string(),
            Ty::Decl { name, type_args } => {
                if type_args.args.is_empty() {
                    name.name.clone()
                } else {
                    let args = type_args
                        .args
                        .iter()
                        .map(Self::mangle_ty)
                        .collect::<Vec<_>>()
                        .join("_");
                    format!("{}_{}", name.name, args)
                }
            }
            Ty::Var(_) => unreachable!("Type variables should not appear in the naming table"),
        }
    }
}
