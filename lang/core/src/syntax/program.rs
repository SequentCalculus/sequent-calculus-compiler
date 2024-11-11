use printer::{DocAllocator, Print};

use crate::traits::focus::{Focusing, FocusingState};

use super::{
    context::{context_covars, context_vars},
    declaration::{CodataDeclaration, DataDeclaration},
    Def,
};

// Prog
//
//

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

pub fn transform_def(def: Def, st: &mut FocusingState) -> Def {
    st.used_vars = context_vars(&def.context);
    st.used_covars = context_covars(&def.context).into_keys().collect();

    Def {
        name: def.name,
        context: def.context,
        body: def.body.focus(st),
    }
}

pub fn transform_decl(decl: Declaration, st: &mut FocusingState) -> Declaration {
    match decl {
        Declaration::Definition(def) => transform_def(def, st).into(),
        Declaration::DataDeclaration(data) => Declaration::DataDeclaration(data),
        Declaration::CodataDeclaration(codata) => Declaration::CodataDeclaration(codata),
    }
}

pub fn transform_prog(prog: Prog) -> Prog {
    let mut st = FocusingState::default();
    Prog {
        prog_decls: prog
            .prog_decls
            .into_iter()
            .map(|decl| transform_decl(decl, &mut st))
            .collect(),
    }
}

#[cfg(test)]
mod transform_prog_tests {
    use super::{transform_def, transform_prog, FocusingState};
    use crate::syntax::{
        context::ContextBinding,
        program::Declaration,
        statement::Cut,
        term::{Cns, Prd, XVar},
        types::Ty,
        Def, Prog, Statement,
    };

    fn example_def1() -> Def {
        Def {
            name: "done".to_owned(),
            context: vec![],
            body: Statement::Done(Ty::Int()),
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
            body: Cut::new(
                XVar {
                    prdcns: Prd,
                    var: "x".to_owned(),
                    ty: Ty::Int(),
                },
                Ty::Int(),
                XVar {
                    prdcns: Cns,
                    var: "a".to_owned(),
                    ty: Ty::Int(),
                },
            )
            .into(),
        }
    }

    fn example_prog1() -> Prog {
        Prog { prog_decls: vec![] }
    }

    fn example_prog2() -> Prog {
        Prog {
            prog_decls: vec![example_def1().into()],
        }
    }

    #[test]
    fn transform_def1() {
        let result = transform_def(example_def1(), &mut FocusingState::default());
        let expected = example_def1();
        assert_eq!(result.name, expected.name);
        assert_eq!(result.context, expected.context);
        assert_eq!(result.body, expected.body);
    }

    #[test]
    fn transform_def2() {
        let result = transform_def(example_def2(), &mut FocusingState::default());
        let expected = example_def2();
        assert_eq!(result.name, expected.name);
        assert_eq!(result.context, expected.context);
        assert_eq!(result.body, expected.body);
    }

    #[test]
    fn transform_prog1() {
        let result = transform_prog(example_prog1());
        assert!(result.prog_decls.is_empty())
    }

    #[test]
    fn transform_prog2() {
        let result = transform_prog(example_prog2());
        assert_eq!(result.prog_decls.len(), 1);
        let def1 = result.prog_decls.get(0);
        assert!(def1.is_some());
        let def1un = def1.unwrap();
        let def = if let Declaration::Definition(def) = def1un {
            Some(def)
        } else {
            None
        }
        .unwrap();
        let ex = example_def1();
        assert_eq!(def.name, ex.name);
        assert_eq!(def.context, ex.context);
        assert_eq!(def.body, ex.body);
    }
}
