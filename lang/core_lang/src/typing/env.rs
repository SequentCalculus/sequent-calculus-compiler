use crate::{
    mono::errors::MonoError,
    syntax::{CodataDeclaration, CtorSig, DataDeclaration, Def, DtorSig, Identifier, TypeParam},
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

    /// Looks up a constructor signature ([`CtorSig`]) by its identifier.
    pub fn lookup_xtor_for_data_decl(&self, name: &Identifier) -> Result<CtorSig, MonoError> {
        self.data_decls
            .iter()
            .find_map(|decl| decl.xtors.iter().find(|xtor| xtor.name == *name).cloned())
            .ok_or_else(|| MonoError::UndeclaredXtor {
                type_name: "unknown".to_owned(),
                xtor_name: name.name.clone(),
            })
    }

    /// Looks up a destructor signature ([`DtorSig`]) by its identifier.
    pub fn lookup_xtor_for_codata_decl(&self, name: &Identifier) -> Result<DtorSig, MonoError> {
        self.codata_decls
            .iter()
            .find_map(|decl| decl.xtors.iter().find(|xtor| xtor.name == *name).cloned())
            .ok_or_else(|| MonoError::UndeclaredXtor {
                type_name: "unknown".to_owned(),
                xtor_name: name.name.clone(),
            })
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
