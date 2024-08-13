use crate::{
    naming_transformation::{Bind, NamingTransformation, TransformState},
    syntax::{Constructor, Name, Statement},
};

impl NamingTransformation for Constructor {
    ///N (K (pi ; c j )) = μα .bind(pi ) [λas.bind(c j ) [λbs.⟨K (as; bs) | α⟩]]
    fn transform(self, _st: &mut TransformState) -> Constructor {
        todo!("not implemented")
    }
}

impl Bind for Constructor {
    ///bind(K (pi ; c j )) [k] =  bind(p i ) [λas.bind(c j ) [λbs.⟨K (as; bs) | μx  ̃ .k (x)⟩]]
    fn bind<F>(self, _k: F, _st: &mut TransformState) -> Statement
    where
        F: Fn(Name) -> Statement,
    {
        todo!("not impleneted")
    }
}
