use super::super::{
    naming_transformation::{NamingTransformation, TransformState},
    syntax::Fun,
};

impl NamingTransformation for Fun {
    type Target = Fun;
    ///N (f (pi ; c j )) = bind(pi ) [λas.bind(c j ) [λbs.f (as; bs)]]
    fn transform(self, _st: &mut TransformState) -> Fun {
        todo!("nor implemented")
    }
}
