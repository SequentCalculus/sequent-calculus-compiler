use crate::syntax::{Clause, Consumer, Def, Producer, Prog, Statement};
use std::rc::Rc;

use super::syntax::{
    BinOp, Cocase, Constructor, Covariable, Ctor, Cut, Destructor, Dtor, Fun, IfZ, Literal, Mu,
    MuTilde, Op, Var,
};
use super::traits::substitution::Subst;

fn eval<T>(st: Statement, p: &Prog<T>, tr: &mut Vec<Statement>) -> Vec<Statement> {
    let st_eval: Option<Statement> = st.eval_once(p);
    match st_eval {
        None => tr.clone(),
        Some(st_new) => {
            tr.push(st_new.clone());
            eval(st_new, p, tr)
        }
    }
}

macro_rules! eval {
    ($st: expr,$p:expr,$tr:expr) => {
        eval($st, $p, $tr)
    };
    ($st:expr,$p:expr) => {
        eval($st, $p, &mut vec![])
    };
}

trait EvalOnce {
    fn eval_once<T>(self, p: &Prog<T>) -> Option<Statement>;
}

impl EvalOnce for Statement {
    fn eval_once<T>(self, p: &Prog<T>) -> Option<Statement> {
        match self {
            Statement::Cut(c) => c.eval_once(p),
            Statement::Op(o) => o.eval_once(p),
            Statement::IfZ(i) => i.eval_once(p),
            Statement::Fun(f) => f.eval_once(p),
            Statement::Done() => Some(Statement::Done()),
        }
    }
}

impl EvalOnce for Cut {
    fn eval_once<T>(self, _p: &Prog<T>) -> Option<Statement> {
        let Cut { producer, consumer } = self;
        match (Rc::unwrap_or_clone(producer), Rc::unwrap_or_clone(consumer)) {
            (
                Producer::Mu(Mu {
                    covariable,
                    statement,
                }),
                cons,
            ) => {
                let st_subst: Rc<Statement> = Subst::subst_covar(&statement, cons, covariable);
                Some(Rc::unwrap_or_clone(st_subst))
            }
            (
                prod,
                Consumer::MuTilde(MuTilde {
                    variable: v,
                    statement: mu_st,
                }),
            ) => {
                let st_subst: Rc<Statement> = Subst::subst_var(&mu_st, prod, v);
                Some(Rc::unwrap_or_clone(st_subst))
            }
            (
                Producer::Constructor(Constructor {
                    id,
                    producers,
                    consumers,
                }),
                Consumer::Case(pts),
            ) => {
                let ct_pt: &Clause<Ctor> = pts.iter().find(|pt| pt.xtor == id)?;
                let prod_subst: Vec<(Producer, Var)> =
                    producers.iter().cloned().zip(ct_pt.vars.clone()).collect();
                let cons_subst: Vec<(Consumer, Covariable)> = consumers
                    .iter()
                    .cloned()
                    .zip(ct_pt.covars.clone())
                    .collect();
                let new_st: Rc<Statement> = ct_pt.rhs.subst_sim(&prod_subst, &cons_subst);
                Some(Rc::unwrap_or_clone(new_st))
            }
            (
                Producer::Cocase(Cocase { cocases }),
                Consumer::Destructor(Destructor {
                    id: dtor,
                    producers: pargs,
                    consumers: cargs,
                }),
            ) => {
                let dt_pt: &Clause<Dtor> = cocases.iter().find(|pt| pt.xtor == dtor)?;
                let prod_subst: Vec<(Producer, Var)> =
                    pargs.iter().cloned().zip(dt_pt.vars.clone()).collect();
                let cons_subst: Vec<(Consumer, Covariable)> =
                    cargs.iter().cloned().zip(dt_pt.covars.clone()).collect();
                let new_st: Rc<Statement> = Subst::subst_sim(&dt_pt.rhs, &prod_subst, &cons_subst);
                Some(Rc::unwrap_or_clone(new_st))
            }
            (_, _) => None,
        }
    }
}
impl EvalOnce for Op {
    fn eval_once<T>(self, _p: &Prog<T>) -> Option<Statement> {
        let Op {
            fst,
            op,
            snd,
            continuation,
        } = self;
        match (Rc::unwrap_or_clone(fst), Rc::unwrap_or_clone(snd)) {
            (Producer::Literal(Literal { lit: n }), Producer::Literal(Literal { lit: m })) => {
                let new_int: i64 = match op {
                    BinOp::Prod => n * m,
                    BinOp::Sum => n + m,
                    BinOp::Sub => n - m,
                };
                let new_lit: Rc<Producer> = Rc::new(Literal { lit: new_int }.into());
                Some(
                    Cut {
                        producer: new_lit,
                        consumer: continuation,
                    }
                    .into(),
                )
            }
            (_, _) => None,
        }
    }
}

impl EvalOnce for IfZ {
    fn eval_once<T>(self, _p: &Prog<T>) -> Option<Statement> {
        let IfZ { ifc, thenc, elsec } = self;
        match Rc::unwrap_or_clone(ifc) {
            Producer::Literal(Literal { lit: 0 }) => Some(Rc::unwrap_or_clone(thenc)),
            Producer::Literal(Literal { lit: n }) if n != 0 => Some(Rc::unwrap_or_clone(elsec)),
            _ => None,
        }
    }
}

impl EvalOnce for Fun {
    fn eval_once<T>(self, p: &Prog<T>) -> Option<Statement> {
        let Fun {
            name,
            producers,
            consumers,
        } = self;
        let nm_def: &Def<T> = p.prog_defs.iter().find(|df| df.name == name)?;
        let prod_vars: Vec<Var> = nm_def.pargs.iter().map(|(var, _)| var.clone()).collect();
        let prod_subst: Vec<(Producer, Var)> = producers.iter().cloned().zip(prod_vars).collect();
        let cons_covars: Vec<Covariable> = nm_def
            .cargs
            .iter()
            .map(|(covar, _)| covar.clone())
            .collect();
        let cons_subst: Vec<(Consumer, Covariable)> =
            consumers.iter().cloned().zip(cons_covars).collect();
        let new_st = nm_def.body.subst_sim(&prod_subst, &cons_subst);
        Some(new_st)
    }
}

pub fn eval_main<T>(prog: Prog<T>) -> Option<Vec<Statement>> {
    let main_def: &Def<T> = prog.prog_defs.iter().find(|df| df.name == "main")?;
    let main_cont: &(String, T) = main_def.cargs.first()?;
    let main_body = main_def
        .body
        .subst_covar(Consumer::Covar(String::from("*")), main_cont.0.clone());
    Some(eval!(main_body, &prog))
}
