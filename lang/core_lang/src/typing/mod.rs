//! Type checking for annotated, unfocused Core with flat matches.
//!
//! Provides a validation pass intended for the output of pattern compilation,
//! before focusing. Terms and statements are checked against declarations
//! and the current typing context.
//!
//! Assumes well-formed data and codata declarations and binders renamed
//! to avoid shadowing collisions.

use crate::pattern_typing::{
    PatternError, PatternSignature, extend_context, prepare_clause_contexts,
};
use crate::syntax::arguments::Argument;
use crate::{
    syntax::{
        AnyPattern, Arguments, Chi, Chirality, Clause, Cns, ContextBinding, Def, Identifier,
        NestedClause, NestedXCase, Pattern, PatternVec, Prd, Prog, Statement, SubjectType, Term,
        Ty, TypingContext, XCase, XVar, Xtor, subject_typing,
    },
    traits::Typed,
};

/// A borrowed view of the declarations available during Core type checking.
///
/// Contains all top-level definitions, including definitions that occur
/// later in the program and definitions used recursively.
pub struct TypingSignature<'a> {
    /// Data and codata declarations used to resolve types and xtor signatures.
    pub patterns: PatternSignature<'a>,
    /// Top-level definitions used to check call arguments.
    pub defs: &'a [Def],
}

impl<'a> TypingSignature<'a> {
    /// Finds a top-level definition by its identifier.
    fn find_def(&self, name: &Identifier) -> Option<&Def> {
        self.defs.iter().find(|def| &def.name == name)
    }
}

/// Errors encountered while checking annotated Core programs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypingError {
    /// A variable or covariable is absent from the current typing context.
    UnboundBinder { name: Identifier },
    /// The actual chirality differs from the expected chirality.
    WrongChirality {
        expected: Chirality,
        found: Chirality,
    },
    /// The actual type annotation differs from the expected type.
    WrongType { expected: Ty, found: Ty },
    /// An argument list has a different length than its declared context.
    WrongNumberOfArguments { expected: usize, found: usize },
    /// A call refers to an unknown top-level definition.
    DefinitionNotFound { name: Identifier },
    /// Pattern checking or context extension failed.
    Pattern(PatternError),
    /// Flat branch names differ from the declarations in number or order.
    WrongFlatBranches {
        expected: Vec<Identifier>,
        found: Vec<Identifier>,
    },
    /// A type name refers to neither a data nor a codata declaration.
    TypeNotFound { name: Identifier },
    /// More than one top-level definition has the same identifier.
    DuplicateDefinition { name: Identifier },
}

/// Wraps pattern errors so they can propagate through Core checking with '?'.
impl From<PatternError> for TypingError {
    fn from(error: PatternError) -> Self {
        TypingError::Pattern(error)
    }
}

/// Returns the syntactic chirality of a term.
///
/// Literals and arithmetic operations are always producers.
fn term_chirality<C: Chi>(term: &Term<C>) -> Chirality {
    let marker = match term {
        Term::XVar(xvar) => &xvar.prdcns,
        Term::Mu(mu) => &mu.prdcns,
        Term::Xtor(xtor) => &xtor.prdcns,
        Term::XCase(xcase) => &xcase.prdcns,
        Term::Literal(_) => return Chirality::Prd,
        Term::Op(_) => return Chirality::Prd,
    };

    if marker.is_prd() {
        Chirality::Prd
    } else {
        Chirality::Cns
    }
}

/// Accepts I64 and names declared as data or codata types.
///
/// Checks name resolution without validating the declaration contents.
fn check_type_exists(ty: &Ty, signature: &TypingSignature<'_>) -> Result<(), TypingError> {
    match ty {
        Ty::I64 => Ok(()),
        Ty::Decl(name) => {
            for declaration in signature.patterns.data_types {
                if &declaration.name == name {
                    return Ok(());
                }
            }
            for declaration in signature.patterns.codata_types {
                if &declaration.name == name {
                    return Ok(());
                }
            }
            Err(TypingError::TypeNotFound { name: name.clone() })
        }
    }
}

/// Checks a term's outer chirality and type against one expected position.
///
/// The term's internal structure is checked separately by 'check_term'.
fn check_term_annotation<C: Chi>(
    term: &Term<C>,
    expected: &SubjectType,
) -> Result<(), TypingError> {
    let term_chi = term_chirality(term);
    if term_chi != expected.chi {
        return Err(TypingError::WrongChirality {
            expected: expected.chi.clone(),
            found: term_chi.clone(),
        });
    }

    let term_ty = term.get_type();
    if term_ty != expected.ty {
        return Err(TypingError::WrongType {
            expected: expected.ty.clone(),
            found: term_ty.clone(),
        });
    }

    Ok(())
}

/// Checks a variable or covariable occurrence against its context binding.
///
/// The identifier must be bound with matching chirality and type.
fn check_xvar<C: Chi>(xvar: &XVar<C>, context: &TypingContext) -> Result<(), TypingError> {
    let binding = match context.lookup_binding(&xvar.var) {
        Some(binding) => binding,
        None => {
            return Err(TypingError::UnboundBinder {
                name: xvar.var.clone(),
            });
        }
    };

    let var_chi = if xvar.prdcns.is_prd() {
        Chirality::Prd
    } else {
        Chirality::Cns
    };

    if var_chi != binding.chi {
        return Err(TypingError::WrongChirality {
            expected: binding.chi.clone(),

            found: var_chi.clone(),
        });
    }

    if xvar.ty != binding.ty {
        return Err(TypingError::WrongType {
            expected: binding.ty.clone(),
            found: xvar.ty.clone(),
        });
    }

    Ok(())
}

/// Checks an ordered argument list against the subject-typing vector Omega.
///
/// Requires equal lengths and recursively checks each argument against
/// the expected chirality and type at the corresponding position.
fn check_arguments(
    args: &Arguments,
    expected: &[SubjectType],
    context: &TypingContext,
    signature: &TypingSignature<'_>,
) -> Result<(), TypingError> {
    if args.entries.len() != expected.len() {
        return Err(TypingError::WrongNumberOfArguments {
            expected: expected.len(),
            found: args.entries.len(),
        });
    }

    for (arg, subject_type) in args.entries.iter().zip(expected.iter()) {
        let chi = match arg {
            Argument::Producer(_) => Chirality::Prd,
            Argument::Consumer(_) => Chirality::Cns,
        };
        if chi != subject_type.chi {
            return Err(TypingError::WrongChirality {
                expected: subject_type.chi.clone(),
                found: chi.clone(),
            });
        }
        match arg {
            Argument::Producer(p) => check_term(p, subject_type, context, signature)?,
            Argument::Consumer(c) => check_term(c, subject_type, context, signature)?,
        }
    }

    Ok(())
}

/// Represents a flat clause context as an ordered vector of binder patterns.
///
/// Preserves identifiers, chiralities, and types so the existing pattern
/// checker can validate the context against an xtor declaration.
fn context_patterns(context: &TypingContext) -> PatternVec {
    let mut res = vec![];

    for binding in &context.bindings {
        let pattern = match binding.chi {
            Chirality::Prd => AnyPattern::Producer(Pattern::XVar(XVar {
                prdcns: Prd,
                var: binding.var.clone(),
                ty: binding.ty.clone(),
            })),
            Chirality::Cns => AnyPattern::Consumer(Pattern::XVar(XVar {
                prdcns: Cns,
                var: binding.var.clone(),
                ty: binding.ty.clone(),
            })),
        };

        res.push(pattern);
    }

    res
}

/// Represents a flat clause using the nested pattern syntax for type checking.
///
/// The clause's xtor becomes the outer pattern and its context becomes
/// the argument pattern vector. The statement body is shared unchanged.
fn flat_clause_to_nested<C: Chi>(clause: &Clause<C>, match_ty: &Ty) -> NestedClause {
    let pattern_vec = context_patterns(&clause.context);
    // A case consumer inspects producer patterns - a new producer handles consumer patterns.
    let outer_pattern_chi = if clause.prdcns.is_prd() {
        Chirality::Cns
    } else {
        Chirality::Prd
    };

    let xtor_pattern = match outer_pattern_chi {
        Chirality::Prd => AnyPattern::Producer(Pattern::Xtor(Xtor {
            prdcns: Prd,
            name: clause.xtor.clone(),
            args: pattern_vec,
            ty: match_ty.clone(),
        })),
        Chirality::Cns => AnyPattern::Consumer(Pattern::Xtor(Xtor {
            prdcns: Cns,
            name: clause.xtor.clone(),
            args: pattern_vec,
            ty: match_ty.clone(),
        })),
    };

    NestedClause {
        pattern: xtor_pattern,
        body: clause.body.clone(),
    }
}

/// Adapts a flat match or comatch to the shared pattern-checking representation.
///
/// Preserves the term's chirality, type, clause order, and bodies.
fn flat_xcase_to_nested<C: Chi>(xcase: &XCase<C>) -> NestedXCase<C> {
    let mut clause_vec = vec![];

    for clause in &xcase.clauses {
        let nested_clause = flat_clause_to_nested(clause, &xcase.ty);
        clause_vec.push(nested_clause);
    }

    NestedXCase {
        prdcns: xcase.prdcns.clone(),
        clauses: clause_vec,
        ty: xcase.ty.clone(),
    }
}

/// Checks clause patterns and their statement bodies.
///
/// Each body is checked under the outer context extended by the binders
/// introduced by its own pattern.
fn check_xcase_patterns_and_bodies<C: Chi>(
    xcase: &NestedXCase<C>,
    context: &TypingContext,
    signature: &TypingSignature<'_>,
) -> Result<(), TypingError> {
    // Each clause receives its own Gamma extended by its pattern context Delta.
    let clause_contexts = prepare_clause_contexts(xcase, context, &signature.patterns)?;

    for (clause, clause_context) in xcase.clauses.iter().zip(clause_contexts.iter()) {
        check_statement(clause.body.as_ref(), clause_context, signature)?;
    }

    Ok(())
}

/// Requires exactly the declared xtor names in declaration order.
///
/// Flat Core uses this fixed order for positional branch dispatch.
fn check_flat_branch_names<C: Chi>(
    xcase: &XCase<C>,
    expected: &[Identifier],
) -> Result<(), TypingError> {
    let mut names = vec![];
    for clause in &xcase.clauses {
        names.push(clause.xtor.clone());
    }

    if names.as_slice() != expected {
        return Err(TypingError::WrongFlatBranches {
            expected: expected.to_vec(),
            found: names,
        });
    }

    Ok(())
}

/// Checks the branch layout, field contexts, and bodies of a flat XCase.
///
/// Reuses nested pattern checking by representing each flat field binding
/// as a binder pattern.
fn check_flat_xcase<C: Chi>(
    xcase: &XCase<C>,
    context: &TypingContext,
    signature: &TypingSignature<'_>,
) -> Result<(), TypingError> {
    let expected_names = signature
        .patterns
        .expected_xtor_names(&xcase.ty, &xcase.prdcns)?;
    check_flat_branch_names(xcase, &expected_names)?;

    let nested_xcase = flat_xcase_to_nested(xcase);
    check_xcase_patterns_and_bodies(&nested_xcase, context, signature)
}

/// Checks a term against an expected chirality and type.
///
/// Validates the outer annotation and recursively checks the term's
/// structure under the current typing context.
fn check_term<C: Chi>(
    term: &Term<C>,
    expected: &SubjectType,
    context: &TypingContext,
    signature: &TypingSignature<'_>,
) -> Result<(), TypingError> {
    check_type_exists(&expected.ty, signature)?;
    check_term_annotation(term, expected)?;

    match term {
        Term::XVar(xvar) => check_xvar(xvar, context),
        Term::Mu(mu) => {
            // A producer mu binds a consumer - a consumer mu-tilde binds a producer.
            let binder_chi = if mu.prdcns.is_prd() {
                Chirality::Cns
            } else {
                Chirality::Prd
            };

            let typing_context = TypingContext {
                bindings: vec![ContextBinding {
                    var: mu.variable.clone(),
                    chi: binder_chi.clone(),
                    ty: mu.ty.clone(),
                }],
            };

            let extended_context = extend_context(context, &typing_context)?;

            check_statement(mu.statement.as_ref(), &extended_context, signature)
        }
        Term::Xtor(xtor) => {
            // Obtain argument expectations from the declaration rather than
            // trusting the annotations of the supplied arguments.
            let xtor_context = signature.patterns.xtor_context(xtor)?;
            let omega = subject_typing(xtor_context);
            check_arguments(&xtor.args, &omega, context, signature)
        }
        Term::XCase(xcase) => check_flat_xcase(xcase, context, signature),
        // Literal types are fixed to I64 - the outer annotation check is sufficient.
        Term::Literal(_) => Ok(()),
        Term::Op(op) => {
            let op_expected = SubjectType {
                chi: Chirality::Prd,
                ty: Ty::I64,
            };
            check_term(op.fst.as_ref(), &op_expected, context, signature)?;
            check_term(op.snd.as_ref(), &op_expected, context, signature)?;
            Ok(())
        }
    }
}

/// Checks a command under the current typing context.
///
/// Statements are checked for well-typedness without an expected result type.
fn check_statement(
    statement: &Statement,
    context: &TypingContext,
    signature: &TypingSignature<'_>,
) -> Result<(), TypingError> {
    match statement {
        Statement::Cut(cut) => {
            let expected_prd = SubjectType {
                chi: Chirality::Prd,
                ty: cut.ty.clone(),
            };
            let expected_cns = SubjectType {
                chi: Chirality::Cns,
                ty: cut.ty.clone(),
            };
            // Both sides of a cut must have the cut's type and opposite chiralities.
            check_term(cut.producer.as_ref(), &expected_prd, context, signature)?;
            check_term(cut.consumer.as_ref(), &expected_cns, context, signature)?;
            Ok(())
        }
        Statement::IfC(ifc) => {
            let expected_ifc = SubjectType {
                chi: Chirality::Prd,
                ty: Ty::I64,
            };
            check_term(ifc.fst.as_ref(), &expected_ifc, context, signature)?;
            if let Some(s) = &ifc.snd {
                check_term(s.as_ref(), &expected_ifc, context, signature)?;
            }
            check_statement(ifc.thenc.as_ref(), context, signature)?;
            check_statement(ifc.elsec.as_ref(), context, signature)?;
            Ok(())
        }
        Statement::PrintI64(print) => {
            let expected_print = SubjectType {
                chi: Chirality::Prd,
                ty: Ty::I64,
            };
            check_term(print.arg.as_ref(), &expected_print, context, signature)?;
            check_statement(print.next.as_ref(), context, signature)
        }
        Statement::Call(call) => {
            // Check arguments against the callee's parameter context.
            // Its body is checked separately by 'check_program', allowing recursion.
            let omega = match signature.find_def(&call.name) {
                Some(def) => subject_typing(&def.context),
                None => {
                    return Err(TypingError::DefinitionNotFound {
                        name: call.name.clone(),
                    });
                }
            };
            check_arguments(&call.args, &omega, context, signature)
        }
        Statement::Exit(exit) => {
            let expected_exit = SubjectType {
                chi: Chirality::Prd,
                ty: Ty::I64,
            };
            check_term(exit.arg.as_ref(), &expected_exit, context, signature)
        }
    }
}

/// Checks that context identifiers are distinct and all binding types exist.
///
/// Also validates bindings that are never used in a statement.
fn check_context(
    context: &TypingContext,
    signature: &TypingSignature<'_>,
) -> Result<(), TypingError> {
    // Extending an empty context detects duplicates within this context.
    extend_context(&TypingContext::default(), context)?;
    for binding in &context.bindings {
        check_type_exists(&binding.ty, signature)?;
    }

    Ok(())
}

/// Checks a definition's parameter context and its body under that context.
fn check_def(def: &Def, signature: &TypingSignature<'_>) -> Result<(), TypingError> {
    check_context(&def.context, signature)?;
    check_statement(&def.body, &def.context, signature)
}

/// Checks all definitions of an annotated Core program.
///
/// Rejects duplicate definition identifiers and checks every parameter
/// context and body, including definitions that are never called.
///
/// Assumes well-formed data and codata declarations and binders renamed
/// to avoid shadowing collisions. Call after uniquification and before focusing.
pub fn check_program(program: &Prog) -> Result<(), TypingError> {
    // Make all definitions available before checking bodies, so forward
    // references and recursive calls can be resolved.
    let signature = TypingSignature {
        patterns: PatternSignature {
            data_types: &program.data_types,
            codata_types: &program.codata_types,
        },
        defs: &program.defs,
    };

    let mut names = vec![];
    for def in &program.defs {
        if names.contains(&def.name) {
            return Err(TypingError::DuplicateDefinition {
                name: def.name.clone(),
            });
        }
        names.push(def.name.clone());
        check_def(def, &signature)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syntax::{Cns, ContextBinding, Prd};

    fn context_with_x(chi: Chirality, ty: Ty) -> TypingContext {
        TypingContext {
            bindings: vec![ContextBinding {
                var: Identifier::new("x".to_string()),
                chi,
                ty,
            }],
        }
    }

    #[test]
    fn matching_producer_variable_is_accepted() {
        let context = context_with_x(Chirality::Prd, Ty::I64);
        let variable = XVar {
            prdcns: Prd,
            var: Identifier::new("x".to_string()),
            ty: Ty::I64,
        };

        let res = check_xvar(&variable, &context);

        assert_eq!(res, Ok(()));
    }

    #[test]
    fn matching_consumer_variable_is_accepted() {
        let context = context_with_x(Chirality::Cns, Ty::I64);
        let variable = XVar {
            prdcns: Cns,
            var: Identifier::new("x".to_string()),
            ty: Ty::I64,
        };

        let res = check_xvar(&variable, &context);

        assert_eq!(res, Ok(()));
    }

    #[test]
    fn unbound_variable_is_rejected() {
        let context = context_with_x(Chirality::Prd, Ty::I64);
        let y_name = Identifier::new("y".to_string());
        let variable = XVar {
            prdcns: Prd,
            var: y_name.clone(),
            ty: Ty::I64,
        };

        let res = check_xvar(&variable, &context);

        assert_eq!(res, Err(TypingError::UnboundBinder { name: y_name }));
    }

    #[test]
    fn variable_with_wrong_chirality_is_rejected() {
        let context = context_with_x(Chirality::Cns, Ty::I64);
        let variable = XVar {
            prdcns: Prd,
            var: Identifier::new("x".to_string()),
            ty: Ty::I64,
        };

        let res = check_xvar(&variable, &context);

        assert_eq!(
            res,
            Err(TypingError::WrongChirality {
                expected: Chirality::Cns,
                found: Chirality::Prd,
            })
        );
    }

    #[test]
    fn variable_with_wrong_type_is_rejected() {
        let context = context_with_x(Chirality::Prd, Ty::I64);
        let nat = Ty::Decl(Identifier::new("Nat".to_string()));
        let variable = XVar {
            prdcns: Prd,
            var: Identifier::new("x".to_string()),
            ty: nat.clone(),
        };

        let res = check_xvar(&variable, &context);

        assert_eq!(
            res,
            Err(TypingError::WrongType {
                expected: Ty::I64,
                found: nat,
            })
        );
    }
}

/// Tests program validation, definition lookup and context validity.
#[cfg(test)]
mod program_tests {
    use super::*;
    use crate::syntax::{
        Call, Codata, CodataDeclaration, Cut, Data, DataDeclaration, Exit, Literal, Mu,
    };
    use std::rc::Rc;

    fn name(text: &str) -> Identifier {
        Identifier::new(text.to_string())
    }

    fn exit_zero() -> Statement {
        Statement::Exit(Exit {
            arg: Rc::new(Term::Literal(Literal { lit: 0 })),
            ty: Ty::I64,
        })
    }

    fn definition(text: &str, body: Statement) -> Def {
        Def {
            name: name(text),
            context: TypingContext::default(),
            body,
        }
    }

    fn program(defs: Vec<Def>) -> Prog {
        Prog {
            defs,
            data_types: vec![],
            codata_types: vec![],
            max_id: 0,
        }
    }

    fn producer_binding(text: &str, ty: Ty) -> ContextBinding {
        ContextBinding {
            var: name(text),
            chi: Chirality::Prd,
            ty,
        }
    }

    fn call_without_arguments(text: &str) -> Statement {
        Statement::Call(Call {
            name: name(text),
            args: Arguments { entries: vec![] },
            ty: Ty::I64,
        })
    }

    #[test]
    fn program_with_integer_exit_is_accepted() {
        let prog = program(vec![definition("main", exit_zero())]);

        assert_eq!(check_program(&prog), Ok(()));
    }

    #[test]
    fn declared_data_and_codata_parameter_types_are_accepted() {
        let mut def = definition("main", exit_zero());
        def.context.bindings = vec![
            producer_binding("x", Ty::Decl(name("Empty"))),
            ContextBinding {
                var: name("alpha"),
                chi: Chirality::Cns,
                ty: Ty::Decl(name("CoEmpty")),
            },
        ];

        let mut prog = program(vec![def]);
        prog.data_types.push(DataDeclaration {
            dat: Data,
            name: name("Empty"),
            xtors: vec![],
        });
        prog.codata_types.push(CodataDeclaration {
            dat: Codata,
            name: name("CoEmpty"),
            xtors: vec![],
        });

        assert_eq!(check_program(&prog), Ok(()));
    }

    #[test]
    fn unused_parameter_with_unknown_type_is_rejected() {
        let mut def = definition("main", exit_zero());
        def.context
            .bindings
            .push(producer_binding("unused", Ty::Decl(name("Ghost"))));

        let prog = program(vec![def]);

        assert_eq!(
            check_program(&prog),
            Err(TypingError::TypeNotFound {
                name: name("Ghost"),
            })
        );
    }

    #[test]
    fn duplicate_parameters_are_rejected() {
        let mut def = definition("main", exit_zero());
        def.context.bindings = vec![
            producer_binding("x", Ty::I64),
            producer_binding("x", Ty::I64),
        ];

        let prog = program(vec![def]);

        assert_eq!(
            check_program(&prog),
            Err(TypingError::Pattern(PatternError::BinderAlreadyInContext {
                name: name("x"),
            }))
        );
    }

    #[test]
    fn duplicate_definition_names_are_rejected() {
        let prog = program(vec![
            definition("main", exit_zero()),
            definition("main", exit_zero()),
        ]);

        assert_eq!(
            check_program(&prog),
            Err(TypingError::DuplicateDefinition { name: name("main") })
        );
    }

    #[test]
    fn invalid_uncalled_definition_body_is_rejected() {
        let invalid_body = Statement::Exit(Exit {
            arg: Rc::new(Term::XVar(XVar {
                prdcns: Prd,
                var: name("missing"),
                ty: Ty::I64,
            })),
            ty: Ty::I64,
        });

        let prog = program(vec![
            definition("main", exit_zero()),
            definition("unused", invalid_body),
        ]);

        assert_eq!(
            check_program(&prog),
            Err(TypingError::UnboundBinder {
                name: name("missing"),
            })
        );
    }

    #[test]
    fn call_to_later_definition_is_accepted() {
        let prog = program(vec![
            definition("main", call_without_arguments("later")),
            definition("later", exit_zero()),
        ]);

        assert_eq!(check_program(&prog), Ok(()));
    }

    #[test]
    fn recursive_call_is_accepted_without_checking_body_again() {
        let prog = program(vec![definition("main", call_without_arguments("main"))]);

        assert_eq!(check_program(&prog), Ok(()));
    }

    #[test]
    fn call_to_unknown_definition_is_rejected() {
        let prog = program(vec![definition("main", call_without_arguments("missing"))]);

        assert_eq!(
            check_program(&prog),
            Err(TypingError::DefinitionNotFound {
                name: name("missing"),
            })
        );
    }

    #[test]
    fn call_with_missing_argument_is_rejected() {
        let mut target = definition("target", exit_zero());
        target.context.bindings.push(producer_binding("x", Ty::I64));

        let prog = program(vec![
            definition("main", call_without_arguments("target")),
            target,
        ]);

        assert_eq!(
            check_program(&prog),
            Err(TypingError::WrongNumberOfArguments {
                expected: 1,
                found: 0,
            })
        );
    }

    #[test]
    fn cut_with_mu_terms_of_unknown_type_is_rejected() {
        let ghost = Ty::Decl(name("Ghost"));

        let body = Statement::Cut(Cut {
            producer: Rc::new(Term::Mu(Mu {
                prdcns: Prd,
                variable: name("alpha"),
                statement: Rc::new(exit_zero()),
                ty: ghost.clone(),
            })),
            consumer: Rc::new(Term::Mu(Mu {
                prdcns: Cns,
                variable: name("x"),
                statement: Rc::new(exit_zero()),
                ty: ghost.clone(),
            })),
            ty: ghost,
        });

        let prog = program(vec![definition("main", body)]);

        assert_eq!(
            check_program(&prog),
            Err(TypingError::TypeNotFound {
                name: name("Ghost"),
            })
        );
    }
}

/// Tests recursive checking of terms, statements and flat matches.
#[cfg(test)]
mod structural_tests {
    use super::*;
    use crate::syntax::statements::print::PrintI64;
    use crate::syntax::{
        BinOp, Codata, CodataDeclaration, CtorSig, Cut, Data, DataDeclaration, DtorSig, Exit, IfC,
        IfSort, Literal, Mu, Op,
    };
    use std::rc::Rc;

    fn id(text: &str) -> Identifier {
        Identifier::new(text.to_string())
    }

    fn declared(text: &str) -> Ty {
        Ty::Decl(id(text))
    }

    fn binding(text: &str, chi: Chirality, ty: Ty) -> TypingContext {
        TypingContext {
            bindings: vec![ContextBinding {
                var: id(text),
                chi,
                ty,
            }],
        }
    }

    fn integer() -> Term<Prd> {
        Term::Literal(Literal { lit: 0 })
    }

    fn variable(text: &str) -> Term<Prd> {
        Term::XVar(XVar {
            prdcns: Prd,
            var: id(text),
            ty: Ty::I64,
        })
    }

    fn covariable(text: &str) -> Term<Cns> {
        Term::XVar(XVar {
            prdcns: Cns,
            var: id(text),
            ty: Ty::I64,
        })
    }

    fn exit(arg: Term<Prd>) -> Statement {
        Statement::Exit(Exit {
            arg: Rc::new(arg),
            ty: Ty::I64,
        })
    }

    fn cut(producer: Term<Prd>, consumer: Term<Cns>) -> Statement {
        Statement::Cut(Cut {
            producer: Rc::new(producer),
            consumer: Rc::new(consumer),
            ty: Ty::I64,
        })
    }

    fn environment() -> Prog {
        Prog {
            defs: vec![],
            data_types: vec![DataDeclaration {
                dat: Data,
                name: id("Choice"),
                xtors: vec![
                    CtorSig {
                        xtor: Data,
                        name: id("Stop"),
                        args: TypingContext::default(),
                    },
                    CtorSig {
                        xtor: Data,
                        name: id("Value"),
                        args: binding("field", Chirality::Prd, Ty::I64),
                    },
                ],
            }],
            codata_types: vec![CodataDeclaration {
                dat: Codata,
                name: id("Stream"),
                xtors: vec![
                    DtorSig {
                        xtor: Codata,
                        name: id("head"),
                        args: binding("result", Chirality::Cns, Ty::I64),
                    },
                    DtorSig {
                        xtor: Codata,
                        name: id("tail"),
                        args: binding("result", Chirality::Cns, declared("Stream")),
                    },
                ],
            }],
            max_id: 0,
        }
    }

    fn signature_for(prog: &Prog) -> TypingSignature<'_> {
        TypingSignature {
            patterns: PatternSignature {
                data_types: &prog.data_types,
                codata_types: &prog.codata_types,
            },
            defs: &prog.defs,
        }
    }

    fn check_test_term<C: Chi>(term: Term<C>, chi: Chirality, ty: Ty) -> Result<(), TypingError> {
        let prog = environment();

        check_term(
            &term,
            &SubjectType { chi, ty },
            &TypingContext::default(),
            &signature_for(&prog),
        )
    }

    fn check_test_statement(statement: Statement) -> Result<(), TypingError> {
        let prog = environment();

        check_statement(&statement, &TypingContext::default(), &signature_for(&prog))
    }

    fn value(args: Vec<Argument>) -> Term<Prd> {
        Term::Xtor(Xtor {
            prdcns: Prd,
            name: id("Value"),
            args: Arguments { entries: args },
            ty: declared("Choice"),
        })
    }

    fn flat_case() -> XCase<Cns> {
        XCase {
            prdcns: Cns,
            ty: declared("Choice"),
            clauses: vec![
                Clause {
                    prdcns: Cns,
                    xtor: id("Stop"),
                    context: TypingContext::default(),
                    body: Rc::new(exit(integer())),
                },
                Clause {
                    prdcns: Cns,
                    xtor: id("Value"),
                    context: binding("x", Chirality::Prd, Ty::I64),
                    body: Rc::new(exit(variable("x"))),
                },
            ],
        }
    }

    fn flat_new() -> XCase<Prd> {
        XCase {
            prdcns: Prd,
            ty: declared("Stream"),
            clauses: vec![
                Clause {
                    prdcns: Prd,
                    xtor: id("head"),
                    context: binding("alpha", Chirality::Cns, Ty::I64),
                    body: Rc::new(cut(integer(), covariable("alpha"))),
                },
                Clause {
                    prdcns: Prd,
                    xtor: id("tail"),
                    context: binding("beta", Chirality::Cns, declared("Stream")),
                    body: Rc::new(exit(integer())),
                },
            ],
        }
    }

    #[test]
    fn mu_introduces_a_consumer_binding() {
        let term = Term::Mu(Mu {
            prdcns: Prd,
            variable: id("alpha"),
            statement: Rc::new(cut(integer(), covariable("alpha"))),
            ty: Ty::I64,
        });

        assert_eq!(check_test_term(term, Chirality::Prd, Ty::I64), Ok(()));
    }

    #[test]
    fn mu_tilde_introduces_a_producer_binding() {
        let term = Term::Mu(Mu {
            prdcns: Cns,
            variable: id("x"),
            statement: Rc::new(exit(variable("x"))),
            ty: Ty::I64,
        });

        assert_eq!(check_test_term(term, Chirality::Cns, Ty::I64), Ok(()));
    }

    #[test]
    fn mu_body_is_checked() {
        let term = Term::Mu(Mu {
            prdcns: Prd,
            variable: id("alpha"),
            statement: Rc::new(exit(variable("missing"))),
            ty: Ty::I64,
        });

        assert_eq!(
            check_test_term(term, Chirality::Prd, Ty::I64),
            Err(TypingError::UnboundBinder {
                name: id("missing"),
            })
        );
    }

    #[test]
    fn constructor_term_with_matching_argument_is_accepted() {
        let term = value(vec![Argument::Producer(integer())]);

        assert_eq!(
            check_test_term(term, Chirality::Prd, declared("Choice")),
            Ok(())
        );
    }

    #[test]
    fn destructor_term_with_matching_consumer_is_accepted() {
        let consumer = Term::Mu(Mu {
            prdcns: Cns,
            variable: id("x"),
            statement: Rc::new(exit(variable("x"))),
            ty: Ty::I64,
        });

        let term = Term::Xtor(Xtor {
            prdcns: Cns,
            name: id("head"),
            args: Arguments {
                entries: vec![Argument::Consumer(consumer)],
            },
            ty: declared("Stream"),
        });

        assert_eq!(
            check_test_term(term, Chirality::Cns, declared("Stream")),
            Ok(())
        );
    }

    #[test]
    fn constructor_argument_with_wrong_chirality_is_rejected() {
        let term = value(vec![Argument::Consumer(covariable("alpha"))]);

        assert_eq!(
            check_test_term(term, Chirality::Prd, declared("Choice")),
            Err(TypingError::WrongChirality {
                expected: Chirality::Prd,
                found: Chirality::Cns,
            })
        );
    }

    #[test]
    fn constructor_argument_with_wrong_type_is_rejected() {
        let stop = Term::Xtor(Xtor {
            prdcns: Prd,
            name: id("Stop"),
            args: Arguments::default(),
            ty: declared("Choice"),
        });

        let term = value(vec![Argument::Producer(stop)]);

        assert_eq!(
            check_test_term(term, Chirality::Prd, declared("Choice")),
            Err(TypingError::WrongType {
                expected: Ty::I64,
                found: declared("Choice"),
            })
        );
    }

    #[test]
    fn constructor_argument_term_is_checked_recursively() {
        let term = value(vec![Argument::Producer(variable("missing"))]);

        assert_eq!(
            check_test_term(term, Chirality::Prd, declared("Choice")),
            Err(TypingError::UnboundBinder {
                name: id("missing"),
            })
        );
    }

    #[test]
    fn flat_case_body_can_use_its_pattern_binder() {
        assert_eq!(
            check_test_term(Term::XCase(flat_case()), Chirality::Cns, declared("Choice"),),
            Ok(())
        );
    }

    #[test]
    fn flat_case_rejects_missing_duplicate_and_reordered_branches() {
        let original = flat_case();

        let mut missing = original.clone();
        missing.clauses.pop();

        let mut duplicate = original.clone();
        duplicate.clauses[1] = duplicate.clauses[0].clone();

        let mut reordered = original;
        reordered.clauses.swap(0, 1);

        for case in [missing, duplicate, reordered] {
            let found = case
                .clauses
                .iter()
                .map(|clause| clause.xtor.clone())
                .collect();

            assert_eq!(
                check_test_term(Term::XCase(case), Chirality::Cns, declared("Choice"),),
                Err(TypingError::WrongFlatBranches {
                    expected: vec![id("Stop"), id("Value")],
                    found,
                })
            );
        }
    }

    #[test]
    fn flat_case_rejects_a_wrong_field_type() {
        let mut case = flat_case();
        case.clauses[1].context.bindings[0].ty = declared("Choice");

        assert_eq!(
            check_test_term(Term::XCase(case), Chirality::Cns, declared("Choice"),),
            Err(TypingError::Pattern(PatternError::WrongType {
                expected: Ty::I64,
                found: declared("Choice"),
            }))
        );
    }

    #[test]
    fn flat_case_checks_every_branch_body() {
        let mut case = flat_case();
        case.clauses[1].body = Rc::new(exit(variable("missing")));

        assert_eq!(
            check_test_term(Term::XCase(case), Chirality::Cns, declared("Choice"),),
            Err(TypingError::UnboundBinder {
                name: id("missing"),
            })
        );
    }

    #[test]
    fn flat_new_body_can_use_its_consumer_binder() {
        assert_eq!(
            check_test_term(Term::XCase(flat_new()), Chirality::Prd, declared("Stream"),),
            Ok(())
        );
    }

    #[test]
    fn flat_new_rejects_a_wrong_field_chirality() {
        let mut new = flat_new();
        new.clauses[0].context.bindings[0].chi = Chirality::Prd;

        assert_eq!(
            check_test_term(Term::XCase(new), Chirality::Prd, declared("Stream"),),
            Err(TypingError::Pattern(PatternError::WrongChirality {
                expected: Chirality::Cns,
                found: Chirality::Prd,
            }))
        );
    }

    #[test]
    fn flat_new_rejects_reordered_destructors() {
        let mut new = flat_new();
        new.clauses.swap(0, 1);

        assert_eq!(
            check_test_term(Term::XCase(new), Chirality::Prd, declared("Stream"),),
            Err(TypingError::WrongFlatBranches {
                expected: vec![id("head"), id("tail")],
                found: vec![id("tail"), id("head")],
            })
        );
    }

    #[test]
    fn flat_new_checks_its_branch_body() {
        let mut new = flat_new();
        new.clauses[0].body = Rc::new(cut(integer(), covariable("missing")));

        assert_eq!(
            check_test_term(Term::XCase(new), Chirality::Prd, declared("Stream"),),
            Err(TypingError::UnboundBinder {
                name: id("missing"),
            })
        );
    }

    #[test]
    fn operation_checks_its_second_operand() {
        let term: Term<Prd> = Term::Op(Op {
            fst: Rc::new(integer()),
            op: BinOp::Sum,
            snd: Rc::new(variable("missing")),
        });

        assert_eq!(
            check_test_term(term, Chirality::Prd, Ty::I64),
            Err(TypingError::UnboundBinder {
                name: id("missing"),
            })
        );
    }

    #[test]
    fn conditional_checks_else_branch_for_both_comparison_forms() {
        for snd in [None, Some(Rc::new(integer()))] {
            let statement = Statement::IfC(IfC {
                sort: IfSort::Equal,
                fst: Rc::new(integer()),
                snd,
                thenc: Rc::new(exit(integer())),
                elsec: Rc::new(exit(variable("missing"))),
            });

            assert_eq!(
                check_test_statement(statement),
                Err(TypingError::UnboundBinder {
                    name: id("missing"),
                })
            );
        }
    }

    #[test]
    fn print_checks_the_following_statement() {
        let statement = Statement::PrintI64(PrintI64 {
            newline: true,
            arg: Rc::new(integer()),
            next: Rc::new(exit(variable("missing"))),
        });

        assert_eq!(
            check_test_statement(statement),
            Err(TypingError::UnboundBinder {
                name: id("missing"),
            })
        );
    }

    #[test]
    fn arithmetic_conditional_and_print_can_be_combined() {
        let sum = Term::Op(Op {
            fst: Rc::new(integer()),
            op: BinOp::Sum,
            snd: Rc::new(integer()),
        });

        let statement = Statement::IfC(IfC {
            sort: IfSort::Equal,
            fst: Rc::new(sum),
            snd: Some(Rc::new(integer())),
            thenc: Rc::new(Statement::PrintI64(PrintI64 {
                newline: true,
                arg: Rc::new(integer()),
                next: Rc::new(exit(integer())),
            })),
            elsec: Rc::new(exit(integer())),
        });

        assert_eq!(check_test_statement(statement), Ok(()));
    }
}
