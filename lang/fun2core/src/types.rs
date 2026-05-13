//! This module defines the trivial translation on types.

use core_lang::syntax::{ID, fresh_identifier, names::Identifier};
use printer::Print;
use std::collections::HashMap;

/// This function converts [types in Fun](fun::syntax::types::Ty) to
/// [types in Core](core_lang::syntax::types::Ty).
pub fn compile_ty(ty: &fun::syntax::types::Ty) -> core_lang::syntax::types::Ty {
    match ty {
        fun::syntax::types::Ty::I64 { .. } => core_lang::syntax::types::Ty::I64,
        fun::syntax::types::Ty::Decl { .. } => {
            core_lang::syntax::types::Ty::Decl(Identifier::new(ty.print_to_string(None)))
        }
    }
}

/// This function converts [types in Fun](fun::syntax::types::Ty) to
/// [types in Core](core_lang::syntax::types::Ty), replacing type parameters with the given core
/// identifiers.
/// - `ty` is the Fun type to translate.
/// - `type_params` maps Fun type parameter names to fresh Core identifiers.
pub fn compile_ty_with_subst(
    ty: &fun::syntax::types::Ty,
    type_params: &HashMap<String, Identifier>,
) -> core_lang::syntax::types::Ty {
    match ty {
        fun::syntax::types::Ty::I64 { .. } => core_lang::syntax::types::Ty::I64,
        fun::syntax::types::Ty::Decl {
            name, type_args, ..
        } => {
            // If the type is a bare type parameter, replace it directly with the fresh Core identifier.
            if type_args.args.is_empty() {
                if let Some(identifier) = type_params.get(name) {
                    return core_lang::syntax::types::Ty::Decl(identifier.clone());
                }
            }

            // Otherwise, recursively translate the type arguments first so nested references to
            // type parameters also use the fresh Core identifiers.
            let translated_args = type_args
                .args
                .iter()
                .map(|arg| compile_ty_with_subst(arg, type_params).print_to_string(None))
                .collect::<Vec<_>>()
                .join(", ");

            let translated_name = if translated_args.is_empty() {
                name.clone()
            } else {
                format!("{name}[{translated_args}]")
            };

            core_lang::syntax::types::Ty::Decl(Identifier::new(translated_name))
        }
    }
}

/// This function converts [type parameters in Fun](fun::syntax::context::TypeContext) to
/// [type parameters in Core](core_lang::syntax::names::Identifier).
pub fn compile_type_params(
    params: &fun::syntax::context::TypeContext,
    max_id: &mut ID,
) -> Vec<Identifier> {
    params
        .bindings
        .iter()
        .map(|param| fresh_identifier(max_id, param))
        .collect()
}

#[cfg(test)]
mod compile_tests {
    use super::compile_ty_with_subst;
    use core_lang::syntax::names::Identifier;
    use core_macros::{id, ty};
    use fun::syntax::types::{Ty, TypeArgs};
    use std::collections::HashMap;

    #[test]
    fn compile_ty_with_subst_rewrites_nested_type_arguments() {
        let ty = Ty::mk_decl(
            "List",
            TypeArgs::mk(vec![Ty::mk_decl(
                "List",
                TypeArgs::mk(vec![Ty::mk_decl("A", TypeArgs::default())]),
            )]),
        );

        let subst = HashMap::from([(
            "A".to_string(),
            Identifier {
                name: "A".to_string(),
                id: 1,
            },
        )]);

        let result = compile_ty_with_subst(&ty, &subst);
        let expected = ty!(id!("List[List[A_1]]"));
        assert_eq!(result, expected);
    }
}
