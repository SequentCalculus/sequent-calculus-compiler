use super::super::{
    naming_transformation::{NamingTransformation, TransformState},
    syntax::Destructor,
};

impl NamingTransformation for Destructor {
    fn transform(self, _st: &mut TransformState) -> Destructor {
        todo!("not implemented")
    }
}
