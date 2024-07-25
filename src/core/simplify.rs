use crate::core::substitution::Subst;
use crate::core::syntax::{Consumer, Def, Pattern, Producer, Prog, Statement};
use crate::fun::syntax::{Ctor, Dtor};
use std::rc::Rc;

pub trait Simplify {
    fn simplify(self) -> Self;
}

impl<T: Clone> Simplify for Prog<T> {
    fn simplify(self) -> Prog<T> {
        Prog {
            prog_defs: self
                .prog_defs
                .iter()
                .cloned()
                .map(Simplify::simplify)
                .collect(),
        }
    }
}

impl<T> Simplify for Def<T> {
    fn simplify(self) -> Def<T> {
        Def {
            name: self.name,
            pargs: self.pargs,
            cargs: self.cargs,
            body: Simplify::simplify(self.body),
        }
    }
}

impl<T> Simplify for Pattern<T> {
    fn simplify(self) -> Pattern<T> {
        Pattern {
            xtor: self.xtor,
            vars: self.vars,
            covars: self.covars,
            rhs: Rc::new(Simplify::simplify(Rc::unwrap_or_clone(self.rhs))),
        }
    }
}

impl Simplify for Statement {
    fn simplify(self) -> Statement {
        match self {
            Statement::Cut(p, c) => match (
                Rc::unwrap_or_clone(p.clone()),
                Rc::unwrap_or_clone(c.clone()),
            ) {
                (Producer::Mu(cv, st), _) => {
                    let st_subst: Rc<Statement> =
                        Subst::subst_covar(Rc::as_ref(&st), Rc::unwrap_or_clone(c), cv);
                    Simplify::simplify(Rc::unwrap_or_clone(st_subst))
                }
                (Producer::MuDyn(cv, st1), Consumer::MuTilde(v, st2)) => {
                    let new_mu: Rc<Producer> = Rc::new(Producer::MuDyn(
                        cv,
                        Rc::new(Simplify::simplify(Rc::unwrap_or_clone(st1))),
                    ));
                    let new_tilde: Rc<Consumer> = Rc::new(Consumer::MuTilde(
                        v,
                        Rc::new(Simplify::simplify(Rc::unwrap_or_clone(st2))),
                    ));
                    Statement::Cut(new_mu, new_tilde)
                }
                (_, Consumer::MuTilde(v, st)) => {
                    let st_subst: Rc<Statement> =
                        Subst::subst_var(Rc::as_ref(&st), Rc::unwrap_or_clone(p), v);
                    Simplify::simplify(Rc::unwrap_or_clone(st_subst))
                }
                (p_inner, c_inner) => {
                    let p_simpl: Rc<Producer> = Rc::new(Simplify::simplify(p_inner));
                    let c_simpl: Rc<Consumer> = Rc::new(Simplify::simplify(c_inner));
                    Statement::Cut(p_simpl, c_simpl)
                }
            },
            Statement::Op(p1, op, p2, c) => {
                let p1_simpl: Rc<Producer> = Rc::new(Simplify::simplify(Rc::unwrap_or_clone(p1)));
                let p2_simpl: Rc<Producer> = Rc::new(Simplify::simplify(Rc::unwrap_or_clone(p2)));
                let c_simpl: Rc<Consumer> = Rc::new(Simplify::simplify(Rc::unwrap_or_clone(c)));
                Statement::Op(p1_simpl, op, p2_simpl, c_simpl)
            }
            Statement::IfZ(p, st1, st2) => {
                let p_simpl: Rc<Producer> = Rc::new(Simplify::simplify(Rc::unwrap_or_clone(p)));
                let st1_simpl: Rc<Statement> =
                    Rc::new(Simplify::simplify(Rc::unwrap_or_clone(st1)));
                let st2_simpl: Rc<Statement> =
                    Rc::new(Simplify::simplify(Rc::unwrap_or_clone(st2)));
                Statement::IfZ(p_simpl, st1_simpl, st2_simpl)
            }
            Statement::Fun(nm, args, coargs) => {
                let args_simpl: Vec<Rc<Producer>> = args
                    .iter()
                    .cloned()
                    .map(|arg| Rc::new(Simplify::simplify(Rc::unwrap_or_clone(arg))))
                    .collect();
                let coargs_simpl: Vec<Rc<Consumer>> = coargs
                    .iter()
                    .cloned()
                    .map(|arg| Rc::new(Simplify::simplify(Rc::unwrap_or_clone(arg))))
                    .collect();
                Statement::Fun(nm, args_simpl, coargs_simpl)
            }
            Statement::Done() => Statement::Done(),
        }
    }
}

impl Simplify for Producer {
    fn simplify(self) -> Producer {
        match self {
            Producer::Var(v) => Producer::Var(v),
            Producer::Lit(n) => Producer::Lit(n),
            Producer::Mu(cv, st) => {
                let st_simpl: Rc<Statement> = Rc::new(Simplify::simplify(Rc::unwrap_or_clone(st)));
                Producer::Mu(cv, st_simpl)
            }
            Producer::MuDyn(cv, st) => {
                let st_simpl: Rc<Statement> = Rc::new(Simplify::simplify(Rc::unwrap_or_clone(st)));
                Producer::MuDyn(cv, st_simpl)
            }
            Producer::Constructor(ctor, args, coargs) => {
                let args_simpl: Vec<Rc<Producer>> = args
                    .iter()
                    .cloned()
                    .map(|arg| Rc::new(Simplify::simplify(Rc::unwrap_or_clone(arg))))
                    .collect();
                let coargs_simpl: Vec<Rc<Consumer>> = coargs
                    .iter()
                    .cloned()
                    .map(|arg| Rc::new(Simplify::simplify(Rc::unwrap_or_clone(arg))))
                    .collect();

                Producer::Constructor(ctor, args_simpl, coargs_simpl)
            }
            Producer::Cocase(pts) => {
                let pts_simpl: Vec<Pattern<Dtor>> =
                    pts.iter().cloned().map(Simplify::simplify).collect();
                Producer::Cocase(pts_simpl)
            }
        }
    }
}

impl Simplify for Consumer {
    fn simplify(self) -> Consumer {
        match self {
            Consumer::Covar(cv) => Consumer::Covar(cv),
            Consumer::MuTilde(v, st) => {
                let st_simpl: Rc<Statement> = Rc::new(Simplify::simplify(Rc::unwrap_or_clone(st)));
                Consumer::MuTilde(v, st_simpl)
            }
            Consumer::MuTildeDyn(v, st) => {
                let st_simpl: Rc<Statement> = Rc::new(Simplify::simplify(Rc::unwrap_or_clone(st)));
                Consumer::MuTildeDyn(v, st_simpl)
            }
            Consumer::Case(pts) => {
                let pts_simpl: Vec<Pattern<Ctor>> =
                    pts.iter().cloned().map(Simplify::simplify).collect();
                Consumer::Case(pts_simpl)
            }
            Consumer::Destructor(dtor, pargs, cargs) => {
                let pargs_simpl: Vec<Rc<Producer>> = pargs
                    .iter()
                    .cloned()
                    .map(|p| Rc::new(Simplify::simplify(Rc::unwrap_or_clone(p))))
                    .collect();
                let cargs_simpl: Vec<Rc<Consumer>> = cargs
                    .iter()
                    .cloned()
                    .map(|c| Rc::new(Simplify::simplify(Rc::unwrap_or_clone(c))))
                    .collect();
                Consumer::Destructor(dtor, pargs_simpl, cargs_simpl)
            }
        }
    }
}
