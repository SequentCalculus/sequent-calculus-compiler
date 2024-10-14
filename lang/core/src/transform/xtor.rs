use crate::{
    naming_transformation::{bind_many, Bind, Continuation, NamingTransformation, TransformState},
    syntax::term::{Cns, Mu, Prd, Term, XVar, Xtor},
    syntax::{statement::Cut, Statement},
};
use std::rc::Rc;

impl NamingTransformation for Xtor<Prd> {
    type Target = Term<Prd>;
    ///N(K(p_i; c_j)) = μa.bind(p_i)[λas.bind(c_j)[λbs.⟨K(as; bs) | a⟩]]
    fn transform(self, st: &mut TransformState) -> Self::Target {
        let new_covar = st.fresh_covar();
        let new_covar_clone = new_covar.clone();
        let new_statement = bind_many(
            self.args.into(),
            Box::new(|vars, _: &mut TransformState| {
                Cut {
                    producer: Rc::new(
                        Term::Xtor(Xtor {
                            prdcns: self.prdcns,
                            id: self.id,
                            args: vars.into_iter().collect(),
                        })
                        .into(),
                    ),
                    consumer: Rc::new(
                        Term::XVar(XVar {
                            prdcns: Cns,
                            var: new_covar,
                        })
                        .into(),
                    ),
                }
                .into()
            }),
            st,
        );

        Term::Mu(Mu {
            prdcns: Prd,
            variable: new_covar_clone,
            statement: Rc::new(new_statement),
        })
        .into()
    }
}

impl NamingTransformation for Xtor<Cns> {
    type Target = Term<Cns>;
    ///N(D(p_i; cj)) =  ~μx.bind(p_i)[λas.bind(c_j)[λbs.⟨x | D(as; bs)⟩]]
    fn transform(self, state: &mut TransformState) -> Term<Cns> {
        let new_var = state.fresh_var();
        let new_var_clone = new_var.clone();
        let new_statement = bind_many(
            self.args.into(),
            Box::new(|args, _: &mut TransformState| {
                Cut {
                    producer: Rc::new(
                        Term::XVar(XVar {
                            prdcns: Prd,
                            var: new_var,
                        })
                        .into(),
                    ),
                    consumer: Rc::new(
                        Term::Xtor(Xtor {
                            prdcns: Cns,
                            id: self.id,
                            args: args.into_iter().collect(),
                        })
                        .into(),
                    ),
                }
                .into()
            }),
            state,
        );
        Term::Mu(Mu {
            prdcns: Cns,
            variable: new_var_clone,
            statement: Rc::new(new_statement),
        })
        .into()
    }
}

impl Bind for Xtor<Prd> {
    fn bind(self, k: Continuation, st: &mut TransformState) -> Statement {
        let new_var = st.fresh_var();
        bind_many(
            self.args.into(),
            Box::new(|vars, state: &mut TransformState| {
                Cut {
                    producer: Rc::new(
                        Term::Xtor(Xtor {
                            prdcns: Prd,
                            id: self.id,
                            args: vars.into_iter().collect(),
                        })
                        .into(),
                    ),
                    consumer: Rc::new(
                        Term::Mu(Mu {
                            prdcns: Cns,
                            variable: new_var.clone(),
                            statement: Rc::new(k(new_var, state)),
                        })
                        .into(),
                    ),
                }
                .into()
            }),
            st,
        )
    }
}

impl Bind for Xtor<Cns> {
    ///bind(D(p_i; c_j))[k] = bind(p_i)[λas.bind(c_j)[λbs.⟨μa.k(a) | D(as; bs)⟩]]
    fn bind(self, k: Continuation, state: &mut TransformState) -> Statement {
        let new_covar = state.fresh_covar();
        bind_many(
            self.args.into(),
            Box::new(|args, state: &mut TransformState| {
                Cut {
                    producer: Rc::new(
                        Term::Mu(Mu {
                            prdcns: Prd,
                            variable: new_covar.clone(),
                            statement: Rc::new(k(new_covar, state)),
                        })
                        .into(),
                    ),
                    consumer: Rc::new(
                        Term::Xtor(Xtor {
                            prdcns: Cns,
                            id: self.id,
                            args: args.into_iter().collect(),
                        })
                        .into(),
                    ),
                }
                .into()
            }),
            state,
        )
    }
}
