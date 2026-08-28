use macro_utils::{expr_to_string, parse_args};
use proc_macro::TokenStream;
use quote::quote;

/// Create a [`core_lang::syntax::type_params::TypeParam`] from an identifier expression and a
/// mandatory polarity sigil, `"+"` (data/positive) or `"-"` (codata/negative), mirroring the
/// surface syntax's `A+`/`A-` annotation.
pub fn tparam(input: TokenStream) -> TokenStream {
    let args = parse_args(input.into(), ["Type Parameter Identifier", "Polarity"], &[]);
    let id_expr = &args[0];
    let polarity_str = expr_to_string(&args[1], 1);
    let polarity = match polarity_str.as_str() {
        "+" => quote! { core_lang::syntax::type_params::ParamPolarity::Data },
        "-" => quote! { core_lang::syntax::type_params::ParamPolarity::Codata },
        other => panic!("invalid polarity sigil {other:?}, expected \"+\" or \"-\""),
    };

    quote! {
        core_lang::syntax::type_params::TypeParam{
            id: #id_expr,
            polarity: #polarity,
        }
    }
    .into()
}
