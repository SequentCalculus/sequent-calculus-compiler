use std::fmt;

use crate::syntax::{Covariable, Name, Term, Variable};

// Def
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Def<T> {
    pub name: Name,
    pub args: Vec<(Variable, T)>,
    pub cont: Vec<(Covariable, T)>,
    pub body: Term,
    pub ret_ty: T,
}

impl<T> fmt::Display for Def<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let args_str: Vec<String> = self.args.iter().map(|(x, _)| x.to_string()).collect();
        let cont_str: Vec<String> = self.cont.iter().map(|(x, _)| x.to_string()).collect();
        write!(
            f,
            "def {}({};{}) := {};",
            self.name,
            args_str.join(", "),
            cont_str.join(", "),
            self.body
        )
    }
}

// Prog
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Prog<T> {
    pub prog_defs: Vec<Def<T>>,
}

impl<T> fmt::Display for Prog<T> {
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

    // Empty program
    //
    //

    fn example_empty() -> Prog<()> {
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

    fn example_simple() -> Prog<()> {
        Prog {
            prog_defs: vec![Def {
                name: "x".to_string(),
                args: vec![],
                cont: vec![],
                body: Term::Lit(4),
                ret_ty: (),
            }],
        }
    }

    #[test]
    fn display_simple() {
        assert_eq!(
            format!("{}", example_simple()),
            "def x(;) := 4;".to_string()
        )
    }

    #[test]
    fn parse_simple() {
        let parser = fun::ProgParser::new();
        assert_eq!(parser.parse("def x(;) := 4;"), Ok(example_simple().into()));
    }

    // Program with one definition which takes arguments
    //
    //

    fn example_args() -> Prog<()> {
        Prog {
            prog_defs: vec![Def {
                name: "f".to_string(),
                args: vec![("x".to_string(), ())],
                cont: vec![("a".to_string(), ())],
                body: Term::Lit(4),
                ret_ty: (),
            }],
        }
    }

    #[test]
    fn display_args() {
        assert_eq!(
            format!("{}", example_args()),
            "def f(x;a) := 4;".to_string(),
        )
    }

    #[test]
    fn parse_args() {
        let parser = fun::ProgParser::new();
        assert_eq!(parser.parse("def f(x;a) := 4;"), Ok(example_args().into()))
    }

    // Program with two definitions
    //
    //

    fn example_two() -> Prog<()> {
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
            prog_defs: vec![d1, d2],
        }
    }

    #[test]
    fn display_two() {
        assert_eq!(
            format!("{}", example_two()),
            "def f(;) := 2;\ndef g(;) := 4;".to_string(),
        )
    }

    #[test]
    fn parse_two() {
        let parser = fun::ProgParser::new();
        assert_eq!(
            parser.parse("def f(;) := 2;\n def g(;) := 4;"),
            Ok(example_two().into())
        )
    }
}
