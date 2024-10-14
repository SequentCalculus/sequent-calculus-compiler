use crate::{
    naming_transformation::{Bind, Continuation, NamingTransformation, TransformState},
    syntax::{
        term::{Cns, Prd, Term},
        Statement,
    },
};

impl NamingTransformation for Term<Prd> {
    type Target = Term<Prd>;

    fn transform(self, st: &mut TransformState) -> Self::Target {
        match self {
            Term::XVar(var) => Term::XVar(var),
            Term::Literal(lit) => Term::Literal(lit),
            Term::Mu(mu) => mu.transform(st).into(),
            Term::Xtor(xtor) => xtor.transform(st),
            Term::XCase(xcase) => xcase.transform(st).into(),
        }
    }
}

impl NamingTransformation for Term<Cns> {
    type Target = Term<Cns>;

    fn transform(self, st: &mut TransformState) -> Self::Target {
        match self {
            Term::XVar(var) => Term::XVar(var),
            Term::Literal(lit) => Term::Literal(lit),
            Term::Mu(mu) => mu.transform(st).into(),
            Term::Xtor(xtor) => xtor.transform(st),
            Term::XCase(xcase) => xcase.transform(st).into(),
        }
    }
}

impl Bind for Term<Prd> {
    fn bind(self, k: Continuation, st: &mut TransformState) -> Statement {
        match self {
            Term::XVar(xvar) => k(xvar.var, st),
            Term::Literal(lit) => lit.bind(k, st),
            Term::Mu(mu) => mu.bind(k, st),
            Term::Xtor(xtor) => xtor.bind(k, st),
            Term::XCase(xcase) => xcase.bind(k, st),
        }
    }
}

impl Bind for Term<Cns> {
    fn bind(self, k: Continuation, st: &mut TransformState) -> Statement {
        match self {
            Term::XVar(xvar) => k(xvar.var, st),
            Term::Literal(lit) => lit.bind(k, st),
            Term::Mu(mu) => mu.bind(k, st),
            Term::Xtor(xtor) => xtor.bind(k, st),
            Term::XCase(xcase) => xcase.bind(k, st),
        }
    }
}
