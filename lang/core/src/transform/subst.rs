use crate::{
    naming_transformation::{Bind, Continuation, NamingTransformation, TransformState},
    syntax::{substitution::SubstitutionBinding, Statement},
};

impl NamingTransformation for SubstitutionBinding {
    type Target = SubstitutionBinding;
    fn transform(self, state: &mut TransformState) -> Self::Target {
        match self {
            SubstitutionBinding::ProducerBinding(prod) => {
                SubstitutionBinding::ProducerBinding(prod.transform(state))
            }
            SubstitutionBinding::ConsumerBinding(cons) => {
                SubstitutionBinding::ConsumerBinding(cons.transform(state))
            }
        }
    }
}

impl Bind for SubstitutionBinding {
    fn bind(self, k: Continuation, state: &mut TransformState) -> Statement {
        match self {
            SubstitutionBinding::ProducerBinding(prod) => prod.bind(k, state),
            SubstitutionBinding::ConsumerBinding(cons) => cons.bind(k, state),
        }
    }
}
