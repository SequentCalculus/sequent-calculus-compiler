use super::{Name, Statement, TypingContext, Var};
use printer::{
    theme::ThemeExt,
    tokens::{COLONEQ, DEF},
    DocAllocator, Print,
};

use crate::traits::linearize::Linearizing;

use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Def {
    pub name: Name,
    pub context: TypingContext,
    pub body: Statement,
}

impl Linearizing for Def {
    type Target = Def;
    fn linearize(self, context: Vec<Var>, used_vars: &mut HashSet<Var>) -> Def {
        Def {
            name: self.name,
            context: self.context,
            body: self.body.linearize(context, used_vars),
        }
    }
}

impl Print for Def {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc
            .keyword(DEF)
            .append(alloc.space())
            .append(&self.name)
            .append(self.context.print(cfg, alloc).parens())
            .append(alloc.space())
            .append(COLONEQ)
            .append(alloc.space())
            .append(self.body.print(cfg, alloc))
    }
}

#[cfg(test)]
mod def_tests {
    use super::{Def, Linearizing};
    use crate::syntax::{
        context::ContextBinding,
        statements::{Op, Return},
        types::Ty,
        BinOp, Chirality,
    };
    use printer::Print;
    use std::{collections::HashSet, rc::Rc};

    fn example_main() -> Def {
        Def {
            name: "main".to_owned(),
            context: vec![],
            body: Return {
                var: "x".to_owned(),
            }
            .into(),
        }
    }

    fn example_mult() -> Def {
        Def {
            name: "mult".to_owned(),
            context: vec![
                ContextBinding {
                    var: "x".to_owned(),
                    chi: Chirality::Prd,
                    ty: Ty::Int,
                },
                ContextBinding {
                    var: "y".to_owned(),
                    chi: Chirality::Prd,
                    ty: Ty::Int,
                },
            ],
            body: Op {
                fst: "x".to_owned(),
                op: BinOp::Sum,
                snd: "y".to_owned(),
                var: "z".to_owned(),
                case: Rc::new(
                    Return {
                        var: "z".to_owned(),
                    }
                    .into(),
                ),
            }
            .into(),
        }
    }

    #[test]
    fn linearize_main() {
        let result = example_main().linearize(vec![], &mut HashSet::new());
        let expected = example_main();
        assert_eq!(result, expected)
    }

    #[test]
    fn linearize_mult() {
        let result = example_mult().linearize(vec![], &mut HashSet::new());
        let expected = Def {
            name: "mult".to_owned(),
            context: vec![
                ContextBinding {
                    var: "x".to_owned(),
                    chi: Chirality::Prd,
                    ty: Ty::Int,
                },
                ContextBinding {
                    var: "y".to_owned(),
                    chi: Chirality::Prd,
                    ty: Ty::Int,
                },
            ],
            body: Op {
                fst: "x".to_owned(),
                op: BinOp::Sum,
                snd: "y".to_owned(),
                var: "z".to_owned(),
                case: Rc::new(
                    Return {
                        var: "z".to_owned(),
                    }
                    .into(),
                ),
            }
            .into(),
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn print_main() {
        let result = example_main().print_to_string(Default::default());
        let expected = "def main() := return x";
        assert_eq!(result, expected)
    }

    #[test]
    fn print_mult() {
        let result = example_mult().print_to_string(Default::default());
        let expected = "def mult(x :prd: Int, y :prd: Int) := z <- x + y; return z";
        assert_eq!(result, expected)
    }
}
