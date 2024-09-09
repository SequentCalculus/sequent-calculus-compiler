use std::{collections::HashSet, fmt};

use crate::syntax::terms::Term;
use crate::syntax::{Covariable, Name, Variable};

use super::types::Ty;

// Def
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Def {
    pub name: Name,
    pub args: Vec<(Variable, ())>,
    pub cont: Vec<(Covariable, ())>,
    pub body: Term,
    pub ret_ty: (),
}

impl fmt::Display for Def {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let args_str: Vec<String> = self.args.iter().map(|(x, _)| x.to_string()).collect();
        let cont_str: Vec<String> = self.cont.iter().map(|(x, _)| x.to_string()).collect();
        write!(
            f,
            "def {}({}; {}) := {};",
            self.name,
            args_str.join(", "),
            cont_str.join(", "),
            self.body
        )
    }
}

impl From<Def> for Decl {
    fn from(value: Def) -> Self {
        Decl::Def(value)
    }
}

// Decl
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Decl {
    Def(Def),
    DataDefinition(DataDefinition),
    CodataDefinition(CodataDefinition),
}

impl fmt::Display for Decl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Decl::Def(d) => d.fmt(f),
            Decl::DataDefinition(d) => d.fmt(f),
            Decl::CodataDefinition(c) => c.fmt(f),
        }
    }
}

// Prog
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Prog {
    pub prog_defs: Vec<Decl>,
}

impl Prog {
    pub fn data_types(&self) -> HashSet<Name> {
        let mut names = HashSet::new();

        for decl in &self.prog_defs {
            if let Decl::DataDefinition(data) = decl {
                names.insert(data.name.clone());
            }
        }

        names
    }

    pub fn codata_types(&self) -> HashSet<Name> {
        let mut names = HashSet::new();

        for decl in &self.prog_defs {
            if let Decl::CodataDefinition(codata) = decl {
                names.insert(codata.name.clone());
            }
        }
        names
    }
}

impl fmt::Display for Prog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let defs_joined: String = self
            .prog_defs
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "{}", defs_joined)
    }
}

#[cfg(test)]
mod prog_tests {
    use super::{Def, Prog, Term};
    use crate::parser::fun;
    use std::collections::HashSet;

    // Empty program
    //
    //

    fn example_empty() -> Prog {
        Prog { prog_defs: vec![] }
    }

    #[test]
    fn display_empty() {
        assert_eq!(format!("{}", example_empty()), "".to_string())
    }

    #[test]
    fn parse_empty() {
        let parser = fun::ProgParser::new();
        assert_eq!(parser.parse(" "), Ok(example_empty().into()));
    }

    // Program with one definition without arguments
    //
    //

    fn example_simple() -> Prog {
        Prog {
            prog_defs: vec![Def {
                name: "x".to_string(),
                args: vec![],
                cont: vec![],
                body: Term::Lit(4),
                ret_ty: (),
            }
            .into()],
        }
    }

    #[test]
    fn display_simple() {
        assert_eq!(
            format!("{}", example_simple()),
            "def x(; ) := 4;".to_string()
        )
    }

    #[test]
    fn parse_simple() {
        let parser = fun::ProgParser::new();
        assert_eq!(parser.parse("def x(; ) := 4;"), Ok(example_simple().into()));
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

    fn example_args() -> Prog {
        Prog {
            prog_defs: vec![Def {
                name: "f".to_string(),
                args: vec![("x".to_string(), ())],
                cont: vec![("a".to_string(), ())],
                body: Term::Lit(4),
                ret_ty: (),
            }
            .into()],
        }
    }

    #[test]
    fn display_args() {
        assert_eq!(
            format!("{}", example_args()),
            "def f(x; a) := 4;".to_string(),
        )
    }

    #[test]
    fn parse_args() {
        let parser = fun::ProgParser::new();
        assert_eq!(parser.parse("def f(x; a) := 4;"), Ok(example_args().into()))
    }

    // Program with two definitions
    //
    //

    fn example_two() -> Prog {
        let d1 = Def {
            name: "f".to_string(),
            args: vec![],
            cont: vec![],
            body: Term::Lit(2),
            ret_ty: (),
        };

        let d2 = Def {
            name: "g".to_string(),
            args: vec![],
            cont: vec![],
            body: Term::Lit(4),
            ret_ty: (),
        };
        Prog {
            prog_defs: vec![d1.into(), d2.into()],
        }
    }

    #[test]
    fn display_two() {
        assert_eq!(
            format!("{}", example_two()),
            "def f(; ) := 2;\ndef g(; ) := 4;".to_string(),
        )
    }

    #[test]
    fn parse_two() {
        let parser = fun::ProgParser::new();
        assert_eq!(
            parser.parse("def f(; ) := 2;\n def g(; ) := 4;"),
            Ok(example_two().into())
        )
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct DataDefinition {
    pub name: Name,
    pub ctors: Vec<CtorSig>,
}

impl From<DataDefinition> for Decl {
    fn from(data: DataDefinition) -> Decl {
        Decl::DataDefinition(data)
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct CodataDefinition {
    pub name: Name,
    pub dtors: Vec<DtorSig>,
}

impl From<CodataDefinition> for Decl {
    fn from(codata: CodataDefinition) -> Decl {
        Decl::CodataDefinition(codata)
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct CtorSig {
    pub name: Name,
    pub args: Vec<(Variable, Ty)>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct DtorSig {
    pub name: Name,
    pub args: Vec<(Variable, Ty)>,
    pub cont_ty: Ty,
}

impl fmt::Display for DataDefinition {
    fn fmt(&self, frmt: &mut fmt::Formatter) -> fmt::Result {
        let ctor_strs: Vec<String> = self.ctors.iter().map(|ctor| format!("{ctor}")).collect();
        frmt.write_str(&format!(
            "data {} {{\n\t{}\n}}",
            self.name,
            ctor_strs.join(",\n\t")
        ))
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
mod typedef_tests {
    use crate::syntax::types::Ty;

    use super::{CodataDefinition, CtorSig, DataDefinition, DtorSig};

    fn example_nil() -> CtorSig {
        CtorSig {
            name: "Nil".to_owned(),
            args: vec![],
        }
    }

    fn example_cons() -> CtorSig {
        CtorSig {
            name: "Cons".to_owned(),
            args: vec![
                ("x".to_owned(), Ty::Int()),
                ("xs".to_owned(), Ty::Decl("Listint".to_owned())),
            ],
        }
    }

    fn example_tup() -> CtorSig {
        CtorSig {
            name: "Tup".to_owned(),
            args: vec![("x".to_owned(), Ty::Int()), ("y".to_owned(), Ty::Int())],
        }
    }

    fn example_list() -> DataDefinition {
        DataDefinition {
            name: "Listint".to_owned(),
            ctors: vec![example_nil(), example_cons()],
        }
    }

    fn example_pair() -> DataDefinition {
        DataDefinition {
            name: "Pair".to_owned(),
            ctors: vec![example_tup()],
        }
    }

    fn example_hd() -> DtorSig {
        DtorSig {
            name: "hd".to_owned(),
            args: vec![],
            cont_ty: Ty::Int(),
        }
    }

    fn example_tl() -> DtorSig {
        DtorSig {
            name: "tl".to_owned(),
            args: vec![],
            cont_ty: Ty::Decl("Streamint".to_owned()),
        }
    }

    fn example_fst() -> DtorSig {
        DtorSig {
            name: "fst".to_owned(),
            args: vec![],
            cont_ty: Ty::Int(),
        }
    }

    fn example_snd() -> DtorSig {
        DtorSig {
            name: "snd".to_owned(),
            args: vec![],
            cont_ty: Ty::Int(),
        }
    }

    fn example_ap() -> DtorSig {
        DtorSig {
            name: "ap".to_owned(),
            args: vec![("x".to_owned(), Ty::Int())],
            cont_ty: Ty::Int(),
        }
    }

    fn example_stream() -> CodataDefinition {
        CodataDefinition {
            name: "Streamint".to_owned(),
            dtors: vec![example_hd(), example_tl()],
        }
    }

    fn example_lpair() -> CodataDefinition {
        CodataDefinition {
            name: "LPair".to_owned(),
            dtors: vec![example_fst(), example_snd()],
        }
    }

    fn example_fun() -> CodataDefinition {
        CodataDefinition {
            name: "Fun".to_owned(),
            dtors: vec![example_ap()],
        }
    }

    #[test]
    fn display_nil() {
        let result = format!("{}", example_nil());
        let expected = "Nil()";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_cons() {
        let result = format!("{}", example_cons());
        let expected = "Cons(x : Int, xs : Listint)";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_tup() {
        let result = format!("{}", example_tup());
        let expected = "Tup(x : Int, y : Int)";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_list() {
        let result = format!("{}", example_list());
        let expected = "data Listint {\n\tNil(),\n\tCons(x : Int, xs : Listint)\n}";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_pair() {
        let result = format!("{}", example_pair());
        let expected = "data Pair {\n\tTup(x : Int, y : Int)\n}";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_hd() {
        let result = format!("{}", example_hd());
        let expected = "hd() : Int";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_tl() {
        let result = format!("{}", example_tl());
        let expected = "tl() : Streamint";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_fst() {
        let result = format!("{}", example_fst());
        let expected = "fst() : Int";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_snd() {
        let result = format!("{}", example_snd());
        let expected = "snd() : Int";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_ap() {
        let result = format!("{}", example_ap());
        let expected = "ap(x : Int) : Int";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_stream() {
        let result = format!("{}", example_stream());
        let expected = "codata Streamint {\n\thd() : Int,\n\ttl() : Streamint\n}";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_lpair() {
        let result = format!("{}", example_lpair());
        let expected = "codata LPair {\n\tfst() : Int,\n\tsnd() : Int\n}";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_fun() {
        let result = format!("{}", example_fun());
        let expected = "codata Fun {\n\tap(x : Int) : Int\n}";
        assert_eq!(result, expected)
    }
}
