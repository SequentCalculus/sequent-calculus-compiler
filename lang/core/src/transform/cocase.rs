use crate::{
    naming_transformation::{Bind, NamingTransformation, TransformState},
    syntax::{Cocase, Cut, MuTilde, Name, Statement},
};
use std::rc::Rc;

impl NamingTransformation for Cocase {
    type Target = Cocase;
    ///N (cocase {cocases}) = cocase { N(cocases) }
    fn transform(self, st: &mut TransformState) -> Cocase {
        Cocase {
            cocases: self.cocases.transform(st),
        }
    }
}

impl Bind for Cocase {
    ///bind(cocase {cocases) [k] = ⟨cocase N(cocases) | μxk (x)⟩
    fn bind<F>(self, k: F, st: &mut TransformState) -> Statement
    where
        F: FnOnce(Name) -> Statement,
    {
        let new_v = st.fresh_var();
        Cut {
            producer: Rc::new(self.transform(st).into()),
            consumer: Rc::new(
                MuTilde {
                    variable: new_v.clone(),
                    statement: Rc::new(k(new_v)),
                }
                .into(),
            ),
        }
        .into()
    }
}
