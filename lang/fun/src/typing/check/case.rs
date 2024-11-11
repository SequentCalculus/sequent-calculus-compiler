use super::{context::compare_typing_contexts, declarations::lookup_ty_for_ctor, terms::Check};
use crate::{
    parser::util::ToMiette,
    syntax::{
        context::TypingContext,
        terms::{Case, Clause},
        types::Ty,
    },
    typing::{errors::Error, symbol_table::SymbolTable},
};

impl Check for Case {
    fn check(
        self,
        symbol_table: &SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<Case, Error> {
        // Find out the type on which we pattern match by inspecting the first case.
        // We throw an error for empty cases.
        let (ty, mut expected_ctors) = match self.cases.first() {
            Some(case) => lookup_ty_for_ctor(&self.span.to_miette(), &case.xtor, symbol_table)?,
            None => {
                return Err(Error::EmptyMatch {
                    span: self.span.to_miette(),
                })
            }
        };

        // We check the "e" in "case e of {...}" against this type.
        let destructee_checked = self.destructee.check(symbol_table, context, &ty)?;

        let mut new_cases = vec![];
        for case in self.cases.into_iter() {
            if !expected_ctors.remove(&case.xtor) {
                return Err(Error::UnexpectedCtorInCase {
                    span: case.span.to_miette(),
                    ctor: case.xtor.clone(),
                });
            }
            match symbol_table.ctors.get(&case.xtor) {
                Some(ctor_ctx) => {
                    compare_typing_contexts(&case.span.to_miette(), ctor_ctx, &case.context)?;

                    let mut new_context = context.clone();
                    new_context.append(&mut case.context.clone());

                    let new_rhs = case.rhs.check(symbol_table, &new_context, expected)?;
                    new_cases.push(Clause {
                        span: case.span,
                        xtor: case.xtor,
                        rhs: new_rhs,
                        context: case.context,
                    })
                }
                None => {
                    return Err(Error::Undefined {
                        span: case.span.to_miette(),
                        name: case.xtor.clone(),
                    })
                }
            }
        }
        if !expected_ctors.is_empty() {
            return Err(Error::MissingCtorsInCase {
                span: self.span.to_miette(),
            });
        }
        Ok(Case {
            span: self.span,
            destructee: destructee_checked,
            cases: new_cases,
            ty: Some(expected.clone()),
        })
    }
}
