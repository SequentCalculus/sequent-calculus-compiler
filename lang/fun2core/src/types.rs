//! This module defines the trivial translation on types.

use core_lang::syntax::{
    ID, fresh_identifier,
    names::Identifier,
    type_params::{ParamPolarity, TypeParam},
};
use std::{collections::HashMap, rc::Rc};

/// This function converts a [polarity in Fun](fun::syntax::declarations::Polarity) to a
/// [polarity in Core](ParamPolarity).
pub fn translate_polarity(polarity: &fun::syntax::declarations::Polarity) -> ParamPolarity {
    match polarity {
        fun::syntax::declarations::Polarity::Data => ParamPolarity::Data,
        fun::syntax::declarations::Polarity::Codata => ParamPolarity::Codata,
    }
}

/// This function converts [types in Fun](fun::syntax::types::Ty) to
/// [types in Core](core_lang::syntax::types::Ty), replacing type parameters with the given core
/// identifiers.
/// - `ty` is the Fun type to translate.
/// - `type_params` maps Fun type parameter names to their fresh Core identifier and declared
///   polarity.
pub fn compile_ty(
    ty: &fun::syntax::types::Ty,
    type_params: Rc<HashMap<String, (Identifier, ParamPolarity)>>,
) -> core_lang::syntax::types::Ty {
    match ty {
        fun::syntax::types::Ty::I64 { .. } => core_lang::syntax::types::Ty::I64,
        fun::syntax::types::Ty::Decl {
            name, type_args, ..
        } => {
            // Bare type parameter -> explicit Core type variable
            if type_args.args.is_empty() {
                if let Some((identifier, _)) = type_params.get(name) {
                    return core_lang::syntax::types::Ty::Var(identifier.clone());
                }
            }

            // Otherwise, recursively translate the type arguments first so nested references to type parameters also use the fresh Core identifiers.
            let translated_args = type_args
                .args
                .iter()
                .map(|arg| compile_ty(arg, type_params.clone()))
                .collect::<Vec<_>>();

            core_lang::syntax::types::Ty::Decl {
                name: Identifier::new(name.clone()),
                type_args: core_lang::syntax::types::TypeArgs {
                    args: translated_args,
                },
            }
        }
    }
}

/// This function converts [declaration-site type parameters in Fun](fun::syntax::type_params::TypeParams)
/// to [type parameters in Core](TypeParam), minting a fresh Core identifier for each while
/// preserving its declared polarity.
pub fn compile_type_params(
    params: &fun::syntax::type_params::TypeParams,
    max_id: &mut ID,
) -> Vec<TypeParam> {
    params
        .bindings
        .iter()
        .map(|param| TypeParam {
            name: fresh_identifier(max_id, &param.name),
            polarity: translate_polarity(&param.polarity),
        })
        .collect()
}

/// This function converts [type arguments in Fun](fun::syntax::types::TypeArgs) to
/// [type arguments in Core](core_lang::syntax::types::TypeArgs).
pub fn compile_type_args(
    args: &fun::syntax::types::TypeArgs,
    type_params: Rc<HashMap<String, (Identifier, ParamPolarity)>>,
) -> core_lang::syntax::types::TypeArgs {
    core_lang::syntax::types::TypeArgs {
        args: args
            .args
            .iter()
            .map(|arg| compile_ty(arg, type_params.clone()))
            .collect(),
    }
}

#[cfg(test)]
mod compile_tests {
    use super::compile_ty;
    use core_lang::syntax::{names::Identifier, type_params::ParamPolarity};
    use core_macros::{id, tvar, ty};
    use fun::syntax::types::{Ty, TypeArgs};
    use std::{collections::HashMap, rc::Rc};

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
            (
                Identifier {
                    name: "A".to_string(),
                    id: 1,
                },
                ParamPolarity::Data,
            ),
        )]);

        let result = compile_ty(&ty, Rc::new(subst));

        let expected = ty!(id!("List"), [ty!(id!("List"), [tvar!(id!("A", 1))])]);

        assert_eq!(result, expected);
    }
}
