use crate::{
    naming_transformation::{Bind, NamingTransformation, TransformState},
    syntax::{Case, Cut, Mu, Name, Statement},
};
use std::rc::Rc;

impl NamingTransformation for Case {
    ///N (case {cases}) = case { N(cases) }
    fn transform(self, st: &mut TransformState) -> Case {
        Case {
            cases: self.cases.transform(st),
        }
    }
}

impl Bind for Case {
    ///bind(case {cases) [k] =  ⟨case N{cases} | μx  ̃ .k (x)⟩
    fn bind<F>(self, k: F, st: &mut TransformState) -> Statement
    where
        F: Fn(Name) -> Statement,
    {
        let new_cv = st.fresh_covar();
        Cut {
            consumer: Rc::new(
                Case {
                    cases: self.cases.transform(st),
                }
                .into(),
            ),
            producer: Rc::new(
                Mu {
                    covariable: new_cv.clone(),
                    statement: Rc::new(k(new_cv)),
                }
                .into(),
            ),
        }
        .into()
    }
}
