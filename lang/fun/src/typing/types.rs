use std::{collections::HashSet, fmt};

pub type Typevar = String;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ty {
    Var(Typevar),
    Int(),
    List(Box<Ty>),
    Stream(Box<Ty>),
    Pair(Box<Ty>, Box<Ty>),
    LPair(Box<Ty>, Box<Ty>),
    Fun(Box<Ty>, Box<Ty>),
}

impl fmt::Display for Ty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Ty::Var(v) => write!(f, "{}", v),
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
            Ty::Var(v) => HashSet::from([v.clone()]),
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

#[cfg(test)]
mod type_tests {
    use super::Ty;

    #[test]
    fn free_tyvars_var() {
        let ex = Ty::Var("a".to_string());
        assert_eq!(
            ex.free_tyvars(),
            vec!["a".to_string()].into_iter().collect()
        )
    }

    #[test]
    fn free_tyvars_fun() {
        let ex = Ty::Fun(
            Box::new(Ty::Var("a".to_string())),
            Box::new(Ty::Var("b".to_string())),
        );
        assert_eq!(
            ex.free_tyvars(),
            vec!["a".to_string(), "b".to_string()].into_iter().collect()
        )
    }
}
