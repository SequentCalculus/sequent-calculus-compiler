use crate::traits::transform::{NamingTransformation, TransformState};

use super::{
    context::{context_covars, context_vars},
    declaration::{CodataDeclaration, DataDeclaration},
    Def,
};
use std::fmt;

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

impl fmt::Display for Prog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let defs_joined: String = self
            .prog_decls
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "{}", defs_joined)
    }
}

impl fmt::Display for Declaration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Declaration::Definition(def) => def.fmt(f),
            Declaration::DataDeclaration(data) => data.fmt(f),
            Declaration::CodataDeclaration(codata) => codata.fmt(f),
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
            body: Statement::Done(),
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
        let result = format!("{}", example_prog());
        let expected = "data ListInt { Nil, Cons(x : Int, xs : ListInt) }\ncodata StreamInt { hd, tl }\ndef main() := Done;";
        assert_eq!(result, expected)
    }
}

pub fn transform_def(def: Def) -> Def {
    let mut initial_state = TransformState {
        used_vars: context_vars(&def.context),
        used_covars: context_covars(&def.context),
    };

    Def {
        name: def.name,
        context: def.context,
        body: def.body.transform(&mut initial_state),
    }
}

pub fn transform_decl(decl: Declaration) -> Declaration {
    match decl {
        Declaration::Definition(def) => transform_def(def).into(),
        _ => decl,
    }
}

pub fn transform_prog(prog: Prog) -> Prog {
    Prog {
        prog_decls: prog.prog_decls.into_iter().map(transform_decl).collect(),
    }
}

#[cfg(test)]
mod transform_prog_tests {
    use super::{transform_def, transform_prog};
    use crate::syntax::{
        context::ContextBinding,
        program::Declaration,
        statement::Cut,
        term::{Cns, Prd, XVar},
        types::Ty,
        Def, Prog, Statement,
    };
    use std::rc::Rc;

    fn example_def1() -> Def {
        Def {
            name: "done".to_owned(),
            context: vec![],
            body: Statement::Done(),
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
                    }
                    .into(),
                ),
                consumer: Rc::new(
                    XVar {
                        prdcns: Cns,
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
            prog_decls: vec![example_def1().into()],
        }
    }

    #[test]
    fn transform_def1() {
        let result = transform_def(example_def1());
        let expected = example_def1();
        assert_eq!(result.name, expected.name);
        assert_eq!(result.context, expected.context);
        assert_eq!(result.body, expected.body);
    }

    #[test]
    fn transform_def2() {
        let result = transform_def(example_def2());
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
