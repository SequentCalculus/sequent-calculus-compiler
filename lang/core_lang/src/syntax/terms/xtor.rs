use printer::{theme::ThemeExt, DocAllocator, Print};

use super::{Cns, ContextBinding, FsTerm, Mu, Prd, PrdCns, Term};
use crate::{
    syntax::{
        fresh_covar, fresh_var, statements::FsCut, Chirality, Covar, FsStatement, Name,
        Substitution, Ty, TypingContext, Var,
    },
    traits::*,
};

use std::collections::{BTreeSet, HashSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Xtor<T: PrdCns> {
    pub prdcns: T,
    pub id: Name,
    pub args: Substitution,
    pub ty: Ty,
}

impl Xtor<Prd> {
    /// Create a new constructor
    pub fn ctor(name: &str, subst: Substitution, ty: Ty) -> Self {
        Xtor {
            prdcns: Prd,
            id: name.to_string(),
            args: subst,
            ty,
        }
    }
}
impl Xtor<Cns> {
    /// Create a new destructor
    pub fn dtor(name: &str, subst: Substitution, ty: Ty) -> Self {
        Xtor {
            prdcns: Cns,
            id: name.to_string(),
            args: subst,
            ty,
        }
    }
}

impl<T: PrdCns> Typed for Xtor<T> {
    fn get_type(&self) -> Ty {
        self.ty.clone()
    }
}

impl<T: PrdCns> Print for Xtor<T> {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        let args = if self.args.0.is_empty() {
            alloc.nil()
        } else {
            self.args.print(cfg, alloc).parens()
        };
        if self.prdcns.is_prd() {
            alloc.ctor(&self.id).append(args)
        } else {
            alloc.dtor(&self.id).append(args)
        }
    }
}

impl<T: PrdCns> From<Xtor<T>> for Term<T> {
    fn from(value: Xtor<T>) -> Self {
        Term::Xtor(value)
    }
}

impl<T: PrdCns> Subst for Xtor<T> {
    type Target = Xtor<T>;
    fn subst_sim(
        mut self,
        prod_subst: &[(Var, Term<Prd>)],
        cons_subst: &[(Covar, Term<Cns>)],
    ) -> Self::Target {
        self.args = self.args.subst_sim(prod_subst, cons_subst);
        self
    }
}

impl<T: PrdCns> TypedFreeVars for Xtor<T> {
    fn typed_free_vars(&self, vars: &mut BTreeSet<ContextBinding>) {
        self.args.typed_free_vars(vars)
    }
}

impl<T: PrdCns> Uniquify for Xtor<T> {
    fn uniquify(mut self, seen_vars: &mut HashSet<Var>, used_vars: &mut HashSet<Var>) -> Xtor<T> {
        self.args = self.args.uniquify(seen_vars, used_vars);
        self
    }
}

impl<T: PrdCns> Focusing for Xtor<T> {
    type Target = FsTerm<T>;
    fn focus(self, _: &mut HashSet<Var>) -> Self::Target {
        panic!("Constructors and destructors should always be focused in cuts directly");
    }
}

impl Bind for Xtor<Prd> {
    ///bind(C(t_i))[k] = bind(t_i)[λas.⟨C(as) | ~μx.k(x)⟩]
    fn bind(self, k: Continuation, used_vars: &mut HashSet<Var>) -> FsStatement {
        let new_var = fresh_var(used_vars);
        let new_binding = ContextBinding {
            var: new_var.clone(),
            chi: Chirality::Prd,
            ty: self.ty.clone(),
        };
        bind_many(
            self.args.into(),
            Box::new(move |bindings, used_vars: &mut HashSet<Var>| {
                FsCut::new(
                    FsTerm::Xtor(FsXtor {
                        prdcns: self.prdcns,
                        id: self.id,
                        args: bindings.into(),
                        ty: self.ty.clone(),
                    }),
                    Mu::tilde_mu(&new_var, k(new_binding, used_vars), self.ty.clone()),
                    self.ty,
                )
                .into()
            }),
            used_vars,
        )
    }
}
impl Bind for Xtor<Cns> {
    ///bind(D(t_i))[k] = bind(t_i)[λas.⟨μa.k(a) | D(as)⟩]
    fn bind(self, k: Continuation, used_vars: &mut HashSet<Var>) -> FsStatement {
        let new_covar = fresh_covar(used_vars);
        let new_binding = ContextBinding {
            var: new_covar.clone(),
            chi: Chirality::Cns,
            ty: self.ty.clone(),
        };
        bind_many(
            self.args.into(),
            Box::new(move |bindings, used_vars: &mut HashSet<Var>| {
                FsCut::new(
                    Mu::mu(&new_covar, k(new_binding, used_vars), self.ty.clone()),
                    FsTerm::Xtor(FsXtor {
                        prdcns: self.prdcns,
                        id: self.id,
                        args: bindings.into(),
                        ty: self.ty.clone(),
                    }),
                    self.ty,
                )
                .into()
            }),
            used_vars,
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FsXtor<T: PrdCns> {
    pub prdcns: T,
    pub id: Name,
    pub args: TypingContext,
    pub ty: Ty,
}

impl FsXtor<Prd> {
    /// Create a new constructor
    pub fn ctor(name: &str, args: TypingContext, ty: Ty) -> Self {
        FsXtor {
            prdcns: Prd,
            id: name.to_string(),
            args,
            ty,
        }
    }
}
impl FsXtor<Cns> {
    /// Create a new destructor
    pub fn dtor(name: &str, args: TypingContext, ty: Ty) -> Self {
        FsXtor {
            prdcns: Cns,
            id: name.to_string(),
            args,
            ty,
        }
    }
}

impl<T: PrdCns> Print for FsXtor<T> {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        let args = if self.args.bindings.is_empty() {
            alloc.nil()
        } else {
            self.args.print(cfg, alloc).parens()
        };
        if self.prdcns.is_prd() {
            alloc.ctor(&self.id).append(args)
        } else {
            alloc.dtor(&self.id).append(args)
        }
    }
}

impl<T: PrdCns> From<FsXtor<T>> for FsTerm<T> {
    fn from(value: FsXtor<T>) -> Self {
        FsTerm::Xtor(value)
    }
}

impl<T: PrdCns> SubstVar for FsXtor<T> {
    type Target = FsXtor<T>;
    fn subst_sim(mut self, subst: &[(Var, Var)]) -> Self::Target {
        self.args = self.args.subst_sim(subst);
        self
    }
}

impl<T: PrdCns> TypedFreeVars for FsXtor<T> {
    fn typed_free_vars(&self, vars: &mut BTreeSet<ContextBinding>) {
        vars.extend(self.args.bindings.iter().cloned())
    }
}

#[cfg(test)]
mod xtor_tests {
    use super::{Subst, Xtor};
    use crate::{
        syntax::{
            substitution::Substitution,
            terms::{Prd, XVar},
            types::Ty,
        },
        test_common::example_subst,
    };
    use printer::Print;

    fn example() -> Xtor<Prd> {
        let mut subst = Substitution::default();
        subst.add_prod(XVar::var("x", Ty::I64));
        subst.add_prod(XVar::var("xs", Ty::Decl("ListInt".to_string())));
        Xtor::ctor("Cons", subst, Ty::Decl("ListInt".to_string()))
    }

    #[test]
    fn display_const() {
        assert_eq!(example().print_to_string(None), "Cons(x, xs)")
    }

    #[test]
    fn subst_const() {
        let subst = example_subst();
        let result = example().subst_sim(&subst.0, &subst.1);
        let mut substitution = Substitution::default();
        substitution.add_prod(XVar::var("y", Ty::I64));
        substitution.add_prod(XVar::var("xs", Ty::Decl("ListInt".to_string())));
        let expected = Xtor::ctor("Cons", substitution, Ty::Decl("ListInt".to_string()));
        assert_eq!(result, expected)
    }
}
