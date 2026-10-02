//! Defines `GlobalEnv`, the read-only view of a program's top-level declarations used during
//! type checking and constraint collection.

use crate::{
    mono::errors::MonoError,
    syntax::{
        CodataDeclaration, CtorSig, DataDeclaration, Def, DtorSig, Identifier, Ty, TypeParam,
        declaration::{Polarity, TypeDeclaration, XtorSig},
    },
};

/// Global environment holding immutable references to all top-level program declarations used during type checking.
#[derive(Default)]
pub struct GlobalEnv<'a> {
    pub data_decls: &'a [DataDeclaration],
    pub codata_decls: &'a [CodataDeclaration],
    pub defs: &'a [Def],
}

impl<'a> GlobalEnv<'a> {
    /// Creates a new global environment from the provided program components.
    pub fn new(
        data_decls: &'a [DataDeclaration],
        codata_decls: &'a [CodataDeclaration],
        defs: &'a [Def],
    ) -> GlobalEnv<'a> {
        Self {
            data_decls,
            codata_decls,
            defs,
        }
    }

    /// Looks up a [`DataDeclaration`] by its identifier.
    pub fn lookup_data_decl(&self, name: &Identifier) -> Option<&DataDeclaration> {
        self.data_decls.iter().find(|d| d.name == *name)
    }

    /// Looks up a [`CodataDeclaration`] by its identifier.
    pub fn lookup_codata_decl(&self, name: &Identifier) -> Option<&CodataDeclaration> {
        self.codata_decls.iter().find(|d| d.name == *name)
    }

    /// Looks up the constructor signature ([`CtorSig`]) `name` in the data declaration of `ty`.
    /// Xtor names are only unique within one declaration, so the lookup goes through the type
    /// the constructor belongs to.
    pub fn lookup_xtor_for_data_decl(
        &self,
        ty: &Ty,
        name: &Identifier,
    ) -> Result<CtorSig, MonoError> {
        lookup_xtor_in(self.data_decls, ty, name)
    }

    /// Looks up the destructor signature ([`DtorSig`]) `name` in the codata declaration of `ty`.
    /// Xtor names are only unique within one declaration, so the lookup goes through the type
    /// the destructor belongs to.
    pub fn lookup_xtor_for_codata_decl(
        &self,
        ty: &Ty,
        name: &Identifier,
    ) -> Result<DtorSig, MonoError> {
        lookup_xtor_in(self.codata_decls, ty, name)
    }

    /// Looks up a top-level function definition ([`Def`]) by its identifier.
    pub fn lookup_def(&self, name: &Identifier) -> Option<&Def> {
        self.defs.iter().find(|d| d.name == *name)
    }

    /// Searches both [`DataDeclaration`] and [`CodataDeclaration`] for a type matching the given identifier, returning a slice of its associated type parameters if found.
    pub fn lookup_type_params(&self, name: &Identifier) -> Option<&[TypeParam]> {
        self.lookup_data_decl(name)
            .map(|decl| decl.type_params.as_slice())
            .or_else(|| {
                self.lookup_codata_decl(name)
                    .map(|decl| decl.type_params.as_slice())
            })
    }
}

/// Finds the xtor `name` in the declaration among `decls` that `ty` refers to.
fn lookup_xtor_in<P: Polarity + Clone>(
    decls: &[TypeDeclaration<P>],
    ty: &Ty,
    name: &Identifier,
) -> Result<XtorSig<P>, MonoError> {
    let undeclared = |type_name: String| MonoError::UndeclaredXtor {
        type_name,
        xtor_name: name.name.clone(),
    };
    let Ty::Decl {
        name: type_name, ..
    } = ty
    else {
        return Err(undeclared("unknown".to_owned()));
    };
    decls
        .iter()
        .find(|decl| decl.name == *type_name)
        .and_then(|decl| decl.xtors.iter().find(|xtor| xtor.name == *name))
        .cloned()
        .ok_or_else(|| undeclared(type_name.name.clone()))
}
