use super::{
    declaration::{CodataDeclaration, DataDeclaration},
    Def,
};
use std::fmt;

// Prog
//
//

#[derive(Debug, Clone)]
pub enum Declaration {
    Definition(Def),
    DataDeclaration(DataDeclaration),
    CodataDeclaration(CodataDeclaration),
}

#[derive(Debug, Clone)]
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
