use std::collections::HashMap;

use crate::program::{Def, Prog};

pub mod generate;
pub mod result;
pub mod solver;
pub mod types;

use generate::generate_constraints;
use result::TypeError;
pub use solver::*;
pub use types::*;

//---------------------------------------------------------------
//---------------------- Zonking --------------------------------
//---------------------------------------------------------------
trait Zonk {
    fn zonk(&mut self, varmap: &HashMap<Typevar, Ty>);
}

impl Zonk for Ty {
    fn zonk(&mut self, varmap: &HashMap<Typevar, Ty>) {
        match self {
            Ty::Var(v) => match varmap.get(v) {
                None => {}
                Some(ty) => *self = ty.clone(),
            },
            Ty::Int() => {}
            Ty::List(ty) => ty.zonk(varmap),
            Ty::Stream(ty) => ty.zonk(varmap),
            Ty::Pair(ty1, ty2) => {
                ty1.zonk(varmap);
                ty2.zonk(varmap)
            }
            Ty::LPair(ty1, ty2) => {
                ty1.zonk(varmap);
                ty2.zonk(varmap)
            }
            Ty::Fun(ty1, ty2) => {
                ty1.zonk(varmap);
                ty2.zonk(varmap)
            }
        }
    }
}

impl Zonk for Def<Ty> {
    fn zonk(&mut self, varmap: &HashMap<Typevar, Ty>) {
        for e in self.args.iter_mut() {
            e.1.zonk(varmap)
        }
        for e in self.cont.iter_mut() {
            e.1.zonk(varmap)
        }
        self.ret_ty.zonk(varmap)
    }
}

impl Zonk for Prog<Ty> {
    fn zonk(&mut self, varmap: &HashMap<Typevar, Ty>) {
        for def in self.prog_defs.iter_mut() {
            def.zonk(varmap)
        }
    }
}

impl Zonk for HashMap<Typevar, Ty> {
    fn zonk(&mut self, varmap: &HashMap<Typevar, Ty>) {
        for ty in self.values_mut() {
            ty.zonk(varmap)
        }
    }
}

#[cfg(test)]
mod zonk_tests {

    use crate::syntax::Term;

    use super::{Def, Ty, Zonk};
    use std::collections::HashMap;

    #[test]
    fn zonk_int() {
        let mut ty = Ty::Int();
        ty.zonk(&HashMap::new());
        assert_eq!(ty, Ty::Int())
    }

    #[test]
    fn zonk_var() {
        let mut ty = Ty::Var("X".to_owned());
        let mut varmap = HashMap::new();
        varmap.insert("X".to_owned(), Ty::Int());
        ty.zonk(&varmap);
        assert_eq!(ty, Ty::Int())
    }

    #[test]
    fn zonk_var2() {
        let mut ty = Ty::Var("X".to_owned());
        let mut varmap = HashMap::new();
        varmap.insert("Y".to_owned(), Ty::Int());
        ty.zonk(&varmap);
        assert_eq!(ty, Ty::Var("X".to_owned()))
    }

    #[test]
    fn zonk_list() {
        let mut ty = Ty::List(Box::new(Ty::Var("X".to_owned())));
        let mut varmap = HashMap::new();
        varmap.insert("X".to_owned(), Ty::Int());
        ty.zonk(&varmap);
        assert_eq!(ty, Ty::List(Box::new(Ty::Int())));
    }

    #[test]
    fn zonk_pair() {
        let mut ty = Ty::Pair(Box::new(Ty::Int()), Box::new(Ty::Var("X".to_owned())));
        let mut varmap = HashMap::new();
        varmap.insert("Y".to_owned(), Ty::Int());
        ty.zonk(&varmap);
        assert_eq!(
            ty,
            Ty::Pair(Box::new(Ty::Int()), Box::new(Ty::Var("X".to_owned())))
        )
    }

    #[test]
    fn zonk_stream() {
        let mut ty = Ty::Stream(Box::new(Ty::Int()));
        let mut varmap = HashMap::new();
        varmap.insert("X".to_owned(), Ty::Int());
        ty.zonk(&varmap);
        assert_eq!(ty, Ty::Stream(Box::new(Ty::Int())));
    }

    #[test]
    fn zonk_lpair() {
        let mut ty = Ty::LPair(Box::new(Ty::Int()), Box::new(Ty::Var("X".to_owned())));
        let mut varmap = HashMap::new();
        varmap.insert("X".to_owned(), Ty::Var("Y".to_owned()));
        ty.zonk(&varmap);
        assert_eq!(
            ty,
            Ty::LPair(Box::new(Ty::Int()), Box::new(Ty::Var("Y".to_owned())))
        );
    }

    #[test]
    fn zonk_fun() {
        let mut ty = Ty::Fun(Box::new(Ty::Int()), Box::new(Ty::Int()));
        let varmap = HashMap::new();
        ty.zonk(&varmap);
        assert_eq!(ty, Ty::Fun(Box::new(Ty::Int()), Box::new(Ty::Int())));
    }

    #[test]
    fn zonk_def() {
        let mut def = Def {
            name: "main".to_owned(),
            args: vec![("x".to_owned(), Ty::Var("X".to_owned()))],
            cont: vec![("a".to_owned(), Ty::Var("Y".to_owned()))],
            body: Term::Var("x".to_owned()),
            ret_ty: Ty::Var("X".to_owned()),
        };
        let mut varmap = HashMap::new();
        varmap.insert("X".to_owned(), Ty::Int());
        varmap.insert("Y".to_owned(), Ty::Int());
        def.zonk(&varmap);
        assert_eq!(def.name, "main".to_owned());
        assert_eq!(def.args, vec![("x".to_owned(), Ty::Int())]);
        assert_eq!(def.cont, vec![("a".to_owned(), Ty::Int())]);
        assert_eq!(def.body, Term::Var("x".to_owned()));
        assert_eq!(def.ret_ty, Ty::Int())
    }
}

//---------------------------------------------------------------
//---------------- Type Inference -------------------------------
//---------------------------------------------------------------

pub fn infer_types(prog: Prog<()>) -> Result<Prog<Ty>, TypeError> {
    let (mut prog, constraints): (Prog<Ty>, Vec<Constraint>) = generate_constraints(prog)?;
    let subst = solve_constraints(constraints)?;
    prog.zonk(&subst);
    Ok(prog)
}
