use super::super::{
    naming_transformation::{NamingTransformation, TransformState},
    syntax::IfZ,
};

impl NamingTransformation for IfZ {
    ///N (ifz(p, s1 , s2 )) = bind(p) [λa.ifz(a, N (s 1), N (s 2 ))]
    fn transform(self, _st: &mut TransformState) -> IfZ {
        todo!("nor implemented")
    }
}
