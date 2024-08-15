use std::{collections::HashSet, fmt, rc::Rc};

pub type Typevar = String;

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

impl Ty {
    /// Compute the free type variables of a type.
    pub fn free_tyvars(&self) -> HashSet<Typevar> {
        match self {
            Ty::Tyvar(v) => HashSet::from([v.clone()]),
            Ty::Int() => HashSet::new(),
            Ty::List(ty) => ty.free_tyvars(),
            Ty::Stream(ty) => ty.free_tyvars(),
            Ty::Pair(ty1, ty2) => {
                let mut fv = ty1.free_tyvars();
                fv.extend(ty2.free_tyvars());
                fv
            }
            Ty::LPair(ty1, ty2) => {
                let mut fv = ty1.free_tyvars();
                fv.extend(ty2.free_tyvars());
                fv
            }
            Ty::Fun(ty1, ty2) => {
                let mut fv = ty1.free_tyvars();
                fv.extend(ty2.free_tyvars());
                fv
            }
        }
    }
}
