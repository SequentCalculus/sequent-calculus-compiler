//! Nested Core patterns and their type annotations.
//!
//! Producer and consumer patterns share a generic representation.
//! Pattern vectors may contain both chiralities.
//! Annotation access and binder collection do not perform type checking.

use crate::syntax::{Chi, Chirality, Cns, ContextBinding, Prd, Ty, TypingContext, XVar, Xtor};

/// A nested pattern whose outer chirality is determined by `C`.
///
/// Constructor and destructor arguments may contain patterns of either
/// chirality. Their admissibility is checked against the declared signature.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Pattern<C: Chi> {
    /// A variable or covariable binder, depending on `C`.
    XVar(XVar<C>),
    /// Accepts a subject at the annotated position without introducing a binder.
    Wildcard { chi: C, ty: Ty },
    /// A constructor or destructor pattern using nested patterns as arguments.
    Xtor(Xtor<C, PatternVec>),
}

/// A producer or consumer pattern stored in a common representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnyPattern {
    Producer(Pattern<Prd>),
    Consumer(Pattern<Cns>),
}

/// An ordered vector of patterns with potentially different chiralities.
pub type PatternVec = Vec<AnyPattern>;

impl<C: Chi> Pattern<C> {
    /// Returns the outer chirality marker without inspecting subpatterns.
    pub fn chirality(&self) -> &C {
        match self {
            Pattern::XVar(var) => &var.prdcns,
            Pattern::Wildcard { chi, .. } => chi,
            Pattern::Xtor(xtor) => &xtor.prdcns,
        }
    }

    /// Returns the stored type annotation without validating it.
    pub fn ty(&self) -> &Ty {
        match self {
            Pattern::XVar(var) => &var.ty,
            Pattern::Wildcard { ty, .. } => ty,
            Pattern::Xtor(xtor) => &xtor.ty,
        }
    }

    /// Returns whether the pattern is a binder or wildcard.
    pub fn is_irrefutable(&self) -> bool {
        matches!(self, Pattern::XVar(_) | Pattern::Wildcard { .. })
    }

    /// Appends all annotated binders in left-to-right traversal order.
    ///
    /// Does not validate annotations, reject duplicate binders, or generate
    /// fresh identifiers.
    pub fn collect_binders(&self, output: &mut Vec<ContextBinding>) {
        match self {
            Pattern::XVar(var) => output.push(ContextBinding {
                var: var.var.clone(),
                chi: if var.prdcns.is_prd() {
                    Chirality::Prd
                } else {
                    Chirality::Cns
                },
                ty: var.ty.clone(),
            }),
            Pattern::Wildcard { .. } => {}
            Pattern::Xtor(xtor) => {
                for any_pattern in &xtor.args {
                    any_pattern.collect_binders(output);
                }
            }
        }
    }
}

impl AnyPattern {
    /// Delegates binder collection to the contained pattern.
    pub fn collect_binders(&self, output: &mut Vec<ContextBinding>) {
        match self {
            AnyPattern::Producer(prd) => prd.collect_binders(output),
            AnyPattern::Consumer(cns) => cns.collect_binders(output),
        }
    }
}

impl<C: Chi> crate::traits::Typed for Pattern<C> {
    /// Exposes the stored annotation through Core's common typing interface.
    fn get_type(&self) -> Ty {
        self.ty().clone()
    }
}

/// The expected chirality and type of one subject position.
///
/// Contains no subject identifier or pattern binder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubjectType {
    pub chi: Chirality,
    pub ty: Ty,
}

/// The ordered subject-typing vector Omega.
pub type SubjectTyping = Vec<SubjectType>;

/// Converts a signature context into a subject-typing vector.
///
/// Preserves field order, chiralities, and types while discarding names.
pub fn subject_typing(context: &TypingContext) -> SubjectTyping {
    let mut res = Vec::new();

    for binding in &context.bindings {
        res.push(SubjectType {
            chi: binding.chi.clone(),
            ty: binding.ty.clone(),
        });
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syntax::Identifier;

    #[test]
    fn collect_binders_from_nested_pattern() {
        let x_name = Identifier::new("x".to_string());
        let nat = Ty::Decl(Identifier::new("Nat".to_string()));
        let x_var = Pattern::XVar(XVar {
            prdcns: Prd,
            var: x_name.clone(),
            ty: nat.clone(),
        });

        let succ_name = Identifier::new("Succ".to_string());
        let succ_ctor = Pattern::Xtor(Xtor {
            prdcns: Prd,
            name: succ_name.clone(),
            args: vec![AnyPattern::Producer(x_var)],
            ty: nat.clone(),
        });

        let a_name = Identifier::new("alpha".to_string());
        let res = Ty::Decl(Identifier::new("Result".to_string()));
        let alpha_covar = Pattern::XVar(XVar {
            prdcns: Cns,
            var: a_name.clone(),
            ty: res.clone(),
        });

        let send_name = Identifier::new("Send".to_string());
        let request = Ty::Decl(Identifier::new("Request".to_string()));
        let send_pattern = Pattern::Xtor(Xtor {
            prdcns: Prd,
            name: send_name.clone(),
            args: vec![
                AnyPattern::Producer(succ_ctor),
                AnyPattern::Consumer(alpha_covar),
            ],
            ty: request.clone(),
        });

        let mut out = Vec::new();
        send_pattern.collect_binders(&mut out);

        let expected = vec![
            ContextBinding {
                var: x_name.clone(),
                chi: Chirality::Prd,
                ty: nat.clone(),
            },
            ContextBinding {
                var: a_name.clone(),
                chi: Chirality::Cns,
                ty: res.clone(),
            },
        ];

        assert_eq!(out, expected);
    }
}
