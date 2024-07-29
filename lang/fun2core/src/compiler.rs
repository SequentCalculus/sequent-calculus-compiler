use std::collections::HashSet;
use std::rc::Rc;

use core::traits::free_vars::{fresh_covar, FreeV};

use fun::syntax::Covariable;

struct CompileState {
    covars: HashSet<Covariable>,
}

impl CompileState {
    fn add_covars<T: FreeV>(&mut self, new_cv: &T) {
        let fr_cv = FreeV::free_covars(new_cv);
        self.covars.extend(fr_cv);
    }

    fn free_covar_from_state(&mut self) -> Covariable {
        let new_cv: Covariable = fresh_covar(&self.covars);
        self.covars.insert(new_cv.clone());
        new_cv
    }
}

trait Compile {
    type Target;

    fn compile(self, state: &mut CompileState) -> Self::Target;
}

impl<T: Compile + Clone> Compile for Rc<T> {
    type Target = Rc<T::Target>;

    fn compile(self, state: &mut CompileState) -> Self::Target {
        Rc::new(Rc::unwrap_or_clone(self).compile(state))
    }
}

impl Compile for fun::syntax::Ctor {
    type Target = core::syntax::Ctor;
    fn compile(self, _state: &mut CompileState) -> Self::Target {
        match self {
            fun::syntax::Ctor::Nil => core::syntax::Ctor::Nil,
            fun::syntax::Ctor::Cons => core::syntax::Ctor::Cons,
            fun::syntax::Ctor::Tup => core::syntax::Ctor::Tup,
        }
    }
}

impl Compile for fun::syntax::Dtor {
    type Target = core::syntax::Dtor;
    fn compile(self, _state: &mut CompileState) -> Self::Target {
        match self {
            fun::syntax::Dtor::Hd => core::syntax::Dtor::Hd,
            fun::syntax::Dtor::Tl => core::syntax::Dtor::Tl,
            fun::syntax::Dtor::Fst => core::syntax::Dtor::Fst,
            fun::syntax::Dtor::Snd => core::syntax::Dtor::Snd,
        }
    }
}

impl Compile for fun::syntax::BinOp {
    type Target = core::syntax::BinOp;
    fn compile(self, _state: &mut CompileState) -> Self::Target {
        match self {
            fun::syntax::BinOp::Prod => core::syntax::BinOp::Prod,
            fun::syntax::BinOp::Sum => core::syntax::BinOp::Sum,
            fun::syntax::BinOp::Sub => core::syntax::BinOp::Sub,
        }
    }
}

impl Compile for fun::syntax::Op {
    type Target = core::syntax::Producer;

    fn compile(self, state: &mut CompileState) -> Self::Target {
        let p1 = self.fst.compile(state);
        let p2 = self.snd.compile(state);
        state.add_covars(Rc::as_ref(&p1));
        state.add_covars(Rc::as_ref(&p2));
        let new_cv = state.free_covar_from_state();
        let new_op = core::syntax::Op {
            fst: p1,
            op: self.op.compile(state),
            snd: p2,
            continuation: Rc::new(core::syntax::Consumer::Covar(new_cv.clone())),
        }
        .into();
        core::syntax::Mu {
            covariable: new_cv,
            statement: Rc::new(new_op),
        }
        .into()
    }
}

impl Compile for fun::syntax::IfZ {
    type Target = core::syntax::Producer;

    fn compile(self, state: &mut CompileState) -> Self::Target {
        let p1 = self.ifc.compile(state);
        let p2 = self.thenc.compile(state);
        let p3 = self.elsec.compile(state);
        state.add_covars(Rc::as_ref(&p1));
        state.add_covars(Rc::as_ref(&p2));
        state.add_covars(Rc::as_ref(&p3));
        let new_cv = state.free_covar_from_state();
        let new_cons = Rc::new(core::syntax::Consumer::Covar(new_cv.clone()));
        let s1 = Rc::new(
            core::syntax::Cut {
                producer: p2,
                consumer: new_cons.clone(),
            }
            .into(),
        );
        let s2 = Rc::new(
            core::syntax::Cut {
                producer: p3,
                consumer: new_cons,
            }
            .into(),
        );
        let new_if = Rc::new(
            core::syntax::IfZ {
                ifc: p1,
                thenc: s1,
                elsec: s2,
            }
            .into(),
        );
        core::syntax::Mu {
            covariable: new_cv,
            statement: new_if,
        }
        .into()
    }
}

impl Compile for fun::syntax::Let {
    type Target = core::syntax::Producer;
    fn compile(self, state: &mut CompileState) -> Self::Target {
        let p1 = self.bound_term.compile(state);
        let p2 = self.in_term.compile(state);
        state.add_covars(Rc::as_ref(&p1));
        state.add_covars(Rc::as_ref(&p2));
        let new_cv = state.free_covar_from_state();
        let new_cons = Rc::new(core::syntax::Consumer::Covar(new_cv.clone()));
        let cut_inner = Rc::new(
            core::syntax::Cut {
                producer: p2,
                consumer: new_cons,
            }
            .into(),
        );
        let new_mutilde = Rc::new(core::syntax::Consumer::MuTilde(core::syntax::MuTilde {
            variable: self.variable.clone(),
            statement: cut_inner,
        }));
        let cut_outer = Rc::new(
            core::syntax::Cut {
                producer: p1,
                consumer: new_mutilde,
            }
            .into(),
        );
        core::syntax::Mu {
            covariable: new_cv,
            statement: cut_outer,
        }
        .into()
    }
}

impl Compile for fun::syntax::Fun {
    type Target = core::syntax::Producer;

    fn compile(self, state: &mut CompileState) -> Self::Target {
        let mut args_comp: Vec<Rc<core::syntax::Producer>> = vec![];
        for arg in self.args.iter().cloned() {
            let arg_comp: Rc<core::syntax::Producer> = arg.compile(state);
            state.add_covars(Rc::as_ref(&arg_comp));
            args_comp.insert(0, arg_comp);
        }
        for cv in self.coargs.iter() {
            state.covars.insert(cv.clone());
        }
        let new_cv: Covariable = state.free_covar_from_state();
        let new_covar: Rc<core::syntax::Consumer> =
            Rc::new(core::syntax::Consumer::Covar(new_cv.clone()));
        let mut new_cvs: Vec<Rc<core::syntax::Consumer>> = self
            .coargs
            .iter()
            .map(|cv| Rc::new(core::syntax::Consumer::Covar(cv.clone())))
            .collect();
        new_cvs.insert(new_cvs.len(), new_covar);
        let new_fun: Rc<core::syntax::Statement> = Rc::new(
            core::syntax::Fun {
                name: self.name,
                producers: args_comp,
                consumers: new_cvs,
            }
            .into(),
        );
        core::syntax::Mu {
            covariable: new_cv,
            statement: new_fun,
        }
        .into()
    }
}

impl Compile for fun::syntax::Constructor {
    type Target = core::syntax::Producer;

    fn compile(self, state: &mut CompileState) -> Self::Target {
        core::syntax::Constructor {
            id: self.id.compile(state),
            producers: self
                .args
                .iter()
                .cloned()
                .map(|arg| arg.compile(state))
                .collect(),
            consumers: vec![],
        }
        .into()
    }
}

impl Compile for fun::syntax::Term {
    type Target = core::syntax::Producer;

    fn compile(self, state: &mut CompileState) -> Self::Target {
        match self {
            fun::syntax::Term::Var(v) => core::syntax::Variable { var: v }.into(),
            fun::syntax::Term::Lit(n) => core::syntax::Literal { lit: n }.into(),
            fun::syntax::Term::Op(o) => o.compile(state),
            fun::syntax::Term::IfZ(i) => i.compile(state),
            fun::syntax::Term::Let(l) => l.compile(state),
            fun::syntax::Term::Fun(f) => f.compile(state),
            fun::syntax::Term::Constructor(c) => c.compile(state),
            fun::syntax::Term::Destructor(t, dtor, args) => {
                let p = t.compile(state);
                state.add_covars(Rc::as_ref(&p));
                let args_comp: Vec<Rc<core::syntax::Producer>> =
                    args.iter().cloned().map(|arg| arg.compile(state)).collect();
                state.add_covars(&args_comp);
                let new_cv: Covariable = state.free_covar_from_state();
                let new_dt: Rc<core::syntax::Consumer> = Rc::new(
                    core::syntax::Destructor {
                        id: dtor.compile(state),
                        producers: args_comp,
                        consumers: vec![Rc::new(core::syntax::Consumer::Covar(new_cv.clone()))],
                    }
                    .into(),
                );
                let new_cut: Rc<core::syntax::Statement> = Rc::new(
                    core::syntax::Cut {
                        producer: p,
                        consumer: new_dt,
                    }
                    .into(),
                );
                core::syntax::Mu {
                    covariable: new_cv,
                    statement: new_cut,
                }
                .into()
            }
            fun::syntax::Term::Case(t, pts) => {
                let p = t.compile(state);
                state.add_covars(Rc::as_ref(&p));
                let rhs_comp: Vec<Rc<core::syntax::Producer>> = pts
                    .iter()
                    .cloned()
                    .map(|pt| Rc::new(Rc::unwrap_or_clone(pt).rhs.compile(state)))
                    .collect();
                let _ = rhs_comp.iter().map(|p| state.add_covars(Rc::as_ref(p)));
                let new_cv: Covariable = state.free_covar_from_state();
                let new_covar: Rc<core::syntax::Consumer> =
                    Rc::new(core::syntax::Consumer::Covar(new_cv.clone()));
                let rhs_cuts: Vec<Rc<core::syntax::Statement>> = rhs_comp
                    .iter()
                    .cloned()
                    .map(|p| {
                        Rc::new(
                            core::syntax::Cut {
                                producer: p,
                                consumer: Rc::clone(&new_covar),
                            }
                            .into(),
                        )
                    })
                    .collect();

                let mut new_pts: Vec<core::syntax::Clause<core::syntax::Ctor>> = vec![];
                for i in 0..pts.len() - 1 {
                    let pt_i: &Rc<fun::syntax::Clause<fun::syntax::Ctor>> =
                        pts.get(i).expect("Invalid pattern (should never happen");
                    let rhs_i: &Rc<core::syntax::Statement> = rhs_cuts
                        .get(i)
                        .expect("Invalid pattern (should never happen");
                    let new_pt: core::syntax::Clause<core::syntax::Ctor> = core::syntax::Clause {
                        xtor: pt_i.xtor.clone().compile(state),
                        vars: pt_i.vars.clone(),
                        covars: vec![],
                        rhs: Rc::clone(rhs_i),
                    };
                    new_pts.insert(0, new_pt);
                }
                let new_cut: Rc<core::syntax::Statement> = Rc::new(
                    core::syntax::Cut {
                        producer: p,
                        consumer: Rc::new(core::syntax::Consumer::Case(new_pts)),
                    }
                    .into(),
                );
                core::syntax::Mu {
                    covariable: new_cv,
                    statement: new_cut,
                }
                .into()
            }
            fun::syntax::Term::Cocase(pts) => {
                let mut new_pts: Vec<core::syntax::Clause<core::syntax::Dtor>> = vec![];
                for pt in pts.iter().cloned() {
                    let pt_cloned: Rc<fun::syntax::Clause<fun::syntax::Dtor>> = pt.clone();
                    let rhs: Rc<core::syntax::Producer> =
                        Rc::new(Rc::unwrap_or_clone(pt).rhs.compile(state));
                    state.add_covars(Rc::as_ref(&rhs));
                    let new_cv: Covariable = state.free_covar_from_state();
                    let new_covar = Rc::new(core::syntax::Consumer::Covar(new_cv.clone()));
                    let new_cut: Rc<core::syntax::Statement> = Rc::new(
                        core::syntax::Cut {
                            producer: rhs,
                            consumer: new_covar,
                        }
                        .into(),
                    );
                    let new_pt: core::syntax::Clause<core::syntax::Dtor> = core::syntax::Clause {
                        xtor: pt_cloned.xtor.clone().compile(state),
                        vars: pt_cloned.vars.clone(),
                        covars: vec![new_cv],
                        rhs: new_cut,
                    };
                    new_pts.insert(0, new_pt);
                }
                core::syntax::Cocase { cocases: new_pts }.into()
            }
            fun::syntax::Term::Lam(var, t1) => {
                let p = t1.compile(state);
                state.add_covars(Rc::as_ref(&p));
                let new_cv: Covariable = state.free_covar_from_state();
                let new_covar: Rc<core::syntax::Consumer> =
                    Rc::new(core::syntax::Consumer::Covar(new_cv.clone()));
                let new_rhs: Rc<core::syntax::Statement> = Rc::new(
                    core::syntax::Cut {
                        producer: p,
                        consumer: new_covar,
                    }
                    .into(),
                );
                let new_pt: core::syntax::Clause<core::syntax::Dtor> = core::syntax::Clause {
                    xtor: core::syntax::Dtor::Ap,
                    vars: vec![var],
                    covars: vec![new_cv],
                    rhs: new_rhs,
                };
                core::syntax::Cocase {
                    cocases: vec![new_pt],
                }
                .into()
            }
            fun::syntax::Term::App(t1, t2) => {
                let p1 = t1.compile(state);
                state.add_covars(Rc::as_ref(&p1));
                let p2 = t2.compile(state);
                state.add_covars(Rc::as_ref(&p2));
                let new_cv: Covariable = state.free_covar_from_state();
                let new_covar: Rc<core::syntax::Consumer> =
                    Rc::new(core::syntax::Consumer::Covar(new_cv.clone()));
                let new_dt: Rc<core::syntax::Consumer> = Rc::new(
                    core::syntax::Destructor {
                        id: core::syntax::Dtor::Ap,
                        producers: vec![p2],
                        consumers: vec![new_covar],
                    }
                    .into(),
                );
                let new_cut: Rc<core::syntax::Statement> = Rc::new(
                    core::syntax::Cut {
                        producer: p1,
                        consumer: new_dt,
                    }
                    .into(),
                );
                core::syntax::Mu {
                    covariable: new_cv,
                    statement: new_cut,
                }
                .into()
            }
            fun::syntax::Term::Goto(t, covar) => {
                state.covars.insert(covar.clone());
                let p = t.compile(state);
                state.add_covars(Rc::as_ref(&p));
                let new_cv: Covariable = state.free_covar_from_state();
                let new_covar: Rc<core::syntax::Consumer> =
                    Rc::new(core::syntax::Consumer::Covar(covar));
                let new_cut: Rc<core::syntax::Statement> = Rc::new(
                    core::syntax::Cut {
                        producer: p,
                        consumer: new_covar,
                    }
                    .into(),
                );
                core::syntax::Mu {
                    covariable: new_cv,
                    statement: new_cut,
                }
                .into()
            }
            fun::syntax::Term::Label(covar, t) => {
                state.covars.insert(covar.clone());
                let p = t.compile(state);
                let new_cv: Rc<core::syntax::Consumer> =
                    Rc::new(core::syntax::Consumer::Covar(covar.clone()));
                let new_cut: Rc<core::syntax::Statement> = Rc::new(
                    core::syntax::Cut {
                        producer: p,
                        consumer: new_cv,
                    }
                    .into(),
                );
                core::syntax::Mu {
                    covariable: covar,
                    statement: new_cut,
                }
                .into()
            }
        }
    }
}

pub fn compile_def<T>(def: fun::syntax::Def<T>) -> core::syntax::Def<T> {
    let mut initial_state: CompileState = CompileState {
        covars: def.cont.iter().map(|(cv, _)| cv).cloned().collect(),
    };
    let new_body: Rc<core::syntax::Producer> = Rc::new(def.body.compile(&mut initial_state));
    initial_state.add_covars(Rc::as_ref(&new_body));
    let new_cv: Covariable = initial_state.free_covar_from_state();
    let new_covar: Rc<core::syntax::Consumer> =
        Rc::new(core::syntax::Consumer::Covar(new_cv.clone()));
    let new_cut: core::syntax::Statement = core::syntax::Cut {
        producer: new_body,
        consumer: new_covar,
    }
    .into();
    let mut new_cont: Vec<(Covariable, T)> = def.cont;
    new_cont.insert(new_cont.len(), (new_cv, def.ret_ty));
    core::syntax::Def {
        name: def.name,
        pargs: def.args,
        cargs: new_cont,
        body: new_cut,
    }
}

pub fn compile_prog<T: Clone>(prog: fun::syntax::Prog<T>) -> core::syntax::Prog<T> {
    let new_defs: Vec<core::syntax::Def<T>> = prog
        .prog_defs
        .iter()
        .cloned()
        .map(|x| compile_def(x.clone()))
        .collect();
    core::syntax::Prog {
        prog_defs: new_defs,
    }
}
