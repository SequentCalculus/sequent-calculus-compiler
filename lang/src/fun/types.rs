use std::collections::{HashMap, HashSet};
use std::fmt;
use std::rc::Rc;

use crate::fun::syntax::{Clause, Covariable, Ctor, Def, Dtor, Name, Prog, Term, Variable};

type Typevar = String;

#[derive(Debug, Clone)]
pub enum Ty {
    Tyvar(Typevar),
    Int(),
    List(Rc<Ty>),
    Stream(Rc<Ty>),
    Pair(Rc<Ty>, Rc<Ty>),
    LPair(Rc<Ty>, Rc<Ty>),
    Fun(Rc<Ty>, Rc<Ty>),
}
impl fmt::Display for Ty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Ty::Tyvar(v) => write!(f, "{}", v),
            Ty::Int() => write!(f, "Int"),
            Ty::List(ty) => write!(f, "List({})", ty),
            Ty::Stream(ty) => write!(f, "Stream({})", ty),
            Ty::Pair(ty1, ty2) => write!(f, "Pair({},{})", ty1, ty2),
            Ty::LPair(ty1, ty2) => write!(f, "LPair({},{})", ty1, ty2),
            Ty::Fun(ty1, ty2) => write!(f, "{} -> {}", ty1, ty2),
        }
    }
}

type Constraint = (Ty, Ty);

fn free_tyvars(ty: &Ty) -> HashSet<Typevar> {
    match ty {
        Ty::Tyvar(v) => HashSet::from([v.clone()]),
        Ty::Int() => HashSet::new(),
        Ty::List(ty) => free_tyvars(ty),
        Ty::Stream(ty) => free_tyvars(ty),
        Ty::Pair(ty1, ty2) => {
            let fr1: HashSet<Typevar> = free_tyvars(ty1);
            let fr2: HashSet<Typevar> = free_tyvars(ty2);
            fr1.union(&fr2).cloned().collect()
        }
        Ty::LPair(ty1, ty2) => {
            let fr1: HashSet<Typevar> = free_tyvars(ty1);
            let fr2: HashSet<Typevar> = free_tyvars(ty2);
            fr1.union(&fr2).cloned().collect()
        }
        Ty::Fun(ty1, ty2) => {
            let fr1: HashSet<Typevar> = free_tyvars(ty1);
            let fr2: HashSet<Typevar> = free_tyvars(ty2);
            fr1.union(&fr2).cloned().collect()
        }
    }
}

//---------------------------------------------------------------
//---------------------- Zonking --------------------------------
//---------------------------------------------------------------
trait Zonk {
    fn zonk(&self, varmap: &HashMap<Typevar, Ty>) -> Self;
}

impl Zonk for Ty {
    fn zonk(&self, varmap: &HashMap<Typevar, Ty>) -> Ty {
        match self {
            Ty::Tyvar(v) => match varmap.get(v) {
                None => Ty::Tyvar(v.clone()),
                Some(ty) => ty.clone(),
            },
            Ty::Int() => Ty::Int(),
            Ty::List(ty) => {
                let ty_zonked: Ty = Zonk::zonk(ty, varmap);
                Ty::List(Rc::new(ty_zonked))
            }
            Ty::Stream(ty) => {
                let ty_zonked: Ty = Zonk::zonk(ty, varmap);
                Ty::Stream(Rc::new(ty_zonked))
            }
            Ty::Pair(ty1, ty2) => {
                let ty1_zonked: Ty = Zonk::zonk(ty1, varmap);
                let ty2_zonked: Ty = Zonk::zonk(ty2, varmap);
                Ty::Pair(Rc::new(ty1_zonked), Rc::new(ty2_zonked))
            }
            Ty::LPair(ty1, ty2) => {
                let ty1_zonked: Ty = Zonk::zonk(ty1, varmap);
                let ty2_zonked: Ty = Zonk::zonk(ty2, varmap);
                Ty::LPair(Rc::new(ty1_zonked), Rc::new(ty2_zonked))
            }
            Ty::Fun(ty1, ty2) => {
                let ty1_zonked: Ty = Zonk::zonk(ty1, varmap);
                let ty2_zonked: Ty = Zonk::zonk(ty2, varmap);
                Ty::Fun(Rc::new(ty1_zonked), Rc::new(ty2_zonked))
            }
        }
    }
}

impl Zonk for Def<Ty> {
    fn zonk(&self, varmap: &HashMap<Typevar, Ty>) -> Def<Ty> {
        Def {
            name: self.name.clone(),
            args: self
                .args
                .iter()
                .map(|(v, ty)| (v.clone(), Zonk::zonk(ty, varmap)))
                .collect(),
            cont: self
                .cont
                .iter()
                .map(|(cv, ty)| (cv.clone(), Zonk::zonk(ty, varmap)))
                .collect(),
            body: self.body.clone(),
            ret_ty: Zonk::zonk(&self.ret_ty, varmap),
        }
    }
}

impl Zonk for Prog<Ty> {
    fn zonk(&self, varmap: &HashMap<Typevar, Ty>) -> Prog<Ty> {
        Prog {
            prog_defs: self
                .prog_defs
                .iter()
                .map(|def| Zonk::zonk(def, varmap))
                .collect(),
        }
    }
}

impl Zonk for Constraint {
    fn zonk(&self, varmap: &HashMap<Typevar, Ty>) -> Constraint {
        (Zonk::zonk(&self.0, varmap), Zonk::zonk(&self.1, varmap))
    }
}

impl Zonk for HashMap<Typevar, Ty> {
    fn zonk(&self, varmap: &HashMap<Typevar, Ty>) -> HashMap<Typevar, Ty> {
        self.iter()
            .map(|(var, ty)| (var.clone(), Zonk::zonk(ty, varmap)))
            .collect::<HashMap<Typevar, Ty>>()
    }
}

//---------------------------------------------------------------
//--------------- Constraint Generation -------------------------
//---------------------------------------------------------------

pub type Error = String;

struct GenReader {
    gen_vars: HashMap<Variable, Ty>,
    gen_covars: HashMap<Covariable, Ty>,
    gen_defs: Prog<Ty>,
}

impl GenReader {
    fn add_var_bindings(&self, new_bindings: Vec<(Variable, Ty)>) -> GenReader {
        let new_map: HashMap<Variable, Ty> = new_bindings
            .iter()
            .map(|(var, ty)| (var.clone(), ty.clone()))
            .collect();
        let new_gen_vars: HashMap<Variable, Ty> =
            self.gen_vars.clone().into_iter().chain(new_map).collect();
        GenReader {
            gen_vars: new_gen_vars,
            gen_covars: self.gen_covars.clone(),
            gen_defs: self.gen_defs.clone(),
        }
    }

    fn add_covar_bindings(&self, new_bindings: Vec<(Covariable, Ty)>) -> GenReader {
        let new_map: HashMap<Covariable, Ty> = new_bindings
            .iter()
            .map(|(covar, ty)| (covar.clone(), ty.clone()))
            .collect();
        let new_gen_covars: HashMap<Covariable, Ty> =
            self.gen_covars.clone().into_iter().chain(new_map).collect();
        GenReader {
            gen_vars: self.gen_vars.clone(),
            gen_covars: new_gen_covars,
            gen_defs: self.gen_defs.clone(),
        }
    }

    fn lookup_definition(&self, nm: &Name) -> Result<(Vec<Ty>, Vec<Ty>, Ty), Error> {
        match &self.gen_defs.prog_defs.iter().find(|df| df.name == *nm) {
            None => Err(format!(
                "A top-level function named {} is not contained in the program.",
                nm
            )),
            Some(df) => {
                let arg_tys = df.args.iter().map(|(_, x)| x.clone()).collect();
                let cont_tys = df.cont.iter().map(|(_, x)| x.clone()).collect();
                Ok((arg_tys, cont_tys, df.ret_ty.clone()))
            }
        }
    }
}

struct GenState {
    varcnt: i64,
    ctrs: Vec<Constraint>,
}

impl GenState {
    fn fresh_var(&mut self) -> Ty {
        let new_var: String = format!("{}", self.varcnt);
        self.varcnt += self.varcnt;
        Ty::Tyvar(new_var)
    }

    fn add_constraint(&mut self, ctr: Constraint) {
        self.ctrs.push(ctr)
    }
}

fn gen_constraints_term(t: &Term, env: &GenReader, st: &mut GenState) -> Result<Ty, Error> {
    match t {
        Term::Var(v) => match env.gen_vars.get(v) {
            None => Err(format!("Variable {} not bound in environment", v)),
            Some(ty) => Ok(ty.clone()),
        },
        Term::Lit(_) => Ok(Ty::Int()),
        Term::Op(t1, _, t2) => {
            let ty1 = gen_constraints_term(t1, env, st)?;
            let ty2 = gen_constraints_term(t2, env, st)?;
            st.add_constraint((ty1, Ty::Int()));
            st.add_constraint((ty2, Ty::Int()));
            Ok(Ty::Int())
        }
        Term::IfZ(t1, t2, t3) => {
            let ty1 = gen_constraints_term(t1, env, st)?;
            let ty2 = gen_constraints_term(t2, env, st)?;
            let ty3 = gen_constraints_term(t3, env, st)?;
            st.add_constraint((ty1, Ty::Int()));
            st.add_constraint((ty2.clone(), ty3));
            Ok(ty2)
        }
        Term::Let(x, xdef, t) => {
            let ty = gen_constraints_term(xdef, env, st)?;
            let new_reader: GenReader = env.add_var_bindings(vec![(x.clone(), ty)]);
            gen_constraints_term(t, &new_reader, st)
        }
        Term::Fun(nm, args, coargs) => {
            let (arg_tys, coarg_tys, ret_ty) = env.lookup_definition(nm)?;
            if args.len() != arg_tys.len() || coargs.len() != coarg_tys.len() {
                Err(format!(
                    "{} called with wrong number of arguments. Expected: {} + {} Got: {} {}",
                    nm,
                    arg_tys.len(),
                    coarg_tys.len(),
                    args.len(),
                    coargs.len()
                ))
            } else {
                let arg_tys1: Vec<Ty> = args
                    .iter()
                    .map(|x| gen_constraints_term(x, env, st))
                    .collect::<Result<Vec<Ty>, Error>>()?;
                let args_zipped = arg_tys1.iter().cloned().zip(arg_tys);
                for (arg_ty, arg_ty_def) in args_zipped {
                    st.add_constraint((arg_ty, arg_ty_def));
                }
                let coargs_zipped = coargs.iter().cloned().zip(coarg_tys);
                for (coarg, coarg_ty) in coargs_zipped {
                    match env.gen_covars.iter().find(|(cv, _)| **cv == coarg) {
                        None => Err(format!("Variable {} not found in environment", coarg)),
                        Some((_, ty)) => {
                            st.add_constraint((ty.clone(), coarg_ty));
                            Ok(())
                        }
                    }?;
                }
                Ok(ret_ty.clone())
            }
        }
        Term::Constructor(Ctor::Nil, args) if args.is_empty() => Ok(st.fresh_var()),
        Term::Constructor(Ctor::Cons, args) => {
            let arg1: &Rc<Term> = args
                .first()
                .ok_or(format!("Wrong number of arguments for {}", Ctor::Cons))?;
            let arg2: &Rc<Term> = args
                .get(1)
                .ok_or(format!("Wrong number of arguments for {}", Ctor::Cons))?;
            if args.len() > 2 {
                Err(format!("Wrong number of arguments for {}", Ctor::Cons))
            } else {
                let ty1: Ty = gen_constraints_term(arg1, env, st)?;
                let ty2: Ty = gen_constraints_term(arg2, env, st)?;
                st.add_constraint((Ty::List(Rc::new(ty1)), ty2.clone()));
                Ok(ty2)
            }
        }
        Term::Constructor(Ctor::Tup, args) => {
            let arg1: &Rc<Term> = args
                .first()
                .ok_or(format!("Wrong number of arguments for {}", Ctor::Tup))?;
            let arg2: &Rc<Term> = args
                .get(2)
                .ok_or(format!("Wrong number of arguments for {}", Ctor::Tup))?;
            if args.len() > 2 {
                Err(format!("Wrong number of arguments for {}", Ctor::Cons))
            } else {
                let ty1: Ty = gen_constraints_term(arg1, env, st)?;
                let ty2: Ty = gen_constraints_term(arg2, env, st)?;
                Ok(Ty::Pair(Rc::new(ty1), Rc::new(ty2)))
            }
        }
        Term::Constructor(ctor, _) => Err(format!(
            "Constructor {} aplied to wrong number of arguments",
            ctor
        )),
        //List
        t @ Term::Case(t_bound, pts) if pts.len() == 2 => {
            let pt_nil: &Rc<Clause<Ctor>> = pts
                .iter()
                .find(|pt| pt.xtor == Ctor::Nil)
                .ok_or(format!("Invalid case expression: {}", t))?;
            let pt_cons: &Rc<Clause<Ctor>> = pts
                .iter()
                .find(|pt| pt.xtor == Ctor::Cons)
                .ok_or(format!("Invalid case expression: {}", t))?;
            let ty_bound: Ty = gen_constraints_term(t_bound, env, st)?;
            let list_arg: Rc<Ty> = Rc::new(st.fresh_var());
            let list_ty: Ty = Ty::List(list_arg.clone());
            st.add_constraint((ty_bound, list_ty.clone()));
            let ty_nil: Ty = gen_constraints_term(&pt_nil.rhs, env, st)?;
            let pt_x: &Variable = pt_cons.vars.first().ok_or(format!(
                "Wrong number of bound variables for {}",
                pt_cons.xtor
            ))?;
            let pt_xs: &Variable = pt_cons.vars.get(1).ok_or(format!(
                "Wrong number of bound variables for {}",
                pt_cons.xtor
            ))?;
            let new_env: GenReader = env.add_var_bindings(vec![
                (pt_x.clone(), Rc::unwrap_or_clone(list_arg)),
                (pt_xs.clone(), list_ty),
            ]);
            let ty_cons: Ty = gen_constraints_term(&pt_cons.rhs, &new_env, st)?;
            st.add_constraint((ty_nil.clone(), ty_cons));
            Ok(ty_nil)
        }
        // Tup
        t @ Term::Case(t_bound, pts) if pts.len() == 1 => {
            let pt_tup: &Rc<Clause<Ctor>> = pts
                .first()
                .ok_or(format!("Invalid case expression: {}", t))?;
            let ty_bound: Ty = gen_constraints_term(t_bound, env, st)?;
            let ty_a: Ty = st.fresh_var();
            let ty_b: Ty = st.fresh_var();
            st.add_constraint((
                ty_bound,
                Ty::Pair(Rc::new(ty_a.clone()), Rc::new(ty_b.clone())),
            ));
            let pt_x: &Variable = pt_tup.vars.first().ok_or(format!(
                "Wrong number of bound variables for {}",
                pt_tup.xtor
            ))?;
            let pt_y: &Variable = pt_tup.vars.get(1).ok_or(format!(
                "Wrong number of bound variables for {}",
                pt_tup.xtor
            ))?;
            let new_env: GenReader =
                env.add_var_bindings(vec![(pt_x.clone(), ty_a), (pt_y.clone(), ty_b)]);
            gen_constraints_term(&pt_tup.rhs, &new_env, st)
        }
        t @ Term::Case(_, _) => Err(format!("Invalid case expression: {}", t)),
        Term::Destructor(t, Dtor::Hd, args) if args.is_empty() => {
            let ty_bound: Ty = gen_constraints_term(t, env, st)?;
            let ty_a: Ty = st.fresh_var();
            st.add_constraint((ty_bound, Ty::Stream(Rc::new(ty_a.clone()))));
            Ok(ty_a)
        }
        Term::Destructor(t, Dtor::Tl, args) if args.is_empty() => {
            let ty_bound: Ty = gen_constraints_term(t, env, st)?;
            let ty_str: Ty = Ty::Stream(Rc::new(st.fresh_var()));
            st.add_constraint((ty_bound, ty_str.clone()));
            Ok(ty_str)
        }
        Term::Destructor(t, Dtor::Fst, args) if args.is_empty() => {
            let ty_bound: Ty = gen_constraints_term(t, env, st)?;
            let ty_a: Ty = st.fresh_var();
            let ty_b: Ty = st.fresh_var();
            st.add_constraint((ty_bound, Ty::LPair(Rc::new(ty_a.clone()), Rc::new(ty_b))));
            Ok(ty_a)
        }
        Term::Destructor(t, Dtor::Snd, args) if args.is_empty() => {
            let ty_bound: Ty = gen_constraints_term(t, env, st)?;
            let ty_a: Ty = st.fresh_var();
            let ty_b: Ty = st.fresh_var();
            st.add_constraint((ty_bound, Ty::LPair(Rc::new(ty_a), Rc::new(ty_b.clone()))));
            Ok(ty_b)
        }
        Term::Destructor(_, dtor, _) => Err(format!(
            "Destructor {} called with wrong number of arguments",
            dtor
        )),
        t @ Term::Cocase(pts) if pts.len() == 2 => {
            let err_str = format!("Invalid cocase expression {}", t);
            let pt1: &Rc<Clause<Dtor>> = pts.first().ok_or(err_str.clone())?;
            let _ = if pt1.vars.is_empty() {
                Ok("")
            } else {
                Err(err_str.clone())
            }?;
            let pt2: &Rc<Clause<Dtor>> = pts.get(1).ok_or(err_str.clone())?;
            let _ = if pt1.vars.is_empty() {
                Ok("")
            } else {
                Err(err_str.clone())
            }?;
            let ty1: Ty = gen_constraints_term(&pt1.rhs, env, st)?;
            let ty2: Ty = gen_constraints_term(&pt2.rhs, env, st)?;
            if pt1.xtor == Dtor::Hd && pt2.xtor == Dtor::Tl {
                let str_ty: Ty = Ty::Stream(Rc::new(ty1));
                st.add_constraint((str_ty.clone(), ty2));
                Ok(str_ty)
            } else if pt1.xtor == Dtor::Fst && pt2.xtor == Dtor::Snd {
                let pair_ty: Ty = Ty::LPair(Rc::new(ty1), Rc::new(ty2));
                Ok(pair_ty)
            } else {
                Err(err_str)
            }
        }
        t @ Term::Cocase(_) => Err(format!("Invalid cocase expression {}", t)),
        Term::Lam(v, body) => {
            let ty_a: Ty = st.fresh_var();
            let new_env: GenReader = env.add_var_bindings(vec![(v.clone(), ty_a.clone())]);
            let ty_body = gen_constraints_term(body, &new_env, st)?;
            Ok(Ty::Fun(Rc::new(ty_a), Rc::new(ty_body)))
        }
        Term::App(t1, t2) => {
            let ty1: Ty = gen_constraints_term(t1, env, st)?;
            let ty2: Ty = gen_constraints_term(t2, env, st)?;
            let ret_ty: Ty = st.fresh_var();
            st.add_constraint((ty1, Ty::Fun(Rc::new(ty2), Rc::new(ret_ty.clone()))));
            Ok(ret_ty)
        }
        Term::Goto(t, cv) => {
            let ty1: Ty = gen_constraints_term(t, env, st)?;
            let (_, ty2): (&String, &Ty) = env
                .gen_covars
                .iter()
                .find(|(cv1, _)| **cv == **cv1)
                .ok_or(format!("Covariable {} not bound in environment", cv))?;
            st.add_constraint((ty1, ty2.clone()));
            Ok(st.fresh_var())
        }
        Term::Label(cv, t) => {
            let ty_a: Ty = st.fresh_var();
            env.add_covar_bindings(vec![(cv.clone(), ty_a.clone())]);
            let ty: Ty = gen_constraints_term(t, env, st)?;
            st.add_constraint((ty.clone(), ty_a));
            Ok(ty)
        }
    }
}

fn gen_constraints_def(def: &Def<Ty>, env: &mut GenReader, st: &mut GenState) -> Result<(), Error> {
    let env_with_vars: GenReader = env.add_var_bindings(def.args.clone());
    let env_with_covars: GenReader = env_with_vars.add_covar_bindings(def.cont.clone());
    let ty: Ty = gen_constraints_term(&def.body, &env_with_covars, st)?;
    st.add_constraint((ty, def.ret_ty.clone()));
    Ok(())
}

fn annotate_program(prog: Prog<()>) -> Prog<Ty> {
    let mut var_cnt = 0;
    let mut fresh = || {
        var_cnt += 1;
        Ty::Tyvar(format!("b{}", var_cnt))
    };
    Prog {
        prog_defs: prog
            .prog_defs
            .iter()
            .map(|def| Def {
                name: def.name.clone(),
                args: def.args.iter().map(|(v, _)| (v.clone(), fresh())).collect(),
                cont: def
                    .cont
                    .iter()
                    .map(|(cv, _)| (cv.clone(), fresh()))
                    .collect(),
                body: def.body.clone(),
                ret_ty: fresh(),
            })
            .collect(),
    }
}

fn generate_constraints(prog: Prog<()>) -> Result<(Prog<Ty>, Vec<Constraint>), Error> {
    let prog_annot: Prog<Ty> = annotate_program(prog);
    let mut initial_reader: GenReader = GenReader {
        gen_vars: HashMap::new(),
        gen_covars: HashMap::new(),
        gen_defs: prog_annot.clone(),
    };
    let mut initial_state: GenState = GenState {
        varcnt: 0,
        ctrs: vec![],
    };

    let _: Vec<()> = prog_annot
        .prog_defs
        .iter()
        .map(|df| gen_constraints_def(df, &mut initial_reader, &mut initial_state))
        .collect::<Result<Vec<()>, Error>>()?;
    Ok((prog_annot, initial_state.ctrs))
}

//---------------------------------------------------------------
//---------------- Constraint Solving ---------------------------
//---------------------------------------------------------------

struct SolverState {
    todo: Vec<Constraint>,
    subst: HashMap<Typevar, Ty>,
}

impl SolverState {
    fn add_constraints(&mut self, new_ctrs: Vec<Constraint>) {
        self.todo.extend(new_ctrs);
    }
}

fn solve_constraints(ctrs: Vec<Constraint>) -> Result<HashMap<Typevar, Ty>, Error> {
    let mut initial = SolverState {
        todo: ctrs,
        subst: HashMap::new(),
    };
    run(&mut initial)?;
    Ok(initial.subst)
}

fn perform_subst(var: Typevar, ty: Ty, st: &mut SolverState) {
    let m: HashMap<Variable, Ty> = HashMap::from([(var, ty)]);
    let new_todo: Vec<Constraint> = st.todo.iter().map(|ctr| Zonk::zonk(ctr, &m)).collect();
    let mut new_subst: HashMap<String, Ty> = Zonk::zonk(&st.subst, &m);
    new_subst.extend(m);
    st.subst = new_subst;
    st.todo = new_todo;
}

fn run(st: &mut SolverState) -> Result<(), Error> {
    if st.todo.is_empty() {
        Ok(())
    } else {
        let next_ctr: Constraint = st.todo.remove(0);
        solve_constraint(next_ctr, st)?;
        run(st)
    }
}

fn solve_constraint(ctr: Constraint, st: &mut SolverState) -> Result<(), Error> {
    match ctr {
        (Ty::Tyvar(a), Ty::Tyvar(b)) if a == b => Ok(()),
        (Ty::Tyvar(a), ty) => {
            if free_tyvars(&ty).contains(&a) {
                Err(format!("Occurs check! {} occurs in {}", a, ty))
            } else {
                perform_subst(a, ty, st);
                Ok(())
            }
        }
        (ty, Ty::Tyvar(a)) => {
            if free_tyvars(&ty).contains(&a) {
                Err(format!("Occurs check! {} occurs in {}", a, ty))
            } else {
                perform_subst(a, ty, st);
                Ok(())
            }
        }
        (Ty::Int(), Ty::Int()) => Ok(()),
        (Ty::List(ty1), Ty::List(ty2)) => {
            st.add_constraints(vec![(Rc::unwrap_or_clone(ty1), Rc::unwrap_or_clone(ty2))]);
            Ok(())
        }
        (Ty::Pair(ty1, ty2), Ty::Pair(ty3, ty4)) => {
            st.add_constraints(vec![
                (Rc::unwrap_or_clone(ty1), Rc::unwrap_or_clone(ty3)),
                (Rc::unwrap_or_clone(ty2), Rc::unwrap_or_clone(ty4)),
            ]);
            Ok(())
        }
        (Ty::Stream(ty1), Ty::Stream(ty2)) => {
            st.add_constraints(vec![(Rc::unwrap_or_clone(ty1), Rc::unwrap_or_clone(ty2))]);
            Ok(())
        }
        (Ty::LPair(ty1, ty2), Ty::LPair(ty3, ty4)) => {
            st.add_constraints(vec![
                (Rc::unwrap_or_clone(ty1), Rc::unwrap_or_clone(ty3)),
                (Rc::unwrap_or_clone(ty2), Rc::unwrap_or_clone(ty4)),
            ]);
            Ok(())
        }
        (Ty::Fun(ty1, ty2), Ty::Fun(ty3, ty4)) => {
            st.add_constraints(vec![
                (Rc::unwrap_or_clone(ty1), Rc::unwrap_or_clone(ty3)),
                (Rc::unwrap_or_clone(ty2), Rc::unwrap_or_clone(ty4)),
            ]);
            Ok(())
        }
        (ty1, ty2) => Err(format!("Cannot unify types: {} and {}", ty1, ty2)),
    }
}

//---------------------------------------------------------------
//---------------- Type Inference -------------------------------
//---------------------------------------------------------------

pub fn infer_types(prog: Prog<()>) -> Result<Prog<Ty>, Error> {
    let (prog_typed, constrs): (Prog<Ty>, Vec<Constraint>) = generate_constraints(prog)?;
    let subst: HashMap<Typevar, Ty> = solve_constraints(constrs)?;
    let prog_zonked: Prog<Ty> = Zonk::zonk(&prog_typed, &subst);
    Ok(prog_zonked)
}
