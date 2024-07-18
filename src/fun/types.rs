use std::rc::Rc;
use std::fmt;
use std::collections::HashSet;

type Typevar = &'static str; 
#[derive(Debug,Clone)]
enum Ty {
    Tyvar(Typevar),
    IntTy(),
    ListTy(Rc<Ty>),
    StreamTy(Rc<Ty>),
    PairTy(Rc<Ty>,Rc<Ty>),
    LPairTy(Rc<Ty>,Rc<Ty>),
    FunTy(Rc<Ty>,Rc<Ty>)
}
impl fmt::Display for Ty {
    fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result{
       match self {
           Ty::Tyvar(v) => write!(f,"{}",v),
           Ty::IntTy() => write!(f,"Int"),
           Ty::ListTy(ty) => write!(f,"List({})",ty),
           Ty::StreamTy(ty) => write!(f,"Stream({})",ty),
           Ty::PairTy(ty1,ty2) => write!(f,"Pair({},{})",ty1,ty2),
           Ty::LPairTy(ty1,ty2) => write!(f,"LPairTy({},{})",ty1,ty2),
           Ty::FunTy(ty1,ty2) => write!(f,"{} -> {}",ty1,ty2)
       }
    }
}

fn freeTyvars(ty:Ty) -> HashSet<Typevar> {
    match ty {
        Ty::Tyvar(v) => HashSet::from([v]),
        Ty::IntTy() => HashSet::new(),
        Ty::ListTy(ty) => freeTyvars(Rc::unwrap_or_clone(ty)),
        Ty::StreamTy(ty) => freeTyvars(Rc::unwrap_or_clone(ty)),
        Ty::PairTy(ty1,ty2) => {
            let fr1 : HashSet<Typevar> = freeTyvars(Rc::unwrap_or_clone(ty1));
            let fr2 : HashSet<Typevar> = freeTyvars(Rc::unwrap_or_clone(ty2));
            fr1.union(&fr2).copied().collect()
        },
        Ty::LPairTy(ty1,ty2) => {
            let fr1 : HashSet<Typevar> = freeTyvars(Rc::unwrap_or_clone(ty1));
            let fr2 : HashSet<Typevar> = freeTyvars(Rc::unwrap_or_clone(ty2));
            fr1.union(&fr2).copied().collect() 
        },
        Ty::FunTy(ty1,ty2) => {
            let fr1 : HashSet<Typevar> = freeTyvars(Rc::unwrap_or_clone(ty1));
            let fr2 : HashSet<Typevar> = freeTyvars(Rc::unwrap_or_clone(ty2));
            fr1.union(&fr2).copied().collect()
        }
    }
}
