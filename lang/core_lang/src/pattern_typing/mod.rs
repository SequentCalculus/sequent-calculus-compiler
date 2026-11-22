//! Type checking for annotated nested Core patterns.
//!
//! Patterns are checked against expected subject types and declared xtor
//! signatures. Successful checks return the introduced binder context.
//!
//! Assumes well-formed declaration environment and expected types.
//! Does not check clause bodies or match exhaustiveness.

use crate::syntax::{
    AnyPattern, Chi, Chirality, CodataDeclaration, ContextBinding, CtorSig, DataDeclaration,
    DtorSig, Identifier, NestedClause, NestedXCase, Pattern, SubjectType, Ty, TypingContext, Xtor,
    subject_typing,
};

/// Errors encountered while checking pattern annotations and structure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PatternError {
    /// The pattern's chirality differs from the expected position.
    WrongChirality {
        expected: Chirality,
        found: Chirality,
    },
    /// The pattern's annotation differs from the expected type.
    WrongType { expected: Ty, found: Ty },
    /// An xtor pattern was checked against a primitive type.
    XtorRequiresDeclaredType { found: Ty },
    /// No matching xtor signature exists for the expected type and chirality.
    XtorSignatureNotFound {
        ty: Ty,
        chi: Chirality,
        name: Identifier,
    },
    /// The pattern vector and subject-type vector have different lengths.
    WrongNumberOfPatterns { expected: usize, found: usize },
    /// The same binder identifier occurs more than once in a pattern vector.
    DuplicateBinder { name: Identifier },
    /// A binder identifier is already present during context extension.
    BinderAlreadyInContext { name: Identifier },
    /// A case annotation does not refer to a declared data type.
    ExpectedDataType { found: Ty },
    /// A new annotation does not refer to a declared codata type.
    ExpectedCodataType { found: Ty },
}

/// Checks that the found chirality equals the expected chirality.
fn check_chirality(expected: &Chirality, found: &Chirality) -> Result<(), PatternError> {
    if expected == found {
        Ok(())
    } else {
        Err(PatternError::WrongChirality {
            expected: expected.clone(),
            found: found.clone(),
        })
    }
}

/// Checks equality of the expected and found Core types.
fn check_type(expected: &Ty, found: &Ty) -> Result<(), PatternError> {
    if expected == found {
        Ok(())
    } else {
        Err(PatternError::WrongType {
            expected: expected.clone(),
            found: found.clone(),
        })
    }
}

/// Checks a pattern's stored chirality and type against one expected position.
///
/// Does not inspect the pattern's arguments or validate xtor membership.
fn check_pattern_annotation<C: Chi>(
    pattern: &Pattern<C>,
    expected: &SubjectType,
) -> Result<(), PatternError> {
    let found_chirality = if pattern.chirality().is_prd() {
        Chirality::Prd
    } else {
        Chirality::Cns
    };

    check_chirality(&expected.chi, &found_chirality)?;
    check_type(&expected.ty, pattern.ty())?;

    Ok(())
}

/// A borrowed view of the data and codata declarations used for checking.
///
/// Represents the declaration information required from Theta.
pub struct PatternSignature<'a> {
    pub data_types: &'a [DataDeclaration],
    pub codata_types: &'a [CodataDeclaration],
}

impl<'a> PatternSignature<'a> {
    /// Finds a data declaration by its type name.
    fn find_data_type(&self, name: &Identifier) -> Option<&DataDeclaration> {
        self.data_types
            .iter()
            .find(|declaration| &declaration.name == name)
    }

    /// Finds a constructor only within the specified data declaration.
    fn find_constructor(
        &self,
        type_name: &Identifier,
        constructor_name: &Identifier,
    ) -> Option<&CtorSig> {
        let declaration = self.find_data_type(type_name)?;

        declaration
            .xtors
            .iter()
            .find(|xtor| &xtor.name == constructor_name)
    }

    /// Finds a codata declaration by its type name.
    fn find_codata_type(&self, name: &Identifier) -> Option<&CodataDeclaration> {
        self.codata_types
            .iter()
            .find(|declaration| &declaration.name == name)
    }

    /// Finds a destructor only within the specified codata declaration.
    fn find_destructor(
        &self,
        type_name: &Identifier,
        destructor_name: &Identifier,
    ) -> Option<&DtorSig> {
        let declaration = self.find_codata_type(type_name)?;

        declaration
            .xtors
            .iter()
            .find(|xtor| &xtor.name == destructor_name)
    }

    /// Resolves an xtor's declared argument context.
    ///
    /// Producer xtors are constructors of data types - consumer xtors are
    /// destructors of codata types. The generic argument representation allows
    /// this lookup to be shared by term checking and pattern checking.
    pub(crate) fn xtor_context<C: Chi, A>(
        &self,
        xtor: &Xtor<C, A>,
    ) -> Result<&TypingContext, PatternError> {
        let chi = if xtor.prdcns.is_prd() {
            Chirality::Prd
        } else {
            Chirality::Cns
        };

        let type_name = match &xtor.ty {
            Ty::I64 => {
                return Err(PatternError::XtorRequiresDeclaredType {
                    found: xtor.ty.clone(),
                });
            }
            Ty::Decl(d) => d,
        };

        match chi {
            Chirality::Prd => match self.find_constructor(type_name, &xtor.name) {
                Some(s) => Ok(&s.args),
                None => Err(PatternError::XtorSignatureNotFound {
                    ty: xtor.ty.clone(),
                    chi: chi.clone(),
                    name: xtor.name.clone(),
                }),
            },
            Chirality::Cns => match self.find_destructor(type_name, &xtor.name) {
                Some(s) => Ok(&s.args),
                None => Err(PatternError::XtorSignatureNotFound {
                    ty: xtor.ty.clone(),
                    chi: chi.clone(),
                    name: xtor.name.clone(),
                }),
            },
        }
    }

    /// Returns the required flat branch names in declaration order.
    ///
    /// A producer XCase represents new and requires codata destructors.
    /// A consumer XCase represents case and requires data constructors.
    pub(crate) fn expected_xtor_names<C: Chi>(
        &self,
        match_ty: &Ty,
        match_chi: &C,
    ) -> Result<Vec<Identifier>, PatternError> {
        let chi = if match_chi.is_prd() {
            Chirality::Prd
        } else {
            Chirality::Cns
        };

        match chi {
            Chirality::Prd => match match_ty {
                Ty::I64 => Err(PatternError::ExpectedCodataType {
                    found: match_ty.clone(),
                }),
                Ty::Decl(name) => match self.find_codata_type(name) {
                    Some(s) => {
                        let mut names = vec![];

                        for xtor in &s.xtors {
                            names.push(xtor.name.clone());
                        }

                        Ok(names)
                    }
                    None => Err(PatternError::ExpectedCodataType {
                        found: match_ty.clone(),
                    }),
                },
            },
            Chirality::Cns => match match_ty {
                Ty::I64 => Err(PatternError::ExpectedDataType {
                    found: match_ty.clone(),
                }),
                Ty::Decl(name) => match self.find_data_type(name) {
                    Some(s) => {
                        let mut names = vec![];

                        for xtor in &s.xtors {
                            names.push(xtor.name.clone());
                        }

                        Ok(names)
                    }
                    None => Err(PatternError::ExpectedDataType {
                        found: match_ty.clone(),
                    }),
                },
            },
        }
    }
}

/// Checks one annotated pattern against an expected subject position.
///
/// Binders introduce one context entry - wildcards introduce none.
/// Xtor patterns are checked recursively against their declared arguments.
/// Corresponds to the single-pattern typing judgement returning Delta.
fn check_pattern<C: Chi>(
    pattern: &Pattern<C>,
    expected: &SubjectType,
    signature: &PatternSignature<'_>,
) -> Result<TypingContext, PatternError> {
    check_pattern_annotation(pattern, expected)?;

    match pattern {
        Pattern::XVar(xvar) => Ok(TypingContext {
            bindings: vec![ContextBinding {
                var: xvar.var.clone(),
                chi: expected.chi.clone(),
                ty: expected.ty.clone(),
            }],
        }),
        Pattern::Wildcard { .. } => Ok(TypingContext::default()),
        Pattern::Xtor(xtor) => {
            // Xtor patterns require a declared data or codata type.
            let expected_context: &TypingContext = signature.xtor_context(xtor)?;

            // Derive argument expectations from the declaration, not from subpatterns.
            let omega = subject_typing(expected_context);

            check_patterns(&xtor.args, &omega, signature)
        }
    }
}

/// Checks an ordered pattern vector against the subject-typing vector Omega.
///
/// Requires equal lengths, checks corresponding positions, and combines
/// the resulting contexts in pattern order while rejecting duplicate
/// binder identifiers.
/// Corresponds to the pattern-vector typing judgement returning Delta.
pub fn check_patterns(
    patterns: &[AnyPattern],
    expected: &[SubjectType],
    signature: &PatternSignature<'_>,
) -> Result<TypingContext, PatternError> {
    if patterns.len() != expected.len() {
        return Err(PatternError::WrongNumberOfPatterns {
            expected: expected.len(),
            found: patterns.len(),
        });
    }

    let mut context = TypingContext::default();

    for (pattern, subject_type) in patterns.iter().zip(expected.iter()) {
        let checked_context = match pattern {
            AnyPattern::Producer(p) => check_pattern(p, subject_type, signature)?,
            AnyPattern::Consumer(c) => check_pattern(c, subject_type, signature)?,
        };

        for binder in checked_context.bindings {
            for collected_binder in &context.bindings {
                // Reject repeated identifiers before adding new binders to the result.
                if binder.var == collected_binder.var {
                    return Err(PatternError::DuplicateBinder {
                        name: binder.var.clone(),
                    });
                }
            }

            context.bindings.push(binder);
        }
    }

    Ok(context)
}

/// Derives the expected subject type for an outer clause pattern.
///
/// The pattern has the opposite chirality to the XCase term and the same type.
fn clause_subject_type<C: Chi>(xcase: &NestedXCase<C>) -> SubjectType {
    let clause_chi = if xcase.prdcns.is_prd() {
        Chirality::Cns
    } else {
        Chirality::Prd
    };
    let ty = xcase.ty.clone();

    SubjectType {
        chi: clause_chi,
        ty,
    }
}

/// Checks one clause pattern and returns its introduced binder context Delta.
fn check_clause_pattern(
    clause: &NestedClause,
    expected: &SubjectType,
    signature: &PatternSignature<'_>,
) -> Result<TypingContext, PatternError> {
    match &clause.pattern {
        AnyPattern::Producer(p) => check_pattern(p, expected, signature),
        AnyPattern::Consumer(c) => check_pattern(c, expected, signature),
    }
}

/// Extends Gamma with the supplied binders in order.
///
/// Rejects identifiers already present in Gamma or among earlier added
/// binders. Assumes Gamma is already valid and performs no type lookup.
pub(crate) fn extend_context(
    outer: &TypingContext,
    binders: &TypingContext,
) -> Result<TypingContext, PatternError> {
    let mut res_context = outer.clone();

    for binder in &binders.bindings {
        for existing_binder in &res_context.bindings {
            if binder.var == existing_binder.var {
                return Err(PatternError::BinderAlreadyInContext {
                    name: binder.var.clone(),
                });
            }
        }
        res_context.bindings.push(binder.clone());
    }

    Ok(res_context)
}

/// Requires a declared data type for case and a declared codata type for new.
fn check_xcase_type<C: Chi>(
    xcase: &NestedXCase<C>,
    signature: &PatternSignature<'_>,
) -> Result<(), PatternError> {
    if xcase.prdcns.is_prd() {
        match &xcase.ty {
            Ty::Decl(name) => match signature.find_codata_type(name) {
                Some(_) => Ok(()),
                None => Err(PatternError::ExpectedCodataType {
                    found: xcase.ty.clone(),
                }),
            },
            Ty::I64 => Err(PatternError::ExpectedCodataType {
                found: xcase.ty.clone(),
            }),
        }
    } else {
        match &xcase.ty {
            Ty::Decl(name) => match signature.find_data_type(name) {
                Some(_) => Ok(()),
                None => Err(PatternError::ExpectedDataType {
                    found: xcase.ty.clone(),
                }),
            },
            Ty::I64 => Err(PatternError::ExpectedDataType {
                found: xcase.ty.clone(),
            }),
        }
    }
}

/// Checks clause patterns and prepares their individual body contexts.
///
/// For each clause, derives Delta from its pattern and returns Gamma
/// extended by Delta. Results preserve clause order.
///
/// Statement bodies and exhaustiveness are checked separately.
pub(crate) fn prepare_clause_contexts<C: Chi>(
    xcase: &NestedXCase<C>,
    outer: &TypingContext,
    signature: &PatternSignature<'_>,
) -> Result<Vec<TypingContext>, PatternError> {
    check_xcase_type(xcase, signature)?;
    let expected_subject_type = clause_subject_type(xcase);
    let mut res = vec![];

    for clause in &xcase.clauses {
        // Each clause starts from the same outer Gamma - clause binders do not
        // extend the contexts of other clauses.
        let delta = check_clause_pattern(clause, &expected_subject_type, signature)?;
        let clause_context = extend_context(outer, &delta)?;

        res.push(clause_context);
    }

    Ok(res)
}

/// Tests variable and covariable lookup against typing contexts.
#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use std::vec;

    use crate::syntax::{
        Cns, Codata, Data, Exit, Identifier, Literal, Prd, Statement, Term, XVar, Xtor,
    };

    use super::*;

    #[test]
    fn same_chirality_is_accepted() {
        let res = check_chirality(&Chirality::Prd, &Chirality::Prd);
        assert_eq!(res, Ok(()));
    }

    #[test]
    fn different_chirality_is_rejected() {
        let res = check_chirality(&Chirality::Prd, &Chirality::Cns);
        assert_eq!(
            res,
            Err(PatternError::WrongChirality {
                expected: Chirality::Prd,
                found: Chirality::Cns,
            })
        )
    }

    #[test]
    fn same_type_is_accepted() {
        let nat = Ty::Decl(Identifier::new("Nat".to_string()));
        let res = check_type(&nat, &nat);
        assert_eq!(res, Ok(()));
    }

    #[test]
    fn different_type_is_rejected() {
        let nat = Ty::Decl(Identifier::new("Nat".to_string()));
        let bool_ty = Ty::Decl(Identifier::new("Bool".to_string()));
        let res = check_type(&nat, &bool_ty);
        assert_eq!(
            res,
            Err(PatternError::WrongType {
                expected: nat.clone(),
                found: bool_ty.clone(),
            })
        )
    }

    #[test]
    fn pattern_annotation_accepts_matching_binder() {
        let nat = Ty::Decl(Identifier::new("Nat".to_string()));
        let x_var = Pattern::XVar(XVar {
            prdcns: Prd,
            var: Identifier::new("x".to_string()),
            ty: nat.clone(),
        });
        let subject_type = SubjectType {
            chi: Chirality::Prd,
            ty: nat.clone(),
        };

        let res = check_pattern_annotation(&x_var, &subject_type);
        assert_eq!(res, Ok(()))
    }

    #[test]
    fn pattern_annotation_rejects_wrong_chirality() {
        let nat = Ty::Decl(Identifier::new("Nat".to_string()));
        let x_var = Pattern::XVar(XVar {
            prdcns: Prd,
            var: Identifier::new("x".to_string()),
            ty: nat.clone(),
        });
        let subject_type = SubjectType {
            chi: Chirality::Cns,
            ty: nat.clone(),
        };

        let res = check_pattern_annotation(&x_var, &subject_type);
        assert_eq!(
            res,
            Err(PatternError::WrongChirality {
                expected: Chirality::Cns,
                found: Chirality::Prd,
            })
        )
    }

    #[test]
    fn pattern_annotation_rejects_wrong_type() {
        let nat = Ty::Decl(Identifier::new("Nat".to_string()));
        let bool_ty = Ty::Decl(Identifier::new("Bool".to_string()));
        let x_var = Pattern::XVar(XVar {
            prdcns: Prd,
            var: Identifier::new("x".to_string()),
            ty: nat.clone(),
        });
        let subject_type = SubjectType {
            chi: Chirality::Prd,
            ty: bool_ty.clone(),
        };

        let res = check_pattern_annotation(&x_var, &subject_type);
        assert_eq!(
            res,
            Err(PatternError::WrongType {
                expected: bool_ty,
                found: nat,
            })
        )
    }

    #[test]
    fn find_data_type_returns_matching_declaration() {
        let empty_name = Identifier::new("Empty".to_string());
        let declaration = DataDeclaration {
            dat: Data,
            name: empty_name.clone(),
            xtors: vec![],
        };
        let data_types = vec![declaration.clone()];
        let signature = PatternSignature {
            data_types: &data_types,
            codata_types: &[],
        };

        let found = signature.find_data_type(&empty_name);

        assert_eq!(found, Some(&data_types[0]));
    }

    #[test]
    fn find_data_type_returns_none_for_unknown_name() {
        let empty_name = Identifier::new("Empty".to_string());
        let missing_name = Identifier::new("Missing".to_string());
        let declaration = DataDeclaration {
            dat: Data,
            name: empty_name.clone(),
            xtors: vec![],
        };
        let data_types = vec![declaration.clone()];
        let signature = PatternSignature {
            data_types: &data_types,
            codata_types: &[],
        };

        let found = signature.find_data_type(&missing_name);

        assert_eq!(found, None);
    }

    #[test]
    fn find_constructor_returns_matching_signature() {
        let succ_name = Identifier::new("Succ".to_string());
        let x_name = Identifier::new("x".to_string());
        let nat_name = Identifier::new("Nat".to_string());
        let nat = Ty::Decl(nat_name.clone());
        let ctor_sig = CtorSig {
            xtor: Data,
            name: succ_name.clone(),
            args: TypingContext {
                bindings: vec![ContextBinding {
                    var: x_name.clone(),
                    chi: Chirality::Prd,
                    ty: nat.clone(),
                }],
            },
        };
        let declaration = DataDeclaration {
            dat: Data,
            name: nat_name.clone(),
            xtors: vec![ctor_sig.clone()],
        };
        let data_types = vec![declaration];

        let signature = PatternSignature {
            data_types: &data_types,
            codata_types: &[],
        };

        let found = signature.find_constructor(&nat_name, &succ_name);

        assert_eq!(found, Some(&data_types[0].xtors[0]));
    }

    #[test]
    fn find_constructor_returns_none_for_unknown_ctor() {
        let succ_name = Identifier::new("Succ".to_string());
        let missing_name = Identifier::new("Missing".to_string());
        let x_name = Identifier::new("x".to_string());
        let nat_name = Identifier::new("Nat".to_string());
        let nat = Ty::Decl(nat_name.clone());
        let ctor_sig = CtorSig {
            xtor: Data,
            name: succ_name.clone(),
            args: TypingContext {
                bindings: vec![ContextBinding {
                    var: x_name.clone(),
                    chi: Chirality::Prd,
                    ty: nat.clone(),
                }],
            },
        };
        let declaration = DataDeclaration {
            dat: Data,
            name: nat_name.clone(),
            xtors: vec![ctor_sig.clone()],
        };
        let data_types = vec![declaration];

        let signature = PatternSignature {
            data_types: &data_types,
            codata_types: &[],
        };

        let found = signature.find_constructor(&nat_name, &missing_name);

        assert_eq!(found, None);
    }

    #[test]
    fn find_constructor_returns_none_for_unknown_type() {
        let succ_name = Identifier::new("Succ".to_string());
        let missing_name = Identifier::new("Missing".to_string());
        let x_name = Identifier::new("x".to_string());
        let nat_name = Identifier::new("Nat".to_string());
        let nat = Ty::Decl(nat_name.clone());
        let ctor_sig = CtorSig {
            xtor: Data,
            name: succ_name.clone(),
            args: TypingContext {
                bindings: vec![ContextBinding {
                    var: x_name.clone(),
                    chi: Chirality::Prd,
                    ty: nat.clone(),
                }],
            },
        };
        let declaration = DataDeclaration {
            dat: Data,
            name: nat_name.clone(),
            xtors: vec![ctor_sig.clone()],
        };
        let data_types = vec![declaration];

        let signature = PatternSignature {
            data_types: &data_types,
            codata_types: &[],
        };

        let found = signature.find_constructor(&missing_name, &succ_name);

        assert_eq!(found, None);
    }

    #[test]
    fn find_codata_type_returns_matching_declaration() {
        let empty_name = Identifier::new("Empty".to_string());
        let declaration = CodataDeclaration {
            dat: Codata,
            name: empty_name.clone(),
            xtors: vec![],
        };
        let codata_types = vec![declaration.clone()];
        let signature = PatternSignature {
            data_types: &[],
            codata_types: &codata_types,
        };

        let found = signature.find_codata_type(&empty_name);

        assert_eq!(found, Some(&codata_types[0]));
    }

    #[test]
    fn find_codata_type_returns_none_for_unknown_name() {
        let empty_name = Identifier::new("Empty".to_string());
        let missing_name = Identifier::new("Missing".to_string());
        let declaration = CodataDeclaration {
            dat: Codata,
            name: empty_name.clone(),
            xtors: vec![],
        };
        let codata_types = vec![declaration.clone()];
        let signature = PatternSignature {
            data_types: &[],
            codata_types: &codata_types,
        };

        let found = signature.find_codata_type(&missing_name);

        assert_eq!(found, None);
    }

    #[test]
    fn find_destructor_returns_matching_signature() {
        let head_name = Identifier::new("Head".to_string());
        let another_name = Identifier::new("Another".to_string());
        let result_name = Identifier::new("result".to_string());
        let stream_name = Identifier::new("Stream".to_string());
        let nat_name: Identifier = Identifier::new("Nat".to_string());
        let nat = Ty::Decl(nat_name.clone());
        let dtor_sig1 = DtorSig {
            xtor: Codata,
            name: another_name.clone(),
            args: TypingContext::default(),
        };
        let dtor_sig2 = DtorSig {
            xtor: Codata,
            name: head_name.clone(),
            args: TypingContext {
                bindings: vec![ContextBinding {
                    var: result_name.clone(),
                    chi: Chirality::Cns,
                    ty: nat.clone(),
                }],
            },
        };
        let declaration = CodataDeclaration {
            dat: Codata,
            name: stream_name.clone(),
            xtors: vec![dtor_sig1.clone(), dtor_sig2.clone()],
        };
        let codata_types = vec![declaration];

        let signature = PatternSignature {
            data_types: &[],
            codata_types: &codata_types,
        };

        let found = signature.find_destructor(&stream_name, &head_name);

        assert_eq!(found, Some(&codata_types[0].xtors[1]));
    }

    #[test]
    fn find_destructor_returns_none_for_unknown_dtor() {
        let head_name = Identifier::new("Head".to_string());
        let another_name = Identifier::new("Another".to_string());
        let missing_name = Identifier::new("Missing".to_string());
        let result_name = Identifier::new("result".to_string());
        let stream_name = Identifier::new("Stream".to_string());
        let nat_name: Identifier = Identifier::new("Nat".to_string());
        let nat = Ty::Decl(nat_name.clone());
        let dtor_sig1 = DtorSig {
            xtor: Codata,
            name: another_name.clone(),
            args: TypingContext::default(),
        };
        let dtor_sig2 = DtorSig {
            xtor: Codata,
            name: head_name.clone(),
            args: TypingContext {
                bindings: vec![ContextBinding {
                    var: result_name.clone(),
                    chi: Chirality::Cns,
                    ty: nat.clone(),
                }],
            },
        };
        let declaration = CodataDeclaration {
            dat: Codata,
            name: stream_name.clone(),
            xtors: vec![dtor_sig1.clone(), dtor_sig2.clone()],
        };
        let codata_types = vec![declaration];

        let signature = PatternSignature {
            data_types: &[],
            codata_types: &codata_types,
        };

        let found = signature.find_destructor(&stream_name, &missing_name);

        assert_eq!(found, None);
    }

    #[test]
    fn find_destructor_returns_none_for_unknown_type() {
        let head_name = Identifier::new("Head".to_string());
        let another_name = Identifier::new("Another".to_string());
        let missing_name = Identifier::new("Missing".to_string());
        let result_name = Identifier::new("result".to_string());
        let stream_name = Identifier::new("Stream".to_string());
        let nat_name: Identifier = Identifier::new("Nat".to_string());
        let nat = Ty::Decl(nat_name.clone());
        let dtor_sig1 = DtorSig {
            xtor: Codata,
            name: another_name.clone(),
            args: TypingContext::default(),
        };
        let dtor_sig2 = DtorSig {
            xtor: Codata,
            name: head_name.clone(),
            args: TypingContext {
                bindings: vec![ContextBinding {
                    var: result_name.clone(),
                    chi: Chirality::Cns,
                    ty: nat.clone(),
                }],
            },
        };
        let declaration = CodataDeclaration {
            dat: Codata,
            name: stream_name.clone(),
            xtors: vec![dtor_sig1.clone(), dtor_sig2.clone()],
        };
        let codata_types = vec![declaration];

        let signature = PatternSignature {
            data_types: &[],
            codata_types: &codata_types,
        };

        let found = signature.find_destructor(&missing_name, &head_name);

        assert_eq!(found, None);
    }

    #[test]
    fn binder_pattern_intrduces_context() {
        let nat = Ty::Decl(Identifier::new("Nat".to_string()));
        let x_var = Pattern::XVar(XVar {
            prdcns: Prd,
            var: Identifier::new("x".to_string()),
            ty: nat.clone(),
        });
        let subject_type = SubjectType {
            chi: Chirality::Prd,
            ty: nat.clone(),
        };
        let signature = PatternSignature {
            data_types: &[],
            codata_types: &[],
        };

        let res = check_pattern(&x_var, &subject_type, &signature);
        assert_eq!(
            res,
            Ok(TypingContext {
                bindings: vec![ContextBinding {
                    var: Identifier::new("x".to_string()),
                    chi: Chirality::Prd,
                    ty: nat.clone(),
                }]
            })
        )
    }

    #[test]
    fn wildcard_pattern_intrduces_empty_context() {
        let nat = Ty::Decl(Identifier::new("Nat".to_string()));
        let wildcard = Pattern::Wildcard {
            chi: Prd,
            ty: nat.clone(),
        };
        let subject_type = SubjectType {
            chi: Chirality::Prd,
            ty: nat.clone(),
        };
        let signature = PatternSignature {
            data_types: &[],
            codata_types: &[],
        };

        let res = check_pattern(&wildcard, &subject_type, &signature);
        assert_eq!(res, Ok(TypingContext::default()))
    }

    fn nat_declaration_for_pattern_tests() -> DataDeclaration {
        let nat_name = Identifier::new("Nat".to_string());
        let nat = Ty::Decl(nat_name.clone());

        DataDeclaration {
            dat: Data,
            name: nat_name,
            xtors: vec![
                CtorSig {
                    xtor: Data,
                    name: Identifier::new("Zero".to_string()),
                    args: TypingContext::default(),
                },
                CtorSig {
                    xtor: Data,
                    name: Identifier::new("Succ".to_string()),
                    args: TypingContext {
                        bindings: vec![ContextBinding {
                            var: Identifier::new("n".to_string()),
                            chi: Chirality::Prd,
                            ty: nat,
                        }],
                    },
                },
            ],
        }
    }

    #[test]
    fn nested_constructor_pattern_introduces_context() {
        let nat_name = Identifier::new("Nat".to_string());
        let succ_name = Identifier::new("Succ".to_string());
        let x_name = Identifier::new("x".to_string());
        let nat = Ty::Decl(nat_name.clone());

        let nat_declaration = nat_declaration_for_pattern_tests();
        let pat_sig = PatternSignature {
            data_types: &[nat_declaration],
            codata_types: &[],
        };

        let x_pattern = Pattern::XVar(XVar {
            prdcns: Prd,
            var: x_name.clone(),
            ty: nat.clone(),
        });
        let inner_succ_pattern = Pattern::Xtor(Xtor {
            prdcns: Prd,
            name: succ_name.clone(),
            ty: nat.clone(),
            args: vec![AnyPattern::Producer(x_pattern)],
        });
        let outer_succ_pattern = Pattern::Xtor(Xtor {
            prdcns: Prd,
            name: succ_name.clone(),
            ty: nat.clone(),
            args: vec![AnyPattern::Producer(inner_succ_pattern)],
        });

        let expected = SubjectType {
            chi: Chirality::Prd,
            ty: nat.clone(),
        };

        let res = check_pattern(&outer_succ_pattern, &expected, &pat_sig);
        assert_eq!(
            res,
            Ok(TypingContext {
                bindings: vec![ContextBinding {
                    var: x_name.clone(),
                    chi: Chirality::Prd,
                    ty: nat.clone(),
                }]
            })
        )
    }

    #[test]
    fn nested_destructor_pattern_introduces_context() {
        let nat_name = Identifier::new("Nat".to_string());
        let stream_name = Identifier::new("Stream".to_string());
        let tail_name = Identifier::new("tail".to_string());
        let head_name = Identifier::new("head".to_string());
        let alpha_name = Identifier::new("alpha".to_string());
        let nat = Ty::Decl(nat_name.clone());
        let stream = Ty::Decl(stream_name.clone());
        let dtor_head = DtorSig {
            xtor: Codata,
            name: head_name.clone(),
            args: TypingContext {
                bindings: vec![ContextBinding {
                    var: alpha_name.clone(),
                    chi: Chirality::Cns,
                    ty: nat.clone(),
                }],
            },
        };
        let dtor_tail = DtorSig {
            xtor: Codata,
            name: tail_name.clone(),
            args: TypingContext {
                bindings: vec![ContextBinding {
                    var: alpha_name.clone(),
                    chi: Chirality::Cns,
                    ty: stream.clone(),
                }],
            },
        };
        let stream_declaration = CodataDeclaration {
            dat: Codata,
            name: stream_name.clone(),
            xtors: vec![dtor_head, dtor_tail],
        };
        let pat_sig = PatternSignature {
            data_types: &[],
            codata_types: &[stream_declaration],
        };

        let alpha_pattern = Pattern::XVar(XVar {
            prdcns: Cns,
            var: alpha_name.clone(),
            ty: nat.clone(),
        });
        let inner_head_pattern = Pattern::Xtor(Xtor {
            prdcns: Cns,
            name: head_name.clone(),
            ty: stream.clone(),
            args: vec![AnyPattern::Consumer(alpha_pattern)],
        });
        let outer_tail_pattern = Pattern::Xtor(Xtor {
            prdcns: Cns,
            name: tail_name.clone(),
            ty: stream.clone(),
            args: vec![AnyPattern::Consumer(inner_head_pattern)],
        });

        let expected = SubjectType {
            chi: Chirality::Cns,
            ty: stream.clone(),
        };

        let res = check_pattern(&outer_tail_pattern, &expected, &pat_sig);
        assert_eq!(
            res,
            Ok(TypingContext {
                bindings: vec![ContextBinding {
                    var: alpha_name.clone(),
                    chi: Chirality::Cns,
                    ty: nat.clone(),
                }]
            })
        )
    }

    #[test]
    fn mixed_nested_patterns_preserve_binder_order() {
        let nat_name = Identifier::new("Nat".to_string());
        let stream_name = Identifier::new("Stream".to_string());
        let request_name = Identifier::new("Request".to_string());
        let send_name = Identifier::new("Send".to_string());
        let tail_name = Identifier::new("tail".to_string());
        let head_name = Identifier::new("head".to_string());
        let succ_name = Identifier::new("Succ".to_string());
        let alpha_name = Identifier::new("alpha".to_string());
        let x_name = Identifier::new("x".to_string());
        let nat = Ty::Decl(nat_name.clone());
        let stream = Ty::Decl(stream_name.clone());
        let request = Ty::Decl(request_name.clone());

        let dtor_head = DtorSig {
            xtor: Codata,
            name: head_name.clone(),
            args: TypingContext {
                bindings: vec![ContextBinding {
                    var: alpha_name.clone(),
                    chi: Chirality::Cns,
                    ty: nat.clone(),
                }],
            },
        };
        let dtor_tail = DtorSig {
            xtor: Codata,
            name: tail_name.clone(),
            args: TypingContext {
                bindings: vec![ContextBinding {
                    var: alpha_name.clone(),
                    chi: Chirality::Cns,
                    ty: stream.clone(),
                }],
            },
        };
        let ctor_send = CtorSig {
            xtor: Data,
            name: send_name.clone(),
            args: TypingContext {
                bindings: vec![
                    ContextBinding {
                        var: x_name.clone(),
                        chi: Chirality::Prd,
                        ty: nat.clone(),
                    },
                    ContextBinding {
                        var: alpha_name.clone(),
                        chi: Chirality::Cns,
                        ty: stream.clone(),
                    },
                ],
            },
        };

        let stream_declaration = CodataDeclaration {
            dat: Codata,
            name: stream_name.clone(),
            xtors: vec![dtor_head, dtor_tail],
        };
        let nat_declaration = nat_declaration_for_pattern_tests();
        let request_declaration = DataDeclaration {
            dat: Data,
            name: request_name.clone(),
            xtors: vec![ctor_send],
        };
        let pat_sig = PatternSignature {
            data_types: &[nat_declaration, request_declaration],
            codata_types: &[stream_declaration],
        };

        let alpha_pattern = Pattern::XVar(XVar {
            prdcns: Cns,
            var: alpha_name.clone(),
            ty: nat.clone(),
        });
        let x_pattern = Pattern::XVar(XVar {
            prdcns: Prd,
            var: x_name.clone(),
            ty: nat.clone(),
        });
        let inner_head_pattern = Pattern::Xtor(Xtor {
            prdcns: Cns,
            name: head_name.clone(),
            ty: stream.clone(),
            args: vec![AnyPattern::Consumer(alpha_pattern)],
        });
        let outer_tail_pattern = Pattern::Xtor(Xtor {
            prdcns: Cns,
            name: tail_name.clone(),
            ty: stream.clone(),
            args: vec![AnyPattern::Consumer(inner_head_pattern)],
        });
        let outer_succ_pattern = Pattern::Xtor(Xtor {
            prdcns: Prd,
            name: succ_name.clone(),
            args: vec![AnyPattern::Producer(x_pattern)],
            ty: nat.clone(),
        });
        let outer_outer_send_pattern = Pattern::Xtor(Xtor {
            prdcns: Prd,
            name: send_name.clone(),
            args: vec![
                AnyPattern::Producer(outer_succ_pattern),
                AnyPattern::Consumer(outer_tail_pattern),
            ],
            ty: request.clone(),
        });

        let expected = SubjectType {
            chi: Chirality::Prd,
            ty: request.clone(),
        };

        let res = check_pattern(&outer_outer_send_pattern, &expected, &pat_sig);
        assert_eq!(
            res,
            Ok(TypingContext {
                bindings: vec![
                    ContextBinding {
                        var: x_name.clone(),
                        chi: Chirality::Prd,
                        ty: nat.clone(),
                    },
                    ContextBinding {
                        var: alpha_name.clone(),
                        chi: Chirality::Cns,
                        ty: nat.clone(),
                    },
                ]
            })
        )
    }

    #[test]
    fn empty_pattern_vector_introduces_empty_context() {
        let signature = PatternSignature {
            data_types: &[],
            codata_types: &[],
        };

        let res = check_patterns(&[], &[], &signature);

        assert_eq!(res, Ok(TypingContext::default()))
    }

    #[test]
    fn mismatched_pattern_count_is_rejected() {
        let signature = PatternSignature {
            data_types: &[],
            codata_types: &[],
        };
        let patterns = vec![AnyPattern::Producer(Pattern::Wildcard {
            chi: Prd,
            ty: Ty::I64,
        })];
        let expected = vec![SubjectType {
            chi: Chirality::Prd,
            ty: Ty::I64,
        }];

        let too_few = check_patterns(&[], &expected, &signature);
        assert_eq!(
            too_few,
            Err(PatternError::WrongNumberOfPatterns {
                expected: 1,
                found: 0,
            })
        );

        let too_many = check_patterns(&patterns, &[], &signature);
        assert_eq!(
            too_many,
            Err(PatternError::WrongNumberOfPatterns {
                expected: 0,
                found: 1,
            })
        )
    }

    #[test]
    fn duplicate_binders_are_rejected() {
        let x_name = Identifier::new("x".to_string());
        let binder = AnyPattern::Producer(Pattern::XVar(XVar {
            prdcns: Prd,
            var: x_name.clone(),
            ty: Ty::I64,
        }));
        let patterns = vec![binder.clone(), binder];
        let subject = SubjectType {
            chi: Chirality::Prd,
            ty: Ty::I64,
        };
        let expected = vec![subject.clone(), subject];
        let signature = PatternSignature {
            data_types: &[],
            codata_types: &[],
        };

        let res = check_patterns(&patterns, &expected, &signature);

        assert_eq!(res, Err(PatternError::DuplicateBinder { name: x_name }))
    }

    #[test]
    fn duplicate_binders_across_nested_patterns_are_rejected() {
        let nat = Ty::Decl(Identifier::new("Nat".to_string()));
        let pair_name = Identifier::new("NatPair".to_string());
        let pair_ty = Ty::Decl(pair_name.clone());
        let pair_ctor_name = Identifier::new("Pair".to_string());
        let x_name = Identifier::new("x".to_string());

        let pair_declaration = DataDeclaration {
            dat: Data,
            name: pair_name,
            xtors: vec![CtorSig {
                xtor: Data,
                name: pair_ctor_name.clone(),
                args: TypingContext {
                    bindings: vec![
                        ContextBinding {
                            var: Identifier::new("left".to_string()),
                            chi: Chirality::Prd,
                            ty: nat.clone(),
                        },
                        ContextBinding {
                            var: Identifier::new("right".to_string()),
                            chi: Chirality::Prd,
                            ty: nat.clone(),
                        },
                    ],
                },
            }],
        };
        let data_types = vec![nat_declaration_for_pattern_tests(), pair_declaration];
        let signature = PatternSignature {
            data_types: &data_types,
            codata_types: &[],
        };

        let subtree = Pattern::Xtor(Xtor {
            prdcns: Prd,
            name: Identifier::new("Succ".to_string()),
            args: vec![AnyPattern::Producer(Pattern::XVar(XVar {
                prdcns: Prd,
                var: x_name.clone(),
                ty: nat.clone(),
            }))],
            ty: nat,
        });
        let pattern = Pattern::Xtor(Xtor {
            prdcns: Prd,
            name: pair_ctor_name,
            args: vec![
                AnyPattern::Producer(subtree.clone()),
                AnyPattern::Producer(subtree),
            ],
            ty: pair_ty.clone(),
        });
        let expected = SubjectType {
            chi: Chirality::Prd,
            ty: pair_ty,
        };

        let res = check_pattern(&pattern, &expected, &signature);

        assert_eq!(res, Err(PatternError::DuplicateBinder { name: x_name }))
    }

    #[test]
    fn unknown_constructor_pattern_is_rejected() {
        let nat = Ty::Decl(Identifier::new("Nat".to_string()));
        let missing_name = Identifier::new("Missing".to_string());
        let data_types = vec![nat_declaration_for_pattern_tests()];
        let signature = PatternSignature {
            data_types: &data_types,
            codata_types: &[],
        };
        let pattern = Pattern::Xtor(Xtor {
            prdcns: Prd,
            name: missing_name.clone(),
            args: vec![],
            ty: nat.clone(),
        });
        let expected = SubjectType {
            chi: Chirality::Prd,
            ty: nat.clone(),
        };

        let res = check_pattern(&pattern, &expected, &signature);

        assert_eq!(
            res,
            Err(PatternError::XtorSignatureNotFound {
                ty: nat,
                chi: Chirality::Prd,
                name: missing_name,
            })
        )
    }

    #[test]
    fn unknown_destructor_pattern_is_rejected() {
        let stream_name = Identifier::new("Stream".to_string());
        let stream = Ty::Decl(stream_name.clone());
        let nat = Ty::Decl(Identifier::new("Nat".to_string()));
        let missing_name = Identifier::new("missing".to_string());
        let data_types = vec![nat_declaration_for_pattern_tests()];
        let codata_types = vec![CodataDeclaration {
            dat: Codata,
            name: stream_name,
            xtors: vec![DtorSig {
                xtor: Codata,
                name: Identifier::new("head".to_string()),
                args: TypingContext {
                    bindings: vec![ContextBinding {
                        var: Identifier::new("result".to_string()),
                        chi: Chirality::Cns,
                        ty: nat,
                    }],
                },
            }],
        }];
        let signature = PatternSignature {
            data_types: &data_types,
            codata_types: &codata_types,
        };
        let pattern = Pattern::Xtor(Xtor {
            prdcns: Cns,
            name: missing_name.clone(),
            args: vec![],
            ty: stream.clone(),
        });
        let expected = SubjectType {
            chi: Chirality::Cns,
            ty: stream.clone(),
        };

        let res = check_pattern(&pattern, &expected, &signature);

        assert_eq!(
            res,
            Err(PatternError::XtorSignatureNotFound {
                ty: stream,
                chi: Chirality::Cns,
                name: missing_name,
            })
        )
    }

    #[test]
    fn xtor_pattern_on_i64_is_rejected() {
        let signature = PatternSignature {
            data_types: &[],
            codata_types: &[],
        };
        let pattern = Pattern::Xtor(Xtor {
            prdcns: Prd,
            name: Identifier::new("Invalid".to_string()),
            args: vec![],
            ty: Ty::I64,
        });
        let expected = SubjectType {
            chi: Chirality::Prd,
            ty: Ty::I64,
        };

        let res = check_pattern(&pattern, &expected, &signature);

        assert_eq!(
            res,
            Err(PatternError::XtorRequiresDeclaredType { found: Ty::I64 })
        )
    }

    #[test]
    fn wrong_nested_pattern_type_is_rejected() {
        let nat = Ty::Decl(Identifier::new("Nat".to_string()));
        let data_types = vec![nat_declaration_for_pattern_tests()];
        let signature = PatternSignature {
            data_types: &data_types,
            codata_types: &[],
        };
        let pattern = Pattern::Xtor(Xtor {
            prdcns: Prd,
            name: Identifier::new("Succ".to_string()),
            args: vec![AnyPattern::Producer(Pattern::XVar(XVar {
                prdcns: Prd,
                var: Identifier::new("x".to_string()),
                ty: Ty::I64,
            }))],
            ty: nat.clone(),
        });
        let expected = SubjectType {
            chi: Chirality::Prd,
            ty: nat.clone(),
        };

        let res = check_pattern(&pattern, &expected, &signature);

        assert_eq!(
            res,
            Err(PatternError::WrongType {
                expected: nat,
                found: Ty::I64,
            })
        )
    }

    #[test]
    fn wrong_nested_pattern_chirality_is_rejected() {
        let nat = Ty::Decl(Identifier::new("Nat".to_string()));
        let data_types = vec![nat_declaration_for_pattern_tests()];
        let signature = PatternSignature {
            data_types: &data_types,
            codata_types: &[],
        };
        let pattern = Pattern::Xtor(Xtor {
            prdcns: Prd,
            name: Identifier::new("Succ".to_string()),
            args: vec![AnyPattern::Consumer(Pattern::XVar(XVar {
                prdcns: Cns,
                var: Identifier::new("alpha".to_string()),
                ty: nat.clone(),
            }))],
            ty: nat.clone(),
        });
        let expected = SubjectType {
            chi: Chirality::Prd,
            ty: nat,
        };

        let res = check_pattern(&pattern, &expected, &signature);

        assert_eq!(
            res,
            Err(PatternError::WrongChirality {
                expected: Chirality::Prd,
                found: Chirality::Cns,
            })
        )
    }

    fn context_test_clause(pattern: AnyPattern) -> NestedClause {
        NestedClause {
            pattern,
            body: Rc::new(Statement::Exit(Exit {
                arg: Rc::new(Term::Literal(Literal { lit: 0 })),
                ty: Ty::I64,
            })),
        }
    }

    fn stream_declaration_for_context_tests() -> CodataDeclaration {
        CodataDeclaration {
            dat: Codata,
            name: Identifier::new("Stream".to_string()),
            xtors: vec![DtorSig {
                xtor: Codata,
                name: Identifier::new("head".to_string()),
                args: TypingContext {
                    bindings: vec![ContextBinding {
                        var: Identifier::new("result".to_string()),
                        chi: Chirality::Cns,
                        ty: Ty::Decl(Identifier::new("Nat".to_string())),
                    }],
                },
            }],
        }
    }

    #[test]
    fn case_clause_contexts_are_independent() {
        let nat = Ty::Decl(Identifier::new("Nat".to_string()));
        let x_name = Identifier::new("x".to_string());
        let data_types = [nat_declaration_for_pattern_tests()];
        let signature = PatternSignature {
            data_types: &data_types,
            codata_types: &[],
        };
        let outer_binding = ContextBinding {
            var: Identifier::new("k".to_string()),
            chi: Chirality::Cns,
            ty: Ty::I64,
        };
        let outer = TypingContext {
            bindings: vec![outer_binding.clone()],
        };
        let clause = context_test_clause(AnyPattern::Producer(Pattern::XVar(XVar {
            prdcns: Prd,
            var: x_name.clone(),
            ty: nat.clone(),
        })));
        let xcase = NestedXCase {
            prdcns: Cns,
            clauses: vec![clause.clone(), clause],
            ty: nat.clone(),
        };
        let expected_context = TypingContext {
            bindings: vec![
                outer_binding,
                ContextBinding {
                    var: x_name,
                    chi: Chirality::Prd,
                    ty: nat,
                },
            ],
        };

        let res = prepare_clause_contexts(&xcase, &outer, &signature);

        assert_eq!(res, Ok(vec![expected_context.clone(), expected_context]));
    }

    #[test]
    fn new_clause_context_contains_consumer_binder() {
        let nat = Ty::Decl(Identifier::new("Nat".to_string()));
        let stream = Ty::Decl(Identifier::new("Stream".to_string()));
        let alpha_name = Identifier::new("alpha".to_string());
        let data_types = [nat_declaration_for_pattern_tests()];
        let codata_types = [stream_declaration_for_context_tests()];
        let signature = PatternSignature {
            data_types: &data_types,
            codata_types: &codata_types,
        };
        let outer_binding = ContextBinding {
            var: Identifier::new("n".to_string()),
            chi: Chirality::Prd,
            ty: Ty::I64,
        };
        let outer = TypingContext {
            bindings: vec![outer_binding.clone()],
        };
        let pattern = Pattern::Xtor(Xtor {
            prdcns: Cns,
            name: Identifier::new("head".to_string()),
            args: vec![AnyPattern::Consumer(Pattern::XVar(XVar {
                prdcns: Cns,
                var: alpha_name.clone(),
                ty: nat.clone(),
            }))],
            ty: stream.clone(),
        });
        let xcase = NestedXCase {
            prdcns: Prd,
            clauses: vec![context_test_clause(AnyPattern::Consumer(pattern))],
            ty: stream,
        };

        let res = prepare_clause_contexts(&xcase, &outer, &signature);

        assert_eq!(
            res,
            Ok(vec![TypingContext {
                bindings: vec![
                    outer_binding,
                    ContextBinding {
                        var: alpha_name,
                        chi: Chirality::Cns,
                        ty: nat,
                    },
                ],
            }])
        );
    }

    #[test]
    fn case_rejects_non_data_types() {
        let data_types = [nat_declaration_for_pattern_tests()];
        let codata_types = [stream_declaration_for_context_tests()];
        let signature = PatternSignature {
            data_types: &data_types,
            codata_types: &codata_types,
        };
        let outer = TypingContext::default();

        for ty in [
            Ty::I64,
            Ty::Decl(Identifier::new("Stream".to_string())),
            Ty::Decl(Identifier::new("Missing".to_string())),
        ] {
            let xcase = NestedXCase {
                prdcns: Cns,
                clauses: vec![context_test_clause(AnyPattern::Producer(
                    Pattern::Wildcard {
                        chi: Prd,
                        ty: ty.clone(),
                    },
                ))],
                ty: ty.clone(),
            };

            let res = prepare_clause_contexts(&xcase, &outer, &signature);

            assert_eq!(res, Err(PatternError::ExpectedDataType { found: ty }));
        }
    }

    #[test]
    fn new_rejects_non_codata_types() {
        let data_types = [nat_declaration_for_pattern_tests()];
        let codata_types = [stream_declaration_for_context_tests()];
        let signature = PatternSignature {
            data_types: &data_types,
            codata_types: &codata_types,
        };
        let outer = TypingContext::default();

        for ty in [
            Ty::I64,
            Ty::Decl(Identifier::new("Nat".to_string())),
            Ty::Decl(Identifier::new("Missing".to_string())),
        ] {
            let xcase = NestedXCase {
                prdcns: Prd,
                clauses: vec![context_test_clause(AnyPattern::Consumer(
                    Pattern::Wildcard {
                        chi: Cns,
                        ty: ty.clone(),
                    },
                ))],
                ty: ty.clone(),
            };

            let res = prepare_clause_contexts(&xcase, &outer, &signature);

            assert_eq!(res, Err(PatternError::ExpectedCodataType { found: ty }));
        }
    }

    #[test]
    fn case_rejects_consumer_clause_pattern() {
        let nat = Ty::Decl(Identifier::new("Nat".to_string()));
        let data_types = [nat_declaration_for_pattern_tests()];
        let signature = PatternSignature {
            data_types: &data_types,
            codata_types: &[],
        };
        let xcase = NestedXCase {
            prdcns: Cns,
            clauses: vec![context_test_clause(AnyPattern::Consumer(Pattern::XVar(
                XVar {
                    prdcns: Cns,
                    var: Identifier::new("alpha".to_string()),
                    ty: nat.clone(),
                },
            )))],
            ty: nat,
        };

        let res = prepare_clause_contexts(&xcase, &TypingContext::default(), &signature);

        assert_eq!(
            res,
            Err(PatternError::WrongChirality {
                expected: Chirality::Prd,
                found: Chirality::Cns,
            })
        );
    }

    #[test]
    fn clause_binder_must_be_fresh_in_outer_context() {
        let nat = Ty::Decl(Identifier::new("Nat".to_string()));
        let x_name = Identifier::new("x".to_string());
        let data_types = [nat_declaration_for_pattern_tests()];
        let signature = PatternSignature {
            data_types: &data_types,
            codata_types: &[],
        };
        let outer = TypingContext {
            bindings: vec![ContextBinding {
                var: x_name.clone(),
                chi: Chirality::Prd,
                ty: Ty::I64,
            }],
        };
        let xcase = NestedXCase {
            prdcns: Cns,
            clauses: vec![context_test_clause(AnyPattern::Producer(Pattern::XVar(
                XVar {
                    prdcns: Prd,
                    var: x_name.clone(),
                    ty: nat.clone(),
                },
            )))],
            ty: nat,
        };

        let res = prepare_clause_contexts(&xcase, &outer, &signature);

        assert_eq!(
            res,
            Err(PatternError::BinderAlreadyInContext { name: x_name })
        );
    }
}
