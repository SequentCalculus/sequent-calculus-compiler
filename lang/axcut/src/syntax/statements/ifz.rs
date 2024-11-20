use printer::theme::ThemeExt;
use printer::tokens::{COMMA, FAT_ARROW, IFZ};
use printer::util::BracesExt;
use printer::{DocAllocator, Print};

use crate::syntax::{Statement, Var};
use crate::traits::free_vars::FreeVars;
use crate::traits::linearize::{Linearizing, UsedBinders};
use crate::traits::substitution::Subst;

use std::collections::HashSet;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IfZ {
    pub ifc: Var,
    pub thenc: Rc<Statement>,
    pub elsec: Rc<Statement>,
}

impl Print for IfZ {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc
            .keyword(IFZ)
            .append(alloc.space())
            .append(self.ifc.print(cfg, alloc))
            .append(alloc.space())
            .append(
                alloc
                    .text("()")
                    .append(alloc.space())
                    .append(alloc.text(FAT_ARROW))
                    .append(alloc.space())
                    .append(self.thenc.print(cfg, alloc))
                    .append(COMMA)
                    .append(alloc.space())
                    .append("()")
                    .append(alloc.space())
                    .append(alloc.text(FAT_ARROW))
                    .append(alloc.space())
                    .append(self.elsec.print(cfg, alloc))
                    .braces_anno(),
            )
    }
}

impl From<IfZ> for Statement {
    fn from(value: IfZ) -> Self {
        Statement::IfZ(value)
    }
}

impl FreeVars for IfZ {
    fn free_vars(&self, vars: &mut HashSet<Var>) {
        self.thenc.free_vars(vars);
        self.elsec.free_vars(vars);
        vars.insert(self.ifc.clone());
    }
}

impl Subst for IfZ {
    type Target = IfZ;

    fn subst_sim(self, subst: &[(Var, Var)]) -> IfZ {
        IfZ {
            ifc: self.ifc.subst_sim(subst),
            thenc: self.thenc.subst_sim(subst),
            elsec: self.elsec.subst_sim(subst),
        }
    }
}

impl UsedBinders for IfZ {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        self.thenc.used_binders(used);
        self.elsec.used_binders(used);
    }
}

impl Linearizing for IfZ {
    type Target = IfZ;
    fn linearize(self, context: Vec<Var>, used_vars: &mut HashSet<Var>) -> IfZ {
        IfZ {
            ifc: self.ifc,
            thenc: self.thenc.linearize(context.clone(), used_vars),
            elsec: self.elsec.linearize(context, used_vars),
        }
    }
}

#[cfg(test)]
mod ifz_tests {
    use super::{FreeVars, IfZ, Linearizing, Subst, UsedBinders};
    use crate::syntax::statements::Return;
    use printer::Print;
    use std::{collections::HashSet, rc::Rc};

    fn example_ifz() -> IfZ {
        IfZ {
            ifc: "x".to_owned(),
            thenc: Rc::new(
                Return {
                    var: "y".to_owned(),
                }
                .into(),
            ),
            elsec: Rc::new(
                Return {
                    var: "z".to_owned(),
                }
                .into(),
            ),
        }
    }

    #[test]
    fn print_ifz() {
        let result = example_ifz().print_to_string(Default::default());
        let expected = "ifz x {() => return y, () => return z}";
        assert_eq!(result, expected)
    }

    #[test]
    fn free_vars_ifz() {
        let mut result = HashSet::new();
        example_ifz().free_vars(&mut result);
        let expected = HashSet::from(["x".to_owned(), "y".to_owned(), "z".to_owned()]);
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_ifz() {
        let result = example_ifz().subst_sim(&vec![
            ("x".to_owned(), "y".to_owned()),
            ("y".to_owned(), "z".to_owned()),
            ("z".to_owned(), "x".to_owned()),
        ]);
        let expected = IfZ {
            ifc: "y".to_owned(),
            thenc: Rc::new(
                Return {
                    var: "z".to_owned(),
                }
                .into(),
            ),
            elsec: Rc::new(
                Return {
                    var: "x".to_owned(),
                }
                .into(),
            ),
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn used_binders_ifz() {
        let mut result = HashSet::new();
        example_ifz().used_binders(&mut result);
        let expected = HashSet::new();
        assert_eq!(result, expected)
    }

    #[test]
    fn linearlize_ifz() {
        let result = example_ifz().linearize(vec![], &mut HashSet::new());
        let expected = example_ifz();
        assert_eq!(result, expected)
    }
}
