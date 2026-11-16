//! Type checking for annotated nested Core patterns.
//!
//! Patterns are checked against expected subject types and declared xtor
//! signatures. Successful checks return the introduced binder context.
//!
//! Assumes well-formed declaration environment and expected types.
//! Does not check clause bodies or match exhaustiveness.

use crate::syntax::{
    AnyPattern, Chi, Chirality, CodataDeclaration, ContextBinding, CtorSig, DataDeclaration,
    DtorSig, Identifier, Pattern, SubjectType, Ty, TypingContext, subject_typing,
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
    /// The same binder indentifier occurs more than once in a pattern vector.
    DuplicateBinder { name: Identifier },
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
        for declaration in self.data_types {
            if &declaration.name == name {
                return Some(declaration);
            }
        }
        None
    }

    /// Finds a constructor only within the specified data declaration.
    fn find_constructor(
        &self,
        type_name: &Identifier,
        constructor_name: &Identifier,
    ) -> Option<&CtorSig> {
        let declaration = self.find_data_type(type_name)?;

        for xtor in &declaration.xtors {
            if &xtor.name == constructor_name {
                return Some(xtor);
            }
        }
        None
    }

    /// Finds a codata declaration by its type name.
    fn find_codata_type(&self, name: &Identifier) -> Option<&CodataDeclaration> {
        for declaration in self.codata_types {
            if &declaration.name == name {
                return Some(declaration);
            }
        }
        None
    }

    /// Finds a destructor only within the specified codata declaration.
    fn find_destructor(
        &self,
        type_name: &Identifier,
        destructor_name: &Identifier,
    ) -> Option<&DtorSig> {
        let declaration = self.find_codata_type(type_name)?;

        for xtor in &declaration.xtors {
            if &xtor.name == destructor_name {
                return Some(xtor);
            }
        }
        None
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
            let type_name = match &expected.ty {
                Ty::Decl(name) => name,
                Ty::I64 => {
                    return Err(PatternError::XtorRequiresDeclaredType {
                        found: expected.ty.clone(),
                    });
                }
            };

            let expected_context: &TypingContext = match &expected.chi {
                Chirality::Prd => {
                    let ctor_sig = signature.find_constructor(type_name, &xtor.name);

                    match ctor_sig {
                        Some(ctor) => &ctor.args,
                        None => {
                            return Err(PatternError::XtorSignatureNotFound {
                                ty: expected.ty.clone(),
                                chi: expected.chi.clone(),
                                name: xtor.name.clone(),
                            });
                        }
                    }
                }
                Chirality::Cns => {
                    let dtor_sig = signature.find_destructor(type_name, &xtor.name);

                    match dtor_sig {
                        Some(dtor) => &dtor.args,
                        None => {
                            return Err(PatternError::XtorSignatureNotFound {
                                ty: expected.ty.clone(),
                                chi: expected.chi.clone(),
                                name: xtor.name.clone(),
                            });
                        }
                    }
                }
            };

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

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::syntax::{Cns, Codata, Data, Identifier, Prd, XVar, Xtor};

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
}
