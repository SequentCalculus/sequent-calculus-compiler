use crate::core::syntax::{Consumer, Pattern, Producer, Statement};
use crate::fun::syntax::{Covariable, Dtor, Variable};
use std::collections::HashSet;
use std::rc::Rc;

use super::syntax::{Cocase, Constructor, Cut, Literal, Mu, Op};

//---------------------------------------------------
//---------------Free (Co-) Variables----------------
//---------------------------------------------------
pub trait FreeV {
    fn free_vars(&self) -> HashSet<Variable>;
    fn free_covars(&self) -> HashSet<Covariable>;
}

impl<T: FreeV> FreeV for Vec<T> {
    fn free_vars(self: &Vec<T>) -> HashSet<Variable> {
        self.iter().fold(HashSet::new(), |free_vars, t| {
            free_vars.union(&t.free_vars()).cloned().collect()
        })
    }
    fn free_covars(self: &Vec<T>) -> HashSet<Covariable> {
        self.iter().fold(HashSet::new(), |free_vars, t| {
            free_vars.union(&t.free_vars()).cloned().collect()
        })
    }
}

impl<T: FreeV> FreeV for Rc<T> {
    fn free_vars(&self) -> HashSet<Variable> {
        (**self).free_vars()
    }

    fn free_covars(&self) -> HashSet<Covariable> {
        (**self).free_covars()
    }
}

impl FreeV for Consumer {
    fn free_vars(self: &Consumer) -> HashSet<Variable> {
        match self {
            Consumer::Covar(_) => HashSet::new(),
            Consumer::MuTilde(var, st) => {
                let mut fr_st = st.free_vars();
                fr_st.remove(var);
                fr_st
            }
            Consumer::Case(pts) => FreeV::free_vars(pts),
            Consumer::Destructor(_, pargs, cargs) => {
                let free_args = pargs.free_vars();
                let free_coargs = cargs.free_vars();
                free_args.union(&free_coargs).cloned().collect()
            }
        }
    }

    fn free_covars(self: &Consumer) -> HashSet<Covariable> {
        match self {
            Consumer::Covar(covar) => HashSet::from([covar.clone()]),
            Consumer::MuTilde(_, st) => st.free_covars(),
            Consumer::Case(pts) => FreeV::free_covars(pts),
            Consumer::Destructor(_, pargs, cargs) => {
                let free_args = cargs.free_covars();
                let free_coargs = pargs.free_covars();
                free_args.union(&free_coargs).cloned().collect()
            }
        }
    }
}

impl FreeV for Statement {
    fn free_vars(self: &Statement) -> HashSet<Variable> {
        match self {
            Statement::Cut(c) => c.free_vars(),
            Statement::Op(op) => op.free_vars(),
            Statement::IfZ(p, st1, st2) => {
                let free_p = p.free_vars();
                let free_st1 = st1.free_vars();
                let free_st2 = st2.free_vars();
                let free_st: HashSet<Variable> = free_st1.union(&free_st2).cloned().collect();
                free_st.union(&free_p).cloned().collect()
            }
            Statement::Fun(_, pargs, cargs) => {
                let free_p = pargs.free_vars();
                let free_c = cargs.free_vars();
                free_p.union(&free_c).cloned().collect()
            }
            Statement::Done() => HashSet::new(),
        }
    }
    fn free_covars(self: &Statement) -> HashSet<Covariable> {
        match self {
            Statement::Cut(c) => c.free_covars(),
            Statement::Op(op) => op.free_covars(),
            Statement::IfZ(p, st1, st2) => {
                let free_p = p.free_covars();
                let free_st1 = st1.free_covars();
                let free_st2 = st2.free_covars();
                let free_st: HashSet<Variable> = free_st1.union(&free_st2).cloned().collect();
                free_st.union(&free_p).cloned().collect()
            }
            Statement::Fun(_, pargs, cargs) => {
                let free_p = pargs.free_covars();
                let free_c = cargs.free_covars();
                free_p.union(&free_c).cloned().collect()
            }
            Statement::Done() => HashSet::new(),
        }
    }
}

pub fn fresh_var(xs: &HashSet<Variable>) -> Variable {
    fresh_var_n(xs, 0)
}

fn fresh_var_n(xs: &HashSet<Variable>, n: i32) -> Variable {
    let new_var: Variable = format!("x{}", n);
    if xs.contains(&new_var) {
        fresh_var_n(xs, n + 1)
    } else {
        new_var
    }
}

pub fn fresh_covar(xs: &HashSet<Covariable>) -> Covariable {
    fresh_covar_n(xs, 0)
}

fn fresh_covar_n(xs: &HashSet<Covariable>, n: i32) -> Covariable {
    let new_covar: Covariable = format!("a{}", n);
    if xs.contains(&new_covar) {
        fresh_covar_n(xs, n + 1)
    } else {
        new_covar
    }
}

//---------------------------------------------------
//------------------ Substitution -------------------
//---------------------------------------------------

pub trait Subst {
    fn subst_sim(
        &self,
        prod_subst: &[(Producer, Variable)],
        cons_subst: &[(Consumer, Covariable)],
    ) -> Rc<Self>;

    fn subst_var(&self, prod: Producer, var: Variable) -> Rc<Self> {
        self.subst_sim(&[(prod, var)], &[])
    }
    fn subst_covar(&self, cons: Consumer, covar: Covariable) -> Rc<Self> {
        self.subst_sim(&[], &[(cons, covar)])
    }
}

impl<T: Subst + Clone> Subst for Vec<T> {
    fn subst_sim(
        self: &Vec<T>,
        prod_subst: &[(Producer, Variable)],
        cons_subst: &[(Consumer, Covariable)],
    ) -> Rc<Vec<T>> {
        Rc::new(
            self.iter()
                .map(|x| Rc::unwrap_or_clone(x.subst_sim(prod_subst, cons_subst)))
                .collect(),
        )
    }
}

impl<T: Clone> Subst for Pattern<T> {
    fn subst_sim(
        self: &Pattern<T>,
        prod_subst: &[(Producer, Variable)],
        cons_subst: &[(Consumer, Covariable)],
    ) -> Rc<Pattern<T>> {
        let mut fr_v: HashSet<Variable> = FreeV::free_vars(Rc::as_ref(&self.rhs));
        let mut fr_cv: HashSet<Covariable> = FreeV::free_covars(Rc::as_ref(&self.rhs));
        for (prod, var) in prod_subst.iter() {
            fr_v.extend(FreeV::free_vars(prod));
            fr_v.insert(var.clone());

            fr_cv.extend(FreeV::free_covars(prod));
        }
        for (cons, covar) in cons_subst.iter() {
            fr_v.extend(FreeV::free_vars(cons));

            fr_cv.extend(FreeV::free_covars(cons));
            fr_cv.insert(covar.clone());
        }

        let mut new_vars: Vec<Variable> = vec![];
        let mut var_subst: Vec<(Producer, Variable)> = vec![];

        for old_var in self.vars.iter() {
            let new_var: Variable = fresh_var(&fr_v);
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

        let new_st: Rc<Statement> =
            Subst::subst_sim(Rc::as_ref(&self.rhs), &var_subst, &covar_subst);

        let new_pt: Pattern<T> = Pattern {
            xtor: self.xtor.clone(),
            vars: new_vars,
            covars: new_covars,
            rhs: Subst::subst_sim(Rc::as_ref(&new_st), prod_subst, cons_subst),
        };
        Rc::new(new_pt)
    }
}

impl Subst for Producer {
    fn subst_sim(
        self: &Producer,
        prod_subst: &[(Producer, Variable)],
        cons_subst: &[(Consumer, Covariable)],
    ) -> Rc<Producer> {
        match self {
            Producer::Variable(crate::core::syntax::Variable { var }) => {
                match prod_subst.iter().find(|(_, v)| v == var) {
                    None => Rc::new(crate::core::syntax::Variable { var: var.clone() }.into()),
                    Some((p, _)) => Rc::new(p.clone()),
                }
            }
            Producer::Literal(Literal { lit }) => Rc::new(Literal { lit: *lit }.into()),
            Producer::Mu(Mu {
                covariable,
                statement,
            }) => {
                let mut fr_cv: HashSet<Covariable> = FreeV::free_vars(Rc::as_ref(statement));
                for (cons, cv) in cons_subst.iter() {
                    fr_cv.insert(cv.clone());
                    fr_cv.extend(FreeV::free_covars(cons));
                }
                for (prod, _) in prod_subst.iter() {
                    fr_cv.extend(FreeV::free_covars(prod));
                }
                let new_covar: Covariable = fresh_covar(&fr_cv);
                let new_st: Rc<Statement> = Subst::subst_covar(
                    statement,
                    Consumer::Covar(new_covar.clone()),
                    covariable.clone(),
                );
                let new_mu: Producer = Mu {
                    covariable: new_covar,
                    statement: Subst::subst_sim(Rc::as_ref(&new_st), prod_subst, cons_subst),
                }
                .into();
                Rc::new(new_mu)
            }
            Producer::Constructor(Constructor {
                id,
                producers,
                consumers,
            }) => {
                let pargs_subst: Vec<Rc<Producer>> = producers
                    .iter()
                    .map(|p| Subst::subst_sim(Rc::as_ref(p), prod_subst, cons_subst))
                    .collect();
                let cargs_subst: Vec<Rc<Consumer>> = consumers
                    .iter()
                    .map(|c| Subst::subst_sim(Rc::as_ref(c), prod_subst, cons_subst))
                    .collect();
                let new_ctor = Constructor {
                    id: id.clone(),
                    producers: pargs_subst,
                    consumers: cargs_subst,
                }
                .into();
                Rc::new(new_ctor)
            }
            Producer::Cocase(Cocase { cocases }) => {
                let pts_subst: Rc<Vec<Pattern<Dtor>>> = cocases.subst_sim(prod_subst, cons_subst);
                Rc::new(
                    Cocase {
                        cocases: Rc::unwrap_or_clone(pts_subst),
                    }
                    .into(),
                )
            }
        }
    }
}

impl Subst for Consumer {
    fn subst_sim(
        self: &Consumer,
        prod_subst: &[(Producer, Variable)],
        cons_subst: &[(Consumer, Covariable)],
    ) -> Rc<Consumer> {
        match self {
            Consumer::Covar(covar) => match cons_subst.iter().find(|(_, cv)| cv == covar) {
                None => Rc::new(Consumer::Covar(covar.clone())),
                Some((cons, _)) => Rc::new(cons.clone()),
            },
            Consumer::MuTilde(var, st) => {
                let mut fr_v: HashSet<Variable> = FreeV::free_vars(Rc::as_ref(st));
                for (prod, var) in prod_subst.iter() {
                    fr_v.extend(FreeV::free_vars(prod));
                    fr_v.insert(var.clone());
                }
                for (cons, _) in cons_subst.iter() {
                    fr_v.extend(FreeV::free_vars(cons));
                }
                let new_var: Variable = fresh_var(&fr_v);
                let new_st = st.subst_var(
                    crate::core::syntax::Variable {
                        var: new_var.clone(),
                    }
                    .into(),
                    var.clone(),
                );
                let new_mu: Consumer =
                    Consumer::MuTilde(new_var, new_st.subst_sim(prod_subst, cons_subst));
                Rc::new(new_mu)
            }
            Consumer::Case(pts) => {
                let pts_subst = pts.subst_sim(prod_subst, cons_subst);
                Rc::new(Consumer::Case(Rc::unwrap_or_clone(pts_subst)))
            }
            Consumer::Destructor(dtor, pargs, cargs) => {
                let pargs_subst: Vec<Rc<Producer>> = pargs
                    .iter()
                    .map(|p| Subst::subst_sim(Rc::as_ref(p), prod_subst, cons_subst))
                    .collect();
                let cargs_subst: Vec<Rc<Consumer>> = cargs
                    .iter()
                    .map(|c| Subst::subst_sim(Rc::as_ref(c), prod_subst, cons_subst))
                    .collect();
                Rc::new(Consumer::Destructor(dtor.clone(), pargs_subst, cargs_subst))
            }
        }
    }
}
impl Subst for Statement {
    fn subst_sim(
        self: &Statement,
        prod_subst: &[(Producer, Variable)],
        cons_subst: &[(Consumer, Covariable)],
    ) -> Rc<Statement> {
        match self {
            Statement::Cut(c) => {
                let Cut { producer, consumer } = c;
                let p_subst = producer.subst_sim(prod_subst, cons_subst);
                let c_subst = consumer.subst_sim(prod_subst, cons_subst);
                Rc::new(
                    Cut {
                        producer: p_subst,
                        consumer: c_subst,
                    }
                    .into(),
                )
            }
            Statement::Op(Op {
                fst: p1,
                op,
                snd: p2,
                continuation: c,
            }) => {
                let p1_subst = p1.subst_sim(prod_subst, cons_subst);
                let p2_subst = p2.subst_sim(prod_subst, cons_subst);
                let c_subst = c.subst_sim(prod_subst, cons_subst);
                Rc::new(
                    Op {
                        fst: p1_subst,
                        op: op.clone(),
                        snd: p2_subst,
                        continuation: c_subst,
                    }
                    .into(),
                )
            }
            Statement::IfZ(p, st1, st2) => {
                let p_subst = p.subst_sim(prod_subst, cons_subst);
                let st1_subst = st1.subst_sim(prod_subst, cons_subst);
                let st2_subst = st2.subst_sim(prod_subst, cons_subst);
                Rc::new(Statement::IfZ(p_subst, st1_subst, st2_subst))
            }
            Statement::Fun(nm, pargs, cargs) => {
                let pargs_subst: Vec<Rc<Producer>> = pargs
                    .iter()
                    .map(|p| Subst::subst_sim(Rc::as_ref(p), prod_subst, cons_subst))
                    .collect();
                let cargs_subst: Vec<Rc<Consumer>> = cargs
                    .iter()
                    .map(|c| Subst::subst_sim(Rc::as_ref(c), prod_subst, cons_subst))
                    .collect();
                Rc::new(Statement::Fun(nm.clone(), pargs_subst, cargs_subst))
            }
            Statement::Done() => Rc::new(Statement::Done()),
        }
    }
}
