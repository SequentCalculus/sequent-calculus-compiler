use super::super::{
    naming_transformation::{NamingTransformation, TransformState},
    syntax::Op,
};

impl NamingTransformation for Op {
    ///N (⊙(p 1, p 2 ; c)) = bind(p 1) [λa1 .bind(p 2 ) [λa2 .bind(c) [λb. ⊙ (a1 , a 2; b)]]]
    fn transform(self, _st: &mut TransformState) -> Op {
        todo!("nor implemented")
    }
}
