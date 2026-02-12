use macro_utils::{expr_to_string, expr_to_tuple, parse_args};
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_str;

pub fn exit(input: TokenStream) -> TokenStream {
    let args = parse_args(
        input.into(),
        ["Exit Term", "Type"],
        &[(1, parse_str("core_lang::syntax::types::Ty::I64").unwrap())],
    );
    let exit_term = &args[0];
    let exit_ty = &args[1];
    quote! { core_lang::syntax::statements::exit::Exit{
        arg: ::std::rc::Rc::new(core_lang::syntax::terms::Term::from(#exit_term)),
        ty: #exit_ty
        }
    }
    .into()
}

pub fn fs_exit(input: TokenStream) -> TokenStream {
    let args = parse_args(input.into(), ["Exit Var"], &[]);
    let var = expr_to_tuple(&args[0]);
    let var_name = expr_to_string(&var[0], 0);
    let var_id = &var[1];
    quote! {
        core_lang::syntax::statements::exit::FsExit {
            var: core_lang::syntax::names::Var {
                name: #var_name.to_string() ,
                id:#var_id
            }
        }
    }
    .into()
}
