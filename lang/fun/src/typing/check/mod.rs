use std::collections::HashSet;

use codespan::Span;
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
        Name,
    },
    typing::symbol_table::{build_symbol_table, Polarity},
};

use super::{errors::Error, symbol_table::SymbolTable};

pub fn check_module(module: &Module) -> Result<(), Error> {
    let symbol_table = build_symbol_table(module)?;
    check_module_with_table(module, &symbol_table)
}

fn check_module_with_table(module: &Module, symbol_table: &SymbolTable) -> Result<(), Error> {
    for decl in module.declarations.iter() {
        check_declaration(decl, symbol_table)?
    }
    Ok(())
}

// Lookup functions
//
//

fn lookup_ty_for_dtor(
    span: &SourceSpan,
    dtor: &Name,
    symbol_table: &SymbolTable,
) -> Result<Ty, Error> {
    for (ty_ctor, (pol, xtors)) in symbol_table.ty_ctors.iter() {
        if pol == &Polarity::Codata && xtors.contains(dtor) {
            return Ok(Ty::Decl {
                span: Span::default(),
                name: ty_ctor.to_string(),
            });
        }
    }
    Err(Error::Undefined {
        span: *span,
        name: dtor.clone(),
    })
}

fn lookup_ty_for_ctor(
    span: &SourceSpan,
    ctor: &Name,
    symbol_table: &SymbolTable,
) -> Result<(Ty, HashSet<String>), Error> {
    for (ty_ctor, (pol, xtors)) in symbol_table.ty_ctors.iter() {
        if pol == &Polarity::Data && xtors.contains(ctor) {
            return Ok((
                Ty::Decl {
                    span: Span::default(),
                    name: ty_ctor.to_string(),
                },
                xtors.iter().cloned().collect(),
            ));
        }
    }
    Err(Error::Undefined {
        span: *span,
        name: ctor.clone(),
    })
}

// Checking types and typing contexts
//
//

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

// Checking terms
//
//

fn check_args(
    span: &SourceSpan,
    symbol_table: &SymbolTable,
    context: &TypingContext,
    args: &Substitution,
    types: &TypingContext,
) -> Result<(), Error> {
    if types.len() != args.len() {
        return Err(Error::WrongNumberOfArguments {
            span: *span,
            expected: types.len(),
            got: args.len(),
        });
    }
    for c in types.iter().zip(args.iter()) {
        match c {
            (ContextBinding::TypedVar { ty, .. }, SubstitutionBinding::TermBinding(term)) => {
                term.check(symbol_table, context, ty)?
            }
            (ContextBinding::TypedCovar { ty, .. }, SubstitutionBinding::CovarBinding(cov)) => {
                let found_ty = lookup_covar(span, context, cov)?;
                check_equality(span, ty, &found_ty)?;
            }
            (ContextBinding::TypedVar { .. }, SubstitutionBinding::CovarBinding(_)) => {
                return Err(Error::ExpectedTermGotCovariable { span: *span })
            }
            (ContextBinding::TypedCovar { .. }, SubstitutionBinding::TermBinding(..)) => {
                return Err(Error::ExpectedCovariableGotTerm { span: *span })
            }
        }
    }
    Ok(())
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
