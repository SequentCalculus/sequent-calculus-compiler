use printer::{DocAllocator, Print};

use crate::traits::focus::{Focusing, FocusingState};

use super::{
    context::{context_covars, context_vars},
    declaration::{CodataDeclaration, DataDeclaration},
    Def,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Declaration {
    Definition(Def),
    DataDeclaration(DataDeclaration),
    CodataDeclaration(CodataDeclaration),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Prog {
    pub prog_decls: Vec<Declaration>,
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

        let decls = self.prog_decls.iter().map(|decl| decl.print(cfg, alloc));

        alloc.intersperse(decls, sep).append(alloc.line())
    }
}

impl Print for Declaration {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        match self {
            Declaration::Definition(def) => def.print(cfg, alloc),
            Declaration::DataDeclaration(data) => data.print(cfg, alloc),
            Declaration::CodataDeclaration(codata) => codata.print(cfg, alloc),
        }
    }
}

impl From<Def> for Declaration {
    fn from(def: Def) -> Declaration {
        Declaration::Definition(def)
    }
}

impl From<DataDeclaration> for Declaration {
    fn from(data: DataDeclaration) -> Declaration {
        Declaration::DataDeclaration(data)
    }
}

impl From<CodataDeclaration> for Declaration {
    fn from(codata: CodataDeclaration) -> Declaration {
        Declaration::CodataDeclaration(codata)
    }
}

#[cfg(test)]
mod program_tests {
    use printer::Print;

    use super::{CodataDeclaration, DataDeclaration, Def, Prog};
    use crate::syntax::{
        context::ContextBinding,
        declaration::{Codata, Data, TypeDeclaration, XtorSig},
        types::Ty,
        Statement,
    };

    fn example_def() -> Def {
        Def {
            name: "main".to_owned(),
            context: vec![],
            body: Statement::Done(Ty::Int()),
        }
    }
    fn example_data() -> DataDeclaration {
        TypeDeclaration {
            dat: Data,
            name: "ListInt".to_owned(),
            xtors: vec![
                XtorSig {
                    xtor: Data,
                    name: "Nil".to_owned(),
                    args: vec![],
                },
                XtorSig {
                    xtor: Data,
                    name: "Cons".to_owned(),
                    args: vec![
                        ContextBinding::VarBinding {
                            var: "x".to_owned(),
                            ty: Ty::Int(),
                        },
                        ContextBinding::VarBinding {
                            var: "xs".to_owned(),
                            ty: Ty::Decl("ListInt".to_owned()),
                        },
                    ],
                },
            ],
        }
    }
    fn example_codata() -> CodataDeclaration {
        TypeDeclaration {
            dat: Codata,
            name: "StreamInt".to_owned(),
            xtors: vec![
                XtorSig {
                    xtor: Codata,
                    name: "hd".to_owned(),
                    args: vec![],
                },
                XtorSig {
                    xtor: Codata,
                    name: "tl".to_owned(),
                    args: vec![],
                },
            ],
        }
    }

    fn example_prog() -> Prog {
        Prog {
            prog_decls: vec![
                example_data().into(),
                example_codata().into(),
                example_def().into(),
            ],
        }
    }

    #[test]
    fn display_prog() {
        let result = example_prog().print_to_string(None);
        let expected = "data ListInt { Nil, Cons(x : Int, xs : ListInt) }\n\ncodata StreamInt { hd, tl }\n\ndef main() := Done;\n";
        assert_eq!(result, expected)
    }
}

#[must_use]
pub fn transform_prog(prog: Prog) -> crate::syntax_var::Prog {
    let mut defs = Vec::new();
    let mut data_types = Vec::new();
    let mut codata_types = Vec::new();
    for decl in prog.prog_decls {
        match decl {
            Declaration::Definition(def) => defs.push(def),
            Declaration::DataDeclaration(typ) => data_types.push(typ),
            Declaration::CodataDeclaration(typ) => codata_types.push(typ),
        }
    }

    let codata_types_clone = codata_types.clone();
    let mut state = FocusingState {
        codata_types: codata_types_clone.as_slice(),
        ..FocusingState::default()
    };

    crate::syntax_var::Prog {
        defs: defs
            .into_iter()
            .map(|def| {
                let mut used_vars = context_vars(&def.context);
                used_vars.extend(context_covars(&def.context));
                state.used_vars = used_vars;
                def.focus(&mut state)
            })
            .collect(),
        types: [data_types.focus(&mut state), codata_types.focus(&mut state)].concat(),
    }
}

#[cfg(test)]
mod transform_prog_tests {
    use super::transform_prog;
    use crate::syntax::{
        context::ContextBinding,
        statement::Cut,
        term::{Cns, Prd, XVar},
        types::Ty,
        Def, Prog, Statement,
    };
    use crate::syntax_var::Chirality;
    use std::rc::Rc;

    fn example_def1() -> Def {
        Def {
            name: "done".to_owned(),
            context: vec![],
            body: Statement::Done(Ty::Int()),
        }
    }
    fn example_def1_var() -> crate::syntax_var::Def {
        crate::syntax_var::Def {
            name: "done".to_owned(),
            context: vec![],
            body: crate::syntax_var::Statement::Done(),
        }
    }

    fn example_def2() -> Def {
        Def {
            name: "cut".to_owned(),
            context: vec![
                ContextBinding::VarBinding {
                    var: "x".to_owned(),
                    ty: Ty::Int(),
                },
                ContextBinding::CovarBinding {
                    covar: "a".to_owned(),
                    ty: Ty::Int(),
                },
            ],
            body: Cut {
                producer: Rc::new(
                    XVar {
                        prdcns: Prd,
                        var: "x".to_owned(),
                        ty: Ty::Int(),
                    }
                    .into(),
                ),
                ty: Ty::Int(),
                consumer: Rc::new(
                    XVar {
                        prdcns: Cns,
                        var: "a".to_owned(),
                        ty: Ty::Int(),
                    }
                    .into(),
                ),
            }
            .into(),
        }
    }
    fn example_def2_var() -> crate::syntax_var::Def {
        crate::syntax_var::Def {
            name: "cut".to_owned(),
            context: vec![
                crate::syntax_var::ContextBinding {
                    chi: Chirality::Prd,
                    var: "x".to_owned(),
                    ty: crate::syntax_var::Ty::Int,
                },
                crate::syntax_var::ContextBinding {
                    chi: Chirality::Cns,
                    var: "a".to_owned(),
                    ty: crate::syntax_var::Ty::Int,
                },
            ],
            body: crate::syntax_var::statement::Cut {
                producer: Rc::new(
                    crate::syntax_var::term::XVar {
                        chi: Chirality::Prd,
                        var: "x".to_owned(),
                    }
                    .into(),
                ),
                ty: crate::syntax_var::Ty::Int,
                consumer: Rc::new(
                    crate::syntax_var::term::XVar {
                        chi: Chirality::Cns,
                        var: "a".to_owned(),
                    }
                    .into(),
                ),
            }
            .into(),
        }
    }

    fn example_prog1() -> Prog {
        Prog { prog_decls: vec![] }
    }

    fn example_prog2() -> Prog {
        Prog {
            prog_decls: vec![example_def1().into(), example_def2().into()],
        }
    }

    #[test]
    fn transform_prog1() {
        let result = transform_prog(example_prog1());
        assert!(result.defs.is_empty())
    }

    #[test]
    fn transform_prog2() {
        let result = transform_prog(example_prog2());
        assert_eq!(result.defs.len(), 2);
        let def1 = result.defs.get(0);
        let def2 = result.defs.get(1);
        assert!(def1.is_some());
        assert!(def2.is_some());
        let def1 = def1.unwrap();
        let def2 = def2.unwrap();
        let ex1 = example_def1_var();
        assert_eq!(def1.name, ex1.name);
        assert_eq!(def1.context, ex1.context);
        assert_eq!(def1.body, ex1.body);
        let ex2 = example_def2_var();
        assert_eq!(def2.name, ex2.name);
        assert_eq!(def2.context, ex2.context);
        assert_eq!(def2.body, ex2.body);
    }
}
