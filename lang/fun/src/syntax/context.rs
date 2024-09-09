use crate::syntax::{types::Ty, Covariable, Variable};
use std::{collections::HashSet, fmt};

#[derive(Debug, PartialEq, Eq)]
pub struct TypedVar {
    pub var: Variable,
    pub ty: Ty,
}

#[derive(Debug, PartialEq, Eq)]
pub struct TypedCovar {
    pub covar: Covariable,
    pub ty: Ty,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ContextItem {
    TypedVar(TypedVar),
    TypedCovar(TypedCovar),
}

#[derive(Debug, PartialEq, Eq)]
pub struct TypingContext {
    pub items: Vec<ContextItem>,
}

impl TypingContext {
    pub fn vars(&self) -> HashSet<Variable> {
        let mut contained = HashSet::new();

        for item in &self.items {
            if let ContextItem::TypedVar(var) = item {
                contained.insert(var.var.clone());
            }
        }
        contained
    }

    pub fn covars(&self) -> HashSet<Covariable> {
        let mut contained = HashSet::new();

        for item in &self.items {
            if let ContextItem::TypedCovar(covar) = item {
                contained.insert(covar.covar.clone());
            }
        }
        contained
    }
}

impl From<TypedVar> for ContextItem {
    fn from(var: TypedVar) -> ContextItem {
        ContextItem::TypedVar(var)
    }
}

impl From<TypedCovar> for ContextItem {
    fn from(covar: TypedCovar) -> ContextItem {
        ContextItem::TypedCovar(covar)
    }
}

impl fmt::Display for TypingContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.items
                .iter()
                .map(|it| format!("{}", it))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

impl fmt::Display for ContextItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ContextItem::TypedVar(var) => var.fmt(f),
            ContextItem::TypedCovar(covar) => covar.fmt(f),
        }
    }
}

impl fmt::Display for TypedVar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} : {}", self.var, self.ty)
    }
}

impl fmt::Display for TypedCovar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} : {}", self.covar, self.ty)
    }
}

#[cfg(test)]
mod context_tests {
    use super::{ContextItem, Ty, TypedCovar, TypedVar, TypingContext};
    use std::collections::HashSet;

    fn example_typedvar() -> TypedVar {
        TypedVar {
            var: "x".to_owned(),
            ty: Ty::Int(),
        }
    }

    fn example_typedcovar() -> TypedCovar {
        TypedCovar {
            covar: "a".to_owned(),
            ty: Ty::Int(),
        }
    }

    fn example_contextitem_var() -> ContextItem {
        ContextItem::TypedVar(example_typedvar())
    }

    fn example_contextitem_covar() -> ContextItem {
        ContextItem::TypedCovar(example_typedcovar())
    }

    fn example_context() -> TypingContext {
        TypingContext {
            items: vec![example_contextitem_var(), example_contextitem_covar()],
        }
    }
    #[test]
    fn display_context() {
        let result = format!("{}", example_context());
        let expected = "x : Int, a : Int";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_contextitem_var() {
        let result = format!("{}", example_contextitem_var());
        let expected = "x : Int";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_contextitem_covar() {
        let result = format!("{}", example_contextitem_covar());
        let expected = "a : Int";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_typedvar() {
        let result = format!("{}", example_typedvar());
        let expected = "x : Int";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_typedcovar() {
        let result = format!("{}", example_typedcovar());
        let expected = "a : Int";
        assert_eq!(result, expected)
    }

    #[test]
    fn context_vars() {
        let result = example_context().vars();
        let expected = HashSet::from(["x".to_owned()]);
        assert_eq!(result, expected)
    }

    #[test]
    fn context_covars() {
        let result = example_context().covars();
        let expected = HashSet::from(["a".to_owned()]);
        assert_eq!(result, expected)
    }
}
