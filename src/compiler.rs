use crate::core::substitution::{fresh_covar, FreeV};
use crate::core::syntax as core;
use crate::fun::syntax as fun;
use crate::fun::syntax::{Covariable, Ctor, Dtor};
use std::collections::HashSet;
use std::rc::Rc;

type CompileState = HashSet<Covariable>;

fn add_covars<T: FreeV>(new_cv: &T, st: &mut CompileState) -> () {
    let fr_cv: HashSet<Covariable> = FreeV::free_covars(new_cv);
    st.extend(fr_cv);
}

fn free_covar_from_state(st: &mut CompileState) -> Covariable {
    let st_list: Vec<Covariable> = st.clone().into_iter().collect();
    let new_cv: Covariable = fresh_covar(&st_list);
    st.insert(new_cv.clone());
    new_cv
}

fn compile(t: fun::Term, st: &mut CompileState) -> core::Producer {
    match t {
        fun::Term::Var(v) => core::Producer::Var(v),
        fun::Term::Lit(n) => core::Producer::Lit(n),
        fun::Term::Op(t1, op, t2) => {
            let p1: Rc<core::Producer> = Rc::new(compile(Rc::unwrap_or_clone(t1), st));
            let p2: Rc<core::Producer> = Rc::new(compile(Rc::unwrap_or_clone(t2), st));
            add_covars(Rc::as_ref(&p1), st);
            add_covars(Rc::as_ref(&p2), st);
            let new_cv: Covariable = free_covar_from_state(st);
            let new_op: core::Statement =
                core::Statement::Op(p1, op, p2, Rc::new(core::Consumer::Covar(new_cv.clone())));
            core::Producer::Mu(new_cv, Rc::new(new_op))
        }
        fun::Term::IfZ(t1, t2, t3) => {
            let p1: Rc<core::Producer> = Rc::new(compile(Rc::unwrap_or_clone(t1), st));
            let p2: Rc<core::Producer> = Rc::new(compile(Rc::unwrap_or_clone(t2), st));
            let p3: Rc<core::Producer> = Rc::new(compile(Rc::unwrap_or_clone(t3), st));
            add_covars(Rc::as_ref(&p1), st);
            add_covars(Rc::as_ref(&p2), st);
            add_covars(Rc::as_ref(&p3), st);
            let new_cv = free_covar_from_state(st);
            let new_cons: Rc<core::Consumer> = Rc::new(core::Consumer::Covar(new_cv.clone()));
            let s1: Rc<core::Statement> = Rc::new(core::Statement::Cut(p2, new_cons.clone()));
            let s2: Rc<core::Statement> = Rc::new(core::Statement::Cut(p3, new_cons));
            let new_if: Rc<core::Statement> = Rc::new(core::Statement::IfZ(p1, s1, s2));
            core::Producer::Mu(new_cv, new_if)
        }
        fun::Term::Let(var, t1, t2) => {
            let p1: Rc<core::Producer> = Rc::new(compile(Rc::unwrap_or_clone(t1), st));
            let p2: Rc<core::Producer> = Rc::new(compile(Rc::unwrap_or_clone(t2), st));
            add_covars(Rc::as_ref(&p1), st);
            add_covars(Rc::as_ref(&p2), st);
            let new_cv: Covariable = free_covar_from_state(st);
            let new_cons: Rc<core::Consumer> = Rc::new(core::Consumer::Covar(new_cv.clone()));
            let cut_inner: Rc<core::Statement> = Rc::new(core::Statement::Cut(p2, new_cons));
            let new_mutilde: Rc<core::Consumer> =
                Rc::new(core::Consumer::MuTildeDyn(var.clone(), cut_inner));
            let cut_outer: Rc<core::Statement> = Rc::new(core::Statement::Cut(p1, new_mutilde));
            core::Producer::Mu(new_cv, cut_outer)
        }
        fun::Term::Fun(nm, args, coargs) => {
            let mut args_comp: Vec<Rc<core::Producer>> = vec![];
            for arg in args.iter().cloned() {
                let arg_comp: Rc<core::Producer> = Rc::new(compile(Rc::unwrap_or_clone(arg), st));
                add_covars(Rc::as_ref(&arg_comp), st);
                args_comp.insert(0, arg_comp);
            }
            for cv in coargs.iter() {
                st.insert(cv.clone());
            }
            let new_cv: Covariable = free_covar_from_state(st);
            let new_covar: Rc<core::Consumer> = Rc::new(core::Consumer::Covar(new_cv.clone()));
            let mut new_cvs: Vec<Rc<core::Consumer>> = coargs
                .iter()
                .map(|cv| Rc::new(core::Consumer::Covar(cv.clone())))
                .collect();
            new_cvs.insert(new_cvs.len(), new_covar);
            let new_fun: Rc<core::Statement> =
                Rc::new(core::Statement::Fun(nm, args_comp, new_cvs));
            core::Producer::Mu(new_cv, new_fun)
        }
        fun::Term::Constructor(ctor, args) => {
            let args_comp: Vec<Rc<core::Producer>> = args
                .iter()
                .cloned()
                .map(|arg| Rc::new(compile(Rc::unwrap_or_clone(arg), st)))
                .collect();
            core::Producer::Constructor(ctor.clone(), args_comp, vec![])
        }
        fun::Term::Destructor(t, dtor, args) => {
            let p: Rc<core::Producer> = Rc::new(compile(Rc::unwrap_or_clone(t), st));
            add_covars(Rc::as_ref(&p), st);
            let args_comp: Vec<Rc<core::Producer>> = args
                .iter()
                .cloned()
                .map(|arg| Rc::new(compile(Rc::unwrap_or_clone(arg), st)))
                .collect();
            add_covars(&args_comp, st);
            let new_cv: Covariable = free_covar_from_state(st);
            let new_dt: Rc<core::Consumer> = Rc::new(core::Consumer::Destructor(
                dtor.clone(),
                args_comp,
                vec![Rc::new(core::Consumer::Covar(new_cv.clone()))],
            ));
            let new_cut: Rc<core::Statement> = Rc::new(core::Statement::Cut(p, new_dt));
            core::Producer::Mu(new_cv, new_cut)
        }
        fun::Term::Case(t, pts) => {
            let p: Rc<core::Producer> = Rc::new(compile(Rc::unwrap_or_clone(t), st));
            add_covars(Rc::as_ref(&p), st);
            let rhs_comp: Vec<Rc<core::Producer>> = pts
                .iter()
                .cloned()
                .map(|pt| Rc::new(compile(Rc::unwrap_or_clone(pt).rhs, st)))
                .collect();
            let _ = rhs_comp.iter().map(|p| add_covars(Rc::as_ref(&p), st));
            let new_cv: Covariable = free_covar_from_state(st);
            let new_covar: Rc<core::Consumer> = Rc::new(core::Consumer::Covar(new_cv.clone()));
            let rhs_cuts: Vec<Rc<core::Statement>> = rhs_comp
                .iter()
                .cloned()
                .map(|p| Rc::new(core::Statement::Cut(p, Rc::clone(&new_covar))))
                .collect();

            let mut new_pts: Vec<core::Pattern<Ctor>> = vec![];
            for i in 0..pts.len() - 1 {
                let pt_i: &Rc<fun::Clause<Ctor>> =
                    pts.get(i).expect("Invalid pattern (should never happen");
                let rhs_i: &Rc<core::Statement> = rhs_cuts
                    .get(i)
                    .expect("Invalid pattern (should never happen");
                let new_pt: core::Pattern<Ctor> = core::Pattern {
                    xtor: pt_i.xtor.clone(),
                    vars: pt_i.vars.clone(),
                    covars: vec![],
                    rhs: Rc::clone(rhs_i),
                };
                new_pts.insert(0, new_pt);
            }
            let new_cut: Rc<core::Statement> = Rc::new(core::Statement::Cut(
                p,
                Rc::new(core::Consumer::Case(new_pts)),
            ));
            core::Producer::Mu(new_cv, new_cut)
        }
        fun::Term::Cocase(pts) => {
            let mut new_pts: Vec<core::Pattern<Dtor>> = vec![];
            for pt in pts.iter().cloned() {
                let pt_cloned: Rc<fun::Clause<Dtor>> = pt.clone();
                let rhs: Rc<core::Producer> = Rc::new(compile(Rc::unwrap_or_clone(pt).rhs, st));
                add_covars(Rc::as_ref(&rhs), st);
                let new_cv: Covariable = free_covar_from_state(st);
                let new_covar: Rc<core::Consumer> = Rc::new(core::Consumer::Covar(new_cv.clone()));
                let new_cut: Rc<core::Statement> = Rc::new(core::Statement::Cut(rhs, new_covar));
                let new_pt: core::Pattern<Dtor> = core::Pattern {
                    xtor: pt_cloned.xtor.clone(),
                    vars: pt_cloned.vars.clone(),
                    covars: vec![new_cv],
                    rhs: new_cut,
                };
                new_pts.insert(0, new_pt);
            }
            core::Producer::Cocase(new_pts)
        }
        fun::Term::Lam(var, t1) => {
            let p: Rc<core::Producer> = Rc::new(compile(Rc::unwrap_or_clone(t1), st));
            add_covars(Rc::as_ref(&p), st);
            let new_cv: Covariable = free_covar_from_state(st);
            let new_covar: Rc<core::Consumer> = Rc::new(core::Consumer::Covar(new_cv.clone()));
            let new_rhs: Rc<core::Statement> = Rc::new(core::Statement::Cut(p, new_covar));
            let new_pt: core::Pattern<Dtor> = core::Pattern {
                xtor: Dtor::Ap,
                vars: vec![var],
                covars: vec![new_cv],
                rhs: new_rhs,
            };
            core::Producer::Cocase(vec![new_pt])
        }
        fun::Term::App(t1, t2) => {
            let p1: Rc<core::Producer> = Rc::new(compile(Rc::unwrap_or_clone(t1), st));
            add_covars(Rc::as_ref(&p1), st);
            let p2: Rc<core::Producer> = Rc::new(compile(Rc::unwrap_or_clone(t2), st));
            add_covars(Rc::as_ref(&p2), st);
            let new_cv: Covariable = free_covar_from_state(st);
            let new_covar: Rc<core::Consumer> = Rc::new(core::Consumer::Covar(new_cv.clone()));
            let new_dt: Rc<core::Consumer> = Rc::new(core::Consumer::Destructor(
                Dtor::Ap,
                vec![p2],
                vec![new_covar],
            ));
            let new_cut: Rc<core::Statement> = Rc::new(core::Statement::Cut(p1, new_dt));
            core::Producer::Mu(new_cv, new_cut)
        }
        fun::Term::Goto(t, covar) => {
            st.insert(covar.clone());
            let p: Rc<core::Producer> = Rc::new(compile(Rc::unwrap_or_clone(t), st));
            add_covars(Rc::as_ref(&p), st);
            let new_cv: Covariable = free_covar_from_state(st);
            let new_covar: Rc<core::Consumer> = Rc::new(core::Consumer::Covar(covar));
            let new_cut: Rc<core::Statement> = Rc::new(core::Statement::Cut(p, new_covar));
            core::Producer::MuDyn(new_cv, new_cut)
        }
        fun::Term::Label(covar, t) => {
            st.insert(covar.clone());
            let p: Rc<core::Producer> = Rc::new(compile(Rc::unwrap_or_clone(t), st));
            let new_cv: Rc<core::Consumer> = Rc::new(core::Consumer::Covar(covar.clone()));
            let new_cut: Rc<core::Statement> = Rc::new(core::Statement::Cut(p, new_cv));
            core::Producer::MuDyn(covar, new_cut)
        }
    }
}

fn compile_def<T>(def: fun::Def<T>) -> core::Def<T> {
    let mut initial_state: CompileState = def.cont.iter().map(|(cv, _)| cv).cloned().collect();
    let new_body: Rc<core::Producer> = Rc::new(compile(def.body, &mut initial_state));
    add_covars(Rc::as_ref(&new_body), &mut initial_state);
    let new_cv: Covariable = free_covar_from_state(&mut initial_state);
    let new_covar: Rc<core::Consumer> = Rc::new(core::Consumer::Covar(new_cv.clone()));
    let new_cut: core::Statement = core::Statement::Cut(new_body, new_covar);
    let mut new_cont: Vec<(Covariable, T)> = def.cont;
    new_cont.insert(new_cont.len(), (new_cv, def.ret_ty));
    core::Def {
        name: def.name,
        pargs: def.args,
        cargs: new_cont,
        body: new_cut,
    }
}

fn compile_prog<T: Clone>(prog: fun::Prog<T>) -> core::Prog<T> {
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
