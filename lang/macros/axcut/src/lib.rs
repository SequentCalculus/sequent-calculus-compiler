use proc_macro::TokenStream;
use quote::quote;

pub(crate) mod context;
pub(crate) mod declarations;
pub(crate) mod program;
pub(crate) mod statements;
pub(crate) mod types;

/// Create a [`axcut::syntax::types::Ty`] from a string literal. `int` will create
/// [`axcut::syntax::types::Ty::I64`] anything else will create [`axcut::syntax::types::Ty::Decl`].
#[proc_macro]
pub fn ty(input: TokenStream) -> TokenStream {
    types::ty(input)
}

/// Create a [`axcut::syntax::context::ContextBinding`] with given variable, and type.
/// If no type is provided, it defaults to [`axcut::syntax::types::Ty::I64`].
#[proc_macro]
pub fn bind(input: TokenStream) -> TokenStream {
    context::bind(input)
}

/// Create [`axcut::syntax::context::Chirality::Prd`].
#[proc_macro]
pub fn prd(_: TokenStream) -> TokenStream {
    quote! {axcut::syntax::context::Chirality::Prd}.into()
}

/// Create [`axcut::syntax::context::Chirality::Cns`].
#[proc_macro]
pub fn cns(_: TokenStream) -> TokenStream {
    quote! {axcut::syntax::context::Chirality::Cns}.into()
}

/// Create [`axcut::syntax::context::Chirality::Ext`].
#[proc_macro]
pub fn ext(_: TokenStream) -> TokenStream {
    quote! {axcut::syntax::context::Chirality::Ext}.into()
}

// Statements

/// Create a [`axcut::syntax::statements::substitute::Substitute`] with given list of `rearrange`s
/// and next statement. `rearrange` expects a list of tuples `(ContextBinding, &str)`.
#[proc_macro]
pub fn substitute(input: TokenStream) -> TokenStream {
    statements::substitute(input)
}

/// Create a [`axcut::syntax::statements::let::Let`] with given bound variable, type, tag,
/// arguments, next statement, and free variables for next.
/// If no type is provided, it defaults to [`axcut::syntax::types::Ty::I64`].
/// If no `free_vars_next` are provided, it defaults to `None`.
/// Note that if `free_vars_next` are provided, a type also has to be provided.
#[proc_macro]
pub fn letin(input: TokenStream) -> TokenStream {
    statements::letin(input)
}

/// Create a [`axcut::syntax::statements::Switch`] with given switch variable, type, clauses, and
/// free variables clauses.
/// If no type is provided, it defaults to [`axcut::syntax::types::Ty::I64`].
/// If no `free_vars_next` are provided, it defaults to `None`.
/// Note that if `free_vars_next` are provided, a type also needs to be provided.
#[proc_macro]
pub fn switch(input: TokenStream) -> TokenStream {
    statements::switch(input)
}

/// Create a [`axcut::syntax::statements::invoke::Invoke`] with given variable, tag, type, and
/// arguments.
/// If no type is provided, it defaults to [`axcut::syntax::types::Ty::I64`].
#[proc_macro]
pub fn invoke(input: TokenStream) -> TokenStream {
    statements::invoke(input)
}

/// Create a [`axcut::syntax::statements::create::Create`] with given variable, type, context,
/// clauses, free variables clauses, next statement, and free variables for next.
/// `type`, `context`, `free_vars_clauses`, and `free_vars_next` are optional and default to
/// [`axcut::syntax::types::Ty::I64`] (for the type) and `None`, respectively.
/// Note that if any of the optional arguments are provided, all previous ones also need to be
/// provided. E.g., if `free_vars_clauses` is given, both `type` and `context` need to be as well.
#[proc_macro]
pub fn create(input: TokenStream) -> TokenStream {
    statements::create(input)
}

/// Create a [`axcut::syntax::statements::Literal`] with given literal, variable, next statement,
/// and free variables for next
/// If `free_vars_next` are not provided, it defaults to `None`.
#[proc_macro]
pub fn lit(input: TokenStream) -> TokenStream {
    statements::lit(input)
}

/// Create a [`axcut::syntax::statements::call::Call`] with given label and arguments.
#[proc_macro]
pub fn call(input: TokenStream) -> TokenStream {
    statements::call(input)
}

/// Create a [`axcut::syntax::statements::print::PrintI64`] without newline, with given variable,
/// next statement, and free variables for next.
/// If `free_vars_next` are not provided, it defaults to `None`.
#[proc_macro]
pub fn print_i64(input: TokenStream) -> TokenStream {
    statements::print_i64(input)
}

/// Create a [`axcut::syntax::statements::print::PrintI64`] with newline, and given variable, next
/// statement, and free variables for next.
/// If `free_vars_next` is not provided, it defaults to `None`.
#[proc_macro]
pub fn println_i64(input: TokenStream) -> TokenStream {
    statements::println_i64(input)
}

/// Create a [`axcut::syntax::statements::exit::Exit`] with given variable.
#[proc_macro]
pub fn exit(input: TokenStream) -> TokenStream {
    statements::exit(input)
}

/// Create a [`axcut::syntax::statements::Clause`] with given xtor name, context, and body.
#[proc_macro]
pub fn clause(input: TokenStream) -> TokenStream {
    statements::clause(input)
}

/// Create a [`axcut::syntax::statements::op::Op`] with
/// [`axcut::syntax::statements::op::BinOp::Div`] and given first operand, second operand,
/// variable, next statement, and free variables for next.
/// If `free_vars_next` is not provided, it defaults to `None`.
#[proc_macro]
pub fn div(input: TokenStream) -> TokenStream {
    statements::div(input)
}

/// Create a [`axcut::syntax::statements::op::Op`] with
/// [`axcut::syntax::statements::op::BinOp::Prod`] and given first operand, second operand,
/// variable, next statement, and free variables for next.
/// If `free_vars_next` is not provided, it defaults to `None`.
#[proc_macro]
pub fn prod(input: TokenStream) -> TokenStream {
    statements::prod(input)
}

/// Create a [`axcut::syntax::statements::op::Op`] with
/// [`axcut::syntax::statements::op::BinOp::Rem`] and given first operand, second operand,
/// variable, next statement, and free variables for next.
/// If `free_vars_next` is not provided, it defaults to `None`.
#[proc_macro]
pub fn rem(input: TokenStream) -> TokenStream {
    statements::rem(input)
}

/// Create a [`axcut::syntax::statements::op::Op`] with
/// [`axcut::syntax::statements::op::BinOp::Sum`] and given first operand, second operand,
/// variable, next statement, and free variables for next.
/// If `free_vars_next` is not provided, it defaults to `None`.
#[proc_macro]
pub fn sum(input: TokenStream) -> TokenStream {
    statements::sum(input)
}

/// Create a [`axcut::syntax::statements::op::Op`] with
/// [`axcut::syntax::statements::op::BinOp::Sub`] and given first operand, second operand,
/// variable, next statment, and free variables for next.
/// If `free_vars_next` is not provided, it defaults to `None`.
#[proc_macro]
pub fn sub(input: TokenStream) -> TokenStream {
    statements::sub(input)
}

/// Create a [`axcut::syntax::statements::ifc::IfC`] with comparison
/// [`axcut::syntax::statements::ifc::IfSort::Equal`] and given first variable, second variable,
/// then statement, and else statement.
/// If `snd` is not provided, it defaults to `None` (i.e., compares to `0`).
#[proc_macro]
pub fn ife(input: TokenStream) -> TokenStream {
    statements::ife(input)
}

/// Create a [`axcut::syntax::statements::ifc::IfC`] with comparison
/// [`axcut::syntax::statements::ifc::IfSort::NotEqual`] and given first variable, second variable,
/// then statement, and else statement.
/// If `snd` is not provided it defaults to `None` (i.e., compares to `0`).
#[proc_macro]
pub fn ifne(input: TokenStream) -> TokenStream {
    statements::ifne(input)
}

/// Create a [`axcut::syntax::statements::ifc::IfC`] with comparison
/// [`axcut::syntax::statements::ifc::IfSort::Less`] and given first variable, second variable,
/// then statement, and else statement.
/// If `snd` is not provided, it defaults to `None` (i.e., compares to `0`).
#[proc_macro]
pub fn ifl(input: TokenStream) -> TokenStream {
    statements::ifl(input)
}

/// Create a [`axcut::syntax::statements::ifc::IfC`] with comparison
/// [`axcut::syntax::statements::ifc::IfSort::LessOrEqual`] and given first variable, second
/// variable, then statement, and else statement.
/// If `snd` is not provided, it defaults to `None` (i.e., compares to `0`).
#[proc_macro]
pub fn ifle(input: TokenStream) -> TokenStream {
    statements::ifle(input)
}

/// Create a [`axcut::syntax::statements::ifc::IfC`] with comparison
/// [`axcut::syntax::statements::ifc::IfSort::Greater`] and given first variable, second variable,
/// then statement, and else statement.
/// If `snd` is not provided, it defaults to `None` (i.e., compares to `0`).
#[proc_macro]
pub fn ifg(input: TokenStream) -> TokenStream {
    statements::ifg(input)
}

/// Create a [`axcut::syntax::statements::ifc::IfC`] with comparison
/// [`axcut::syntax::statements::ifc::IfSort::GreaterOrEqual`] and given first variable, second
/// variable, then statement, and else statement.
/// If `snd` is not provided, it defaults to `None` (i.e., compares to `0`).
#[proc_macro]
pub fn ifge(input: TokenStream) -> TokenStream {
    statements::ifge(input)
}

// Declarations

/// Create a [`axcut::syntax::declaration::XtorSig`] with given name and arguments.
#[proc_macro]
pub fn xtor_sig(input: TokenStream) -> TokenStream {
    declarations::xtor_sig(input)
}

/// Create a [`axcut::syntax::def::Def`] with given name, context, body, and used variables.
/// If `used_vars` is not provided, it defaults to `HashSet::new()`.
#[proc_macro]
pub fn def(input: TokenStream) -> TokenStream {
    declarations::def(input)
}

/// Create a [`axcut::syntax::declaration::TypeDeclaration`] with given name and list of xtors.
#[proc_macro]
pub fn ty_decl(input: TokenStream) -> TokenStream {
    declarations::ty_decl(input)
}

/// Create a [`axcut::syntax::program::Prog`] with given list of definitions and type declarations.
#[proc_macro]
pub fn prog(input: TokenStream) -> TokenStream {
    program::prog(input)
}
