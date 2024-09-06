use super::{types::Ty, Name, Variable};

use std::fmt;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum TypeDefinition {
    Data(DataDefinition),
    Codata(CodataDefinition),
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct DataDefinition {
    pub name: Name,
    pub ctors: Vec<CtorSig>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct CodataDefinition {
    pub name: Name,
    pub dtors: Vec<DtorSig>,
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

impl From<DataDefinition> for TypeDefinition {
    fn from(def: DataDefinition) -> TypeDefinition {
        TypeDefinition::Data(def)
    }
}

impl From<CodataDefinition> for TypeDefinition {
    fn from(def: CodataDefinition) -> TypeDefinition {
        TypeDefinition::Codata(def)
    }
}

impl fmt::Display for TypeDefinition {
    fn fmt(&self, frmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TypeDefinition::Data(def) => def.fmt(frmt),
            TypeDefinition::Codata(def) => def.fmt(frmt),
        }
    }
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

    use super::{CodataDefinition, CtorSig, DataDefinition, DtorSig, TypeDefinition};

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
