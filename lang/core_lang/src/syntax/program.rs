use printer::{DocAllocator, Print};

use crate::traits::*;

use super::{
    declaration::{CodataDeclaration, DataDeclaration, FsTypeDeclaration},
    def::FsDef,
    Def,
};

use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq)]
pub struct Prog {
    pub defs: Vec<Def>,
    pub data_types: Vec<DataDeclaration>,
    pub codata_types: Vec<CodataDeclaration>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FsProg {
    pub defs: Vec<FsDef>,
    pub types: Vec<FsTypeDeclaration>,
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

        let defs = self.defs.iter().map(|def| def.print(cfg, alloc));
        let data_types = self.data_types.iter().map(|typ| typ.print(cfg, alloc));
        let codata_types = self.codata_types.iter().map(|typ| typ.print(cfg, alloc));

        alloc
            .intersperse(data_types, alloc.line())
            .append(alloc.line())
            .append(alloc.intersperse(codata_types, alloc.line()))
            .append(sep.clone())
            .append(alloc.intersperse(defs, sep))
    }
}

impl Print for FsProg {
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

        let defs = self.defs.iter().map(|def| def.print(cfg, alloc));
        let types = self.types.iter().map(|typ| typ.print(cfg, alloc));

        alloc
            .intersperse(types, alloc.line())
            .append(sep.clone())
            .append(alloc.intersperse(defs, sep))
    }
}

#[must_use]
pub fn transform_prog(prog: Prog) -> FsProg {
    let codata_types_clone = prog.codata_types.clone();
    let mut state = FocusingState {
        codata_types: codata_types_clone.as_slice(),
        ..FocusingState::default()
    };

    FsProg {
        defs: prog
            .defs
            .into_iter()
            .map(|mut def| {
                let mut used_vars = HashSet::new();
                def.body.used_binders(&mut used_vars);
                used_vars.extend(def.context.vars());
                used_vars.extend(def.context.covars());

                let mut seen_vars = def.context.vars();
                seen_vars.extend(def.context.covars());

                def.body = def.body.uniquify(&mut seen_vars, &mut used_vars);

                state.used_vars = used_vars;
                def.focus(&mut state)
            })
            .collect(),
        types: [
            prog.data_types.focus(&mut state),
            prog.codata_types.focus(&mut state),
        ]
        .concat(),
    }
}

#[cfg(test)]
mod program_tests {
    use std::{collections::HashSet, rc::Rc};

    use super::{Def, Prog};
    use crate::syntax::{
        context::{Context, ContextBinding, FsContextBinding},
        def::FsDef,
        program::{transform_prog, FsProg},
        statement::{Cut, FsCut},
        term::{FsXVar, XVar},
        types::Ty,
        Chirality,
    };

    fn example_def2_var() -> FsDef {
        FsDef {
            name: "cut".to_owned(),
            context: Context {
                bindings: vec![
                    FsContextBinding {
                        chi: Chirality::Prd,
                        var: "x".to_owned(),
                        ty: Ty::Int,
                    },
                    FsContextBinding {
                        chi: Chirality::Cns,
                        var: "a".to_owned(),
                        ty: Ty::Int,
                    },
                ],
            },
            body: FsCut {
                producer: Rc::new(
                    FsXVar {
                        chi: Chirality::Prd,
                        var: "x".to_owned(),
                    }
                    .into(),
                ),
                ty: Ty::Int,
                consumer: Rc::new(
                    FsXVar {
                        chi: Chirality::Cns,
                        var: "a".to_owned(),
                    }
                    .into(),
                ),
            }
            .into(),
            used_vars: HashSet::from(["a".to_owned(), "x".to_owned()]),
        }
    }

    #[test]
    fn transform_prog2() {
        let prog = Prog {
            defs: vec![Def {
                name: "cut".to_owned(),
                context: Context {
                    bindings: vec![
                        ContextBinding::VarBinding {
                            var: "x".to_owned(),
                            ty: Ty::Int,
                        },
                        ContextBinding::CovarBinding {
                            covar: "a".to_owned(),
                            ty: Ty::Int,
                        },
                    ],
                },
                body: Cut {
                    producer: Rc::new(XVar::var("x", Ty::Int).into()),
                    ty: Ty::Int,
                    consumer: Rc::new(XVar::covar("a", Ty::Int).into()),
                }
                .into(),
            }
            .into()],
            data_types: vec![],
            codata_types: vec![],
        };
        let result = transform_prog(prog);

        let expected = FsProg {
            defs: vec![example_def2_var()],
            types: vec![],
        };
        assert_eq!(result, expected)
    }
}
