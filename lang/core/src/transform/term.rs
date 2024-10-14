use crate::{
    naming_transformation::{Bind, Continuation, NamingTransformation, TransformState},
    syntax::{
        term::{Cns, Prd, PrdCns, Term},
        Statement,
    },
};

impl<T: PrdCns> NamingTransformation for Term<T> {
    type Target = Term<T>;

    fn transform(self, st: &mut TransformState) -> Self::Target {
        match self {
            Term::XVar(var) => Term::XVar(var),
            Term::Literal(lit) => Term::Literal(lit),
            Term::Mu(mu) => mu.transform(st).into(),
            Term::Xtor(xtor) => todo!(),
            Term::XCase(xcase) => todo!(),
        }
    }
}

impl Bind for Term<Prd> {
    fn bind(self, k: Continuation, st: &mut TransformState) -> Statement {
        match self {
            Term::XVar(xvar) => k(xvar.var, st),
            Term::Literal(lit) => lit.bind(k, st),
            Term::Mu(mu) => mu.bind(k, st),
            Term::Xtor(xtor) => todo!(),
            Term::XCase(xcase) => todo!(),
        }
    }
}
impl Bind for Term<Cns> {
    fn bind(self, k: Continuation, st: &mut TransformState) -> Statement {
        match self {
            Term::XVar(xvar) => k(xvar.var, st),
            Term::Literal(lit) => lit.bind(k, st),
            Term::Mu(mu) => mu.bind(k, st),
            Term::Xtor(xtor) => todo!(),
            Term::XCase(xcase) => todo!(),
        }
    }
}
