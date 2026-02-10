use axcut::syntax::statements::ifc::IfSort;
use macro_utils::{expr_to_string, parse_args, quote_option};
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_str;

pub fn ife(input: TokenStream) -> TokenStream {
    ifc(input, IfSort::Equal)
}
pub fn ifne(input: TokenStream) -> TokenStream {
    ifc(input, IfSort::NotEqual)
}
pub fn ifl(input: TokenStream) -> TokenStream {
    ifc(input, IfSort::Less)
}
pub fn ifle(input: TokenStream) -> TokenStream {
    ifc(input, IfSort::LessOrEqual)
}
pub fn ifg(input: TokenStream) -> TokenStream {
    ifc(input, IfSort::Greater)
}
pub fn ifge(input: TokenStream) -> TokenStream {
    ifc(input, IfSort::GreaterOrEqual)
}

fn ifc(input: TokenStream, sort: IfSort) -> TokenStream {
    let sort = match sort {
        IfSort::Equal => quote! { axcut::syntax::statements::ifc::IfSort::Equal},
        IfSort::NotEqual => quote! { axcut::syntax::statements::ifc::IfSort::NotEqual},
        IfSort::Less => quote! { axcut::syntax::statements::ifc::IfSort::Less},
        IfSort::LessOrEqual => quote! { axcut::syntax::statements::ifc::IfSort::LessOrEqual},
        IfSort::Greater => quote! { axcut::syntax::statements::ifc::IfSort::Greater},
        IfSort::GreaterOrEqual => quote! { axcut::syntax::statements::ifc::IfSort::GreaterOrEqual},
    };
    let args = parse_args(
        input.into(),
        [
            "First Variable",
            "Second Variable",
            "Then Statement",
            "Else Statement",
        ],
        &[(1, parse_str("::std::option::Option::None").unwrap())],
    );
    let fst = expr_to_string(&args[0], 0);
    let snd = quote_option(&args[1], |expr| quote!(#expr.to_string()));
    let thenc = &args[2];
    let elsec = &args[3];
    quote! {
        axcut::syntax::statements::ifc::IfC{
            sort: #sort,
            fst: #fst.to_string(),
            snd: #snd,
            thenc: ::std::rc::Rc::new(Statement::from(#thenc)),
            elsec: ::std::rc::Rc::new(Statement::from(#elsec))
        }
    }
    .into()
}
