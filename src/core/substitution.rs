use crate::core::syntax::{Clause, Consumer, Covariable, Producer, Statement, Var};
use std::collections::HashSet;
use std::rc::Rc;

use super::{
    syntax::{Cocase, Constructor, Cut, Fun, IfZ, Literal, Mu, Op, Variable},
    traits::{
        free_vars::{fresh_covar, fresh_var, FreeV},
        substitution::Subst,
    },
};

impl<T: Clone> Subst for Clause<T> {
    type Target = Clause<T>;
    fn subst_sim(
        self: &Clause<T>,
        prod_subst: &[(Producer, Var)],
        cons_subst: &[(Consumer, Covariable)],
    ) -> Clause<T> {
        let mut fr_v = self.rhs.free_vars();
        let mut fr_cv = self.rhs.free_covars();
        for (prod, var) in prod_subst.iter() {
            fr_v.extend(prod.free_vars());
            fr_v.insert(var.clone());

            fr_cv.extend(prod.free_covars());
        }
        for (cons, covar) in cons_subst.iter() {
            fr_v.extend(cons.free_vars());

            fr_cv.extend(cons.free_covars());
            fr_cv.insert(covar.clone());
        }

        let mut new_vars: Vec<Var> = vec![];
        let mut var_subst: Vec<(Producer, Var)> = vec![];

        for old_var in self.vars.iter() {
            let new_var: Var = fresh_var(&fr_v);
            fr_v.insert(new_var.clone());
            new_vars.insert(0, new_var.clone());
            var_subst.insert(
                0,
                (
                    crate::core::syntax::Variable { var: new_var }.into(),
                    old_var.clone(),
                ),
            )
        }

        let mut new_covars: Vec<Covariable> = vec![];
        let mut covar_subst: Vec<(Consumer, Covariable)> = vec![];

        for old_covar in self.covars.iter() {
            let new_covar: Covariable = fresh_covar(&fr_cv);
            fr_cv.insert(new_covar.clone());
            new_covars.insert(0, new_covar.clone());
            covar_subst.insert(0, (Consumer::Covar(new_covar), old_covar.clone()))
        }

        let new_st = self.rhs.subst_sim(&var_subst, &covar_subst);

        let new_pt: Clause<T> = Clause {
            xtor: self.xtor.clone(),
            vars: new_vars,
            covars: new_covars,
            rhs: new_st.subst_sim(prod_subst, cons_subst),
        };
        new_pt
    }
}

impl Subst for Variable {
    type Target = Producer;

    fn subst_sim(
        &self,
        prod_subst: &[(Producer, Var)],
        _cons_subst: &[(Consumer, Covariable)],
    ) -> Self::Target {
        let crate::core::syntax::Variable { var } = self;
        match prod_subst.iter().find(|(_, v)| v == var) {
            None => crate::core::syntax::Variable { var: var.clone() }.into(),
            Some((p, _)) => p.clone(),
        }
    }
}

impl Subst for Literal {
    type Target = Literal;

    fn subst_sim(
        &self,
        _prod_subst: &[(Producer, Var)],
        _cons_subst: &[(Consumer, Covariable)],
    ) -> Self::Target {
        self.clone()
    }
}

impl Subst for Mu {
    type Target = Mu;

    fn subst_sim(
        &self,
        prod_subst: &[(Producer, Var)],
        cons_subst: &[(Consumer, Covariable)],
    ) -> Self::Target {
        let Mu {
            covariable,
            statement,
        } = self;
        let mut fr_cv: HashSet<Covariable> = statement.free_vars();
        for (cons, cv) in cons_subst.iter() {
            fr_cv.insert(cv.clone());
            fr_cv.extend(cons.free_covars());
        }
        for (prod, _) in prod_subst.iter() {
            fr_cv.extend(prod.free_covars());
        }
        let new_covar: Covariable = fresh_covar(&fr_cv);
        let new_st: Rc<Statement> =
            statement.subst_covar(Consumer::Covar(new_covar.clone()), covariable.clone());
        Mu {
            covariable: new_covar,
            statement: new_st.subst_sim(prod_subst, cons_subst),
        }
    }
}

impl Subst for Cocase {
    type Target = Cocase;

    fn subst_sim(
        &self,
        prod_subst: &[(Producer, Var)],
        cons_subst: &[(Consumer, Covariable)],
    ) -> Self::Target {
        Cocase {
            cocases: self.cocases.subst_sim(prod_subst, cons_subst),
        }
    }
}

impl Subst for Constructor {
    type Target = Constructor;

    fn subst_sim(
        &self,
        prod_subst: &[(Producer, Var)],
        cons_subst: &[(Consumer, Covariable)],
    ) -> Self::Target {
        Constructor {
            id: self.id.clone(),
            producers: self.producers.subst_sim(prod_subst, cons_subst),
            consumers: self.consumers.subst_sim(prod_subst, cons_subst),
        }
    }
}

impl Subst for Producer {
    type Target = Producer;
    fn subst_sim(
        self: &Producer,
        prod_subst: &[(Producer, Var)],
        cons_subst: &[(Consumer, Covariable)],
    ) -> Producer {
        match self {
            Producer::Variable(v) => v.subst_sim(prod_subst, cons_subst),
            Producer::Literal(l) => l.subst_sim(prod_subst, cons_subst).into(),
            Producer::Mu(m) => m.subst_sim(prod_subst, cons_subst).into(),
            Producer::Constructor(c) => c.subst_sim(prod_subst, cons_subst).into(),
            Producer::Cocase(c) => c.subst_sim(prod_subst, cons_subst).into(),
        }
    }
}

impl Subst for Consumer {
    type Target = Consumer;
    fn subst_sim(
        self: &Consumer,
        prod_subst: &[(Producer, Var)],
        cons_subst: &[(Consumer, Covariable)],
    ) -> Consumer {
        match self {
            Consumer::Covar(covar) => match cons_subst.iter().find(|(_, cv)| cv == covar) {
                None => Consumer::Covar(covar.clone()),
                Some((cons, _)) => cons.clone(),
            },
            Consumer::MuTilde(var, st) => {
                let mut fr_v: HashSet<Var> = st.free_vars();
                for (prod, var) in prod_subst.iter() {
                    fr_v.extend(prod.free_vars());
                    fr_v.insert(var.clone());
                }
                for (cons, _) in cons_subst.iter() {
                    fr_v.extend(cons.free_vars());
                }
                let new_var: Var = fresh_var(&fr_v);
                let new_st = st.subst_var(
                    crate::core::syntax::Variable {
                        var: new_var.clone(),
                    }
                    .into(),
                    var.clone(),
                );
                let new_mu: Consumer =
                    Consumer::MuTilde(new_var, new_st.subst_sim(prod_subst, cons_subst));
                new_mu
            }
            Consumer::Case(pts) => Consumer::Case(pts.subst_sim(prod_subst, cons_subst)),
            Consumer::Destructor(dtor, pargs, cargs) => Consumer::Destructor(
                dtor.clone(),
                pargs.subst_sim(prod_subst, cons_subst),
                cargs.subst_sim(prod_subst, cons_subst),
            ),
        }
    }
}
impl Subst for Statement {
    type Target = Statement;
    fn subst_sim(
        self: &Statement,
        prod_subst: &[(Producer, Var)],
        cons_subst: &[(Consumer, Covariable)],
    ) -> Statement {
        match self {
            Statement::Cut(c) => c.subst_sim(prod_subst, cons_subst).into(),
            Statement::Op(o) => o.subst_sim(prod_subst, cons_subst).into(),
            Statement::IfZ(i) => i.subst_sim(prod_subst, cons_subst).into(),
            Statement::Fun(f) => f.subst_sim(prod_subst, cons_subst).into(),
            Statement::Done() => Statement::Done(),
        }
    }
}

impl Subst for IfZ {
    type Target = IfZ;

    fn subst_sim(
        &self,
        prod_subst: &[(Producer, Var)],
        cons_subst: &[(Consumer, Covariable)],
    ) -> Self::Target {
        IfZ {
            ifc: self.ifc.subst_sim(prod_subst, cons_subst),
            thenc: self.thenc.subst_sim(prod_subst, cons_subst),
            elsec: self.elsec.subst_sim(prod_subst, cons_subst),
        }
    }
}
impl Subst for Fun {
    type Target = Fun;

    fn subst_sim(
        &self,
        prod_subst: &[(Producer, Var)],
        cons_subst: &[(Consumer, Covariable)],
    ) -> Self::Target {
        Fun {
            name: self.name.clone(),
            producers: self.producers.subst_sim(prod_subst, cons_subst),
            consumers: self.consumers.subst_sim(prod_subst, cons_subst),
        }
    }
}

impl Subst for Cut {
    type Target = Cut;

    fn subst_sim(
        &self,
        prod_subst: &[(Producer, Var)],
        cons_subst: &[(Consumer, Covariable)],
    ) -> Self::Target {
        Cut {
            producer: self.producer.subst_sim(prod_subst, cons_subst),
            consumer: self.consumer.subst_sim(prod_subst, cons_subst),
        }
    }
}

impl Subst for Op {
    type Target = Op;

    fn subst_sim(
        &self,
        prod_subst: &[(Producer, Var)],
        cons_subst: &[(Consumer, Covariable)],
    ) -> Self::Target {
        Op {
            fst: self.fst.subst_sim(prod_subst, cons_subst),
            op: self.op.clone(),
            snd: self.snd.subst_sim(prod_subst, cons_subst),
            continuation: self.continuation.subst_sim(prod_subst, cons_subst),
        }
    }
}
