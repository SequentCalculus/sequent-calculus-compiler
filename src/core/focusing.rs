use crate::core::substitution::{fresh_covar, fresh_var, FreeV};
use crate::core::syntax::{Consumer, Def, Pattern, Producer, Prog, Statement};
use crate::fun::syntax::{Covariable, Ctor, Dtor, Variable};
use std::collections::HashSet;
use std::rc::Rc;

pub trait Focus {
    fn focus(self) -> Self;
}

impl<T> Focus for Pattern<T> {
    fn focus(self) -> Pattern<T> {
        panic!("")
    }
}

impl Focus for Producer {
    fn focus(self) -> Producer {
        match self {
            Producer::Lit(n) => Producer::Lit(n),
            Producer::Var(v) => Producer::Var(v),
            Producer::Mu(cv, st) => {
                let new_st: Rc<Statement> = Rc::new(Focus::focus(Rc::unwrap_or_clone(st)));
                Producer::Mu(cv, new_st)
            }
            Producer::MuDyn(cv, st) => {
                let new_st: Rc<Statement> = Rc::new(Focus::focus(Rc::unwrap_or_clone(st)));
                Producer::MuDyn(cv, new_st)
            }
            Producer::Cocase(pts) => {
                let new_pts: Vec<Pattern<Dtor>> = pts.iter().cloned().map(Focus::focus).collect();
                Producer::Cocase(new_pts)
            }
            Producer::Constructor(ctor, pargs, cargs) => {
                match pargs.iter().find(|p| !p.is_value()) {
                    None => {
                        let new_pargs: Vec<Rc<Producer>> = pargs
                            .iter()
                            .cloned()
                            .map(|p| Rc::new(Focus::focus(Rc::unwrap_or_clone(p))))
                            .collect();
                        let new_cargs: Vec<Rc<Consumer>> = cargs
                            .iter()
                            .cloned()
                            .map(|c| Rc::new(Focus::focus(Rc::unwrap_or_clone(c))))
                            .collect();
                        Producer::Constructor(ctor, new_pargs, new_cargs)
                    }
                    Some(p) => {
                        let mut fr_v: HashSet<Variable> = FreeV::free_vars(&pargs);
                        fr_v.extend(FreeV::free_vars(&cargs));
                        let new_v: Variable = fresh_var(&fr_v);

                        let mut fr_cv: HashSet<Covariable> = FreeV::free_covars(&pargs);
                        fr_cv.extend(FreeV::free_covars(&cargs));
                        let new_cv: Covariable = fresh_covar(&fr_cv);

                        let new_args: Vec<Rc<Producer>> = pargs
                            .iter()
                            .map(|p2| {
                                if p == p2 {
                                    Rc::new(Producer::Var(new_v.clone()))
                                } else {
                                    Rc::clone(p2)
                                }
                            })
                            .collect();

                        let new_ctor: Rc<Producer> =
                            Rc::new(Focus::focus(Producer::Constructor(ctor, new_args, cargs)));
                        let new_cut_inner = Rc::new(Statement::Cut(
                            new_ctor,
                            Rc::new(Consumer::Covar(new_cv.clone())),
                        ));
                        let new_mu: Rc<Consumer> = Rc::new(Consumer::MuTilde(new_v, new_cut_inner));
                        let new_p: Rc<Producer> =
                            Rc::new(Focus::focus(Rc::unwrap_or_clone(p.clone())));
                        let new_cut_outer: Rc<Statement> = Rc::new(Statement::Cut(new_p, new_mu));
                        Producer::Mu(new_cv, new_cut_outer)
                    }
                }
            }
        }
    }
}

impl Focus for Consumer {
    fn focus(self) -> Consumer {
        match self {
            Consumer::Covar(cv) => Consumer::Covar(cv),
            Consumer::MuTilde(v, st) => {
                let new_st: Rc<Statement> = Rc::new(Focus::focus(Rc::unwrap_or_clone(st)));
                Consumer::MuTilde(v, new_st)
            }
            Consumer::MuTildeDyn(v, st) => {
                let new_st: Rc<Statement> = Rc::new(Focus::focus(Rc::unwrap_or_clone(st)));
                Consumer::MuTilde(v, new_st)
            }
            Consumer::Case(pts) => {
                let new_pts: Vec<Pattern<Ctor>> = pts.iter().cloned().map(Focus::focus).collect();
                Consumer::Case(new_pts)
            }
            Consumer::Destructor(dtor, pargs, cargs) => {
                match pargs.iter().find(|p| !p.is_value()) {
                    None => {
                        let new_pargs: Vec<Rc<Producer>> = pargs
                            .iter()
                            .cloned()
                            .map(|p| Rc::new(Focus::focus(Rc::unwrap_or_clone(p))))
                            .collect();
                        let new_cargs: Vec<Rc<Consumer>> = cargs
                            .iter()
                            .cloned()
                            .map(|c| Rc::new(Focus::focus(Rc::unwrap_or_clone(c))))
                            .collect();
                        Consumer::Destructor(dtor, new_pargs, new_cargs)
                    }
                    Some(p) => {
                        let mut fr_v: HashSet<Variable> = FreeV::free_vars(&pargs);
                        fr_v.extend(FreeV::free_vars(&cargs));
                        let new_v = fresh_var(&fr_v);
                        fr_v.insert(new_v.clone());
                        let new_v2: Variable = fresh_var(&fr_v);
                        let new_pargs: Vec<Rc<Producer>> = pargs
                            .iter()
                            .map(|p2| {
                                if p == p2 {
                                    Rc::new(Producer::Var(new_v.clone()))
                                } else {
                                    Rc::clone(p2)
                                }
                            })
                            .collect();
                        let new_dtor: Rc<Consumer> =
                            Rc::new(Focus::focus(Consumer::Destructor(dtor, new_pargs, cargs)));
                        let new_cut_inner: Rc<Statement> = Rc::new(Statement::Cut(
                            Rc::new(Producer::Var(new_v2.clone())),
                            new_dtor,
                        ));
                        let new_mu: Rc<Consumer> = Rc::new(Consumer::MuTilde(new_v, new_cut_inner));
                        let new_p: Rc<Producer> =
                            Rc::new(Focus::focus(Rc::unwrap_or_clone(p.clone())));
                        let new_cut_outer: Rc<Statement> = Rc::new(Statement::Cut(new_p, new_mu));
                        Consumer::MuTilde(new_v2, new_cut_outer)
                    }
                }
            }
        }
    }
}

impl Focus for Statement {
    fn focus(self) -> Statement {
        match self {
            Statement::Cut(p, c) => {
                let new_p: Rc<Producer> = Rc::new(Focus::focus(Rc::unwrap_or_clone(p)));
                let new_c: Rc<Consumer> = Rc::new(Focus::focus(Rc::unwrap_or_clone(c)));
                Statement::Cut(new_p, new_c)
            }
            Statement::Op(p1, op, p2, c) if p1.is_value() && p2.is_value() => {
                let new_p1: Rc<Producer> = Rc::new(Focus::focus(Rc::unwrap_or_clone(p1)));
                let new_p2: Rc<Producer> = Rc::new(Focus::focus(Rc::unwrap_or_clone(p2)));
                let new_c: Rc<Consumer> = Rc::new(Focus::focus(Rc::unwrap_or_clone(c)));
                Statement::Op(new_p1, op, new_p2, new_c)
            }
            Statement::Op(p1, op, p2, c) if p1.is_value() => {
                let mut fr_v: HashSet<Variable> = FreeV::free_vars(Rc::as_ref(&p1));
                fr_v.extend(FreeV::free_vars(Rc::as_ref(&p2)));
                fr_v.extend(FreeV::free_vars(Rc::as_ref(&c)));
                let new_v: Variable = fresh_var(&fr_v);
                let new_op: Rc<Statement> = Rc::new(Focus::focus(Statement::Op(
                    p1,
                    op,
                    Rc::new(Producer::Var(new_v.clone())),
                    c,
                )));
                let new_mu: Rc<Consumer> = Rc::new(Consumer::MuTilde(new_v, new_op));
                Statement::Cut(Rc::new(Focus::focus(Rc::unwrap_or_clone(p2))), new_mu)
            }
            Statement::Op(p1, op, p2, c) => {
                let mut fr_v: HashSet<Variable> = FreeV::free_vars(Rc::as_ref(&p1));
                fr_v.extend(FreeV::free_vars(Rc::as_ref(&p2)));
                fr_v.extend(FreeV::free_vars(Rc::as_ref(&c)));
                let new_v: Variable = fresh_var(&fr_v);

                let new_op: Rc<Statement> = Rc::new(Focus::focus(Statement::Op(
                    Rc::new(Producer::Var(new_v.clone())),
                    op,
                    p2,
                    c,
                )));
                let new_mu: Rc<Consumer> = Rc::new(Consumer::MuTilde(new_v, new_op));
                Statement::Cut(Rc::new(Focus::focus(Rc::unwrap_or_clone(p1))), new_mu)
            }

            Statement::IfZ(p, st1, st2) if p.is_value() => {
                let new_p: Rc<Producer> = Rc::new(Focus::focus(Rc::unwrap_or_clone(p)));
                let new_st1: Rc<Statement> = Rc::new(Focus::focus(Rc::unwrap_or_clone(st1)));
                let new_st2: Rc<Statement> = Rc::new(Focus::focus(Rc::unwrap_or_clone(st2)));
                Statement::IfZ(new_p, new_st1, new_st2)
            }
            Statement::IfZ(p, st1, st2) => {
                let mut fr_v: HashSet<Variable> = FreeV::free_vars(Rc::as_ref(&p));
                fr_v.extend(FreeV::free_vars(Rc::as_ref(&st1)));
                fr_v.extend(FreeV::free_vars(Rc::as_ref(&st2)));
                let new_v: Variable = fresh_var(&fr_v);
                let new_if: Rc<Statement> = Rc::new(Statement::IfZ(
                    Rc::new(Producer::Var(new_v.clone())),
                    st1,
                    st2,
                ));
                let new_mu: Rc<Consumer> = Rc::new(Consumer::MuTilde(new_v, new_if));
                let new_p: Rc<Producer> = Rc::new(Focus::focus(Rc::unwrap_or_clone(p)));
                Statement::Cut(new_p, new_mu)
            }
            Statement::Fun(nm, pargs, cargs) => match pargs.iter().find(|p| !p.is_value()) {
                None => {
                    let new_pargs: Vec<Rc<Producer>> = pargs
                        .iter()
                        .cloned()
                        .map(|p| Rc::new(Focus::focus(Rc::unwrap_or_clone(p))))
                        .collect();
                    let new_cargs: Vec<Rc<Consumer>> = cargs
                        .iter()
                        .cloned()
                        .map(|c| Rc::new(Focus::focus(Rc::unwrap_or_clone(c))))
                        .collect();
                    Statement::Fun(nm, new_pargs, new_cargs)
                }
                Some(p) => {
                    let mut fr_v: HashSet<Variable> = FreeV::free_vars(&pargs);
                    fr_v.extend(FreeV::free_vars(&cargs));
                    let new_v: Variable = fresh_var(&fr_v);
                    let new_pargs: Vec<Rc<Producer>> = pargs
                        .iter()
                        .map(|p2| {
                            if p2 == p {
                                Rc::new(Producer::Var(new_v.clone()))
                            } else {
                                Rc::clone(p2)
                            }
                        })
                        .collect();
                    let new_fun: Rc<Statement> = Rc::new(Statement::Fun(nm, new_pargs, cargs));
                    let new_mu: Rc<Consumer> = Rc::new(Consumer::MuTilde(new_v, new_fun));
                    let new_p: Rc<Producer> = Rc::new(Focus::focus(Rc::unwrap_or_clone(p.clone())));
                    Statement::Cut(new_p, new_mu)
                }
            },
            Statement::Done() => Statement::Done(),
        }
    }
}

impl<T> Focus for Def<T> {
    fn focus(self) -> Def<T> {
        Def {
            name: self.name,
            pargs: self.pargs,
            cargs: self.cargs,
            body: Focus::focus(self.body),
        }
    }
}

impl<T: Clone> Focus for Prog<T> {
    fn focus(self) -> Prog<T> {
        Prog {
            prog_defs: self.prog_defs.iter().cloned().map(Focus::focus).collect(),
        }
    }
}
