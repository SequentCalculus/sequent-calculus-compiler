use crate::{
    syntax::{CodataDeclaration, DataDeclaration, Def, Identifier, Ty, types::TypeArgs},
    typing::errors::LocatedTypeError,
};

/// This trait defines the type checking behavior for all syntax elements in core. The `check` method takes the current context of type parameters, data declarations, codata declarations, and function definitions, and returns an error if the syntax element is not well-typed.
pub trait Checked: Sized {
    fn check(
        &self,
        type_params: &[Identifier],
        data_declarations: &[DataDeclaration],
        codata_declarations: &[CodataDeclaration],
        defs: &[Def],
    ) -> Result<(), LocatedTypeError>;
}

// instantiate declaration template types with concrete type arguments.
pub fn instantiate_type_params(ty: &Ty, params: &[Identifier], args: &[Ty]) -> Ty {
    match ty {
        Ty::I64 => Ty::I64,
        Ty::Var(id) => {
            if let Some(idx) = params.iter().position(|p| p == id) {
                args[idx].clone()
            } else {
                Ty::Var(id.clone())
            }
        }
        Ty::Decl { name, type_args } => Ty::Decl {
            name: name.clone(),
            type_args: TypeArgs {
                args: type_args
                    .args
                    .iter()
                    .map(|a| instantiate_type_params(a, params, args))
                    .collect(),
            },
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate self as core_lang;
    use core_macros::{id, tvar, ty};

    // instantiate_type_params: simple substitution
    #[test]
    fn instantiate_type_params_replaces_type_var() {
        let template = ty!(id!("List"), [tvar!(id!("A", 1))]);

        let res = instantiate_type_params(&template, &[id!("A", 1)], &[ty!("int")]);
        assert_eq!(res, ty!(id!("List"), [ty!("int")]));
    }
}
