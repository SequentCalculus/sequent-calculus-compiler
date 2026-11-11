use crate::syntax::{
    Chi, Chirality, CodataDeclaration, ContextBinding, CtorSig, DataDeclaration, DtorSig,
    Identifier, Pattern, SubjectType, Ty, TypingContext,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PatternError {
    WrongChirality {
        expected: Chirality,
        found: Chirality,
    },
    WrongType {
        expected: Ty,
        found: Ty,
    },
    XtorRequiresDeclaredType {
        found: Ty,
    },
}

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

pub struct PatternSignature<'a> {
    pub data_types: &'a [DataDeclaration],
    pub codata_types: &'a [CodataDeclaration],
}

impl<'a> PatternSignature<'a> {
    fn find_data_type(&self, name: &Identifier) -> Option<&DataDeclaration> {
        for declaration in self.data_types {
            if &declaration.name == name {
                return Some(declaration);
            }
        }
        None
    }

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

    fn find_codata_type(&self, name: &Identifier) -> Option<&CodataDeclaration> {
        for declaration in self.codata_types {
            if &declaration.name == name {
                return Some(declaration);
            }
        }
        None
    }

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
            let type_name = match &expected.ty {
                Ty::Decl(name) => name,
                Ty::I64 => {
                    return Err(PatternError::XtorRequiresDeclaredType {
                        found: expected.ty.clone(),
                    });
                }
            };
            todo!("find signature")
        }
    }
}

/* pub fn check_patterns(
    patterns: &[Pattern],
    expected: &[SubjectType],
    signature: &PatternSignature<'_>,
) -> Result<TypingContext, PatternError>; */

#[cfg(test)]
mod tests {
    use crate::syntax::{Codata, Data, Identifier, Prd, XVar};

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
}
