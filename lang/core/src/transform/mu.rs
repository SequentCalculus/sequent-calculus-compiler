use super::super::{
    naming_transformation::{NamingTransformation, TransformState},
    syntax::Mu,
};

impl NamingTransformation for Mu {
    fn transform(self, _st: &mut TransformState) -> Mu {
        todo!("not implemented")
    }
}
