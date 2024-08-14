use super::super::{
    naming_transformation::{Bind, NamingTransformation, TransformState},
    syntax::{Destructor, Name, Statement},
};

impl NamingTransformation for Destructor {
    type Target = Destructor;
    ///N (D (pi ; c j )) =  μx  ̃ .bind(pi ) [λas.bind(c j ) [λbs.⟨x | D (as; bs)⟩]]
    fn transform(self, _st: &mut TransformState) -> Destructor {
        todo!("not implemented")
    }
}

impl Bind for Destructor {
    ///bind(D (pi ; c j )) [k] =  bind(p i ) [λas.bind(c j ) [λbs.⟨μα .k (α) | D (as; bs)⟩]]
    fn bind<F, K>(self, _k: F, _st: &mut TransformState) -> Statement
    where
        F: FnOnce(Name) -> K,
        K: FnOnce(&mut TransformState) -> Statement,
    {
        todo!("not impleneted")
    }
}
