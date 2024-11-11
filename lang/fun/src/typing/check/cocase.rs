use super::{context::compare_typing_contexts, terms::Check};
use crate::{
    parser::util::ToMiette,
    syntax::{
        context::TypingContext,
        terms::{Clause, Cocase},
        types::Ty,
    },
    typing::{
        errors::Error,
        symbol_table::{Polarity, SymbolTable},
    },
};
use std::collections::HashSet;

impl Check for Cocase {
    fn check(
        self,
        symbol_table: &SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<Cocase, Error> {
        let name = match expected {
            Ty::Int { .. } => {
                return Err(Error::ExpectedIntForCocase {
                    span: self.span.to_miette(),
                })
            }
            Ty::Decl { name, .. } => name,
        };

        let mut expected_dtors: HashSet<String> = match symbol_table.ty_ctors.get(name) {
            Some((Polarity::Codata, dtors)) => dtors.iter().cloned().collect(),
            Some((Polarity::Data, _)) => {
                return Err(Error::ExpectedDataForCocase {
                    span: self.span.to_miette(),
                    data: name.clone(),
                })
            }
            None => {
                return Err(Error::Undefined {
                    span: self.span.to_miette(),
                    name: name.clone(),
                })
            }
        };

        let mut new_cocases = vec![];
        for cocase in self.cocases.into_iter() {
            if !expected_dtors.remove(&cocase.xtor) {
                return Err(Error::UnexpectedDtorInCocase {
                    span: cocase.span.to_miette(),
                    dtor: cocase.xtor.clone(),
                });
            }
            let (dtor_ctx, dtor_ret_ty) = match symbol_table.dtors.get(&cocase.xtor) {
                None => {
                    return Err(Error::Undefined {
                        span: self.span.to_miette(),
                        name: cocase.xtor.clone(),
                    })
                }
                Some(info) => info,
            };

            compare_typing_contexts(&cocase.span.to_miette(), dtor_ctx, &cocase.context)?;

            let mut new_context = context.clone();
            new_context.append(&mut cocase.context.clone());

            let new_rhs = cocase.rhs.check(symbol_table, &new_context, dtor_ret_ty)?;
            new_cocases.push(Clause {
                span: cocase.span,
                xtor: cocase.xtor,
                context: cocase.context,
                rhs: new_rhs,
            });
        }

        if !expected_dtors.is_empty() {
            return Err(Error::MissingDtorInCocase {
                span: self.span.to_miette(),
            });
        }
        Ok(Cocase {
            span: self.span,
            cocases: new_cocases,
            ty: Some(expected.clone()),
        })
    }
}
