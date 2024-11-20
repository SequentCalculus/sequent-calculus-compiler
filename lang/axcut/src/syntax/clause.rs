use printer::tokens::FAT_ARROW;
use printer::{DocAllocator, Print};

use super::{Name, Statement, TypingContext, Var};
use crate::traits::free_vars::FreeVars;
use crate::traits::linearize::UsedBinders;
use crate::traits::substitution::Subst;

use std::collections::HashSet;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Clause {
    pub xtor: Name,
    pub context: TypingContext,
    pub case: Rc<Statement>,
}

impl FreeVars for Clause {
    fn free_vars(&self, vars: &mut HashSet<Var>) {
        self.case.free_vars(vars);
        for binding in &self.context {
            vars.remove(&binding.var);
        }
    }
}

impl Subst for Clause {
    type Target = Clause;

    fn subst_sim(self, subst: &[(Var, Var)]) -> Clause {
        Clause {
            case: self.case.subst_sim(subst),
            ..self
        }
    }
}

impl UsedBinders for Clause {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        for binding in &self.context {
            used.insert(binding.var.clone());
        }
        self.case.used_binders(used);
    }
}

impl Print for Clause {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc
            .text(&self.xtor)
            .append(self.context.print(cfg, alloc).parens())
            .append(alloc.space())
            .append(FAT_ARROW)
            .append(alloc.space())
            .append(self.case.print(cfg, alloc))
    }
}

#[cfg(test)]
mod clause_tests {
    use super::{Clause, FreeVars, Subst, UsedBinders};
    use crate::syntax::{context::ContextBinding, statements::Return, types::Ty, Chirality};
    use printer::Print;
    use std::{collections::HashSet, rc::Rc};

    fn example_nil() -> Clause {
        Clause {
            xtor: "Nil".to_owned(),
            context: vec![],
            case: Rc::new(
                Return {
                    var: "x".to_owned(),
                }
                .into(),
            ),
        }
    }

    fn example_cons() -> Clause {
        Clause {
            xtor: "Cons".to_owned(),
            context: vec![
                ContextBinding {
                    var: "x".to_owned(),
                    chi: Chirality::Prd,
                    ty: Ty::Int,
                },
                ContextBinding {
                    var: "xs".to_owned(),
                    chi: Chirality::Prd,
                    ty: Ty::Decl("ListInt".to_owned()),
                },
            ],
            case: Rc::new(
                Return {
                    var: "x".to_owned(),
                }
                .into(),
            ),
        }
    }

    #[test]
    fn free_vars_nil() {
        let mut result = HashSet::new();
        example_nil().free_vars(&mut result);
        let expected = HashSet::from(["x".to_owned()]);
        assert_eq!(result, expected)
    }

    #[test]
    fn free_vars_cons() {
        let mut result = HashSet::new();
        example_cons().free_vars(&mut result);
        let expected = HashSet::new();
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_nil() {
        let result = example_nil().subst_sim(&vec![("x".to_owned(), "y".to_owned())]);
        let expected = Clause {
            xtor: "Nil".to_owned(),
            context: vec![],
            case: Rc::new(
                Return {
                    var: "y".to_owned(),
                }
                .into(),
            ),
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_cons() {
        let result = example_cons().subst_sim(&vec![("x".to_owned(), "y".to_owned())]);
        let expected = Clause {
            xtor: "Cons".to_owned(),
            context: vec![
                ContextBinding {
                    var: "x".to_owned(),
                    chi: Chirality::Prd,
                    ty: Ty::Int,
                },
                ContextBinding {
                    var: "xs".to_owned(),
                    chi: Chirality::Prd,
                    ty: Ty::Decl("ListInt".to_owned()),
                },
            ],
            case: Rc::new(
                Return {
                    var: "y".to_owned(),
                }
                .into(),
            ),
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn used_binders_nil() {
        let mut result = HashSet::new();
        example_nil().used_binders(&mut result);
        let expected = HashSet::new();
        assert_eq!(result, expected)
    }

    #[test]
    fn used_binders_cons() {
        let mut result = HashSet::new();
        example_cons().used_binders(&mut result);
        let expected = HashSet::from(["x".to_owned(), "xs".to_owned()]);
        assert_eq!(result, expected)
    }

    #[test]
    fn print_nil() {
        let result = example_nil().print_to_string(Default::default());
        let expected = "Nil() => return x";
        assert_eq!(result, expected)
    }

    #[test]
    fn print_cons() {
        let result = example_cons().print_to_string(Default::default());
        let expected = "Cons(x :prd: Int, xs :prd: ListInt) => return x";
        assert_eq!(result, expected)
    }
}
