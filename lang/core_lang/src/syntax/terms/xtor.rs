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
    pub id: Ident,
    /// The arguments of the xtor
    pub args: A,
    /// The type of the xtor
    pub ty: Ty,
}

#[allow(type_alias_bounds)]
pub type FsXtor<C: Chi> = Xtor<C, TypingContext>;

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
            alloc
                .ctor(&self.id.print_to_string(Some(cfg)))
                .append(args.group())
        } else {
            alloc
                .dtor(&self.id.print_to_string(Some(cfg)))
                .append(args.group())
        }
    }
}

impl<C: Chi> Print for FsXtor<C> {
    fn print<'a>(&'a self, cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        let args = if self.args.bindings.is_empty() {
            alloc.nil()
        } else {
            self.args.print(cfg, alloc).parens()
        };
        if self.prdcns.is_prd() {
            alloc.ctor(&self.id.print_to_string(Some(cfg))).append(args)
        } else {
            alloc.dtor(&self.id.print_to_string(Some(cfg))).append(args)
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
        prod_subst: &[(Ident, Term<Prd>)],
        cons_subst: &[(Ident, Term<Cns>)],
    ) -> Self::Target {
        self.args = self.args.subst_sim(prod_subst, cons_subst);
        self
    }
}

impl<C: Chi> SubstVar for FsXtor<C> {
    type Target = FsXtor<C>;
    fn subst_sim(mut self, subst: &[(Ident, Ident)]) -> Self::Target {
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
    fn uniquify(
        mut self,
        seen_vars: &mut HashSet<Ident>,
        used_vars: &mut HashSet<Ident>,
    ) -> Xtor<C> {
        self.args = self.args.uniquify(seen_vars, used_vars);
        self
    }
}

impl<C: Chi> Focusing for Xtor<C> {
    type Target = FsTerm<C>;
    fn focus(self, _: &mut HashSet<Ident>) -> Self::Target {
        panic!("Constructors and destructors should always be focused in cuts directly");
    }
}

impl Bind for Xtor<Prd> {
    // bind(C(t_i))[k] = bind(t_i)[λas.⟨ C(as) | ~μx.k(x) ⟩]
    fn bind(self, k: Continuation, used_vars: &mut HashSet<Ident>) -> FsStatement {
        bind_many(
            self.args.into(),
            Box::new(|bindings, used_vars: &mut HashSet<Ident>| {
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
                    Mu::tilde_mu(new_var, k(new_binding, used_vars), self.ty.clone()),
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
    fn bind(self, k: Continuation, used_vars: &mut HashSet<Ident>) -> FsStatement {
        bind_many(
            self.args.into(),
            Box::new(|bindings, used_vars: &mut HashSet<Ident>| {
                let new_covar = fresh_covar(used_vars);
                let new_binding = ContextBinding {
                    var: new_covar.clone(),
                    chi: Chirality::Cns,
                    ty: self.ty.clone(),
                };
                FsCut::new(
                    Mu::mu(new_covar, k(new_binding, used_vars), self.ty.clone()),
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
    extern crate self as core_lang;
    use core_macros::{ctor, id, ty, var};

    fn example() -> Xtor<Prd> {
        ctor!(
            id!("Cons"),
            [var!(id!("x")), var!(id!("xs"), ty!(id!("ListInt")))],
            ty!(id!("ListInt"))
        )
    }

    #[test]
    fn display_const() {
        assert_eq!(example().print_to_string(None), "Cons(x, xs)")
    }

    #[test]
    fn subst_const() {
        let subst = example_subst();
        let result = example().subst_sim(&subst.0, &subst.1);
        let expected = ctor!(
            id!("Cons"),
            [var!(id!("y")), var!(id!("xs"), ty!(id!("ListInt")))],
            ty!(id!("ListInt"))
        );
        assert_eq!(result, expected)
    }
}
