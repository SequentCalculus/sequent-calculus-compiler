use super::super::{
    naming_transformation::{Bind, NamingTransformation, TransformState},
    syntax::{Cut, Literal, MuTilde, Name, Statement},
};
use std::rc::Rc;

impl NamingTransformation for Literal {
    fn transform(self, _st: &mut TransformState) -> Literal {
        self
    }
}

impl Bind for Literal {
    fn bind<F>(self, k: F, st: &mut TransformState) -> Statement
    where
        F: Fn(Name) -> Statement,
    {
        let new_v = st.fresh_var();
        Cut {
            producer: Rc::new(self.into()),
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
