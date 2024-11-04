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

#[cfg(test)]
mod check_test {
    use super::{
        check_annot, check_equality, check_type, lookup_covar, lookup_ty_for_ctor,
        lookup_ty_for_dtor, lookup_var,
    };
    use crate::{
        parser::util::ToMiette,
        syntax::{context::ContextBinding, types::Ty},
        typing::symbol_table::{Polarity, SymbolTable},
    };
    use codespan::Span;
    use std::collections::HashSet;

    #[test]
    fn var_lookup() {
        let result = lookup_var(
            &Span::default().to_miette(),
            &vec![ContextBinding::TypedVar {
                var: "x".to_owned(),
                ty: Ty::mk_int(),
            }],
            &"x".to_owned(),
        )
        .unwrap();
        let expected = Ty::mk_int();
        assert_eq!(result, expected)
    }

    #[test]
    fn var_lookup_fail() {
        let result = lookup_var(&Span::default().to_miette(), &vec![], &"x".to_owned());
        assert!(result.is_err())
    }

    #[test]
    fn covar_lookup() {
        let result = lookup_covar(
            &Span::default().to_miette(),
            &vec![ContextBinding::TypedCovar {
                covar: "a".to_owned(),
                ty: Ty::mk_int(),
            }],
            &"a".to_owned(),
        )
        .unwrap();
        let expected = Ty::mk_int();
        assert_eq!(result, expected)
    }

    #[test]
    fn covar_lookup_fail() {
        let result = lookup_covar(&Span::default().to_miette(), &vec![], &"a".to_owned());
        assert!(result.is_err())
    }

    #[test]
    fn dtor_lookup() {
        let mut symbol_table = SymbolTable::default();
        symbol_table.ty_ctors.insert(
            "LPairIntInt".to_owned(),
            (Polarity::Codata, vec!["Fst".to_owned(), "Snd".to_owned()]),
        );
        let result = lookup_ty_for_dtor(
            &Span::default().to_miette(),
            &"Fst".to_owned(),
            &symbol_table,
        )
        .unwrap();
        let expected = Ty::mk_decl("LPairIntInt");
        assert_eq!(result, expected)
    }

    #[test]
    fn dtor_lookup_fail() {
        let result = lookup_ty_for_dtor(
            &Span::default().to_miette(),
            &"Snd".to_owned(),
            &SymbolTable::default(),
        );
        assert!(result.is_err())
    }

    #[test]
    fn ctor_lookup() {
        let mut symbol_table = SymbolTable::default();
        symbol_table.ty_ctors.insert(
            "ListInt".to_owned(),
            (Polarity::Data, vec!["Nil".to_owned(), "Cons".to_owned()]),
        );
        let result = lookup_ty_for_ctor(
            &Span::default().to_miette(),
            &"Nil".to_owned(),
            &symbol_table,
        )
        .unwrap();
        let expected = (
            Ty::mk_decl("ListInt"),
            HashSet::from(["Nil".to_owned(), "Cons".to_owned()]),
        );
        assert_eq!(result, expected)
    }

    #[test]
    fn ctor_lookup_fail() {
        let result = lookup_ty_for_ctor(
            &Span::default().to_miette(),
            &"Nil".to_owned(),
            &SymbolTable::default(),
        );
        assert!(result.is_err())
    }

    #[test]
    fn annot_check_none() {
        let result = check_annot(&Ty::mk_int(), &None, &Span::default().to_miette());
        assert!(result.is_ok())
    }

    #[test]
    fn annot_check_some() {
        let result = check_annot(
            &Ty::mk_int(),
            &Some(Ty::mk_int()),
            &Span::default().to_miette(),
        );
        assert!(result.is_ok())
    }

    #[test]
    fn annot_check_fail() {
        let result = check_annot(
            &Ty::mk_int(),
            &Some(Ty::mk_decl("ListInt")),
            &Span::default().to_miette(),
        );
        assert!(result.is_err())
    }

    #[test]
    fn ty_check_int() {
        let result = check_type(&Ty::mk_int(), &SymbolTable::default());
        assert!(result.is_ok())
    }

    #[test]
    fn ty_check_decl() {
        let mut symbol_table = SymbolTable::default();
        symbol_table
            .ty_ctors
            .insert("ListInt".to_owned(), (Polarity::Data, vec![]));
        let result = check_type(&Ty::mk_decl("ListInt"), &symbol_table);
        assert!(result.is_ok())
    }

    #[test]
    fn ty_check_fail() {
        let result = check_type(&Ty::mk_decl("ListInt"), &SymbolTable::default());
        assert!(result.is_err())
    }

    #[test]
    fn equality_check() {
        let result = check_equality(&Span::default().to_miette(), &Ty::mk_int(), &Ty::mk_int());
        assert!(result.is_ok())
    }

    #[test]
    fn equality_check_fail() {
        let result = check_equality(
            &Span::default().to_miette(),
            &Ty::mk_int(),
            &Ty::mk_decl("ListInt"),
        );
        assert!(result.is_err())
    }
}
