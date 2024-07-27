use crate::core::syntax as core;
use crate::core::traits::free_vars::{fresh_covar, FreeV};
use crate::fun::syntax as fun;
use crate::fun::syntax::{Covariable, Ctor, Dtor};
use std::collections::HashSet;
use std::rc::Rc;

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

impl Compile for fun::Term {
    type Target = core::Producer;

    fn compile(self, state: &mut CompileState) -> Self::Target {
        match self {
            fun::Term::Var(v) => core::Variable { var: v }.into(),
            fun::Term::Lit(n) => core::Literal { lit: n }.into(),
            fun::Term::Op(t1, op, t2) => {
                let p1 = t1.compile(state);
                let p2 = t2.compile(state);
                state.add_covars(Rc::as_ref(&p1));
                state.add_covars(Rc::as_ref(&p2));
                let new_cv = state.free_covar_from_state();
                let new_op = core::Op {
                    fst: p1,
                    op,
                    snd: p2,
                    continuation: Rc::new(core::Consumer::Covar(new_cv.clone())),
                }
                .into();
                core::Mu {
                    covariable: new_cv,
                    statement: Rc::new(new_op),
                }
                .into()
            }
            fun::Term::IfZ(t1, t2, t3) => {
                let p1 = t1.compile(state);
                let p2 = t2.compile(state);
                let p3 = t3.compile(state);
                state.add_covars(Rc::as_ref(&p1));
                state.add_covars(Rc::as_ref(&p2));
                state.add_covars(Rc::as_ref(&p3));
                let new_cv = state.free_covar_from_state();
                let new_cons = Rc::new(core::Consumer::Covar(new_cv.clone()));
                let s1 = Rc::new(
                    core::Cut {
                        producer: p2,
                        consumer: new_cons.clone(),
                    }
                    .into(),
                );
                let s2 = Rc::new(
                    core::Cut {
                        producer: p3,
                        consumer: new_cons,
                    }
                    .into(),
                );
                let new_if = Rc::new(
                    core::IfZ {
                        ifc: p1,
                        thenc: s1,
                        elsec: s2,
                    }
                    .into(),
                );
                core::Mu {
                    covariable: new_cv,
                    statement: new_if,
                }
                .into()
            }
            fun::Term::Let(var, t1, t2) => {
                let p1 = t1.compile(state);
                let p2 = t2.compile(state);
                state.add_covars(Rc::as_ref(&p1));
                state.add_covars(Rc::as_ref(&p2));
                let new_cv = state.free_covar_from_state();
                let new_cons = Rc::new(core::Consumer::Covar(new_cv.clone()));
                let cut_inner = Rc::new(
                    core::Cut {
                        producer: p2,
                        consumer: new_cons,
                    }
                    .into(),
                );
                let new_mutilde = Rc::new(core::Consumer::MuTilde(var.clone(), cut_inner));
                let cut_outer = Rc::new(
                    core::Cut {
                        producer: p1,
                        consumer: new_mutilde,
                    }
                    .into(),
                );
                core::Mu {
                    covariable: new_cv,
                    statement: cut_outer,
                }
                .into()
            }
            fun::Term::Fun(nm, args, coargs) => {
                let mut args_comp: Vec<Rc<core::Producer>> = vec![];
                for arg in args.iter().cloned() {
                    let arg_comp: Rc<core::Producer> = arg.compile(state);
                    state.add_covars(Rc::as_ref(&arg_comp));
                    args_comp.insert(0, arg_comp);
                }
                for cv in coargs.iter() {
                    state.covars.insert(cv.clone());
                }
                let new_cv: Covariable = state.free_covar_from_state();
                let new_covar: Rc<core::Consumer> = Rc::new(core::Consumer::Covar(new_cv.clone()));
                let mut new_cvs: Vec<Rc<core::Consumer>> = coargs
                    .iter()
                    .map(|cv| Rc::new(core::Consumer::Covar(cv.clone())))
                    .collect();
                new_cvs.insert(new_cvs.len(), new_covar);
                let new_fun: Rc<core::Statement> = Rc::new(
                    core::Fun {
                        name: nm,
                        producers: args_comp,
                        consumers: new_cvs,
                    }
                    .into(),
                );
                core::Mu {
                    covariable: new_cv,
                    statement: new_fun,
                }
                .into()
            }
            fun::Term::Constructor(ctor, args) => {
                let args_comp: Vec<Rc<core::Producer>> = args
                    .iter()
                    .cloned()
                    .map(|arg| Rc::new(Rc::unwrap_or_clone(arg).compile(state)))
                    .collect();
                core::Constructor {
                    id: ctor.clone(),
                    producers: args_comp,
                    consumers: vec![],
                }
                .into()
            }
            fun::Term::Destructor(t, dtor, args) => {
                let p = t.compile(state);
                state.add_covars(Rc::as_ref(&p));
                let args_comp: Vec<Rc<core::Producer>> =
                    args.iter().cloned().map(|arg| arg.compile(state)).collect();
                state.add_covars(&args_comp);
                let new_cv: Covariable = state.free_covar_from_state();
                let new_dt: Rc<core::Consumer> = Rc::new(core::Consumer::Destructor(
                    dtor.clone(),
                    args_comp,
                    vec![Rc::new(core::Consumer::Covar(new_cv.clone()))],
                ));
                let new_cut: Rc<core::Statement> = Rc::new(
                    core::Cut {
                        producer: p,
                        consumer: new_dt,
                    }
                    .into(),
                );
                core::Mu {
                    covariable: new_cv,
                    statement: new_cut,
                }
                .into()
            }
            fun::Term::Case(t, pts) => {
                let p = t.compile(state);
                state.add_covars(Rc::as_ref(&p));
                let rhs_comp: Vec<Rc<core::Producer>> = pts
                    .iter()
                    .cloned()
                    .map(|pt| Rc::new(Rc::unwrap_or_clone(pt).rhs.compile(state)))
                    .collect();
                let _ = rhs_comp.iter().map(|p| state.add_covars(Rc::as_ref(p)));
                let new_cv: Covariable = state.free_covar_from_state();
                let new_covar: Rc<core::Consumer> = Rc::new(core::Consumer::Covar(new_cv.clone()));
                let rhs_cuts: Vec<Rc<core::Statement>> = rhs_comp
                    .iter()
                    .cloned()
                    .map(|p| {
                        Rc::new(
                            core::Cut {
                                producer: p,
                                consumer: Rc::clone(&new_covar),
                            }
                            .into(),
                        )
                    })
                    .collect();

                let mut new_pts: Vec<core::Clause<Ctor>> = vec![];
                for i in 0..pts.len() - 1 {
                    let pt_i: &Rc<fun::Clause<Ctor>> =
                        pts.get(i).expect("Invalid pattern (should never happen");
                    let rhs_i: &Rc<core::Statement> = rhs_cuts
                        .get(i)
                        .expect("Invalid pattern (should never happen");
                    let new_pt: core::Clause<Ctor> = core::Clause {
                        xtor: pt_i.xtor.clone(),
                        vars: pt_i.vars.clone(),
                        covars: vec![],
                        rhs: Rc::clone(rhs_i),
                    };
                    new_pts.insert(0, new_pt);
                }
                let new_cut: Rc<core::Statement> = Rc::new(
                    core::Cut {
                        producer: p,
                        consumer: Rc::new(core::Consumer::Case(new_pts)),
                    }
                    .into(),
                );
                core::Mu {
                    covariable: new_cv,
                    statement: new_cut,
                }
                .into()
            }
            fun::Term::Cocase(pts) => {
                let mut new_pts: Vec<core::Clause<Dtor>> = vec![];
                for pt in pts.iter().cloned() {
                    let pt_cloned: Rc<fun::Clause<Dtor>> = pt.clone();
                    let rhs: Rc<core::Producer> =
                        Rc::new(Rc::unwrap_or_clone(pt).rhs.compile(state));
                    state.add_covars(Rc::as_ref(&rhs));
                    let new_cv: Covariable = state.free_covar_from_state();
                    let new_covar = Rc::new(core::Consumer::Covar(new_cv.clone()));
                    let new_cut: Rc<core::Statement> = Rc::new(
                        core::Cut {
                            producer: rhs,
                            consumer: new_covar,
                        }
                        .into(),
                    );
                    let new_pt: core::Clause<Dtor> = core::Clause {
                        xtor: pt_cloned.xtor.clone(),
                        vars: pt_cloned.vars.clone(),
                        covars: vec![new_cv],
                        rhs: new_cut,
                    };
                    new_pts.insert(0, new_pt);
                }
                core::Cocase { cocases: new_pts }.into()
            }
            fun::Term::Lam(var, t1) => {
                let p = t1.compile(state);
                state.add_covars(Rc::as_ref(&p));
                let new_cv: Covariable = state.free_covar_from_state();
                let new_covar: Rc<core::Consumer> = Rc::new(core::Consumer::Covar(new_cv.clone()));
                let new_rhs: Rc<core::Statement> = Rc::new(
                    core::Cut {
                        producer: p,
                        consumer: new_covar,
                    }
                    .into(),
                );
                let new_pt: core::Clause<Dtor> = core::Clause {
                    xtor: Dtor::Ap,
                    vars: vec![var],
                    covars: vec![new_cv],
                    rhs: new_rhs,
                };
                core::Cocase {
                    cocases: vec![new_pt],
                }
                .into()
            }
            fun::Term::App(t1, t2) => {
                let p1 = t1.compile(state);
                state.add_covars(Rc::as_ref(&p1));
                let p2 = t2.compile(state);
                state.add_covars(Rc::as_ref(&p2));
                let new_cv: Covariable = state.free_covar_from_state();
                let new_covar: Rc<core::Consumer> = Rc::new(core::Consumer::Covar(new_cv.clone()));
                let new_dt: Rc<core::Consumer> = Rc::new(core::Consumer::Destructor(
                    Dtor::Ap,
                    vec![p2],
                    vec![new_covar],
                ));
                let new_cut: Rc<core::Statement> = Rc::new(
                    core::Cut {
                        producer: p1,
                        consumer: new_dt,
                    }
                    .into(),
                );
                core::Mu {
                    covariable: new_cv,
                    statement: new_cut,
                }
                .into()
            }
            fun::Term::Goto(t, covar) => {
                state.covars.insert(covar.clone());
                let p = t.compile(state);
                state.add_covars(Rc::as_ref(&p));
                let new_cv: Covariable = state.free_covar_from_state();
                let new_covar: Rc<core::Consumer> = Rc::new(core::Consumer::Covar(covar));
                let new_cut: Rc<core::Statement> = Rc::new(
                    core::Cut {
                        producer: p,
                        consumer: new_covar,
                    }
                    .into(),
                );
                core::Mu {
                    covariable: new_cv,
                    statement: new_cut,
                }
                .into()
            }
            fun::Term::Label(covar, t) => {
                state.covars.insert(covar.clone());
                let p = t.compile(state);
                let new_cv: Rc<core::Consumer> = Rc::new(core::Consumer::Covar(covar.clone()));
                let new_cut: Rc<core::Statement> = Rc::new(
                    core::Cut {
                        producer: p,
                        consumer: new_cv,
                    }
                    .into(),
                );
                core::Mu {
                    covariable: covar,
                    statement: new_cut,
                }
                .into()
            }
        }
    }
}

pub fn compile_def<T>(def: fun::Def<T>) -> core::Def<T> {
    let mut initial_state: CompileState = CompileState {
        covars: def.cont.iter().map(|(cv, _)| cv).cloned().collect(),
    };
    let new_body: Rc<core::Producer> = Rc::new(def.body.compile(&mut initial_state));
    initial_state.add_covars(Rc::as_ref(&new_body));
    let new_cv: Covariable = initial_state.free_covar_from_state();
    let new_covar: Rc<core::Consumer> = Rc::new(core::Consumer::Covar(new_cv.clone()));
    let new_cut: core::Statement = core::Cut {
        producer: new_body,
        consumer: new_covar,
    }
    .into();
    let mut new_cont: Vec<(Covariable, T)> = def.cont;
    new_cont.insert(new_cont.len(), (new_cv, def.ret_ty));
    core::Def {
        name: def.name,
        pargs: def.args,
        cargs: new_cont,
        body: new_cut,
    }
}

pub fn compile_prog<T: Clone>(prog: fun::Prog<T>) -> core::Prog<T> {
    let new_defs: Vec<core::Def<T>> = prog
        .prog_defs
        .iter()
        .cloned()
        .map(|x| compile_def(x.clone()))
        .collect();
    core::Prog {
        prog_defs: new_defs,
    }
}
