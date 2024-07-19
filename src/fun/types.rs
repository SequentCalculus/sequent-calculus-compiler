use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::rc::Rc;

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
