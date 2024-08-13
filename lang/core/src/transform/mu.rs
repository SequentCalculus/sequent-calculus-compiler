use super::super::{
    naming_transformation::{Bind, NamingTransformation, TransformState},
    syntax::{Cut, Mu, MuTilde, Name, Statement},
};
use std::rc::Rc;

impl NamingTransformation for Mu {
    ///N (μα .s) = μα .N (s)
    fn transform(self, st: &mut TransformState) -> Mu {
        Mu {
            covariable: self.covariable,
            statement: self.statement.transform(st),
        }
    }
}

impl Bind for Mu {
    ///bind(μα .s) [k] =  ⟨μα .N (s) | μx  ̃ .k (x)⟩
    fn bind<F>(self, k: F, st: &mut TransformState) -> Statement
    where
        F: Fn(Name) -> Statement,
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
