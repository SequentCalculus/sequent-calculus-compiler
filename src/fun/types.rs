use std::collections::{HashMap, HashSet};
use std::fmt;
use std::rc::Rc;

use crate::fun::syntax::{Clause, Covariable, Ctor, Def, Dtor, Name, Prog, Term, Variable};

type Typevar = String;

#[derive(Debug, Clone)]
enum Ty {
    Tyvar(Typevar),
    IntTy(),
    ListTy(Rc<Ty>),
    StreamTy(Rc<Ty>),
    PairTy(Rc<Ty>, Rc<Ty>),
    LPairTy(Rc<Ty>, Rc<Ty>),
    FunTy(Rc<Ty>, Rc<Ty>),
}
impl fmt::Display for Ty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Ty::Tyvar(v) => write!(f, "{}", v),
            Ty::IntTy() => write!(f, "Int"),
            Ty::ListTy(ty) => write!(f, "List({})", ty),
            Ty::StreamTy(ty) => write!(f, "Stream({})", ty),
            Ty::PairTy(ty1, ty2) => write!(f, "Pair({},{})", ty1, ty2),
            Ty::LPairTy(ty1, ty2) => write!(f, "LPairTy({},{})", ty1, ty2),
            Ty::FunTy(ty1, ty2) => write!(f, "{} -> {}", ty1, ty2),
        }
    }
}

type Constraint = (Ty, Ty);

fn freeTyvars(ty: &Ty) -> HashSet<Typevar> {
    match ty {
        Ty::Tyvar(v) => HashSet::from([v.clone()]),
        Ty::IntTy() => HashSet::new(),
        Ty::ListTy(ty) => freeTyvars(ty),
        Ty::StreamTy(ty) => freeTyvars(ty),
        Ty::PairTy(ty1, ty2) => {
            let fr1: HashSet<Typevar> = freeTyvars(ty1);
            let fr2: HashSet<Typevar> = freeTyvars(ty2);
            fr1.union(&fr2).cloned().collect()
        }
        Ty::LPairTy(ty1, ty2) => {
            let fr1: HashSet<Typevar> = freeTyvars(ty1);
            let fr2: HashSet<Typevar> = freeTyvars(ty2);
            fr1.union(&fr2).cloned().collect()
        }
        Ty::FunTy(ty1, ty2) => {
            let fr1: HashSet<Typevar> = freeTyvars(ty1);
            let fr2: HashSet<Typevar> = freeTyvars(ty2);
            fr1.union(&fr2).cloned().collect()
        }
    }
}

//---------------------------------------------------------------
//---------------------- Zonking --------------------------------
//---------------------------------------------------------------
trait Zonk {
    fn zonk(self, varmap: &HashMap<Typevar, Ty>) -> Self;
}

impl Zonk for Ty {
    fn zonk(self, varmap: &HashMap<Typevar, Ty>) -> Ty {
        match self {
            Ty::Tyvar(v) => match varmap.get(&v) {
                None => Ty::Tyvar(v),
                Some(ty) => ty.clone(),
            },
            Ty::IntTy() => Ty::IntTy(),
            Ty::ListTy(ty) => {
                let ty_zonked: Ty = Zonk::zonk(Rc::unwrap_or_clone(ty), varmap);
                Ty::ListTy(Rc::new(ty_zonked))
            }
            Ty::StreamTy(ty) => {
                let ty_zonked: Ty = Zonk::zonk(Rc::unwrap_or_clone(ty), varmap);
                Ty::StreamTy(Rc::new(ty_zonked))
            }
            Ty::PairTy(ty1, ty2) => {
                let ty1_zonked: Ty = Zonk::zonk(Rc::unwrap_or_clone(ty1), varmap);
                let ty2_zonked: Ty = Zonk::zonk(Rc::unwrap_or_clone(ty2), varmap);
                Ty::PairTy(Rc::new(ty1_zonked), Rc::new(ty2_zonked))
            }
            Ty::LPairTy(ty1, ty2) => {
                let ty1_zonked: Ty = Zonk::zonk(Rc::unwrap_or_clone(ty1), varmap);
                let ty2_zonked: Ty = Zonk::zonk(Rc::unwrap_or_clone(ty2), varmap);
                Ty::LPairTy(Rc::new(ty1_zonked), Rc::new(ty2_zonked))
            }
            Ty::FunTy(ty1, ty2) => {
                let ty1_zonked: Ty = Zonk::zonk(Rc::unwrap_or_clone(ty1), varmap);
                let ty2_zonked: Ty = Zonk::zonk(Rc::unwrap_or_clone(ty2), varmap);
                Ty::FunTy(Rc::new(ty1_zonked), Rc::new(ty2_zonked))
            }
        }
    }
}

impl Zonk for Def<Ty> {
    fn zonk(self, varmap: &HashMap<Typevar, Ty>) -> Def<Ty> {
        Def {
            name: self.name,
            args: self
                .args
                .iter()
                .map(|(v, ty)| (v.clone(), Zonk::zonk(ty.clone(), varmap)))
                .collect(),
            cont: self
                .cont
                .iter()
                .map(|(cv, ty)| (cv.clone(), Zonk::zonk(ty.clone(), varmap)))
                .collect(),
            body: self.body,
            ret_ty: Zonk::zonk(self.ret_ty, varmap),
        }
    }
}

impl Zonk for Prog<Ty> {
    fn zonk(self, varmap: &HashMap<Typevar, Ty>) -> Prog<Ty> {
        Prog {
            prog_defs: self
                .prog_defs
                .iter()
                .map(|def| Zonk::zonk(def.clone(), varmap))
                .collect(),
        }
    }
}

impl Zonk for Constraint {
    fn zonk(self, varmap: &HashMap<Typevar, Ty>) -> Constraint {
        (Zonk::zonk(self.0, varmap), Zonk::zonk(self.1, varmap))
    }
}

//---------------------------------------------------------------
//--------------- Constraint Generation -------------------------
//---------------------------------------------------------------

type Error = String;

struct GenReader {
    gen_vars: HashMap<Variable, Ty>,
    gen_covars: HashMap<Covariable, Ty>,
    gen_defs: Prog<Ty>,
}

impl GenReader {
    fn addVarBindings(&self, new_bindings: Vec<(Variable, Ty)>) -> GenReader {
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

    fn addCovarBindings(&self, new_bindings: Vec<(Covariable, Ty)>) -> GenReader {
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

    fn lookupDefinition(&self, nm: &Name) -> Result<(Vec<Ty>, Vec<Ty>, Ty), Error> {
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
    fn freshVar(&mut self) -> Ty {
        let new_var: String = format!("{}", self.varcnt);
        self.varcnt = self.varcnt + 1;
        Ty::Tyvar(new_var)
    }

    fn addConstraint(&mut self, ctr: Constraint) -> () {
        self.ctrs.push(ctr)
    }
}

fn genConstraintsTm(t: &Term, env: &GenReader, st: &mut GenState) -> Result<Ty, Error> {
    match t {
        Term::Var(v) => match env.gen_vars.get(v) {
            None => Err(format!("Variable {} not bound in environment", v)),
            Some(ty) => Ok(ty.clone()),
        },
        Term::Lit(_) => Ok(Ty::IntTy()),
        Term::Op(t1, _, t2) => {
            let ty1 = genConstraintsTm(t1, env, st)?;
            let ty2 = genConstraintsTm(t2, env, st)?;
            st.addConstraint((ty1, Ty::IntTy()));
            st.addConstraint((ty2, Ty::IntTy()));
            Ok(Ty::IntTy())
        }
        Term::IfZ(t1, t2, t3) => {
            let ty1 = genConstraintsTm(t1, env, st)?;
            let ty2 = genConstraintsTm(t2, env, st)?;
            let ty3 = genConstraintsTm(t3, env, st)?;
            st.addConstraint((ty1, Ty::IntTy()));
            st.addConstraint((ty2.clone(), ty3));
            Ok(ty2)
        }
        Term::Let(x, xdef, t) => {
            let ty = genConstraintsTm(xdef, env, st)?;
            let new_reader: GenReader = env.addVarBindings(vec![(x.clone(), ty)]);
            genConstraintsTm(t, &new_reader, st)
        }
        Term::Fun(nm, args, coargs) => {
            let (arg_tys, coarg_tys, ret_ty) = env.lookupDefinition(nm)?;
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
                    .map(|x| genConstraintsTm(x, env, st))
                    .collect::<Result<Vec<Ty>, Error>>()?;
                let args_zipped = arg_tys1.iter().cloned().zip(arg_tys);
                for (arg_ty, arg_ty_def) in args_zipped {
                    st.addConstraint((arg_ty, arg_ty_def));
                }
                let coargs_zipped = coargs.iter().cloned().zip(coarg_tys).into_iter();
                for (coarg, coarg_ty) in coargs_zipped {
                    let _ = match env.gen_covars.iter().find(|(cv, _)| **cv == coarg) {
                        None => Err(format!("Variable {} not found in environment", coarg)),
                        Some((_, ty)) => Ok(st.addConstraint((ty.clone(), coarg_ty))),
                    }?;
                }
                Ok(ret_ty.clone())
            }
        }
        Term::Constructor(Ctor::Nil, args) if args.len() == 0 => Ok(st.freshVar()),
        Term::Constructor(Ctor::Cons, args) => {
            let arg1: &Rc<Term> = args
                .get(0)
                .ok_or(format!("Wrong number of arguments for {}", Ctor::Cons))?;
            let arg2: &Rc<Term> = args
                .get(1)
                .ok_or(format!("Wrong number of arguments for {}", Ctor::Cons))?;
            if args.len() > 2 {
                Err(format!("Wrong number of arguments for {}", Ctor::Cons))
            } else {
                let ty1: Ty = genConstraintsTm(arg1, env, st)?;
                let ty2: Ty = genConstraintsTm(arg2, env, st)?;
                st.addConstraint((Ty::ListTy(Rc::new(ty1)), ty2.clone()));
                Ok(ty2)
            }
        }
        Term::Constructor(Ctor::Tup, args) => {
            let arg1: &Rc<Term> = args
                .get(0)
                .ok_or(format!("Wrong number of arguments for {}", Ctor::Tup))?;
            let arg2: &Rc<Term> = args
                .get(2)
                .ok_or(format!("Wrong number of arguments for {}", Ctor::Tup))?;
            if args.len() > 2 {
                Err(format!("Wrong number of arguments for {}", Ctor::Cons))
            } else {
                let ty1: Ty = genConstraintsTm(arg1, env, st)?;
                let ty2: Ty = genConstraintsTm(arg2, env, st)?;
                Ok(Ty::PairTy(Rc::new(ty1), Rc::new(ty2)))
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
                .find(|pt| pt.pt_xtor == Ctor::Nil)
                .ok_or(format!("Invalid case expression: {}", t))?;
            let pt_cons: &Rc<Clause<Ctor>> = pts
                .iter()
                .find(|pt| pt.pt_xtor == Ctor::Cons)
                .ok_or(format!("Invalid case expression: {}", t))?;
            let ty_bound: Ty = genConstraintsTm(t_bound, env, st)?;
            let list_arg: Rc<Ty> = Rc::new(st.freshVar());
            let list_ty: Ty = Ty::ListTy(list_arg.clone());
            st.addConstraint((ty_bound, list_ty.clone()));
            let ty_nil: Ty = genConstraintsTm(&pt_nil.pt_t, env, st)?;
            let pt_x: &Variable = pt_cons.pt_vars.get(0).ok_or(format!(
                "Wrong number of bound variables for {}",
                pt_cons.pt_xtor
            ))?;
            let pt_xs: &Variable = pt_cons.pt_vars.get(1).ok_or(format!(
                "Wrong number of bound variables for {}",
                pt_cons.pt_xtor
            ))?;
            let new_env: GenReader = env.addVarBindings(vec![
                (pt_x.clone(), Rc::unwrap_or_clone(list_arg)),
                (pt_xs.clone(), list_ty),
            ]);
            let ty_cons: Ty = genConstraintsTm(&pt_cons.pt_t, &new_env, st)?;
            st.addConstraint((ty_nil.clone(), ty_cons));
            Ok(ty_nil)
        }
        // Tup
        t @ Term::Case(t_bound, pts) if pts.len() == 1 => {
            let pt_tup: &Rc<Clause<Ctor>> = pts
                .get(0)
                .ok_or(format!("Invalid case expression: {}", t))?;
            let ty_bound: Ty = genConstraintsTm(t_bound, env, st)?;
            let ty_a: Ty = st.freshVar();
            let ty_b: Ty = st.freshVar();
            st.addConstraint((
                ty_bound,
                Ty::PairTy(Rc::new(ty_a.clone()), Rc::new(ty_b.clone())),
            ));
            let pt_x: &Variable = pt_tup.pt_vars.get(0).ok_or(format!(
                "Wrong number of bound variables for {}",
                pt_tup.pt_xtor
            ))?;
            let pt_y: &Variable = pt_tup.pt_vars.get(1).ok_or(format!(
                "Wrong number of bound variables for {}",
                pt_tup.pt_xtor
            ))?;
            let new_env: GenReader =
                env.addVarBindings(vec![(pt_x.clone(), ty_a), (pt_y.clone(), ty_b)]);
            genConstraintsTm(&pt_tup.pt_t, &new_env, st)
        }
        t @ Term::Case(_, _) => Err(format!("Invalid case expression: {}", t)),
        Term::Destructor(t, Dtor::Hd, args) if args.len() == 0 => {
            let ty_bound: Ty = genConstraintsTm(t, env, st)?;
            let ty_a: Ty = st.freshVar();
            st.addConstraint((ty_bound, Ty::StreamTy(Rc::new(ty_a.clone()))));
            Ok(ty_a)
        }
        Term::Destructor(t, Dtor::Tl, args) if args.len() == 0 => {
            let ty_bound: Ty = genConstraintsTm(t, env, st)?;
            let ty_str: Ty = Ty::StreamTy(Rc::new(st.freshVar()));
            st.addConstraint((ty_bound, ty_str.clone()));
            Ok(ty_str)
        }
        Term::Destructor(t, Dtor::Fst, args) if args.len() == 0 => {
            let ty_bound: Ty = genConstraintsTm(t, env, st)?;
            let ty_a: Ty = st.freshVar();
            let ty_b: Ty = st.freshVar();
            st.addConstraint((ty_bound, Ty::LPairTy(Rc::new(ty_a.clone()), Rc::new(ty_b))));
            Ok(ty_a)
        }
        Term::Destructor(t, Dtor::Snd, args) if args.len() == 0 => {
            let ty_bound: Ty = genConstraintsTm(t, env, st)?;
            let ty_a: Ty = st.freshVar();
            let ty_b: Ty = st.freshVar();
            st.addConstraint((ty_bound, Ty::LPairTy(Rc::new(ty_a), Rc::new(ty_b.clone()))));
            Ok(ty_b)
        }
        Term::Destructor(_, dtor, _) => Err(format!(
            "Destructor {} called with wrong number of arguments",
            dtor
        )),
        t @ Term::Cocase(pts) if pts.len() == 2 => {
            let err_str = format!("Invalid cocase expression {}", t);
            let pt1: &Rc<Clause<Dtor>> = pts.get(0).ok_or(err_str.clone())?;
            let _ = if pt1.pt_vars.len() == 0 {
                Ok("")
            } else {
                Err(err_str.clone())
            }?;
            let pt2: &Rc<Clause<Dtor>> = pts.get(1).ok_or(err_str.clone())?;
            let _ = if pt1.pt_vars.len() == 0 {
                Ok("")
            } else {
                Err(err_str.clone())
            }?;
            let ty1: Ty = genConstraintsTm(&pt1.pt_t, env, st)?;
            let ty2: Ty = genConstraintsTm(&pt2.pt_t, env, st)?;
            if pt1.pt_xtor == Dtor::Hd && pt2.pt_xtor == Dtor::Tl {
                let str_ty: Ty = Ty::StreamTy(Rc::new(ty1));
                st.addConstraint((str_ty.clone(), ty2));
                Ok(str_ty)
            } else if pt1.pt_xtor == Dtor::Fst && pt2.pt_xtor == Dtor::Snd {
                let pair_ty: Ty = Ty::LPairTy(Rc::new(ty1), Rc::new(ty2));
                Ok(pair_ty)
            } else {
                Err(err_str)
            }
        }
        t @ Term::Cocase(_) => Err(format!("Invalid cocase expression {}", t)),
        Term::Lam(v, body) => {
            let ty_a: Ty = st.freshVar();
            let new_env: GenReader = env.addVarBindings(vec![(v.clone(), ty_a.clone())]);
            let ty_body = genConstraintsTm(body, &new_env, st)?;
            Ok(Ty::FunTy(Rc::new(ty_a), Rc::new(ty_body)))
        }
        Term::App(t1, t2) => {
            let ty1: Ty = genConstraintsTm(t1, env, st)?;
            let ty2: Ty = genConstraintsTm(t2, env, st)?;
            let ret_ty: Ty = st.freshVar();
            st.addConstraint((ty1, Ty::FunTy(Rc::new(ty2), Rc::new(ret_ty.clone()))));
            Ok(ret_ty)
        }
        Term::Goto(t, cv) => {
            let ty1: Ty = genConstraintsTm(t, env, st)?;
            let (_, ty2): (&String, &Ty) = env
                .gen_covars
                .iter()
                .find(|(cv1, _)| **cv == **cv1)
                .ok_or(format!("Covariable {} not bound in environment", cv))?;
            st.addConstraint((ty1, ty2.clone()));
            Ok(st.freshVar())
        }
        Term::Label(cv, t) => {
            let ty_a: Ty = st.freshVar();
            env.addCovarBindings(vec![(cv.clone(), ty_a.clone())]);
            let ty: Ty = genConstraintsTm(t, env, st)?;
            st.addConstraint((ty.clone(), ty_a));
            Ok(ty)
        }
    }
}

fn genConstraintsDef(def: &Def<Ty>, env: &mut GenReader, st: &mut GenState) -> Result<(), Error> {
    let env_with_vars: GenReader = env.addVarBindings(def.args.clone());
    let env_with_covars: GenReader = env_with_vars.addCovarBindings(def.cont.clone());
    let ty: Ty = genConstraintsTm(&def.body, &env_with_covars, st)?;
    st.addConstraint((ty, def.ret_ty.clone()));
    Ok(())
}

fn annotateProgram(prog: Prog<()>) -> Prog<Ty> {
    let mut var_cnt = 0;
    let mut fresh = || {
        var_cnt = var_cnt + 1;
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

fn genConstraints(prog: Prog<()>) -> Result<(Prog<Ty>, Vec<Constraint>), Error> {
    let prog_annot: Prog<Ty> = annotateProgram(prog);
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
        .map(|df| genConstraintsDef(df, &mut initial_reader, &mut initial_state))
        .collect::<Result<Vec<()>, Error>>()?;
    Ok((prog_annot, initial_state.ctrs))
}
