use std::collections::HashSet;

use codespan::Span;
use miette::SourceSpan;
use printer::Print;

use crate::{
    parser::util::ToMiette,
    syntax::{
        context::{ContextBinding, TypingContext},
        types::Ty,
        Covariable, Name, Variable,
    },
    typing::symbol_table::Polarity,
};

use super::{errors::Error, symbol_table::SymbolTable};
pub mod context;
pub mod declaration;
pub mod terms;

// Lookup functions
//
//

pub fn lookup_var(
    span: &SourceSpan,
    ctx: &TypingContext,
    searched_var: &Variable,
) -> Result<Ty, Error> {
    // Due to variable shadowing we have to traverse from
    // right to left.
    for binding in ctx.iter().rev() {
        match binding {
            ContextBinding::TypedVar { var, ty } => {
                if var == searched_var {
                    return Ok(ty.clone());
                }
                continue;
            }
            ContextBinding::TypedCovar { .. } => continue,
        }
    }
    Err(Error::UnboundVariable {
        span: *span,
        var: searched_var.clone(),
    })
}

fn lookup_covar(
    span: &SourceSpan,
    ctx: &TypingContext,
    searched_covar: &Covariable,
) -> Result<Ty, Error> {
    // Due to variable shadowing we have to traverse from
    // right to left.
    for binding in ctx.iter().rev() {
        match binding {
            ContextBinding::TypedVar { .. } => continue,
            ContextBinding::TypedCovar { covar, ty } => {
                if covar == searched_covar {
                    return Ok(ty.clone());
                }
                continue;
            }
        }
    }
    Err(Error::UnboundCovariable {
        span: *span,
        covar: searched_covar.clone(),
    })
}

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

fn check_annot(ty: &Ty, annot: &Option<Ty>, span: &SourceSpan) -> Result<(), Error> {
    match annot {
        None => Ok(()),
        Some(annot_ty) => {
            if ty == annot_ty {
                Ok(())
            } else {
                Err(Error::TypingContextMismatch { span: *span })
            }
        }
    }
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

pub fn check_equality(span: &SourceSpan, expected: &Ty, got: &Ty) -> Result<(), Error> {
    if expected != got {
        return Err(Error::Mismatch {
            span: *span,
            expected: expected.print_to_string(Default::default()),
            got: got.print_to_string(Default::default()),
        });
    }
    Ok(())
}
