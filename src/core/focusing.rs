use crate::core::substitution::{fresh_covar, fresh_var, FreeV};
use crate::core::syntax::{Consumer, Def, Pattern, Producer, Prog, Statement};
use crate::fun::syntax::{Covariable, Ctor, Dtor, Variable};
use std::collections::HashSet;
use std::rc::Rc;

use super::syntax::{Cocase, Constructor, Cut, IfZ, Mu, Op};

pub trait Focus {
    type Target;
    fn focus(self) -> Self::Target;
}

impl<T: Focus + Clone> Focus for Rc<T> {
    type Target = Rc<T::Target>;
    fn focus(self) -> Self::Target {
        Rc::new(Rc::unwrap_or_clone(self).focus())
    }
}

impl<T> Focus for Pattern<T> {
    type Target = Pattern<T>;
    fn focus(self) -> Pattern<T> {
        panic!("")
    }
}

impl Focus for Mu {
    type Target = Mu;
    fn focus(self) -> Self {
        Mu {
            covariable: self.covariable,
            statement: self.statement.focus(),
        }
    }
}

impl Focus for Cocase {
    type Target = Cocase;
    fn focus(self) -> Self {
        let Cocase { cocases } = self;
        let new_pts: Vec<Pattern<Dtor>> = cocases.iter().cloned().map(Focus::focus).collect();
        Cocase { cocases: new_pts }
    }
}

impl Focus for Producer {
    type Target = Producer;
    fn focus(self) -> Producer {
        match self {
            Producer::Literal(n) => Producer::Literal(n),
            Producer::Variable(v) => Producer::Variable(v),
            Producer::Mu(m) => m.focus().into(),
            Producer::Cocase(c) => c.focus().into(),
            Producer::Constructor(Constructor {
                id,
                producers,
                consumers,
            }) => match producers.iter().find(|p| !p.is_value()) {
                None => {
                    let new_pargs: Vec<Rc<Producer>> = producers
                        .iter()
                        .cloned()
                        .map(|p| Rc::new(Focus::focus(Rc::unwrap_or_clone(p))))
                        .collect();
                    let new_cargs: Vec<Rc<Consumer>> = consumers
                        .iter()
                        .cloned()
                        .map(|c| Rc::new(Focus::focus(Rc::unwrap_or_clone(c))))
                        .collect();
                    Constructor {
                        id,
                        producers: new_pargs,
                        consumers: new_cargs,
                    }
                    .into()
                }
                Some(p) => {
                    let mut fr_v: HashSet<Variable> = FreeV::free_vars(&producers);
                    fr_v.extend(FreeV::free_vars(&consumers));
                    let new_v: Variable = fresh_var(&fr_v);

                    let mut fr_cv: HashSet<Covariable> = FreeV::free_covars(&producers);
                    fr_cv.extend(FreeV::free_covars(&consumers));
                    let new_cv: Covariable = fresh_covar(&fr_cv);

                    let new_args: Vec<Rc<Producer>> = producers
                        .iter()
                        .map(|p2| {
                            if p == p2 {
                                Rc::new(crate::core::syntax::Variable { var: new_v.clone() }.into())
                            } else {
                                Rc::clone(p2)
                            }
                        })
                        .collect();

                    let new_ctor: Rc<Producer> =
                        Rc::new(Focus::focus(Producer::Constructor(Constructor {
                            id,
                            producers: new_args,
                            consumers,
                        })));
                    let new_cut_inner = Rc::new(
                        Cut {
                            producer: new_ctor,
                            consumer: Rc::new(Consumer::Covar(new_cv.clone())),
                        }
                        .into(),
                    );

                    let new_mu: Rc<Consumer> = Rc::new(Consumer::MuTilde(new_v, new_cut_inner));
                    let new_p: Rc<Producer> = Rc::new(Focus::focus(Rc::unwrap_or_clone(p.clone())));
                    let new_cut_outer: Rc<Statement> = Rc::new(
                        Cut {
                            producer: new_p,
                            consumer: new_mu,
                        }
                        .into(),
                    );
                    Mu {
                        covariable: new_cv,
                        statement: new_cut_outer,
                    }
                    .into()
                }
            },
        }
    }
}

impl Focus for Consumer {
    type Target = Consumer;
    fn focus(self) -> Consumer {
        match self {
            Consumer::Covar(cv) => Consumer::Covar(cv),
            Consumer::MuTilde(v, st) => {
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
                                    Rc::new(
                                        crate::core::syntax::Variable { var: new_v.clone() }.into(),
                                    )
                                } else {
                                    Rc::clone(p2)
                                }
                            })
                            .collect();
                        let new_dtor: Rc<Consumer> =
                            Rc::new(Focus::focus(Consumer::Destructor(dtor, new_pargs, cargs)));
                        let new_cut_inner: Rc<Statement> = Rc::new(Statement::Cut(Cut {
                            producer: Rc::new(
                                crate::core::syntax::Variable {
                                    var: new_v2.clone(),
                                }
                                .into(),
                            ),
                            consumer: new_dtor,
                        }));
                        let new_mu: Rc<Consumer> = Rc::new(Consumer::MuTilde(new_v, new_cut_inner));
                        let new_p: Rc<Producer> =
                            Rc::new(Focus::focus(Rc::unwrap_or_clone(p.clone())));
                        let new_cut_outer: Rc<Statement> = Rc::new(
                            Cut {
                                producer: new_p,
                                consumer: new_mu,
                            }
                            .into(),
                        );
                        Consumer::MuTilde(new_v2, new_cut_outer)
                    }
                }
            }
        }
    }
}

impl Focus for Statement {
    type Target = Statement;
    fn focus(self) -> Statement {
        match self {
            Statement::Cut(Cut { producer, consumer }) => {
                let new_p: Rc<Producer> = Rc::new(Focus::focus(Rc::unwrap_or_clone(producer)));
                let new_c: Rc<Consumer> = Rc::new(Focus::focus(Rc::unwrap_or_clone(consumer)));
                Cut {
                    producer: new_p,
                    consumer: new_c,
                }
                .into()
            }
            Statement::Op(Op {
                fst: p1,
                op,
                snd: p2,
                continuation: c,
            }) if p1.is_value() && p2.is_value() => {
                let new_p1: Rc<Producer> = Rc::new(Focus::focus(Rc::unwrap_or_clone(p1)));
                let new_p2: Rc<Producer> = Rc::new(Focus::focus(Rc::unwrap_or_clone(p2)));
                let new_c: Rc<Consumer> = Rc::new(Focus::focus(Rc::unwrap_or_clone(c)));
                Op {
                    fst: new_p1,
                    op,
                    snd: new_p2,
                    continuation: new_c,
                }
                .into()
            }
            Statement::Op(Op {
                fst: p1,
                op,
                snd: p2,
                continuation: c,
            }) if p1.is_value() => {
                let mut fr_v: HashSet<Variable> = FreeV::free_vars(Rc::as_ref(&p1));
                fr_v.extend(FreeV::free_vars(Rc::as_ref(&p2)));
                fr_v.extend(FreeV::free_vars(Rc::as_ref(&c)));
                let new_v: Variable = fresh_var(&fr_v);
                let new_op: Rc<Statement> = Rc::new(Focus::focus(Statement::Op(Op {
                    fst: p1,
                    op,
                    snd: Rc::new(crate::core::syntax::Variable { var: new_v.clone() }.into()),
                    continuation: c,
                })));
                let new_mu: Rc<Consumer> = Rc::new(Consumer::MuTilde(new_v, new_op));
                Cut {
                    producer: Rc::new(Focus::focus(Rc::unwrap_or_clone(p2))),
                    consumer: new_mu,
                }
                .into()
            }
            Statement::Op(Op {
                fst: p1,
                op,
                snd: p2,
                continuation: c,
            }) => {
                let mut fr_v: HashSet<Variable> = FreeV::free_vars(Rc::as_ref(&p1));
                fr_v.extend(FreeV::free_vars(Rc::as_ref(&p2)));
                fr_v.extend(FreeV::free_vars(Rc::as_ref(&c)));
                let new_v: Variable = fresh_var(&fr_v);

                let new_op: Rc<Statement> = Rc::new(Focus::focus(Statement::Op(Op {
                    fst: Rc::new(crate::core::syntax::Variable { var: new_v.clone() }.into()),
                    op,
                    snd: p2,
                    continuation: c,
                })));
                let new_mu: Rc<Consumer> = Rc::new(Consumer::MuTilde(new_v, new_op));
                Cut {
                    producer: Rc::new(Focus::focus(Rc::unwrap_or_clone(p1))),
                    consumer: new_mu,
                }
                .into()
            }

            Statement::IfZ(IfZ {
                ifc: p,
                thenc: st1,
                elsec: st2,
            }) if p.is_value() => {
                let new_p: Rc<Producer> = Rc::new(Focus::focus(Rc::unwrap_or_clone(p)));
                let new_st1: Rc<Statement> = Rc::new(Focus::focus(Rc::unwrap_or_clone(st1)));
                let new_st2: Rc<Statement> = Rc::new(Focus::focus(Rc::unwrap_or_clone(st2)));
                IfZ {
                    ifc: new_p,
                    thenc: new_st1,
                    elsec: new_st2,
                }
                .into()
            }
            Statement::IfZ(IfZ {
                ifc: p,
                thenc: st1,
                elsec: st2,
            }) => {
                let mut fr_v: HashSet<Variable> = FreeV::free_vars(Rc::as_ref(&p));
                fr_v.extend(FreeV::free_vars(Rc::as_ref(&st1)));
                fr_v.extend(FreeV::free_vars(Rc::as_ref(&st2)));
                let new_v: Variable = fresh_var(&fr_v);
                let new_if: Rc<Statement> = Rc::new(
                    IfZ {
                        ifc: Rc::new(crate::core::syntax::Variable { var: new_v.clone() }.into()),
                        thenc: st1,
                        elsec: st2,
                    }
                    .into(),
                );
                let new_mu: Rc<Consumer> = Rc::new(Consumer::MuTilde(new_v, new_if));
                let new_p: Rc<Producer> = Rc::new(Focus::focus(Rc::unwrap_or_clone(p)));
                Cut {
                    producer: new_p,
                    consumer: new_mu,
                }
                .into()
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
                                Rc::new(crate::core::syntax::Variable { var: new_v.clone() }.into())
                            } else {
                                Rc::clone(p2)
                            }
                        })
                        .collect();
                    let new_fun: Rc<Statement> = Rc::new(Statement::Fun(nm, new_pargs, cargs));
                    let new_mu: Rc<Consumer> = Rc::new(Consumer::MuTilde(new_v, new_fun));
                    let new_p: Rc<Producer> = Rc::new(Focus::focus(Rc::unwrap_or_clone(p.clone())));
                    Cut {
                        producer: new_p,
                        consumer: new_mu,
                    }
                    .into()
                }
            },
            Statement::Done() => Statement::Done(),
        }
    }
}

impl<T> Focus for Def<T> {
    type Target = Def<T>;
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
    type Target = Prog<T>;
    fn focus(self) -> Prog<T> {
        Prog {
            prog_defs: self.prog_defs.iter().cloned().map(Focus::focus).collect(),
        }
    }
}
