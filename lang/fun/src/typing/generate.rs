//! Constraint generation

use std::collections::HashMap;

use crate::{
    program::{Def, Prog},
    syntax::{
        App, Case, Clause, Cocase, Constructor, Covariable, Ctor, Destructor, Dtor, Fun, Goto, IfZ,
        Label, Lam, Let, Name, Op, Term, Variable,
    },
};

use super::{result::TypeError, Constraint, Ty};

struct GenReader<'a> {
    gen_vars: HashMap<Variable, Ty>,
    gen_covars: HashMap<Covariable, Ty>,
    gen_prog: &'a Prog<Ty>,
}

impl<'a> GenReader<'a> {
    fn add_var_bindings(&self, new_bindings: Vec<(Variable, Ty)>) -> GenReader {
        let new_map: HashMap<Variable, Ty> = new_bindings.into_iter().collect();
        let new_gen_vars: HashMap<Variable, Ty> =
            self.gen_vars.clone().into_iter().chain(new_map).collect();
        GenReader {
            gen_vars: new_gen_vars,
            gen_covars: self.gen_covars.clone(),
            gen_prog: self.gen_prog,
        }
    }

    fn add_covar_bindings(&self, new_bindings: Vec<(Covariable, Ty)>) -> GenReader {
        let new_map: HashMap<Covariable, Ty> = new_bindings.into_iter().collect();
        let new_gen_covars: HashMap<Covariable, Ty> =
            self.gen_covars.clone().into_iter().chain(new_map).collect();
        GenReader {
            gen_vars: self.gen_vars.clone(),
            gen_covars: new_gen_covars,
            gen_prog: self.gen_prog,
        }
    }

    fn lookup_definition(&self, name: &Name) -> Result<(Vec<Ty>, Vec<Ty>, Ty), TypeError> {
        match &self.gen_prog.prog_defs.iter().find(|def| def.name == *name) {
            None => Err(TypeError::FunNotFound { name: name.clone() }),
            Some(def) => {
                let arg_tys = def.args.iter().map(|(_, x)| x.clone()).collect();
                let cont_tys = def.cont.iter().map(|(_, x)| x.clone()).collect();
                Ok((arg_tys, cont_tys, def.ret_ty.clone()))
            }
        }
    }
}

struct GenState {
    varcnt: i64,
    constraints: Vec<Constraint>,
}

impl GenState {
    fn fresh_var(&mut self) -> Ty {
        let new_var: String = self.varcnt.to_string();
        self.varcnt += 1;
        Ty::Var(new_var)
    }

    fn add_constraint(&mut self, constraint: Constraint) {
        self.constraints.push(constraint)
    }
}

trait GenConstraint {
    fn gen_constraints(&self, env: &GenReader, st: &mut GenState) -> Result<Ty, TypeError>;
}

impl GenConstraint for Op {
    fn gen_constraints(&self, env: &GenReader, st: &mut GenState) -> Result<Ty, TypeError> {
        let ty1 = self.fst.gen_constraints(env, st)?;
        let ty2 = self.snd.gen_constraints(env, st)?;
        st.add_constraint((ty1, Ty::Int()));
        st.add_constraint((ty2, Ty::Int()));
        Ok(Ty::Int())
    }
}

impl GenConstraint for IfZ {
    fn gen_constraints(&self, env: &GenReader, st: &mut GenState) -> Result<Ty, TypeError> {
        let ty1 = self.ifc.gen_constraints(env, st)?;
        let ty2 = self.thenc.gen_constraints(env, st)?;
        let ty3 = self.elsec.gen_constraints(env, st)?;
        st.add_constraint((ty1, Ty::Int()));
        st.add_constraint((ty2.clone(), ty3));
        Ok(ty2)
    }
}

impl GenConstraint for Let {
    fn gen_constraints(&self, env: &GenReader, st: &mut GenState) -> Result<Ty, TypeError> {
        let ty = self.bound_term.gen_constraints(env, st)?;
        let new_reader: GenReader = env.add_var_bindings(vec![(self.variable.clone(), ty)]);
        self.in_term.gen_constraints(&new_reader, st)
    }
}

impl GenConstraint for Fun {
    fn gen_constraints(&self, env: &GenReader, st: &mut GenState) -> Result<Ty, TypeError> {
        let (arg_tys, coarg_tys, ret_ty) = env.lookup_definition(&self.name)?;
        if self.args.len() != arg_tys.len() || self.coargs.len() != coarg_tys.len() {
            Err(TypeError::FunWrongNumOfArgs {
                name: self.name.clone(),
                expected_vars: arg_tys.len(),
                actual_vars: self.args.len(),
                expected_covars: coarg_tys.len(),
                actual_covars: self.coargs.len(),
            })
        } else {
            let arg_tys1: Vec<Ty> = self
                .args
                .iter()
                .map(|x| x.gen_constraints(env, st))
                .collect::<Result<Vec<Ty>, TypeError>>()?;
            let args_zipped = arg_tys1.into_iter().zip(arg_tys);
            for constraint in args_zipped {
                st.add_constraint(constraint);
            }
            let coargs_zipped = self.coargs.iter().cloned().zip(coarg_tys);
            for (coarg, coarg_ty) in coargs_zipped {
                match env.gen_covars.iter().find(|(cv, _)| **cv == coarg) {
                    None => Err(TypeError::VarNotFound { name: coarg }),
                    Some((_, ty)) => {
                        st.add_constraint((ty.clone(), coarg_ty));
                        Ok(())
                    }
                }?;
            }
            Ok(ret_ty.clone())
        }
    }
}

impl GenConstraint for Constructor {
    fn gen_constraints(&self, env: &GenReader, st: &mut GenState) -> Result<Ty, TypeError> {
        match &self.id {
            Ctor::Nil if self.args.is_empty() => Ok(Ty::List(Box::new(st.fresh_var()))),
            Ctor::Cons => match self.args.as_slice() {
                [arg1, arg2] => {
                    let ty1: Ty = arg1.gen_constraints(env, st)?;
                    let ty2: Ty = arg2.gen_constraints(env, st)?;
                    st.add_constraint((Ty::List(Box::new(ty1)), ty2.clone()));
                    Ok(ty2)
                }
                _ => Err(TypeError::CtorWrongNumOfArgs { ctor: Ctor::Cons }),
            },
            Ctor::Tup => match self.args.as_slice() {
                [arg1, arg2] => {
                    let ty1: Ty = arg1.gen_constraints(env, st)?;
                    let ty2: Ty = arg2.gen_constraints(env, st)?;
                    Ok(Ty::Pair(Box::new(ty1), Box::new(ty2)))
                }
                _ => Err(TypeError::CtorWrongNumOfArgs { ctor: Ctor::Tup }),
            },
            ctor => Err(TypeError::CtorWrongNumOfArgs { ctor: ctor.clone() }),
        }
    }
}

impl GenConstraint for Destructor {
    fn gen_constraints(&self, env: &GenReader, st: &mut GenState) -> Result<Ty, TypeError> {
        match &self.id {
            Dtor::Hd if self.args.is_empty() => {
                let ty_bound: Ty = self.destructee.gen_constraints(env, st)?;
                let ty_a: Ty = st.fresh_var();
                st.add_constraint((ty_bound, Ty::Stream(Box::new(ty_a.clone()))));
                Ok(ty_a)
            }
            Dtor::Tl if self.args.is_empty() => {
                let ty_bound: Ty = self.destructee.gen_constraints(env, st)?;
                let ty_str: Ty = Ty::Stream(Box::new(st.fresh_var()));
                st.add_constraint((ty_bound, ty_str.clone()));
                Ok(ty_str)
            }
            Dtor::Fst if self.args.is_empty() => {
                let ty_bound: Ty = self.destructee.gen_constraints(env, st)?;
                let ty_a: Ty = st.fresh_var();
                let ty_b: Ty = st.fresh_var();
                st.add_constraint((ty_bound, Ty::LPair(Box::new(ty_a.clone()), Box::new(ty_b))));
                Ok(ty_a)
            }
            Dtor::Snd if self.args.is_empty() => {
                let ty_bound: Ty = self.destructee.gen_constraints(env, st)?;
                let ty_a: Ty = st.fresh_var();
                let ty_b: Ty = st.fresh_var();
                st.add_constraint((ty_bound, Ty::LPair(Box::new(ty_a), Box::new(ty_b.clone()))));
                Ok(ty_b)
            }
            dtor => Err(TypeError::DtorWrongNumOfArgs { dtor: dtor.clone() }),
        }
    }
}

impl GenConstraint for Case {
    fn gen_constraints(&self, env: &GenReader, st: &mut GenState) -> Result<Ty, TypeError> {
        if self.cases.len() == 1 {
            let clause_tup: &Clause<Ctor> = self.cases.first().ok_or(TypeError::InvalidCase)?;
            let ty_bound: Ty = self.destructee.gen_constraints(env, st)?;
            let ty_a: Ty = st.fresh_var();
            let ty_b: Ty = st.fresh_var();
            st.add_constraint((
                ty_bound,
                Ty::Pair(Box::new(ty_a.clone()), Box::new(ty_b.clone())),
            ));
            let var_first: &Variable =
                clause_tup
                    .vars
                    .first()
                    .ok_or(TypeError::PatternWrongNumOfArgs {
                        ctor: clause_tup.xtor.clone(),
                    })?;
            let var_second: &Variable =
                clause_tup
                    .vars
                    .get(1)
                    .ok_or(TypeError::PatternWrongNumOfArgs {
                        ctor: clause_tup.xtor.clone(),
                    })?;
            let new_env: GenReader =
                env.add_var_bindings(vec![(var_first.clone(), ty_a), (var_second.clone(), ty_b)]);
            clause_tup.rhs.gen_constraints(&new_env, st)
        } else if self.cases.len() == 2 {
            let clause_nil: &Clause<Ctor> = self
                .cases
                .iter()
                .find(|clauses| clauses.xtor == Ctor::Nil)
                .ok_or(TypeError::InvalidCase)?;
            let clause_cons: &Clause<Ctor> = self
                .cases
                .iter()
                .find(|clauses| clauses.xtor == Ctor::Cons)
                .ok_or(TypeError::InvalidCase)?;
            let ty_bound: Ty = self.destructee.gen_constraints(env, st)?;
            let list_arg: Box<Ty> = Box::new(st.fresh_var());
            let list_ty: Ty = Ty::List(list_arg.clone());
            st.add_constraint((ty_bound, list_ty.clone()));
            let ty_nil: Ty = clause_nil.rhs.gen_constraints(env, st)?;
            let var_head: &Variable =
                clause_cons
                    .vars
                    .first()
                    .ok_or(TypeError::PatternWrongNumOfArgs {
                        ctor: clause_cons.xtor.clone(),
                    })?;
            let var_tail: &Variable =
                clause_cons
                    .vars
                    .get(1)
                    .ok_or(TypeError::PatternWrongNumOfArgs {
                        ctor: clause_cons.xtor.clone(),
                    })?;
            let new_env: GenReader = env.add_var_bindings(vec![
                (var_head.clone(), *list_arg),
                (var_tail.clone(), list_ty),
            ]);
            let ty_cons: Ty = clause_cons.rhs.gen_constraints(&new_env, st)?;
            st.add_constraint((ty_nil.clone(), ty_cons));
            Ok(ty_nil)
        } else {
            Err(TypeError::InvalidCase)
        }
    }
}

impl GenConstraint for Cocase {
    fn gen_constraints(&self, env: &GenReader, st: &mut GenState) -> Result<Ty, TypeError> {
        if self.cocases.len() == 2 {
            let clause1: &Clause<Dtor> = self.cocases.first().ok_or(TypeError::InvalidCocase)?;
            let _ = if clause1.vars.is_empty() {
                Ok("")
            } else {
                Err(TypeError::InvalidCocase)
            }?;
            let clause2: &Clause<Dtor> = self.cocases.get(1).ok_or(TypeError::InvalidCocase)?;
            let _ = if clause1.vars.is_empty() {
                Ok("")
            } else {
                Err(TypeError::InvalidCocase)
            }?;
            let ty1: Ty = clause1.rhs.gen_constraints(env, st)?;
            let ty2: Ty = clause2.rhs.gen_constraints(env, st)?;
            if clause1.xtor == Dtor::Hd && clause2.xtor == Dtor::Tl {
                let str_ty: Ty = Ty::Stream(Box::new(ty1));
                st.add_constraint((str_ty.clone(), ty2));
                Ok(str_ty)
            } else if clause1.xtor == Dtor::Fst && clause2.xtor == Dtor::Snd {
                let pair_ty: Ty = Ty::LPair(Box::new(ty1), Box::new(ty2));
                Ok(pair_ty)
            } else {
                Err(TypeError::InvalidCocase)
            }
        } else {
            Err(TypeError::InvalidCocase)
        }
    }
}

impl GenConstraint for Lam {
    fn gen_constraints(&self, env: &GenReader, st: &mut GenState) -> Result<Ty, TypeError> {
        let ty_a: Ty = st.fresh_var();
        let new_env: GenReader = env.add_var_bindings(vec![(self.variable.clone(), ty_a.clone())]);
        let ty_body = self.body.gen_constraints(&new_env, st)?;
        Ok(Ty::Fun(Box::new(ty_a), Box::new(ty_body)))
    }
}

impl GenConstraint for App {
    fn gen_constraints(&self, env: &GenReader, st: &mut GenState) -> Result<Ty, TypeError> {
        let ty1: Ty = self.function.gen_constraints(env, st)?;
        let ty2: Ty = self.argument.gen_constraints(env, st)?;
        let ret_ty: Ty = st.fresh_var();
        st.add_constraint((ty1, Ty::Fun(Box::new(ty2), Box::new(ret_ty.clone()))));
        Ok(ret_ty)
    }
}

impl GenConstraint for Goto {
    fn gen_constraints(&self, env: &GenReader, st: &mut GenState) -> Result<Ty, TypeError> {
        let ty1: Ty = self.term.gen_constraints(env, st)?;
        let (_, ty2): (&String, &Ty) = env
            .gen_covars
            .iter()
            .find(|(cv1, _)| *self.target == **cv1)
            .ok_or(TypeError::CovarNotFound {
                name: self.target.clone(),
            })?;
        st.add_constraint((ty1, ty2.clone()));
        Ok(st.fresh_var())
    }
}

impl GenConstraint for Label {
    fn gen_constraints(&self, env: &GenReader, st: &mut GenState) -> Result<Ty, TypeError> {
        let ty_a: Ty = st.fresh_var();
        let new_env = env.add_covar_bindings(vec![(self.label.clone(), ty_a.clone())]);
        let ty: Ty = self.term.gen_constraints(&new_env, st)?;
        st.add_constraint((ty.clone(), ty_a));
        Ok(ty)
    }
}

impl GenConstraint for Term {
    fn gen_constraints(&self, env: &GenReader, st: &mut GenState) -> Result<Ty, TypeError> {
        match self {
            Term::Var(v) => match env.gen_vars.get(v) {
                None => Err(TypeError::VarNotFound { name: v.clone() }),
                Some(ty) => Ok(ty.clone()),
            },
            Term::Lit(_) => Ok(Ty::Int()),
            Term::Op(o) => o.gen_constraints(env, st),
            Term::IfZ(i) => i.gen_constraints(env, st),
            Term::Let(l) => l.gen_constraints(env, st),
            Term::Fun(f) => f.gen_constraints(env, st),
            Term::Constructor(c) => c.gen_constraints(env, st),
            Term::Case(c) => c.gen_constraints(env, st),
            Term::Destructor(d) => d.gen_constraints(env, st),
            Term::Cocase(c) => c.gen_constraints(env, st),
            Term::Lam(l) => l.gen_constraints(env, st),
            Term::App(a) => a.gen_constraints(env, st),
            Term::Goto(g) => g.gen_constraints(env, st),
            Term::Label(l) => l.gen_constraints(env, st),
            Term::Paren(t) => t.inner.gen_constraints(env, st),
        }
    }
}

fn gen_constraints_def(def: &Def<Ty>, env: &GenReader, st: &mut GenState) -> Result<(), TypeError> {
    let env_with_vars: GenReader = env.add_var_bindings(def.args.clone());
    let env_with_covars: GenReader = env_with_vars.add_covar_bindings(def.cont.clone());
    let ty: Ty = def.body.gen_constraints(&env_with_covars, st)?;
    st.add_constraint((ty, def.ret_ty.clone()));
    Ok(())
}

fn annotate_program(prog: Prog<()>) -> Prog<Ty> {
    let mut var_cnt = 0;
    let mut fresh = || {
        var_cnt += 1;
        Ty::Var(format!("b{}", var_cnt))
    };
    Prog {
        prog_defs: prog
            .prog_defs
            .into_iter()
            .map(|def| Def {
                name: def.name,
                args: def.args.into_iter().map(|(v, _)| (v, fresh())).collect(),
                cont: def.cont.into_iter().map(|(cv, _)| (cv, fresh())).collect(),
                body: def.body,
                ret_ty: fresh(),
            })
            .collect(),
    }
}

pub fn generate_constraints(prog: Prog<()>) -> Result<(Prog<Ty>, Vec<Constraint>), TypeError> {
    let prog_annot: Prog<Ty> = annotate_program(prog);
    let initial_reader: GenReader = GenReader {
        gen_vars: HashMap::new(),
        gen_covars: HashMap::new(),
        gen_prog: &prog_annot,
    };
    let mut initial_state: GenState = GenState {
        varcnt: 0,
        constraints: vec![],
    };

    let _: Vec<()> = prog_annot
        .prog_defs
        .iter()
        .map(|def| gen_constraints_def(def, &initial_reader, &mut initial_state))
        .collect::<Result<Vec<()>, TypeError>>()?;
    Ok((prog_annot, initial_state.constraints))
}

#[cfg(test)]
mod generate_tests {
    use super::{Def, GenConstraint, GenReader, GenState, Prog, Ty};
    use crate::syntax::{
        App, BinOp, Case, Clause, Cocase, Constructor, Ctor, Destructor, Dtor, Fun, Goto, IfZ,
        Label, Lam, Let, Op, Paren, Term,
    };
    use std::collections::HashMap;
    use std::rc::Rc;

    fn example_var() -> Term {
        Term::Var("x".to_owned())
    }

    #[test]
    fn gen_constraints_var_err() {
        let prog: Prog<Ty> = Prog { prog_defs: vec![] };
        let reader = GenReader {
            gen_vars: HashMap::new(),
            gen_covars: HashMap::new(),
            gen_prog: &prog,
        };
        let mut state = GenState {
            varcnt: 0,
            constraints: vec![],
        };
        let res = example_var().gen_constraints(&reader, &mut state);
        assert!(res.is_err())
    }

    #[test]
    fn gen_constraints_var_ok() {
        let prog: Prog<Ty> = Prog { prog_defs: vec![] };
        let mut gen_vars = HashMap::new();
        gen_vars.insert("x".to_owned(), Ty::Int());
        let reader = GenReader {
            gen_vars,
            gen_covars: HashMap::new(),
            gen_prog: &prog,
        };
        let mut state = GenState {
            varcnt: 0,
            constraints: vec![],
        };
        let res = example_var().gen_constraints(&reader, &mut state);
        assert_eq!(res, Ok(Ty::Int()));
        assert_eq!(state.varcnt, 0);
        assert!(state.constraints.is_empty())
    }

    fn example_lit() -> Term {
        Term::Lit(1)
    }

    #[test]
    fn gen_constraints_lit() {
        let prog: Prog<Ty> = Prog { prog_defs: vec![] };
        let reader = GenReader {
            gen_vars: HashMap::new(),
            gen_covars: HashMap::new(),
            gen_prog: &prog,
        };
        let mut state = GenState {
            varcnt: 0,
            constraints: vec![],
        };
        let res = example_lit().gen_constraints(&reader, &mut state);
        assert_eq!(res, Ok(Ty::Int()))
    }

    fn example_op() -> Op {
        Op {
            fst: Rc::new(Term::Lit(1)),
            op: BinOp::Sub,
            snd: Rc::new(Term::Lit(1)),
        }
    }

    #[test]
    fn gen_constraints_op() {
        let prog: Prog<Ty> = Prog { prog_defs: vec![] };
        let reader = GenReader {
            gen_vars: HashMap::new(),
            gen_covars: HashMap::new(),
            gen_prog: &prog,
        };
        let mut state = GenState {
            varcnt: 0,
            constraints: vec![],
        };
        let res = example_op().gen_constraints(&reader, &mut state);
        assert_eq!(res, Ok(Ty::Int()));
        assert_eq!(state.varcnt, 0);
        assert_eq!(
            state.constraints,
            vec![(Ty::Int(), Ty::Int()), (Ty::Int(), Ty::Int())]
        )
    }

    fn example_ifz() -> IfZ {
        IfZ {
            ifc: Rc::new(Term::Lit(2)),
            thenc: Rc::new(Term::Lit(1)),
            elsec: Rc::new(Term::Lit(3)),
        }
    }

    #[test]
    fn gen_constraints_ifz() {
        let prog: Prog<Ty> = Prog { prog_defs: vec![] };
        let reader = GenReader {
            gen_vars: HashMap::new(),
            gen_covars: HashMap::new(),
            gen_prog: &prog,
        };
        let mut state = GenState {
            varcnt: 0,
            constraints: vec![],
        };
        let res = example_ifz().gen_constraints(&reader, &mut state);
        assert_eq!(res, Ok(Ty::Int()));
        assert_eq!(state.varcnt, 0);
        assert_eq!(
            state.constraints,
            vec![(Ty::Int(), Ty::Int()), (Ty::Int(), Ty::Int())]
        )
    }

    fn example_let() -> Let {
        Let {
            variable: "x".to_owned(),
            bound_term: Rc::new(Term::Lit(1)),
            in_term: Rc::new(Term::Var("x".to_owned())),
        }
    }

    #[test]
    fn gen_constraints_let() {
        let prog: Prog<Ty> = Prog { prog_defs: vec![] };
        let reader = GenReader {
            gen_vars: HashMap::new(),
            gen_covars: HashMap::new(),
            gen_prog: &prog,
        };
        let mut state = GenState {
            varcnt: 0,
            constraints: vec![],
        };
        let res = example_let().gen_constraints(&reader, &mut state);
        assert_eq!(res, Ok(Ty::Int()));
        assert_eq!(state.varcnt, 0);
        assert_eq!(state.constraints, vec![])
    }

    fn example_fun() -> Fun {
        Fun {
            name: "main".to_owned(),
            args: vec![Term::Var("x".to_owned())],
            coargs: vec![],
        }
    }

    #[test]
    fn gen_constraints_fun1() {
        let prog: Prog<Ty> = Prog {
            prog_defs: vec![Def {
                name: "main".to_owned(),
                args: vec![("x".to_owned(), Ty::Int())],
                cont: vec![],
                body: Term::Lit(1),
                ret_ty: Ty::Int(),
            }],
        };
        let mut gen_vars = HashMap::new();
        gen_vars.insert("x".to_owned(), Ty::Int());
        let reader = GenReader {
            gen_vars,
            gen_covars: HashMap::new(),
            gen_prog: &prog,
        };
        let mut state = GenState {
            varcnt: 0,
            constraints: vec![],
        };
        let res = example_fun().gen_constraints(&reader, &mut state);
        assert_eq!(res, Ok(Ty::Int()));
        assert_eq!(state.varcnt, 0);
        assert_eq!(state.constraints, vec![(Ty::Int(), Ty::Int())])
    }

    #[test]
    fn gen_constraints_fun2() {
        let prog: Prog<Ty> = Prog {
            prog_defs: vec![Def {
                name: "main".to_owned(),
                args: vec![],
                cont: vec![],
                body: Term::Lit(1),
                ret_ty: Ty::Int(),
            }],
        };
        let mut gen_vars = HashMap::new();
        gen_vars.insert("x".to_owned(), Ty::Int());
        let reader = GenReader {
            gen_vars,
            gen_covars: HashMap::new(),
            gen_prog: &prog,
        };
        let mut state = GenState {
            varcnt: 0,
            constraints: vec![],
        };
        let res = example_fun().gen_constraints(&reader, &mut state);
        assert!(res.is_err())
    }

    fn example_ctor() -> Constructor {
        Constructor {
            id: Ctor::Nil,
            args: vec![],
        }
    }

    #[test]
    fn gen_constraints_ctor() {
        let prog: Prog<Ty> = Prog { prog_defs: vec![] };
        let reader = GenReader {
            gen_vars: HashMap::new(),
            gen_covars: HashMap::new(),
            gen_prog: &prog,
        };
        let mut state = GenState {
            varcnt: 0,
            constraints: vec![],
        };
        let res = example_ctor().gen_constraints(&reader, &mut state);
        assert_eq!(res, Ok(Ty::List(Box::new(Ty::Var("0".to_owned())))));
        assert_eq!(state.varcnt, 1);
        assert_eq!(state.constraints, vec![])
    }

    fn example_dtor() -> Destructor {
        Destructor {
            id: Dtor::Hd,
            args: vec![],
            destructee: Rc::new(Term::Var("x".to_owned())),
        }
    }

    #[test]
    fn gen_constraints_dtor() {
        let prog: Prog<Ty> = Prog { prog_defs: vec![] };
        let mut gen_vars = HashMap::new();
        gen_vars.insert("x".to_owned(), Ty::Stream(Box::new(Ty::Int())));
        let reader = GenReader {
            gen_vars,
            gen_covars: HashMap::new(),
            gen_prog: &prog,
        };
        let mut state = GenState {
            varcnt: 0,
            constraints: vec![],
        };
        let res = example_dtor().gen_constraints(&reader, &mut state);
        assert_eq!(res, Ok(Ty::Var("0".to_owned())));
        assert_eq!(state.varcnt, 1);
        assert_eq!(
            state.constraints,
            vec![(
                Ty::Stream(Box::new(Ty::Int())),
                Ty::Stream(Box::new(Ty::Var("0".to_owned())))
            )]
        )
    }

    fn example_case() -> Case {
        Case {
            destructee: Rc::new(Term::Var("x".to_owned())),
            cases: vec![
                Clause {
                    xtor: Ctor::Nil,
                    vars: vec![],
                    rhs: Term::Lit(1),
                },
                Clause {
                    xtor: Ctor::Cons,
                    vars: vec!["x".to_owned(), "xs".to_owned()],
                    rhs: Term::Var("x".to_owned()),
                },
            ],
        }
    }

    #[test]
    fn gen_constraints_case() {
        let prog: Prog<Ty> = Prog { prog_defs: vec![] };
        let mut gen_vars = HashMap::new();
        gen_vars.insert("x".to_owned(), Ty::List(Box::new(Ty::Int())));
        let reader = GenReader {
            gen_vars,
            gen_covars: HashMap::new(),
            gen_prog: &prog,
        };
        let mut state = GenState {
            varcnt: 0,
            constraints: vec![],
        };
        let res = example_case().gen_constraints(&reader, &mut state);
        assert_eq!(res, Ok(Ty::Int()));
        assert_eq!(state.varcnt, 1);
        assert_eq!(
            state.constraints,
            vec![
                (
                    Ty::List(Box::new(Ty::Int())),
                    Ty::List(Box::new(Ty::Var("0".to_owned())))
                ),
                (Ty::Int(), Ty::Var("0".to_owned()))
            ]
        )
    }

    fn example_cocase() -> Cocase {
        Cocase {
            cocases: vec![
                Clause {
                    xtor: Dtor::Fst,
                    vars: vec![],
                    rhs: Term::Lit(1),
                },
                Clause {
                    xtor: Dtor::Snd,
                    vars: vec![],
                    rhs: Term::Lit(2),
                },
            ],
        }
    }

    #[test]
    fn gen_constraints_cocase() {
        let prog: Prog<Ty> = Prog { prog_defs: vec![] };
        let reader = GenReader {
            gen_vars: HashMap::new(),
            gen_covars: HashMap::new(),
            gen_prog: &prog,
        };
        let mut state = GenState {
            varcnt: 0,
            constraints: vec![],
        };
        let res = example_cocase().gen_constraints(&reader, &mut state);
        assert_eq!(res, Ok(Ty::LPair(Box::new(Ty::Int()), Box::new(Ty::Int()))));
        assert_eq!(state.varcnt, 0);
        assert_eq!(state.constraints, vec![])
    }

    fn example_lam() -> Lam {
        Lam {
            variable: "x".to_owned(),
            body: Rc::new(Term::Var("x".to_owned())),
        }
    }

    #[test]
    fn gen_constraints_lam() {
        let prog: Prog<Ty> = Prog { prog_defs: vec![] };
        let reader = GenReader {
            gen_vars: HashMap::new(),
            gen_covars: HashMap::new(),
            gen_prog: &prog,
        };
        let mut state = GenState {
            varcnt: 0,
            constraints: vec![],
        };
        let res = example_lam().gen_constraints(&reader, &mut state);
        assert_eq!(
            res,
            Ok(Ty::Fun(
                Box::new(Ty::Var("0".to_owned())),
                Box::new(Ty::Var("0".to_owned()))
            ))
        );
        assert_eq!(state.varcnt, 1);
        assert_eq!(state.constraints, vec![])
    }

    fn example_app() -> App {
        App {
            function: Rc::new(Term::Var("x".to_owned())),
            argument: Rc::new(Term::Lit(1)),
        }
    }

    #[test]
    fn gen_constraints_app() {
        let prog: Prog<Ty> = Prog { prog_defs: vec![] };
        let mut gen_vars = HashMap::new();
        gen_vars.insert(
            "x".to_owned(),
            Ty::Fun(Box::new(Ty::Int()), Box::new(Ty::Int())),
        );
        let reader = GenReader {
            gen_vars,
            gen_covars: HashMap::new(),
            gen_prog: &prog,
        };
        let mut state = GenState {
            varcnt: 0,
            constraints: vec![],
        };
        let res = example_app().gen_constraints(&reader, &mut state);
        assert_eq!(res, Ok(Ty::Var("0".to_owned())));
        assert_eq!(state.varcnt, 1);
        assert_eq!(
            state.constraints,
            vec![(
                Ty::Fun(Box::new(Ty::Int()), Box::new(Ty::Int())),
                Ty::Fun(Box::new(Ty::Int()), Box::new(Ty::Var("0".to_owned())))
            )]
        )
    }

    fn example_goto() -> Goto {
        Goto {
            term: Rc::new(Term::Var("x".to_owned())),
            target: "a".to_owned(),
        }
    }

    #[test]
    fn gen_constraints_goto() {
        let prog: Prog<Ty> = Prog { prog_defs: vec![] };
        let mut gen_vars = HashMap::new();
        gen_vars.insert("x".to_owned(), Ty::Int());
        let mut gen_covars = HashMap::new();
        gen_covars.insert("a".to_owned(), Ty::Int());
        let reader = GenReader {
            gen_vars,
            gen_covars,
            gen_prog: &prog,
        };
        let mut state = GenState {
            varcnt: 0,
            constraints: vec![],
        };
        let res = example_goto().gen_constraints(&reader, &mut state);
        assert_eq!(res, Ok(Ty::Var("0".to_owned())));
        assert_eq!(state.varcnt, 1);
        assert_eq!(state.constraints, vec![(Ty::Int(), Ty::Int())])
    }

    fn example_label() -> Label {
        Label {
            label: "a".to_owned(),
            term: Rc::new(Term::Lit(1)),
        }
    }

    #[test]
    fn gen_constraints_label() {
        let prog: Prog<Ty> = Prog { prog_defs: vec![] };
        let reader = GenReader {
            gen_vars: HashMap::new(),
            gen_covars: HashMap::new(),
            gen_prog: &prog,
        };
        let mut state = GenState {
            varcnt: 0,
            constraints: vec![],
        };
        let res = example_label().gen_constraints(&reader, &mut state);
        assert_eq!(res, Ok(Ty::Int()));
        assert_eq!(state.varcnt, 1);
        assert_eq!(
            state.constraints,
            vec![(Ty::Int(), Ty::Var("0".to_owned()))]
        )
    }

    fn example_paren() -> Term {
        Paren {
            inner: Rc::new(Term::Lit(1)),
        }
        .into()
    }
    #[test]
    fn gen_constraints_paren() {
        let prog: Prog<Ty> = Prog { prog_defs: vec![] };
        let reader = GenReader {
            gen_vars: HashMap::new(),
            gen_covars: HashMap::new(),
            gen_prog: &prog,
        };
        let mut state = GenState {
            varcnt: 0,
            constraints: vec![],
        };
        let res = example_paren().gen_constraints(&reader, &mut state);
        assert_eq!(res, Ok(Ty::Int()));
        assert_eq!(state.varcnt, 0);
        assert_eq!(state.constraints, vec![])
    }

    #[test]
    fn lookup_test1() {
        let reader = GenReader {
            gen_vars: HashMap::new(),
            gen_covars: HashMap::new(),
            gen_prog: &Prog { prog_defs: vec![] },
        };
        assert!(reader.lookup_definition(&"main".to_owned()).is_err());
    }

    #[test]
    fn lookup_test2() {
        let main_def = Def {
            name: "main".to_owned(),
            args: vec![("x".to_owned(), Ty::Int())],
            cont: vec![("a".to_owned(), Ty::Int())],
            body: Term::Lit(1),
            ret_ty: Ty::Int(),
        };
        let reader = GenReader {
            gen_vars: HashMap::new(),
            gen_covars: HashMap::new(),
            gen_prog: &Prog {
                prog_defs: vec![main_def],
            },
        };
        let looked_up = reader.lookup_definition(&"main".to_owned());
        assert!(looked_up.is_ok());
        let (res_args, res_cont, res_ret) = looked_up.unwrap();
        assert_eq!(res_args, vec![Ty::Int()]);
        assert_eq!(res_cont, vec![Ty::Int()]);
        assert_eq!(res_ret, Ty::Int());
    }
}
