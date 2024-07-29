use crate::syntax::{Clause, Consumer, Def, Producer, Prog, Statement};
use crate::traits::free_vars::{fresh_covar, fresh_var, FreeV};
use std::collections::HashSet;
use std::rc::Rc;

use super::syntax::{Cocase, Constructor, Ctor, Cut, Destructor, Fun, IfZ, Mu, MuTilde, Op, Var};

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

impl<T> Focus for Clause<T> {
    type Target = Clause<T>;
    fn focus(self) -> Clause<T> {
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
        Cocase {
            cocases: self.cocases.iter().cloned().map(Focus::focus).collect(),
        }
    }
}

impl Focus for Constructor {
    type Target = Producer;

    fn focus(self) -> Self::Target {
        let Constructor {
            id,
            producers,
            consumers,
        } = self;
        match producers.iter().find(|p| !p.is_value()) {
            None => Constructor {
                id,
                producers: producers.iter().cloned().map(|p| p.focus()).collect(),
                consumers: consumers.iter().cloned().map(|c| c.focus()).collect(),
            }
            .into(),
            Some(p) => {
                let mut fr_v = producers.free_vars();
                fr_v.extend(consumers.free_vars());
                let new_v = fresh_var(&fr_v);

                let mut fr_cv = producers.free_covars();
                fr_cv.extend(consumers.free_covars());
                let new_cv = fresh_covar(&fr_cv);

                let new_args: Vec<Producer> = producers
                    .iter()
                    .map(|p2| {
                        if p == p2 {
                            crate::syntax::Variable { var: new_v.clone() }.into()
                        } else {
                            p2.clone()
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

                let new_mu: Rc<Consumer> = Rc::new(Consumer::MuTilde(MuTilde {
                    variable: new_v,
                    statement: new_cut_inner,
                }));
                let new_p: Rc<Producer> = Rc::new(p.clone().focus());
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
        }
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
            Producer::Constructor(c) => c.focus(),
        }
    }
}

impl Focus for MuTilde {
    type Target = MuTilde;
    fn focus(self) -> Self::Target {
        MuTilde {
            variable: self.variable,
            statement: self.statement.focus(),
        }
    }
}

impl Focus for Destructor {
    type Target = Consumer;

    fn focus(self) -> Self::Target {
        let Destructor {
            id,
            producers,
            consumers,
        } = self;
        match producers.iter().find(|p| !p.is_value()) {
            None => Destructor {
                id,
                producers: producers.iter().cloned().map(|p| p.focus()).collect(),
                consumers: consumers.iter().cloned().map(|c| c.focus()).collect(),
            }
            .into(),
            Some(p) => {
                let mut fr_v: HashSet<Var> = producers.free_vars();
                fr_v.extend(consumers.free_vars());
                let new_v = fresh_var(&fr_v);
                fr_v.insert(new_v.clone());
                let new_v2: Var = fresh_var(&fr_v);
                let new_pargs: Vec<Producer> = producers
                    .iter()
                    .map(|p2| {
                        if p == p2 {
                            crate::syntax::Variable { var: new_v.clone() }.into()
                        } else {
                            p2.clone()
                        }
                    })
                    .collect();
                let new_dtor: Rc<Consumer> =
                    Rc::new(Focus::focus(Consumer::Destructor(Destructor {
                        id,
                        producers: new_pargs,
                        consumers,
                    })));
                let new_cut_inner: Rc<Statement> = Rc::new(Statement::Cut(Cut {
                    producer: Rc::new(
                        crate::syntax::Variable {
                            var: new_v2.clone(),
                        }
                        .into(),
                    ),
                    consumer: new_dtor,
                }));
                let new_mu: Rc<Consumer> = Rc::new(Consumer::MuTilde(MuTilde {
                    variable: new_v,
                    statement: new_cut_inner,
                }));
                let new_cut_outer: Rc<Statement> = Rc::new(
                    Cut {
                        producer: Rc::new(p.clone().focus()),
                        consumer: new_mu,
                    }
                    .into(),
                );
                Consumer::MuTilde(MuTilde {
                    variable: new_v2,
                    statement: new_cut_outer,
                })
            }
        }
    }
}
impl Focus for Consumer {
    type Target = Consumer;
    fn focus(self) -> Consumer {
        match self {
            Consumer::Covar(cv) => Consumer::Covar(cv),
            Consumer::MuTilde(m) => m.focus().into(),
            Consumer::Case(pts) => {
                let new_pts: Vec<Clause<Ctor>> = pts.iter().cloned().map(Focus::focus).collect();
                Consumer::Case(new_pts)
            }
            Consumer::Destructor(d) => d.focus(),
        }
    }
}

impl Focus for Cut {
    type Target = Cut;

    fn focus(self) -> Self::Target {
        let Cut { producer, consumer } = self;
        let producer = producer.focus();
        let consumer = consumer.focus();
        Cut { producer, consumer }
    }
}

impl Focus for Op {
    type Target = Statement;

    fn focus(self) -> Self::Target {
        let Op {
            fst,
            op,
            snd,
            continuation,
        } = self;
        if fst.is_value() && snd.is_value() {
            Op {
                fst: fst.focus(),
                op,
                snd: snd.focus(),
                continuation: continuation.focus(),
            }
            .into()
        } else if fst.is_value() {
            let mut fr_v = fst.free_vars();
            fr_v.extend(snd.free_vars());
            fr_v.extend(continuation.free_vars());

            let new_v: Var = fresh_var(&fr_v);
            let new_op: Rc<Statement> = Rc::new(Focus::focus(Statement::Op(Op {
                fst,
                op,
                snd: Rc::new(crate::syntax::Variable { var: new_v.clone() }.into()),
                continuation,
            })));
            let new_mu: Rc<Consumer> = Rc::new(Consumer::MuTilde(MuTilde {
                variable: new_v,
                statement: new_op,
            }));
            Cut {
                producer: snd.focus(),
                consumer: new_mu,
            }
            .into()
        } else {
            let mut fr_v = fst.free_vars();
            fr_v.extend(snd.free_vars());
            fr_v.extend(continuation.free_vars());
            let new_v: Var = fresh_var(&fr_v);

            let new_op: Rc<Statement> = Rc::new(Focus::focus(Statement::Op(Op {
                fst: Rc::new(crate::syntax::Variable { var: new_v.clone() }.into()),
                op,
                snd,
                continuation,
            })));
            let new_mu: Rc<Consumer> = Rc::new(Consumer::MuTilde(MuTilde {
                variable: new_v,
                statement: new_op,
            }));
            Cut {
                producer: fst.focus(),
                consumer: new_mu,
            }
            .into()
        }
    }
}

impl Focus for IfZ {
    type Target = Statement;

    fn focus(self) -> Self::Target {
        let IfZ { ifc, thenc, elsec } = self;
        if ifc.is_value() {
            IfZ {
                ifc: ifc.focus(),
                thenc: thenc.focus(),
                elsec: elsec.focus(),
            }
            .into()
        } else {
            let mut fr_v: HashSet<Var> = ifc.free_vars();
            fr_v.extend(thenc.free_vars());
            fr_v.extend(elsec.free_vars());
            let new_v = fresh_var(&fr_v);
            let new_if = Rc::new(
                IfZ {
                    ifc: Rc::new(crate::syntax::Variable { var: new_v.clone() }.into()),
                    thenc,
                    elsec,
                }
                .into(),
            );
            let new_mu: Rc<Consumer> = Rc::new(Consumer::MuTilde(MuTilde {
                variable: new_v,
                statement: new_if,
            }));
            Cut {
                producer: ifc.focus(),
                consumer: new_mu,
            }
            .into()
        }
    }
}

impl Focus for Fun {
    type Target = Statement;

    fn focus(self) -> Self::Target {
        let Fun {
            name,
            producers,
            consumers,
        } = self;
        match producers.iter().find(|p| !p.is_value()) {
            None => Fun {
                name,
                producers: producers.iter().cloned().map(|p| p.focus()).collect(),
                consumers: consumers.iter().cloned().map(|c| c.focus()).collect(),
            }
            .into(),
            Some(p) => {
                let mut fr_v: HashSet<Var> = producers.free_vars();
                fr_v.extend(consumers.free_vars());
                let new_v: Var = fresh_var(&fr_v);
                let new_pargs: Vec<Producer> = producers
                    .iter()
                    .map(|p2| {
                        if p2 == p {
                            crate::syntax::Variable { var: new_v.clone() }.into()
                        } else {
                            p2.clone()
                        }
                    })
                    .collect();
                let new_fun: Rc<Statement> = Rc::new(
                    Fun {
                        name,
                        producers: new_pargs,
                        consumers,
                    }
                    .into(),
                );
                Cut {
                    producer: Rc::new(p.clone().focus()),
                    consumer: Rc::new(Consumer::MuTilde(MuTilde {
                        variable: new_v,
                        statement: new_fun,
                    })),
                }
                .into()
            }
        }
    }
}

impl Focus for Statement {
    type Target = Statement;
    fn focus(self) -> Statement {
        match self {
            Statement::Cut(c) => c.focus().into(),
            Statement::Op(o) => o.focus(),
            Statement::IfZ(i) => i.focus(),
            Statement::Fun(f) => f.focus(),
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
