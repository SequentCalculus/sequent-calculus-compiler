use crate::core::syntax::{Consumer, Pattern, Producer, Statement};
use crate::fun::syntax::{Covariable, Ctor, Dtor, Variable};
use std::collections::HashSet;
use std::rc::Rc;

//---------------------------------------------------
//---------------Free (Co-) Variables----------------
//---------------------------------------------------
pub trait FreeV {
    fn free_vars(&self) -> HashSet<Variable>;
    fn free_covars(&self) -> HashSet<Covariable>;
}

enum Free<T: FreeV> {
    MkFree(T),
}

impl<T: FreeV> FreeV for Free<T> {
    fn free_vars(self: &Free<T>) -> HashSet<Variable> {
        match self {
            Free::MkFree(a) => FreeV::free_vars(a),
        }
    }
    fn free_covars(self: &Free<T>) -> HashSet<Covariable> {
        match self {
            Free::MkFree(a) => FreeV::free_covars(a),
        }
    }
}

impl<T: FreeV> FreeV for Vec<T> {
    fn free_vars(self: &Vec<T>) -> HashSet<Variable> {
        self.iter().fold(HashSet::new(), |free_vars, t| {
            free_vars.union(&FreeV::free_vars(t)).cloned().collect()
        })
    }
    fn free_covars(self: &Vec<T>) -> HashSet<Covariable> {
        self.iter().fold(HashSet::new(), |free_vars, t| {
            free_vars.union(&FreeV::free_vars(t)).cloned().collect()
        })
    }
}

impl<T: FreeV> FreeV for Vec<Rc<T>> {
    fn free_vars(self: &Vec<Rc<T>>) -> HashSet<Variable> {
        self.iter().fold(HashSet::new(), |frv, arg| {
            frv.union(&FreeV::free_vars(Rc::as_ref(&arg)))
                .cloned()
                .collect()
        })
    }
    fn free_covars(self: &Vec<Rc<T>>) -> HashSet<Covariable> {
        self.iter().fold(HashSet::new(), |frv, arg| {
            frv.union(&FreeV::free_covars(Rc::as_ref(&arg)))
                .cloned()
                .collect()
        })
    }
}

impl<T> FreeV for Pattern<T> {
    fn free_vars(self: &Pattern<T>) -> HashSet<Variable> {
        let free_pt: HashSet<Variable> = FreeV::free_vars(Rc::as_ref(&self.rhs));
        let unfree: HashSet<Variable> = HashSet::from_iter(self.vars.iter().cloned());
        free_pt.difference(&unfree).cloned().collect()
    }
    fn free_covars(self: &Pattern<T>) -> HashSet<Covariable> {
        let free_pt: HashSet<Variable> = FreeV::free_covars(Rc::as_ref(&self.rhs));
        let unfree: HashSet<Variable> = HashSet::from_iter(self.covars.iter().cloned());
        free_pt.difference(&unfree).cloned().collect()
    }
}
impl FreeV for Producer {
    fn free_vars(self: &Producer) -> HashSet<Variable> {
        match self {
            Producer::Var(v) => HashSet::from([v.clone()]),
            Producer::Lit(_) => HashSet::new(),
            Producer::Mu(_, st) => FreeV::free_vars(Rc::as_ref(&st)),
            Producer::MuDyn(_, st) => FreeV::free_vars(Rc::as_ref(&st)),
            Producer::Constructor(_, pargs, cargs) => {
                let free_args: HashSet<Variable> = FreeV::free_vars(pargs);
                let free_coargs: HashSet<Variable> = FreeV::free_vars(cargs);
                free_args.union(&free_coargs).cloned().collect()
            }
            Producer::Cocase(pts) => FreeV::free_vars(pts),
        }
    }

    fn free_covars(self: &Producer) -> HashSet<Covariable> {
        match self {
            Producer::Var(_) => HashSet::new(),
            Producer::Lit(_) => HashSet::new(),
            Producer::Mu(covar, st) => {
                let mut fr_cv: HashSet<Covariable> = FreeV::free_covars(Rc::as_ref(&st));
                fr_cv.remove(covar);
                fr_cv
            }
            Producer::MuDyn(covar, st) => {
                let mut fr_cv: HashSet<Covariable> = FreeV::free_covars(Rc::as_ref(&st));
                fr_cv.remove(covar);
                fr_cv
            }
            Producer::Constructor(_, args, coargs) => {
                let free_args: HashSet<Covariable> = FreeV::free_covars(args);
                let free_coargs: HashSet<Covariable> = FreeV::free_covars(coargs);
                free_args.union(&free_coargs).cloned().collect()
            }
            Producer::Cocase(pts) => FreeV::free_covars(pts),
        }
    }
}

impl FreeV for Consumer {
    fn free_vars(self: &Consumer) -> HashSet<Variable> {
        match self {
            Consumer::Covar(_) => HashSet::new(),
            Consumer::MuTilde(var, st) => {
                let mut fr_st: HashSet<Variable> = FreeV::free_vars(Rc::as_ref(&st));
                fr_st.remove(var);
                fr_st
            }
            Consumer::MuTildeDyn(var, st) => {
                let mut fr_st: HashSet<Variable> = FreeV::free_vars(Rc::as_ref(&st));
                fr_st.remove(var);
                fr_st
            }
            Consumer::Case(pts) => FreeV::free_vars(pts),
            Consumer::Destructor(_, pargs, cargs) => {
                let free_args: HashSet<Variable> = FreeV::free_vars(pargs);
                let free_coargs: HashSet<Variable> = FreeV::free_vars(cargs);
                free_args.union(&free_coargs).cloned().collect()
            }
        }
    }

    fn free_covars(self: &Consumer) -> HashSet<Covariable> {
        match self {
            Consumer::Covar(covar) => HashSet::from([covar.clone()]),
            Consumer::MuTilde(_, st) => FreeV::free_covars(Rc::as_ref(&st)),
            Consumer::MuTildeDyn(_, st) => FreeV::free_covars(Rc::as_ref(&st)),
            Consumer::Case(pts) => FreeV::free_covars(pts),
            Consumer::Destructor(_, pargs, cargs) => {
                let free_args: HashSet<Covariable> = FreeV::free_covars(pargs);
                let free_coargs: HashSet<Covariable> = FreeV::free_covars(cargs);
                free_args.union(&free_coargs).cloned().collect()
            }
        }
    }
}

impl FreeV for Statement {
    fn free_vars(self: &Statement) -> HashSet<Variable> {
        match self {
            Statement::Cut(p, c) => {
                let free_p: HashSet<Variable> = FreeV::free_vars(Rc::as_ref(&p));
                let free_c: HashSet<Variable> = FreeV::free_vars(Rc::as_ref(&c));
                free_p.union(&free_c).cloned().collect()
            }
            Statement::Op(p1, _, p2, c) => {
                let free_p1: HashSet<Variable> = FreeV::free_vars(Rc::as_ref(&p1));
                let free_p2: HashSet<Variable> = FreeV::free_vars(Rc::as_ref(&p2));
                let free_c: HashSet<Variable> = FreeV::free_vars(Rc::as_ref(&c));
                let free_p: HashSet<Variable> = free_p1.union(&free_p2).cloned().collect();
                free_p.union(&free_c).cloned().collect()
            }
            Statement::IfZ(p, st1, st2) => {
                let free_p: HashSet<Variable> = FreeV::free_vars(Rc::as_ref(&p));
                let free_st1: HashSet<Variable> = FreeV::free_vars(Rc::as_ref(&st1));
                let free_st2: HashSet<Variable> = FreeV::free_vars(Rc::as_ref(&st2));
                let free_st: HashSet<Variable> = free_st1.union(&free_st2).cloned().collect();
                free_st.union(&free_p).cloned().collect()
            }
            Statement::Fun(_, pargs, cargs) => {
                let free_p: HashSet<Variable> = FreeV::free_vars(pargs);
                let free_c: HashSet<Variable> = FreeV::free_vars(cargs);
                free_p.union(&free_c).cloned().collect()
            }
            Statement::Done() => HashSet::new(),
        }
    }
    fn free_covars(self: &Statement) -> HashSet<Covariable> {
        match self {
            Statement::Cut(p, c) => {
                let free_p: HashSet<Variable> = FreeV::free_covars(Rc::as_ref(&p));
                let free_c: HashSet<Variable> = FreeV::free_covars(Rc::as_ref(&c));
                free_p.union(&free_c).cloned().collect()
            }
            Statement::Op(p1, _, p2, c) => {
                let free_p1: HashSet<Variable> = FreeV::free_covars(Rc::as_ref(&p1));
                let free_p2: HashSet<Variable> = FreeV::free_covars(Rc::as_ref(&p2));
                let free_c: HashSet<Variable> = FreeV::free_covars(Rc::as_ref(&c));
                let free_p: HashSet<Variable> = free_p1.union(&free_p2).cloned().collect();
                free_p.union(&free_c).cloned().collect()
            }
            Statement::IfZ(p, st1, st2) => {
                let free_p: HashSet<Variable> = FreeV::free_covars(Rc::as_ref(&p));
                let free_st1: HashSet<Variable> = FreeV::free_covars(Rc::as_ref(&st1));
                let free_st2: HashSet<Variable> = FreeV::free_covars(Rc::as_ref(&st2));
                let free_st: HashSet<Variable> = free_st1.union(&free_st2).cloned().collect();
                free_st.union(&free_p).cloned().collect()
            }
            Statement::Fun(_, pargs, cargs) => {
                let free_p: HashSet<Variable> = FreeV::free_covars(pargs);
                let free_c: HashSet<Variable> = FreeV::free_covars(cargs);
                free_p.union(&free_c).cloned().collect()
            }
            Statement::Done() => HashSet::new(),
        }
    }
}

fn fresh_var_from<T: FreeV>(xs: &Vec<T>) -> Variable {
    let free_vars: Vec<Variable> = FreeV::free_vars(xs).into_iter().collect();
    fresh_var_n(&free_vars, 0)
}

fn fresh_var(xs: &Vec<Variable>) -> Variable {
    fresh_var_n(xs, 0)
}

fn fresh_var_n(xs: &Vec<Variable>, n: i32) -> Variable {
    let new_var: Variable = format!("x{}", n);
    if xs.contains(&new_var) {
        fresh_var_n(xs, n + 1)
    } else {
        new_var
    }
}

fn fresh_covar_from<T: FreeV>(xs: &Vec<T>) -> Covariable {
    let free_covars: Vec<Covariable> = FreeV::free_covars(xs).into_iter().collect();
    fresh_covar_n(&free_covars, 0)
}

pub fn fresh_covar(xs: &Vec<Covariable>) -> Covariable {
    fresh_covar_n(xs, 0)
}

fn fresh_covar_n(xs: &Vec<Covariable>, n: i32) -> Covariable {
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

trait Subst {
    fn subst_sim(
        &self,
        prod_subst: &Vec<(Producer, Variable)>,
        cons_subst: &Vec<(Consumer, Covariable)>,
    ) -> Rc<Self>;

    fn subst_var(&self, prod: Producer, var: Variable) -> Rc<Self> {
        self.subst_sim(&vec![(prod, var)], &vec![])
    }
    fn subst_covar(&self, cons: Consumer, covar: Covariable) -> Rc<Self> {
        self.subst_sim(&vec![], &vec![(cons, covar)])
    }
}

impl<T: Subst + Clone> Subst for Vec<T> {
    fn subst_sim(
        self: &Vec<T>,
        prod_subst: &Vec<(Producer, Variable)>,
        cons_subst: &Vec<(Consumer, Covariable)>,
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
        prod_subst: &Vec<(Producer, Variable)>,
        cons_subst: &Vec<(Consumer, Covariable)>,
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
        let mut fr_v_list: Vec<Variable> = fr_v.into_iter().collect();
        let mut var_subst: Vec<(Producer, Variable)> = vec![];

        for old_var in self.vars.iter() {
            let new_var: Variable = fresh_var(&fr_v_list);
            fr_v_list.insert(0, new_var.clone());
            new_vars.insert(0, new_var.clone());
            var_subst.insert(0, (Producer::Var(new_var), old_var.clone()))
        }

        let mut new_covars: Vec<Covariable> = vec![];
        let mut fr_cv_list: Vec<Covariable> = fr_cv.into_iter().collect();
        let mut covar_subst: Vec<(Consumer, Covariable)> = vec![];

        for old_covar in self.covars.iter() {
            let new_covar: Covariable = fresh_covar(&fr_cv_list);
            fr_cv_list.insert(0, new_covar.clone());
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
        prod_subst: &Vec<(Producer, Variable)>,
        cons_subst: &Vec<(Consumer, Covariable)>,
    ) -> Rc<Producer> {
        match self {
            Producer::Var(var) => match prod_subst.iter().find(|(_, v)| v == var) {
                None => Rc::new(Producer::Var(var.clone())),
                Some((p, _)) => Rc::new(p.clone()),
            },
            Producer::Lit(n) => Rc::new(Producer::Lit(*n)),
            Producer::Mu(covar, st) => {
                let mut fr_cv: HashSet<Covariable> = FreeV::free_vars(Rc::as_ref(&st));
                for (cons, cv) in cons_subst.iter() {
                    fr_cv.insert(cv.clone());
                    fr_cv.extend(FreeV::free_covars(cons));
                }
                for (prod, _) in prod_subst.iter() {
                    fr_cv.extend(FreeV::free_covars(prod));
                }
                let cv_list: Vec<Covariable> = fr_cv.iter().cloned().collect();
                let new_covar: Covariable = fresh_covar(&cv_list);
                let new_st: Rc<Statement> =
                    Subst::subst_covar(st, Consumer::Covar(new_covar.clone()), covar.clone());
                let new_mu: Producer = Producer::Mu(
                    new_covar,
                    Subst::subst_sim(Rc::as_ref(&new_st), prod_subst, cons_subst),
                );
                Rc::new(new_mu)
            }
            Producer::MuDyn(covar, st) => Subst::subst_sim(
                &Producer::Mu(covar.clone(), st.clone()),
                prod_subst,
                cons_subst,
            ),
            Producer::Constructor(ctor, pargs, cargs) => {
                let pargs_subst: Vec<Rc<Producer>> = pargs
                    .iter()
                    .map(|p| Subst::subst_sim(Rc::as_ref(&p), prod_subst, cons_subst))
                    .collect();
                let cargs_subst: Vec<Rc<Consumer>> = cargs
                    .iter()
                    .map(|c| Subst::subst_sim(Rc::as_ref(&c), prod_subst, cons_subst))
                    .collect();
                let new_ctor: Producer =
                    Producer::Constructor(ctor.clone(), pargs_subst, cargs_subst);
                Rc::new(new_ctor)
            }
            Producer::Cocase(pts) => {
                let pts_subst: Rc<Vec<Pattern<Dtor>>> =
                    Subst::subst_sim(pts, prod_subst, cons_subst);
                Rc::new(Producer::Cocase(Rc::unwrap_or_clone(pts_subst)))
            }
        }
    }
}

impl Subst for Consumer {
    fn subst_sim(
        self: &Consumer,
        prod_subst: &Vec<(Producer, Variable)>,
        cons_subst: &Vec<(Consumer, Covariable)>,
    ) -> Rc<Consumer> {
        match self {
            Consumer::Covar(covar) => match cons_subst.iter().find(|(_, cv)| cv == covar) {
                None => Rc::new(Consumer::Covar(covar.clone())),
                Some((cons, _)) => Rc::new(cons.clone()),
            },
            Consumer::MuTilde(var, st) => {
                let mut fr_v: HashSet<Variable> = FreeV::free_vars(Rc::as_ref(&st));
                for (prod, var) in prod_subst.iter() {
                    fr_v.extend(FreeV::free_vars(prod));
                    fr_v.insert(var.clone());
                }
                for (cons, _) in cons_subst.iter() {
                    fr_v.extend(FreeV::free_vars(cons));
                }
                let fr_v_list: Vec<Variable> = fr_v.into_iter().collect();
                let new_var: Variable = fresh_var(&fr_v_list);
                let new_st: Rc<Statement> =
                    Subst::subst_var(st, Producer::Var(new_var.clone()), var.clone());
                let new_mu: Consumer = Consumer::MuTilde(
                    new_var,
                    Subst::subst_sim(Rc::as_ref(&new_st), prod_subst, cons_subst),
                );
                Rc::new(new_mu)
            }
            Consumer::MuTildeDyn(var, st) => Subst::subst_sim(
                &Consumer::MuTilde(var.clone(), st.clone()),
                prod_subst,
                cons_subst,
            ),
            Consumer::Case(pts) => {
                let pts_subst: Rc<Vec<Pattern<Ctor>>> =
                    Subst::subst_sim(pts, prod_subst, cons_subst);
                Rc::new(Consumer::Case(Rc::unwrap_or_clone(pts_subst)))
            }
            Consumer::Destructor(dtor, pargs, cargs) => {
                let pargs_subst: Vec<Rc<Producer>> = pargs
                    .iter()
                    .map(|p| Subst::subst_sim(Rc::as_ref(&p), prod_subst, cons_subst))
                    .collect();
                let cargs_subst: Vec<Rc<Consumer>> = cargs
                    .iter()
                    .map(|c| Subst::subst_sim(Rc::as_ref(&c), prod_subst, cons_subst))
                    .collect();
                Rc::new(Consumer::Destructor(dtor.clone(), pargs_subst, cargs_subst))
            }
        }
    }
}
impl Subst for Statement {
    fn subst_sim(
        self: &Statement,
        prod_subst: &Vec<(Producer, Variable)>,
        cons_subst: &Vec<(Consumer, Covariable)>,
    ) -> Rc<Statement> {
        match self {
            Statement::Cut(p, c) => {
                let p_subst: Rc<Producer> = Subst::subst_sim(p, prod_subst, cons_subst);
                let c_subst: Rc<Consumer> = Subst::subst_sim(c, prod_subst, cons_subst);
                Rc::new(Statement::Cut(p_subst, c_subst))
            }
            Statement::Op(p1, op, p2, c) => {
                let p1_subst: Rc<Producer> = Subst::subst_sim(p1, prod_subst, cons_subst);
                let p2_subst: Rc<Producer> = Subst::subst_sim(p2, prod_subst, cons_subst);
                let c_subst: Rc<Consumer> = Subst::subst_sim(c, prod_subst, cons_subst);
                Rc::new(Statement::Op(p1_subst, op.clone(), p2_subst, c_subst))
            }
            Statement::IfZ(p, st1, st2) => {
                let p_subst: Rc<Producer> = Subst::subst_sim(p, prod_subst, cons_subst);
                let st1_subst: Rc<Statement> = Subst::subst_sim(st1, prod_subst, cons_subst);
                let st2_subst: Rc<Statement> = Subst::subst_sim(st2, prod_subst, cons_subst);
                Rc::new(Statement::IfZ(p_subst, st1_subst, st2_subst))
            }
            Statement::Fun(nm, pargs, cargs) => {
                let pargs_subst: Vec<Rc<Producer>> = pargs
                    .iter()
                    .map(|p| Subst::subst_sim(Rc::as_ref(&p), prod_subst, cons_subst))
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
