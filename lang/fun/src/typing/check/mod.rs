use miette::SourceSpan;
use printer::Print;

pub mod case;
pub mod cocase;
pub mod context;
pub mod ctor;
pub mod declarations;
pub mod dtor;
pub mod fun;
pub mod goto;
pub mod ifz;
pub mod label;
pub mod let_exp;
pub mod lit;
pub mod op;
pub mod paren;
pub mod terms;
pub mod var;

use context::lookup_covar;
use declarations::check_declaration;
use terms::Check;

use crate::{
    parser::util::ToMiette,
    syntax::{
        context::{ContextBinding, TypingContext},
        declarations::Module,
        substitution::{Substitution, SubstitutionBinding},
        types::Ty,
    },
    typing::symbol_table::build_symbol_table,
};

use super::{errors::Error, symbol_table::SymbolTable};

pub fn check_module(module: Module) -> Result<Module, Error> {
    let symbol_table = build_symbol_table(&module)?;
    check_module_with_table(module, &symbol_table)
}

fn check_module_with_table(module: Module, symbol_table: &SymbolTable) -> Result<Module, Error> {
    let mut new_decls = vec![];
    for decl in module.declarations.into_iter() {
        let decl_checked = check_declaration(decl, symbol_table)?;
        new_decls.push(decl_checked);
    }
    Ok(Module {
        declarations: new_decls,
    })
}

pub fn check_type(ty: &Ty, symbol_table: &SymbolTable) -> Result<(), Error> {
    match ty {
        Ty::Int { .. } => Ok(()),
        Ty::Decl { span, name } => match symbol_table.ty_ctors.get(name) {
            None => Err(Error::Undefined {
                span: span.to_miette(),
                name: name.clone(),
            }),
            Some(_) => Ok(()),
        },
    }
}

fn check_args(
    span: &SourceSpan,
    symbol_table: &SymbolTable,
    context: &TypingContext,
    args: Substitution,
    types: &TypingContext,
) -> Result<Substitution, Error> {
    if types.len() != args.len() {
        return Err(Error::WrongNumberOfArguments {
            span: *span,
            expected: types.len(),
            got: args.len(),
        });
    }
    let mut new_subst = vec![];
    for c in args.into_iter().zip(types.iter()) {
        match c {
            (SubstitutionBinding::TermBinding(term), ContextBinding::TypedVar { ty, .. }) => {
                let term_checked = term.check(symbol_table, context, ty)?;
                new_subst.push(SubstitutionBinding::TermBinding(term_checked));
            }
            (SubstitutionBinding::CovarBinding(cov), ContextBinding::TypedCovar { ty, .. }) => {
                let found_ty = lookup_covar(span, context, &cov)?;
                check_equality(span, ty, &found_ty)?;
                new_subst.push(SubstitutionBinding::CovarBinding(cov));
            }
            (SubstitutionBinding::CovarBinding(_), ContextBinding::TypedVar { .. }) => {
                return Err(Error::ExpectedTermGotCovariable { span: *span })
            }
            (SubstitutionBinding::TermBinding(..), ContextBinding::TypedCovar { .. }) => {
                return Err(Error::ExpectedCovariableGotTerm { span: *span })
            }
        }
    }
    Ok(new_subst)
}

fn check_equality(span: &SourceSpan, expected: &Ty, got: &Ty) -> Result<(), Error> {
    if expected != got {
        return Err(Error::Mismatch {
            span: *span,
            expected: expected.print_to_string(Default::default()),
            got: got.print_to_string(Default::default()),
        });
    }
    Ok(())
}
