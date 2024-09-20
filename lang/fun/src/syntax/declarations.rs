use std::{collections::HashSet, fmt};

use crate::syntax::terms::Term;
use crate::syntax::{context::TypingContext, Name, Variable};

use super::types::Ty;

// Definition
//
//

/// A toplevel function definition in a module.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Definition {
    pub name: Name,
    pub context: TypingContext,
    pub body: Term,
    pub ret_ty: Ty,
}

impl fmt::Display for Definition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let args_str: Vec<String> = self.context.iter().map(|bnd| bnd.to_string()).collect();
        write!(
            f,
            "def {}({}) : {} := {};",
            self.name,
            args_str.join(", "),
            self.ret_ty,
            self.body,
        )
    }
}

impl From<Definition> for Declaration {
    fn from(value: Definition) -> Self {
        Declaration::Definition(value)
    }
}

#[cfg(test)]
mod definition_tests {
    use crate::{
        parser::fun,
        syntax::{
            declarations::Module,
            terms::{Lit, Term},
            types::Ty,
        },
    };

    use super::Definition;

    /// A definition with no arguments:
    fn simple_definition() -> Definition {
        Definition {
            name: "x".to_string(),
            context: vec![],
            body: Term::Lit(Lit { val: 4 }),
            ret_ty: Ty::Int(),
        }
    }

    #[test]
    fn display_simple() {
        assert_eq!(
            format!("{}", simple_definition()),
            "def x() : Int := 4;".to_string()
        )
    }

    #[test]
    fn parse_simple() {
        let parser = fun::ProgParser::new();
        let module = Module {
            declarations: vec![simple_definition().into()],
        };
        assert_eq!(parser.parse("def x() : Int := 4;"), Ok(module));
    }
}

// DataDeclaration
//
//

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct CtorSig {
    pub name: Name,
    pub args: Vec<(Variable, Ty)>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct DataDeclaration {
    pub name: Name,
    pub ctors: Vec<CtorSig>,
}

impl From<DataDeclaration> for Declaration {
    fn from(data: DataDeclaration) -> Declaration {
        Declaration::DataDefinition(data)
    }
}

impl fmt::Display for DataDeclaration {
    fn fmt(&self, frmt: &mut fmt::Formatter) -> fmt::Result {
        let ctor_strs: Vec<String> = self.ctors.iter().map(|ctor| format!("{ctor}")).collect();
        frmt.write_str(&format!(
            "data {} {{\n\t{}\n}}",
            self.name,
            ctor_strs.join(",\n\t")
        ))
    }
}

impl fmt::Display for CtorSig {
    fn fmt(&self, frmt: &mut fmt::Formatter) -> fmt::Result {
        let args_strs: Vec<String> = self
            .args
            .iter()
            .map(|(var, ty)| format!("{} : {}", var, ty))
            .collect();
        frmt.write_str(&format!("{}({})", self.name, args_strs.join(", ")))
    }
}

#[cfg(test)]
mod data_declaration_tests {
    use crate::syntax::types::Ty;

    use super::{CtorSig, DataDeclaration};

    /// Lists containing Int
    fn example_list() -> DataDeclaration {
        let nil = CtorSig {
            name: "Nil".to_owned(),
            args: vec![],
        };
        let cons = CtorSig {
            name: "Cons".to_owned(),
            args: vec![
                ("x".to_owned(), Ty::Int()),
                ("xs".to_owned(), Ty::Decl("ListInt".to_owned())),
            ],
        };

        DataDeclaration {
            name: "ListInt".to_owned(),
            ctors: vec![nil, cons],
        }
    }

    #[test]
    fn display_list() {
        let result = format!("{}", example_list());
        let expected = "data ListInt {\n\tNil(),\n\tCons(x : Int, xs : ListInt)\n}";
        assert_eq!(result, expected)
    }
}

// CodataDefinition
//
//

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct DtorSig {
    pub name: Name,
    pub args: Vec<(Variable, Ty)>,
    pub cont_ty: Ty,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct CodataDefinition {
    pub name: Name,
    pub dtors: Vec<DtorSig>,
}

impl From<CodataDefinition> for Declaration {
    fn from(codata: CodataDefinition) -> Declaration {
        Declaration::CodataDefinition(codata)
    }
}

impl fmt::Display for CodataDefinition {
    fn fmt(&self, frmt: &mut fmt::Formatter) -> fmt::Result {
        let dtor_strs: Vec<String> = self.dtors.iter().map(|dtor| format!("{dtor}")).collect();
        frmt.write_str(&format!(
            "codata {} {{\n\t{}\n}}",
            self.name,
            dtor_strs.join(",\n\t"),
        ))
    }
}

impl fmt::Display for DtorSig {
    fn fmt(&self, frmt: &mut fmt::Formatter) -> fmt::Result {
        let args_strs: Vec<String> = self
            .args
            .iter()
            .map(|(var, ty)| format!("{} : {}", var, ty))
            .collect();
        frmt.write_str(&format!(
            "{}({}) : {}",
            self.name,
            args_strs.join(", "),
            self.cont_ty
        ))
    }
}

#[cfg(test)]
mod codata_declaration_tests {
    use crate::syntax::types::Ty;

    use super::{CodataDefinition, DtorSig};

    // Streams
    fn example_stream() -> CodataDefinition {
        let hd = DtorSig {
            name: "hd".to_owned(),
            args: vec![],
            cont_ty: Ty::Int(),
        };
        let tl = DtorSig {
            name: "tl".to_owned(),
            args: vec![],
            cont_ty: Ty::Decl("IntStream".to_owned()),
        };

        CodataDefinition {
            name: "IntStream".to_owned(),
            dtors: vec![hd, tl],
        }
    }

    #[test]
    fn display_stream() {
        let result = format!("{}", example_stream());
        let expected = "codata IntStream {\n\thd() : Int,\n\ttl() : IntStream\n}";
        assert_eq!(result, expected)
    }

    // Functions from Int to Int
    fn example_fun() -> CodataDefinition {
        let ap = DtorSig {
            name: "ap".to_owned(),
            args: vec![("x".to_owned(), Ty::Int())],
            cont_ty: Ty::Int(),
        };

        CodataDefinition {
            name: "Fun".to_owned(),
            dtors: vec![ap],
        }
    }

    #[test]
    fn display_fun() {
        let result = format!("{}", example_fun());
        let expected = "codata Fun {\n\tap(x : Int) : Int\n}";
        assert_eq!(result, expected)
    }
}

// Declaration
//
//

/// A toplevel declaration in a module
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Declaration {
    Definition(Definition),
    DataDefinition(DataDeclaration),
    CodataDefinition(CodataDefinition),
}

impl fmt::Display for Declaration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Declaration::Definition(d) => d.fmt(f),
            Declaration::DataDefinition(d) => d.fmt(f),
            Declaration::CodataDefinition(c) => c.fmt(f),
        }
    }
}

// Module
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Module {
    pub declarations: Vec<Declaration>,
}

impl Module {
    pub fn data_types(&self) -> HashSet<Name> {
        let mut names = HashSet::new();

        for decl in &self.declarations {
            if let Declaration::DataDefinition(data) = decl {
                names.insert(data.name.clone());
            }
        }

        names
    }

    pub fn codata_types(&self) -> HashSet<Name> {
        let mut names = HashSet::new();

        for decl in &self.declarations {
            if let Declaration::CodataDefinition(codata) = decl {
                names.insert(codata.name.clone());
            }
        }
        names
    }
}

impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let defs_joined: String = self
            .declarations
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "{}", defs_joined)
    }
}

#[cfg(test)]
mod module_tests {
    use super::{Definition, Module, Term};
    use crate::{
        parser::fun,
        syntax::{context::ContextBinding, terms::Lit, types::Ty},
    };
    use std::collections::HashSet;

    // Program with one definition without arguments
    //
    //

    fn example_simple() -> Module {
        Module {
            declarations: vec![Definition {
                name: "x".to_string(),
                context: vec![],
                body: Term::Lit(Lit { val: 4 }),
                ret_ty: Ty::Int(),
            }
            .into()],
        }
    }

    #[test]
    fn display_simple() {
        assert_eq!(
            format!("{}", example_simple()),
            "def x() : Int := 4;".to_string()
        )
    }

    #[test]
    fn parse_simple() {
        let parser = fun::ProgParser::new();
        assert_eq!(
            parser.parse("def x() : Int := 4;"),
            Ok(example_simple().into())
        );
    }

    #[test]
    fn data_simple() {
        let result = example_simple().data_types();
        let expected = HashSet::new();
        assert_eq!(result, expected)
    }

    #[test]
    fn codata_simple() {
        let result = example_simple().codata_types();
        let expected = HashSet::new();
        assert_eq!(result, expected)
    }

    // Program with one definition which takes arguments
    //
    //

    fn example_args() -> Module {
        Module {
            declarations: vec![Definition {
                name: "f".to_string(),
                context: vec![
                    ContextBinding::TypedVar {
                        var: "x".to_string(),
                        ty: Ty::Int(),
                    },
                    ContextBinding::TypedCovar {
                        covar: "a".to_owned(),
                        ty: Ty::Int(),
                    },
                ],
                body: Term::Lit(Lit { val: 4 }),
                ret_ty: Ty::Int(),
            }
            .into()],
        }
    }

    #[test]
    fn display_args() {
        assert_eq!(
            format!("{}", example_args()),
            "def f(x : Int, 'a :cnt Int) : Int := 4;".to_string(),
        )
    }

    #[test]
    fn parse_args() {
        let parser = fun::ProgParser::new();
        assert_eq!(
            parser.parse("def f(x : Int, 'a :cnt Int) : Int := 4;"),
            Ok(example_args().into())
        )
    }

    // Program with two definitions
    //
    //

    fn example_two() -> Module {
        let d1 = Definition {
            name: "f".to_string(),
            context: vec![],
            body: Term::Lit(Lit { val: 2 }),
            ret_ty: Ty::Int(),
        };

        let d2 = Definition {
            name: "g".to_string(),
            context: vec![],
            body: Term::Lit(Lit { val: 4 }),
            ret_ty: Ty::Int(),
        };
        Module {
            declarations: vec![d1.into(), d2.into()],
        }
    }

    #[test]
    fn display_two() {
        assert_eq!(
            format!("{}", example_two()),
            "def f() : Int := 2;\ndef g() : Int := 4;".to_string(),
        )
    }

    #[test]
    fn parse_two() {
        let parser = fun::ProgParser::new();
        assert_eq!(
            parser.parse("def f() : Int := 2;\n def g() : Int := 4;"),
            Ok(example_two().into())
        )
    }
}
