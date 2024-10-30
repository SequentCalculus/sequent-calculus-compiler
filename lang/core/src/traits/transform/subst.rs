use super::{Bind, Continuation, NamingTransformation, TransformState};
use crate::syntax::{substitution::SubstitutionBinding, Statement};

impl NamingTransformation for SubstitutionBinding {
    type Target = SubstitutionBinding;
    fn transform(self, state: &mut TransformState) -> Self::Target {
        match self {
            SubstitutionBinding::ProducerBinding { prd, ty } => {
                SubstitutionBinding::ProducerBinding {
                    prd: prd.transform(state),
                    ty,
                }
            }
            SubstitutionBinding::ConsumerBinding { cns, ty } => {
                SubstitutionBinding::ConsumerBinding {
                    cns: cns.transform(state),
                    ty,
                }
            }
        }
    }
}

impl Bind for SubstitutionBinding {
    fn bind(self, k: Continuation, state: &mut TransformState) -> Statement {
        match self {
            SubstitutionBinding::ProducerBinding { prd, ty: _ } => prd.bind(k, state),
            SubstitutionBinding::ConsumerBinding { cns, ty: _ } => cns.bind(k, state),
        }
    }
}
