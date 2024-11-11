use super::{context::compare_typing_contexts, lookup_ty_for_ctor, terms::Check};
use crate::{
    parser::util::ToMiette,
    syntax::{context::TypingContext, terms::Case, types::Ty},
    typing::{errors::Error, symbol_table::SymbolTable},
};

impl Check for Case {
    fn check(
        &self,
        symbol_table: &SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<(), Error> {
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
        self.destructee.check(symbol_table, context, &ty)?;

        for case in self.cases.iter() {
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

                    case.rhs.check(symbol_table, &new_context, expected)?;
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
        Ok(())
    }
}
