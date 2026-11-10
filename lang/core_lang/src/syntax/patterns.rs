use crate::syntax::{Chi, Chirality, Cns, ContextBinding, Prd, Ty, TypingContext, XVar, Xtor};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Pattern<C: Chi> {
    // A binder with given chirality determines whether variable or covariable
    XVar(XVar<C>),
    // A wildcard with given chirality and type
    Wildcard { chi: C, ty: Ty },
    // Either a con- or destructor with a pattern vector as argument
    Xtor(Xtor<C, PatternVec>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnyPattern {
    Producer(Pattern<Prd>),
    Consumer(Pattern<Cns>),
}

pub type PatternVec = Vec<AnyPattern>;

impl<C: Chi> Pattern<C> {
    pub fn chirality(&self) -> &C {
        match self {
            Pattern::XVar(var) => &var.prdcns,
            Pattern::Wildcard { chi, .. } => chi,
            Pattern::Xtor(xtor) => &xtor.prdcns,
        }
    }

    pub fn ty(&self) -> &Ty {
        match self {
            Pattern::XVar(var) => &var.ty,
            Pattern::Wildcard { ty, .. } => ty,
            Pattern::Xtor(xtor) => &xtor.ty,
        }
    }

    pub fn is_irrefutable(&self) -> bool {
        matches!(self, Pattern::XVar(_) | Pattern::Wildcard { .. })
    }

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
    pub fn collect_binders(&self, output: &mut Vec<ContextBinding>) {
        match self {
            AnyPattern::Producer(prd) => prd.collect_binders(output),
            AnyPattern::Consumer(cns) => cns.collect_binders(output),
        }
    }
}

impl<C: Chi> crate::traits::Typed for Pattern<C> {
    fn get_type(&self) -> Ty {
        self.ty().clone()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubjectType {
    pub chi: Chirality,
    pub ty: Ty,
}

pub type SubjectTyping = Vec<SubjectType>;

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
