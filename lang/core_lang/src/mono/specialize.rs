use crate::{
    mono::{naming_table::NamingTable, solver::Solution},
    syntax::{
        Identifier, Ty,
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

/// Specialization of polymorphic type declarations into monomorphic ones
pub fn specialize_decl<P: Polarity + Clone>(
    decl: &TypeDeclaration<P>,
    solution: &Solution,
    table: &NamingTable,
) -> Vec<TypeDeclaration<P>> {
    let node = &decl.type_params;
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
