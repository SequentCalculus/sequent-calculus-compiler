use super::{Bind, Continuation, NamingTransformation, TransformState};
use crate::syntax::{
    statement::Cut,
    term::{Cns, Mu, Prd, Term, XCase},
    Statement,
};
use std::rc::Rc;

impl NamingTransformation for XCase<Cns> {
    type Target = XCase<Cns>;

    ///N(case {cases}) = case { N(cases) }
    fn transform(self, state: &mut TransformState) -> Self::Target {
        XCase {
            prdcns: Cns,
            clauses: self.clauses.transform(state),
        }
    }
}

impl NamingTransformation for XCase<Prd> {
    type Target = XCase<Prd>;
    ///N(cocase {cocases}) = cocase { N(cocases) }
    fn transform(self, state: &mut TransformState) -> Self::Target {
        XCase {
            prdcns: Prd,
            clauses: self.clauses.transform(state),
        }
    }
}

impl Bind for XCase<Cns> {
    ///bind(case {cases)[k] =  ⟨μa.k(a) | case N{cases}⟩
    fn bind(self, k: Continuation, state: &mut TransformState) -> Statement {
        let new_covar = state.fresh_covar();
        Cut {
            consumer: Rc::new(Term::XCase(XCase {
                prdcns: Cns,
                clauses: self.clauses.transform(state),
            })),
            producer: Rc::new(Term::Mu(Mu {
                prdcns: Prd,
                variable: new_covar.clone(),
                statement: Rc::new(k(new_covar, state)),
            })),
        }
        .into()
    }
}

impl Bind for XCase<Prd> {
    ///bind(cocase {cocases)[k] = ⟨cocase N(cocases) | ~μx.k(x)⟩
    fn bind(self, k: Continuation, state: &mut TransformState) -> Statement {
        let new_var = state.fresh_var();
        Cut {
            producer: Rc::new(Term::XCase(self.transform(state))),
            consumer: Rc::new(Term::Mu(Mu {
                prdcns: Cns,
                variable: new_var.clone(),
                statement: Rc::new(k(new_var, state)),
            })),
        }
        .into()
    }
}
