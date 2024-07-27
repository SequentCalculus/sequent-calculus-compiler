use crate::core::syntax::{Clause, Consumer, Def, Producer, Prog, Statement};
use crate::fun::syntax::Ctor;
use std::rc::Rc;

use super::syntax::{Cocase, Constructor, Cut, Fun, IfZ, Mu, MuTilde, Op};
use super::traits::substitution::Subst;

pub trait Simplify {
    type Target;
    fn simplify(self) -> Self::Target;
}

impl<T: Clone> Simplify for Prog<T> {
    type Target = Prog<T>;
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
    type Target = Def<T>;
    fn simplify(self) -> Def<T> {
        Def {
            name: self.name,
            pargs: self.pargs,
            cargs: self.cargs,
            body: Simplify::simplify(self.body),
        }
    }
}

impl<T> Simplify for Clause<T> {
    type Target = Clause<T>;
    fn simplify(self) -> Clause<T> {
        Clause {
            xtor: self.xtor,
            vars: self.vars,
            covars: self.covars,
            rhs: Rc::new(Simplify::simplify(Rc::unwrap_or_clone(self.rhs))),
        }
    }
}

impl Simplify for Cut {
    type Target = Statement;
    fn simplify(self) -> Self::Target {
        let Cut { producer, consumer } = self;
        match (
            Rc::unwrap_or_clone(producer.clone()),
            Rc::unwrap_or_clone(consumer.clone()),
        ) {
            (
                Producer::Mu(Mu {
                    covariable,
                    statement,
                }),
                _,
            ) => {
                let st_subst: Rc<Statement> =
                    statement.subst_covar(Rc::unwrap_or_clone(consumer), covariable);
                Simplify::simplify(Rc::unwrap_or_clone(st_subst))
            }
            (
                _,
                Consumer::MuTilde(MuTilde {
                    variable: v,
                    statement: st,
                }),
            ) => {
                let st_subst: Rc<Statement> = st.subst_var(Rc::unwrap_or_clone(producer), v);
                Simplify::simplify(Rc::unwrap_or_clone(st_subst))
            }
            (p_inner, c_inner) => {
                let p_simpl: Rc<Producer> = Rc::new(Simplify::simplify(p_inner));
                let c_simpl: Rc<Consumer> = Rc::new(Simplify::simplify(c_inner));
                Cut {
                    producer: p_simpl,
                    consumer: c_simpl,
                }
                .into()
            }
        }
    }
}

impl Simplify for Op {
    type Target = Op;

    fn simplify(self) -> Self::Target {
        Op {
            fst: Rc::new(Simplify::simplify(Rc::unwrap_or_clone(self.fst))),
            op: self.op,
            snd: Rc::new(Simplify::simplify(Rc::unwrap_or_clone(self.snd))),
            continuation: Rc::new(Simplify::simplify(Rc::unwrap_or_clone(self.continuation))),
        }
    }
}

impl Simplify for IfZ {
    type Target = IfZ;

    fn simplify(self) -> Self::Target {
        IfZ {
            ifc: Rc::new(Simplify::simplify(Rc::unwrap_or_clone(self.ifc))),
            thenc: Rc::new(Simplify::simplify(Rc::unwrap_or_clone(self.thenc))),
            elsec: Rc::new(Simplify::simplify(Rc::unwrap_or_clone(self.elsec))),
        }
    }
}

impl Simplify for Fun {
    type Target = Fun;

    fn simplify(self) -> Self::Target {
        Fun {
            name: self.name,
            producers: self
                .producers
                .iter()
                .cloned()
                .map(|arg| Rc::new(Simplify::simplify(Rc::unwrap_or_clone(arg))))
                .collect(),
            consumers: self
                .consumers
                .iter()
                .cloned()
                .map(|arg| Rc::new(Simplify::simplify(Rc::unwrap_or_clone(arg))))
                .collect(),
        }
    }
}

impl Simplify for Statement {
    type Target = Statement;
    fn simplify(self) -> Statement {
        match self {
            Statement::Cut(c) => c.simplify(),
            Statement::Op(o) => o.simplify().into(),
            Statement::IfZ(i) => i.simplify().into(),
            Statement::Fun(f) => f.simplify().into(),
            Statement::Done() => Statement::Done(),
        }
    }
}

impl Simplify for Mu {
    type Target = Mu;
    fn simplify(self) -> Self::Target {
        Mu {
            covariable: self.covariable,
            statement: Rc::new(Simplify::simplify(Rc::unwrap_or_clone(self.statement))),
        }
    }
}

impl Simplify for Constructor {
    type Target = Constructor;
    fn simplify(self) -> Self::Target {
        Constructor {
            id: self.id,
            producers: self
                .producers
                .iter()
                .cloned()
                .map(|arg| Rc::new(Simplify::simplify(Rc::unwrap_or_clone(arg))))
                .collect(),
            consumers: self
                .consumers
                .iter()
                .cloned()
                .map(|arg| Rc::new(Simplify::simplify(Rc::unwrap_or_clone(arg))))
                .collect(),
        }
    }
}

impl Simplify for Cocase {
    type Target = Cocase;
    fn simplify(self) -> Self::Target {
        Cocase {
            cocases: self
                .cocases
                .iter()
                .cloned()
                .map(Simplify::simplify)
                .collect(),
        }
    }
}

impl Simplify for Producer {
    type Target = Producer;
    fn simplify(self) -> Producer {
        match self {
            Producer::Variable(v) => Producer::Variable(v),
            Producer::Literal(n) => Producer::Literal(n),
            Producer::Mu(m) => m.simplify().into(),
            Producer::Constructor(c) => c.simplify().into(),
            Producer::Cocase(c) => c.simplify().into(),
        }
    }
}

impl Simplify for MuTilde {
    type Target = MuTilde;
    fn simplify(self) -> Self::Target {
        MuTilde {
            variable: self.variable,
            statement: Rc::new(Simplify::simplify(Rc::unwrap_or_clone(self.statement))),
        }
    }
}
impl Simplify for Consumer {
    type Target = Consumer;
    fn simplify(self) -> Consumer {
        match self {
            Consumer::Covar(cv) => Consumer::Covar(cv),
            Consumer::MuTilde(m) => m.simplify().into(),
            Consumer::Case(pts) => {
                let pts_simpl: Vec<Clause<Ctor>> =
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
