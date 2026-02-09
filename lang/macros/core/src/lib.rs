use core_lang::syntax::Chirality;
use proc_macro::TokenStream;
use quote::quote;

pub(crate) mod arguments;
pub(crate) mod context;
pub(crate) mod declarations;
pub(crate) mod prog;
pub(crate) mod statements;
pub(crate) mod terms;
pub(crate) mod types;
use terms::{fs_xtor, unfocused_xtor, xcase, xvar};

///Create a [`core_lang::syntax::types::Ty`] from a string literal\
///`int` will create [`core_lang::syntax::types::Ty::I64`] anything else will
///create [`core_lang::syntax::types::Ty::Decl`]
#[proc_macro]
pub fn ty(input: TokenStream) -> TokenStream {
    types::ty(input)
}

///Create a [`core_lang::syntax::context::ContextBinding`] with given name, chirality and type.
///If no type is provided, it defaults to [`core_lang::syntax::types::Ty`]
#[proc_macro]
pub fn bind(input: TokenStream) -> TokenStream {
    context::bind(input)
}

///Create [`core_lang::syntax::context::Chirality::Cns`]
#[proc_macro]
pub fn cns(_: TokenStream) -> TokenStream {
    quote! {core_lang::syntax::context::Chirality::Cns}.into()
}

///Create [`core_lang::syntax::context::Chirality::Prd`]
#[proc_macro]
pub fn prd(_: TokenStream) -> TokenStream {
    quote! {core_lang::syntax::context::Chirality::Prd}.into()
}

// Terms

///Create a [`core_lang::syntax::terms::xvar::XVar`] with chirality
///[`core_lang::syntax::terms::Prd`] and given type.
///If no type is provided, it will default to [`core_lang::syntax::types::Ty::I64`]
#[proc_macro]
pub fn var(input: TokenStream) -> TokenStream {
    xvar(input, Chirality::Prd)
}

///Create a [`core_lang::syntax::terms::xvar::XVar`] with chirality
///[`core_lang::syntax::terms::Cns`] and given type.
///If no type is provided, it will default to [`core_lang::syntax::types::Ty::I64`]
#[proc_macro]
pub fn covar(input: TokenStream) -> TokenStream {
    xvar(input, Chirality::Cns)
}

///Create a [`core_lang::syntax::terms::literal::Literal`] with given value
#[proc_macro]
pub fn lit(input: TokenStream) -> TokenStream {
    terms::lit(input)
}

///Create a [`core_lang::syntax::terms::Xtor`] with chirality
///[`core_lang::syntax::terms::Prd`] (i.e. a constructor)
/// with given name, arguments and type
#[proc_macro]
pub fn ctor(input: TokenStream) -> TokenStream {
    unfocused_xtor(input, Chirality::Prd)
}

///Create a [`core_lang::syntax::terms::Xtor`] with chirality
///[`core_lang::syntax::terms::Cns`] (i.e. a destructor)
/// with given name, arguments and type
#[proc_macro]
pub fn dtor(input: TokenStream) -> TokenStream {
    unfocused_xtor(input, Chirality::Cns)
}

///Create a [`core_lang::syntax::terms::xtor::FsXtor`] with chirality
///[`core_lang::syntax::Prd`], that is, a focused constructor
/// with given name, arguments and type
#[proc_macro]
pub fn fs_ctor(input: TokenStream) -> TokenStream {
    fs_xtor(input, Chirality::Prd)
}

///Create a [`core_lang::syntax::terms::xtor::FsXtor`] with chirality
///[`core_lang::syntax::Cns`], that is, a focussed destructor
/// with given name, arguments and type
#[proc_macro]
pub fn fs_dtor(input: TokenStream) -> TokenStream {
    fs_xtor(input, Chirality::Cns)
}

///Create a [`core_lang::syntax::terms::clause::Clause`]
/// with given chirality, name, arguments and right-hand side
#[proc_macro]
pub fn clause(input: TokenStream) -> TokenStream {
    terms::unfocused_clause(input)
}

///Create a [`core_lang::syntax::terms::clause::FsClause`]
/// with given chirality, name, arguments and right-hand side
#[proc_macro]
pub fn fs_clause(input: TokenStream) -> TokenStream {
    terms::fs_clause(input)
}

///Create a [`core_lang::syntax::terms::xcase::XCase`] with chirality
///[`core_lang::syntax::Cns`], i.e. a case expression,
/// with  given clauses and continuation type.
///If the continuation type is
///not specified, it defaults to [`core_lang::syntax::types::Ty::I64`]
#[proc_macro]
pub fn case(input: TokenStream) -> TokenStream {
    xcase(input, Chirality::Cns)
}

///Create a [`core_lang::syntax::terms::xcase::XCase`] with chirality
///[`core_lang::syntax::Cns`] i.e. a cocase / new expression
/// with given clauses and return type.
/// If the return type is not specified it defaults to [`core_lang::syntax::types::Ty::I64`]
///
#[proc_macro]
pub fn cocase(input: TokenStream) -> TokenStream {
    xcase(input, Chirality::Prd)
}

///Create a [`core_lang::syntax::terms::mu::Mu`] with chirality
///[`core_lang::syntax::Prd`], that is, a mu-binding,
///with given variable name, bound statement andtype
///If no type is provided, defaults to [`core_lang::syntax::types::Ty::I64`]
#[proc_macro]
pub fn mu(input: TokenStream) -> TokenStream {
    terms::unfocused_xmu(input, Chirality::Prd)
}

///Create a [`core_lang::syntax::terms::mu::Mu`] with chirality
///[`core_lang::syntax::Cns`], that is, a mu-tilde-binding
///with given variable name, bound statement and type.
///If no type is provided, defaults to [`core_lang::syntax::types::Ty::I64`]
#[proc_macro]
pub fn mutilde(input: TokenStream) -> TokenStream {
    terms::unfocused_xmu(input, Chirality::Cns)
}

///Create a [`core_lang::syntax::terms::mu::FsMu`] with chirality
///[`core_lang::syntax::Prd`] that is, a focused mu binding
/// with given variable name, bound statement and type.
/// If no type is provided, it defaults to [`core_lang::syntax::types::Ty::I64`]
#[proc_macro]
pub fn fs_mu(input: TokenStream) -> TokenStream {
    terms::fs_xmu(input, Chirality::Prd)
}

///Create a [`core_lang::syntax::terms::mu::FsMu`] with chirality
///[`core_lang::syntax::Cns`] that is, a focused mu-tilde binding
/// with given variable, bound statement and type.
/// If no type is provided, it defaults to [`core_lang::syntax::types::Ty::I64`]
#[proc_macro]
pub fn fs_mutilde(input: TokenStream) -> TokenStream {
    terms::fs_xmu(input, Chirality::Cns)
}

// Statements

///Create a [`core_lang::syntax::statements::Cut`]
/// with given producer, consumer and type.
/// If no type is provided, it will default to [`core_lang::syntax::types::Ty::I64`]
#[proc_macro]
pub fn cut(input: TokenStream) -> TokenStream {
    statements::unfocused_cut(input)
}

///Create a [`core_lang::syntax::statements::cut::FsCut`]
///with given producer, consumer and tpye.
///If no type is provided, it defaults to [`core_lang::syntax::types::Ty::I64`]
#[proc_macro]
pub fn fs_cut(input: TokenStream) -> TokenStream {
    statements::fs_cut(input)
}

///Create a [`core_lang::syntax::statements::ifc::IfC`] with comparison
///[`core_lang::syntax::statements::ifc::IfSort::Equal`]
/// and given first, second, then statement and else statement.
/// If only one comparison argument is provided, default to using zero (i.e. `IfC.snd == None`)
#[proc_macro]
pub fn ife(input: TokenStream) -> TokenStream {
    statements::unfocused_ife(input)
}

///Create a [`core_lang::syntax::statements::ifc::IfC`] with comparison
///[`core_lang::syntax::statements::ifc::IfSort::NotEqual`]
///and given first, second, then statement and else statement.
///If only one comparison argument is provided, default to using zero (i.e. `IfC.snd == None`)
#[proc_macro]
pub fn ifne(input: TokenStream) -> TokenStream {
    statements::unfocused_ifne(input)
}

///Create a [`core_lang::syntax::statements::ifc::IfC`] with comparison
///[`core_lang::syntax::statements::ifc::IfSort::Less`]
///and given first, second, then statement and else statement.
///If only one comparison argument is provided, default to using zero (i.e. `IfC.snd == None`)
#[proc_macro]
pub fn ifl(input: TokenStream) -> TokenStream {
    statements::unfocused_ifl(input)
}

///Create a [`core_lang::syntax::statements::ifc::IfC`] with comparison
///[`core_lang::syntax::statements::ifc::IfSort::LessOrEqual`]
///and given first, second, then statement and else statement.
///If only one comparison argument is provided, default to using zero (i.e. `IfC.snd == None`)
#[proc_macro]
pub fn ifle(input: TokenStream) -> TokenStream {
    statements::unfocused_ifle(input)
}

///Create a [`core_lang::syntax::statements::ifc::IfC`] with comparison
///[`core_lang::syntax::statements::ifc::IfSort::Greater`]
///and given first, second, then statement and else statement.
///If only one comparison argument is provided, default to using zero (i.e. `IfC.snd == None`)
#[proc_macro]
pub fn ifg(input: TokenStream) -> TokenStream {
    statements::unfocused_ifg(input)
}

///Create a [`core_lang::syntax::statements::ifc::IfC`] with comparison
///[`core_lang::syntax::statements::ifc::IfSort::GreaterOrEqual`]
///and given first, second, then statement and else statement.
///If only one comparison argument is provided, default to using zero (i.e. `IfC.snd == None`)
#[proc_macro]
pub fn ifge(input: TokenStream) -> TokenStream {
    statements::unfocused_ifge(input)
}

///Create a [`core_lang::syntax::statements::ifc::FsIfC`] with comparison
///[`core_lang::syntax::statements::ifc::IfSort::Equal`]
///and given first, second, then statement and else statement.
///If only one comparison argument is provided, default to using zero (i.e. `IfC.snd == None`)
#[proc_macro]
pub fn fs_ife(input: TokenStream) -> TokenStream {
    statements::fs_ife(input)
}

///Create a [`core_lang::syntax::statements::ifc::FsIfC`] with comparison
///[`core_lang::syntax::statements::ifc::IfSort::NotEqual`]
///and given first, second, then statement and else statement.
///If only one comparison argument is provided, default to using zero (i.e. `IfC.snd == None`)
#[proc_macro]
pub fn fs_ifne(input: TokenStream) -> TokenStream {
    statements::fs_ifne(input)
}

///Create a [`core_lang::syntax::statements::ifc::FsIfC`] with comparison
///[`core_lang::syntax::statements::ifc::IfSort::Less`]
///and given first, second, then statement and else statement.
///If only one comparison argument is provided, default to using zero (i.e. `IfC.snd == None`)
#[proc_macro]
pub fn fs_ifl(input: TokenStream) -> TokenStream {
    statements::fs_ifl(input)
}

///Create a [`core_lang::syntax::statements::ifc::FsIfC`] with comparison
///[`core_lang::syntax::statements::ifc::IfSort::LessOrEqual`]
///and given first, second, then statement and else statement.
///If only one comparison argument is provided, default to using zero (i.e. `IfC.snd == None`)
#[proc_macro]
pub fn fs_ifle(input: TokenStream) -> TokenStream {
    statements::fs_ifle(input)
}

///Create a [`core_lang::syntax::statements::ifc::FsIfC`] with comparison
///[`core_lang::syntax::statements::ifc::IfSort::Greater`]
///and given first, second, then statement and else statement.
///If only one comparison argument is provided, default to using zero (i.e. `IfC.snd == None`)
#[proc_macro]
pub fn fs_ifg(input: TokenStream) -> TokenStream {
    statements::fs_ifg(input)
}

///Create a [`core_lang::syntax::statements::ifc::FsIfC`] with comparison
///[`core_lang::syntax::statements::ifc::IfSort::GreaterOrEqual`]
///and given first, second, then statement and else statement.
///If only one comparison argument is provided, default to using zero (i.e. `IfC.snd == None`)
#[proc_macro]
pub fn fs_ifge(input: TokenStream) -> TokenStream {
    statements::fs_ifge(input)
}

///Create a [`core_lang::syntax::statements::Call`]
///with given label, arugments and type.
///If no return type is provided it will default to [`core_lang::syntax::types::Ty::I64`]
#[proc_macro]
pub fn call(input: TokenStream) -> TokenStream {
    statements::unfocused_call(input)
}

///Create a [`core_lang::syntax::statements::call::FsCall`]
/// with given label and arguments
#[proc_macro]
pub fn fs_call(input: TokenStream) -> TokenStream {
    statements::fs_call(input)
}

///Create a [`core_lang::syntax::statements::Exit`] with given term and type
///If no type is provided, the type will default to `[core_lang::syntax::types::Ty::I64`]
#[proc_macro]
pub fn exit(input: TokenStream) -> TokenStream {
    statements::exit(input)
}

///Create a [`core_lang::syntax::statements::FsExit`]
///with given variable
#[proc_macro]
pub fn fs_exit(input: TokenStream) -> TokenStream {
    statements::fs_exit(input)
}

///Create a [`core_lang::syntax::terms::op::Op`] with operation
///[`core_lang::syntax::terms::op::BinOp::Div`] and given first and second operand.
#[proc_macro]
pub fn div(input: TokenStream) -> TokenStream {
    terms::unfocused_div(input)
}

///Create a [`core_lang::syntax::terms::op::Op`] with operation
///[`core_lang::syntax::terms::op::BinOp::Prod`] and given first and second operand.
#[proc_macro]
pub fn prod(input: TokenStream) -> TokenStream {
    terms::unfocused_prod(input)
}

///Create a [`core_lang::syntax::terms::op::Op`] with operation
///[`core_lang::syntax::terms::op::BinOp::Rem`] and given first and second operand
#[proc_macro]
pub fn rem(input: TokenStream) -> TokenStream {
    terms::unfocused_rem(input)
}

///Create a [`core_lang::syntax::terms::op::Op`] with operation
///[`core_lang::syntax::terms::op::BinOp::Sub`] and given first and second operand
#[proc_macro]
pub fn sub(input: TokenStream) -> TokenStream {
    terms::unfocused_sub(input)
}

///Create a [`core_lang::syntax::terms::op::Op`] with operation
///[`core_lang::syntax::terms::op::BinOp::Sum`] and given first and second operand
#[proc_macro]
pub fn sum(input: TokenStream) -> TokenStream {
    terms::unfocused_sum(input)
}

///Create a [`core_lang::syntax::terms::op::FsOp`] with operation
///[`core_lang::syntax::terms::op::BinOp::Div`], that is a focused divistion term,
/// and given first and second operand variables
#[proc_macro]
pub fn fs_div(input: TokenStream) -> TokenStream {
    terms::fs_div(input)
}

///Create a [`core_lang::syntax::terms::op::FsOp`] with operation
///[`core_lang::syntax::terms::op::BinOp::Prod`], that is a focused divistion term,
/// and given first and second operand variables
#[proc_macro]
pub fn fs_prod(input: TokenStream) -> TokenStream {
    terms::fs_prod(input)
}

///Create a [`core_lang::syntax::terms::op::FsOp`] with operation
///[`core_lang::syntax::terms::op::BinOp::Rem`], that is a focused divistion term,
///and given first and second operand variables
#[proc_macro]
pub fn fs_rem(input: TokenStream) -> TokenStream {
    terms::fs_rem(input)
}

///Create a [`core_lang::syntax::terms::op::FsOp`] with operation
///[`core_lang::syntax::terms::op::BinOp::Sub`], that is a focused divistion term,
///and given first and second operand variables
#[proc_macro]
pub fn fs_sub(input: TokenStream) -> TokenStream {
    terms::fs_sub(input)
}

///Create a [`core_lang::syntax::terms::op::FsOp`] with operation
///[`core_lang::syntax::terms::op::BinOp::Sum`], that is a focused divistion term,
/// and given first and second operand variables
#[proc_macro]
pub fn fs_sum(input: TokenStream) -> TokenStream {
    terms::fs_sum(input)
}

// Declarations

///Create a [`core_lang::syntax::def::Def`]
/// with given name, arguments, body and used variables
/// if no used variables are provided, defaults to `HashSet::new()`
#[proc_macro]
pub fn def(input: TokenStream) -> TokenStream {
    declarations::unfocused_def(input)
}

///Create a [`core_lang::syntax::def::FsDef`]
/// with given name, arguments, body and used vars.
/// If no used vars are provided, defaults to `HashSet::new()`
#[proc_macro]
pub fn fs_def(input: TokenStream) -> TokenStream {
    declarations::fs_def(input)
}

///Create a [`core_lang::syntax::declaration::DataDeclaration`]
/// with given name and list of constructors
#[proc_macro]
pub fn data(input: TokenStream) -> TokenStream {
    declarations::data(input)
}

///Create a [`core_lang::syntax::declaration::CodataDeclaration`]
/// with given name and list of destructors
#[proc_macro]
pub fn codata(input: TokenStream) -> TokenStream {
    declarations::codata(input)
}

///Create a [`core_lang::syntax::declaration::CtorSig`]
/// with given name and arguments
#[proc_macro]
pub fn ctor_sig(input: TokenStream) -> TokenStream {
    declarations::ctor_sig(input)
}

///Create a [`core_lang::syntax::declaration::DtorSig`]
///with given name and arguments
#[proc_macro]
pub fn dtor_sig(input: TokenStream) -> TokenStream {
    declarations::dtor_sig(input)
}

///Create a [`core_lang::syntax::program::Prog`]
///with given lists of definitions, data declarations and codata declarations
#[proc_macro]
pub fn prog(input: TokenStream) -> TokenStream {
    prog::prog(input)
}
