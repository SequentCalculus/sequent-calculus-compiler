use printer::{DocAllocator, Print};

use super::{context::context_vars, Def, TypeDeclaration};
use crate::traits::linearize::{Linearizing, UsedBinders};

use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Prog {
    pub defs: Vec<Def>,
    pub types: Vec<TypeDeclaration>,
}

impl Print for Prog {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        // We usually separate declarations with an empty line, except when the `omit_decl_sep` option is set.
        // This is useful for typesetting examples in papers which have to make economic use of vertical space.
        let sep = if cfg.omit_decl_sep {
            alloc.line()
        } else {
            alloc.line().append(alloc.line())
        };

        let types = self.types.iter().map(|typ| typ.print(cfg, alloc));
        let defs = self.defs.iter().map(|def| def.print(cfg, alloc));

        alloc
            .intersperse(types, alloc.line())
            .append(sep.clone())
            .append(alloc.intersperse(defs, sep))
    }
}

#[must_use]
pub fn linearize(program: Prog) -> Prog {
    Prog {
        defs: program
            .defs
            .into_iter()
            .map(|def| {
                let context = context_vars(&def.context);
                let mut used_vars = HashSet::new();
                def.body.used_binders(&mut used_vars);
                for var in &context {
                    used_vars.insert(var.clone());
                }
                def.linearize(context, &mut used_vars)
            })
            .collect(),
        types: program.types,
    }
}

#[cfg(test)]
mod prog_tests {
    use super::{linearize, Def, Prog, TypeDeclaration};
    use crate::syntax::{
        context::ContextBinding, declaration::XtorSig, statements::Return, types::Ty, Chirality,
    };
    use printer::Print;

    fn example_prog() -> Prog {
        Prog {
            defs: vec![Def {
                name: "main".to_owned(),
                context: vec![],
                body: Return {
                    var: "x".to_owned(),
                }
                .into(),
            }],
            types: vec![TypeDeclaration {
                name: "ListInt".to_owned(),
                xtors: vec![
                    XtorSig {
                        name: "Nil".to_owned(),
                        args: vec![],
                    },
                    XtorSig {
                        name: "Cons".to_owned(),
                        args: vec![
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
                    },
                ],
            }],
        }
    }

    #[test]
    fn print_prog() {
        let result = example_prog().print_to_string(Default::default());
        let expected =
            "type ListInt {Nil(), Cons(x :prd: Int, xs :prd: ListInt)}\n\ndef main() := return x";
        assert_eq!(result, expected)
    }

    #[test]
    fn linearize_prog() {
        let result = linearize(example_prog());
        let expected = example_prog();
        assert_eq!(result, expected)
    }
}
