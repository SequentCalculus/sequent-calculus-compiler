use super::super::{
    naming_transformation::{Bind, NamingTransformation, TransformState},
    syntax::{Cut, Mu, MuTilde, Name, Statement},
};
use std::rc::Rc;

impl NamingTransformation for MuTilde {
    type Target = MuTilde;
    ///N ( μx  ̃ .s) = μx ̃ .N (s)
    fn transform(self, st: &mut TransformState) -> MuTilde {
        MuTilde {
            variable: self.variable,
            statement: self.statement.transform(st),
        }
    }
}

impl Bind for MuTilde {
    /// bind(μx  ̃ .s) [k] = ⟨μα .k (α) | μx  ̃.N (s)⟩
    fn bind<F, K>(self, k: F, st: &mut TransformState) -> Statement
    where
        F: FnOnce(Name) -> K,
        K: FnOnce(&mut TransformState) -> Statement,
    {
        let new_cv = st.fresh_covar();
        Cut {
            producer: Rc::new(
                Mu {
                    covariable: new_cv.clone(),
                    statement: Rc::new(k(new_cv)(st)),
                }
                .into(),
            ),
            consumer: Rc::new(self.transform(st).into()),
        }
        .into()
    }
}
