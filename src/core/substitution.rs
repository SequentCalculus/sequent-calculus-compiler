use crate::core::syntax::{Clause, Consumer, Covariable, Producer, Statement, Var};
use std::collections::HashSet;
use std::rc::Rc;

use super::{
    syntax::{Cocase, Constructor, Cut, Fun, IfZ, Literal, Mu, Op},
    traits::free_vars::{fresh_covar, fresh_var, FreeV},
};

//---------------------------------------------------
//------------------ Substitution -------------------
//---------------------------------------------------

pub trait Subst: Clone {
    type Target: Clone;
    fn subst_sim(
        &self,
        prod_subst: &[(Producer, Var)],
        cons_subst: &[(Consumer, Covariable)],
    ) -> Self::Target;

    fn subst_var(&self, prod: Producer, var: Var) -> Self::Target {
        self.subst_sim(&[(prod, var)], &[])
    }
    fn subst_covar(&self, cons: Consumer, covar: Covariable) -> Self::Target {
        self.subst_sim(&[], &[(cons, covar)])
    }
}

impl<T: Subst> Subst for Rc<T> {
    type Target = Rc<T::Target>;
    fn subst_sim(
        &self,
        prod_subst: &[(Producer, Var)],
        cons_subst: &[(Consumer, Covariable)],
    ) -> Self::Target {
        Rc::new((**self).subst_sim(prod_subst, cons_subst))
    }
}

impl<T: Subst + Clone> Subst for Vec<T> {
    type Target = Vec<T::Target>;
    fn subst_sim(
        self: &Vec<T>,
        prod_subst: &[(Producer, Var)],
        cons_subst: &[(Consumer, Covariable)],
    ) -> Vec<T::Target> {
        self.iter()
            .map(|x| x.subst_sim(prod_subst, cons_subst))
            .collect()
    }
}

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

impl Subst for Producer {
    type Target = Producer;
    fn subst_sim(
        self: &Producer,
        prod_subst: &[(Producer, Var)],
        cons_subst: &[(Consumer, Covariable)],
    ) -> Producer {
        match self {
            Producer::Variable(crate::core::syntax::Variable { var }) => {
                match prod_subst.iter().find(|(_, v)| v == var) {
                    None => crate::core::syntax::Variable { var: var.clone() }.into(),
                    Some((p, _)) => p.clone(),
                }
            }
            Producer::Literal(Literal { lit }) => Literal { lit: *lit }.into(),
            Producer::Mu(Mu {
                covariable,
                statement,
            }) => {
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
                .into()
            }
            Producer::Constructor(Constructor {
                id,
                producers,
                consumers,
            }) => {
                let pargs_subst: Vec<Rc<Producer>> = producers
                    .iter()
                    .map(|p| p.subst_sim(prod_subst, cons_subst))
                    .collect();
                let cargs_subst: Vec<Rc<Consumer>> = consumers
                    .iter()
                    .map(|c| c.subst_sim(prod_subst, cons_subst))
                    .collect();
                Constructor {
                    id: id.clone(),
                    producers: pargs_subst,
                    consumers: cargs_subst,
                }
                .into()
            }
            Producer::Cocase(Cocase { cocases }) => Cocase {
                cocases: cocases.subst_sim(prod_subst, cons_subst),
            }
            .into(),
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
            Consumer::Case(pts) => {
                let pts_subst = pts.subst_sim(prod_subst, cons_subst);
                Consumer::Case(pts_subst)
            }
            Consumer::Destructor(dtor, pargs, cargs) => {
                let pargs_subst: Vec<Rc<Producer>> = pargs
                    .iter()
                    .map(|p| p.subst_sim(prod_subst, cons_subst))
                    .collect();
                let cargs_subst: Vec<Rc<Consumer>> = cargs
                    .iter()
                    .map(|c| c.subst_sim(prod_subst, cons_subst))
                    .collect();
                Consumer::Destructor(dtor.clone(), pargs_subst, cargs_subst)
            }
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
        let IfZ { ifc, thenc, elsec } = self;
        IfZ {
            ifc: ifc.subst_sim(prod_subst, cons_subst),
            thenc: thenc.subst_sim(prod_subst, cons_subst),
            elsec: elsec.subst_sim(prod_subst, cons_subst),
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
        let Fun {
            name,
            producers,
            consumers,
        } = self;
        Fun {
            name: name.clone(),
            producers: producers
                .iter()
                .map(|p| p.subst_sim(prod_subst, cons_subst))
                .collect(),
            consumers: consumers
                .iter()
                .map(|c| c.subst_sim(prod_subst, cons_subst))
                .collect(),
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
        let Cut { producer, consumer } = self;
        Cut {
            producer: producer.subst_sim(prod_subst, cons_subst),
            consumer: consumer.subst_sim(prod_subst, cons_subst),
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
        let Op {
            fst,
            op,
            snd,
            continuation,
        } = self;
        Op {
            fst: fst.subst_sim(prod_subst, cons_subst),
            op: op.clone(),
            snd: snd.subst_sim(prod_subst, cons_subst),
            continuation: continuation.subst_sim(prod_subst, cons_subst),
        }
    }
}
