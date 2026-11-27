//! This module defines a clause in a [match](crate::syntax::terms::Case) or a
//! [comatch](crate::syntax::terms::New) in Fun.

use derivative::Derivative;
use miette::SourceSpan;
use printer::tokens::{COMMA, FAT_ARROW};
use printer::*;

use crate::syntax::*;
use crate::traits::*;
use crate::typing::{
    Error, SymbolTable,
    inference::{TypeInference, TypeInferenceState, TypeVarMapping, World},
};

use std::collections::HashSet;

/// This struct defines a clause in a match or a comatch in Fun. It consists of a
/// [polarity](Polarity) that determines whether it is in a match (of a data type) or a comatch
/// (of a codata type), of a name of the corresponding xtor, of the context it binds for the
/// arguments, and of the body.
///
/// Example:
/// ```text
/// Cons(x, xs) => 1 + len(xs)
/// ```
/// Matches the constructor `Cons` with arguments `x` and `xs` that can be used in the body.
#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct Clause {
    /// The source location
    #[derivative(PartialEq = "ignore")]
    pub span: SourceSpan,
    /// Whether we have a clause of a match or comatch
    pub pol: Polarity,
    /// The name of the xtor
    pub xtor: Name,
    /// The names (without types) to which the arguments of the xtor are bound
    pub context_names: NameContext,
    /// The bindings (with types) to which the arguments of the xtor are bound
    pub context: TypingContext,
    /// The body of the pattern
    pub body: Term,
}

impl OptTyped for Clause {
    fn get_type(&self) -> Option<Ty> {
        self.body.get_type()
    }
}

impl Print for Clause {
    fn print<'a>(&'a self, cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        let xtor = match self.pol {
            Polarity::Data => alloc.ctor(&self.xtor),
            Polarity::Codata => alloc.dtor(&self.xtor),
        };
        xtor.append(self.context_names.print(cfg, alloc))
            .append(alloc.space())
            .append(FAT_ARROW)
            .align()
            .append(alloc.line())
            .append(self.body.print(cfg, alloc).group())
            .nest(cfg.indent)
    }
}

pub fn print_clauses<'a>(
    clauses: &'a [Clause],
    cfg: &PrintCfg,
    alloc: &'a Alloc<'a>,
) -> Builder<'a> {
    match clauses.len() {
        0 => alloc.space().braces_anno().group(),
        1 => alloc
            .line()
            .append(clauses[0].print(cfg, alloc).group())
            .nest(cfg.indent)
            .append(alloc.line())
            .braces_anno()
            .group(),
        _ => {
            let sep = alloc.text(COMMA).append(alloc.hardline());
            alloc
                .hardline()
                .append(
                    alloc.intersperse(
                        clauses
                            .iter()
                            .map(|clause| clause.print(cfg, alloc).group()),
                        sep,
                    ),
                )
                .nest(cfg.indent)
                .append(alloc.hardline())
                .braces_anno()
        }
    }
}

impl Clause {
    /// This function collects the constraints for a clause of a match or comatch. The arguments of
    /// the xtor are bound to the names of the clause, and the body must have the expected type in
    /// the outer context extended by these bindings.
    /// - `xtor_params` are the parameters of the xtor, with the type parameters of its type
    ///   template already instantiated.
    /// - `context` is the typing context of the match or comatch.
    /// - `state` is the inference state.
    /// - `expected_ty` is the type the body must have.
    pub fn collect_constraints(
        &mut self,
        xtor_params: &TypingContext,
        context: &TypingContext,
        state: &mut TypeInferenceState,
        expected_ty: Ty,
    ) -> Result<(), Error> {
        self.context_names.no_dups(&self.xtor)?;
        self.context = self.context_names.add_types(xtor_params)?;

        // the bindings of the clause are appended to the outer context; since variables are looked
        // up from right to left, they shadow outer bindings with the same name
        let mut body_context = context.clone();
        body_context
            .bindings
            .extend(self.context.bindings.iter().cloned());

        self.body
            .collect_constraints(state, &body_context, expected_ty)
    }

    /// This function inserts the inferred types into the body and the bindings of a clause, see
    /// [`TypeInference::insert_inferred_types`].
    pub fn insert_inferred_types(
        &mut self,
        type_vars: &TypeVarMapping,
        symbol_table: &mut SymbolTable,
        world: &World,
    ) -> Result<(), Error> {
        self.body
            .insert_inferred_types(type_vars, symbol_table, world)?;
        for binding in &mut self.context.bindings {
            binding
                .ty
                .insert_inferred(&self.context.span, type_vars, symbol_table)?;
        }
        Ok(())
    }
}

/// This function checks that the clauses of a match or comatch cover every xtor of the matched
/// type exactly once, and sorts them in the order of the declaration of the type, which the later
/// stages of the compiler rely on.
/// - `clauses` are the clauses, all of the given polarity.
/// - `polarity` is [`Polarity::Data`] for a match and [`Polarity::Codata`] for a comatch.
/// - `declared_xtors` are the xtors of the type in the order of their declaration.
/// - `span` is the source location of the match or comatch.
pub fn check_and_order_clauses(
    clauses: &mut Vec<Clause>,
    polarity: Polarity,
    declared_xtors: &[Name],
    span: SourceSpan,
) -> Result<(), Error> {
    let mut remaining = std::mem::take(clauses);

    for xtor in declared_xtors {
        match remaining.iter().position(|clause| &clause.xtor == xtor) {
            Some(position) => clauses.push(remaining.remove(position)),
            None => {
                return Err(match polarity {
                    Polarity::Data => Error::MissingCtorInCase {
                        span,
                        ctor: xtor.clone(),
                    },
                    Polarity::Codata => Error::MissingDtorInNew {
                        span,
                        dtor: xtor.clone(),
                    },
                });
            }
        }
    }

    // every clause left over is a second clause for an xtor
    if !remaining.is_empty() {
        let xtors = remaining
            .iter()
            .map(|clause| clause.xtor.clone())
            .collect::<Vec<_>>()
            .print_to_string(None);
        return Err(match polarity {
            Polarity::Data => Error::UnexpectedCtorsInCase { span, ctors: xtors },
            Polarity::Codata => Error::UnexpectedDtorsInNew { span, dtors: xtors },
        });
    }

    Ok(())
}

impl UsedBinders for Clause {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        for binding in &self.context_names.bindings {
            used.insert(binding.clone());
        }
        self.body.used_binders(used);
    }
}
