//! This module defines constructors and destructors in Core.

use printer::*;

use crate::syntax::*;
use crate::traits::*;

use std::collections::{BTreeSet, HashSet};

/// This struct defines constructors and destructors in Core. It consists of the information that
/// determines whether it is a constructor (if `C` is instantiated with [`Prd`]) or a destructor
/// (if `C` is instantiated with [`Cns`]), a name for the xtor, the arguments of the xtor, and of
/// the type. The type parameter `A` determines whether this is the unfocused variant (if `A` is
/// instantiated with [`Arguments`], which is the default) or the focused variant (if `A` is
/// instantiated with [`TypingContext`]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Xtor<C: Chi, A = Arguments> {
    /// Whether we have a constructor or destructor
    pub prdcns: C,
    /// The xtor name
    pub id: Name,
    /// The arguments of the xtor
    pub args: A,
    /// The type of the xtor
    pub ty: Ty,
}

#[allow(type_alias_bounds)]
pub type FsXtor<C: Chi> = Xtor<C, TypingContext>;

impl<A> Xtor<Prd, A> {
    /// This functions creates a constructor from a given name, arguments, and its type.
    pub fn ctor(name: &str, args: A, ty: Ty) -> Self {
        Xtor {
            prdcns: Prd,
            id: name.to_string(),
            args,
            ty,
        }
    }
}

impl<A> Xtor<Cns, A> {
    /// This functions creates a destructor from a given name, arguments, and its type.
    pub fn dtor(name: &str, args: A, ty: Ty) -> Self {
        Xtor {
            prdcns: Cns,
            id: name.to_string(),
            args,
            ty,
        }
    }
}

impl<C: Chi, A> Typed for Xtor<C, A> {
    fn get_type(&self) -> Ty {
        self.ty.clone()
    }
}

impl<C: Chi> Print for Xtor<C> {
    fn print<'a>(&'a self, cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        let args = if self.args.entries.is_empty() {
            alloc.nil()
        } else {
            self.args.print(cfg, alloc).parens()
        };
        if self.prdcns.is_prd() {
            alloc.ctor(&self.id).append(args.group())
        } else {
            alloc.dtor(&self.id).append(args.group())
        }
    }
}

impl<C: Chi> Print for FsXtor<C> {
    fn print<'a>(&'a self, cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        let args = if self.args.bindings.is_empty() {
            alloc.nil()
        } else {
            self.args.print(cfg, alloc)
        };
        if self.prdcns.is_prd() {
            alloc.ctor(&self.id).append(args)
        } else {
            alloc.dtor(&self.id).append(args)
        }
    }
}

impl<C: Chi> From<Xtor<C>> for Term<C> {
    fn from(value: Xtor<C>) -> Self {
        Term::Xtor(value)
    }
}

impl<C: Chi> From<FsXtor<C>> for FsTerm<C> {
    fn from(value: FsXtor<C>) -> Self {
        FsTerm::Xtor(value)
    }
}

impl<C: Chi> Subst for Xtor<C> {
    type Target = Xtor<C>;
    fn subst_sim(
        mut self,
        prod_subst: &[(Var, Term<Prd>)],
        cons_subst: &[(Covar, Term<Cns>)],
    ) -> Self::Target {
        self.args = self.args.subst_sim(prod_subst, cons_subst);
        self
    }
}

impl<C: Chi> SubstVar for FsXtor<C> {
    type Target = FsXtor<C>;
    fn subst_sim(mut self, subst: &[(Var, Var)]) -> Self::Target {
        self.args = self.args.subst_sim(subst);
        self
    }
}

impl<C: Chi> TypedFreeVars for Xtor<C> {
    fn typed_free_vars(&self, vars: &mut BTreeSet<ContextBinding>) {
        self.args.typed_free_vars(vars)
    }
}

impl<C: Chi> TypedFreeVars for FsXtor<C> {
    fn typed_free_vars(&self, vars: &mut BTreeSet<ContextBinding>) {
        vars.extend(self.args.bindings.iter().cloned())
    }
}

impl<C: Chi> Uniquify for Xtor<C> {
    fn uniquify(mut self, seen_vars: &mut HashSet<Var>, used_vars: &mut HashSet<Var>) -> Xtor<C> {
        self.args = self.args.uniquify(seen_vars, used_vars);
        self
    }
}

impl<C: Chi> Focusing for Xtor<C> {
    type Target = FsTerm<C>;
    fn focus(self, _: &mut HashSet<Var>) -> Self::Target {
        panic!("Constructors and destructors should always be focused in cuts directly");
    }
}

impl Bind for Xtor<Prd> {
    // bind(C(t_i))[k] = bind(t_i)[λas.⟨ C(as) | ~μx.k(x) ⟩]
    fn bind(self, k: Continuation, used_vars: &mut HashSet<Var>) -> FsStatement {
        bind_many(
            self.args.into(),
            Box::new(|bindings, used_vars: &mut HashSet<Var>| {
                let new_var = fresh_var(used_vars);
                let new_binding = ContextBinding {
                    var: new_var.clone(),
                    chi: Chirality::Prd,
                    ty: self.ty.clone(),
                };
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
    // bind(D(t_i))[k] = bind(t_i)[λas.⟨ μa.k(a) | D(as) ⟩]
    fn bind(self, k: Continuation, used_vars: &mut HashSet<Var>) -> FsStatement {
        bind_many(
            self.args.into(),
            Box::new(|bindings, used_vars: &mut HashSet<Var>| {
                let new_covar = fresh_covar(used_vars);
                let new_binding = ContextBinding {
                    var: new_covar.clone(),
                    chi: Chirality::Cns,
                    ty: self.ty.clone(),
                };
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

#[cfg(test)]
mod xtor_tests {
    use printer::Print;

    use super::Subst;
    use crate::syntax::*;
    use crate::test_common::example_subst;

    fn example() -> Xtor<Prd> {
        let mut arguments = Arguments::default();
        arguments.add_prod(XVar::var("x", Ty::I64));
        arguments.add_prod(XVar::var("xs", Ty::Decl("ListInt".to_string())));
        Xtor::ctor("Cons", arguments, Ty::Decl("ListInt".to_string()))
    }

    #[test]
    fn display_const() {
        assert_eq!(example().print_to_string(None), "Cons(x, xs)")
    }

    #[test]
    fn subst_const() {
        let subst = example_subst();
        let result = example().subst_sim(&subst.0, &subst.1);
        let mut arguments = Arguments::default();
        arguments.add_prod(XVar::var("y", Ty::I64));
        arguments.add_prod(XVar::var("xs", Ty::Decl("ListInt".to_string())));
        let expected = Xtor::ctor("Cons", arguments, Ty::Decl("ListInt".to_string()));
        assert_eq!(result, expected)
    }
}
