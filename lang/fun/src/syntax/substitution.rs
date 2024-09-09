use super::{terms::Term, Covariable};
use std::fmt;

pub enum SubstItem {
    Prod(Term),
    Cons(Covariable),
}

pub struct Substitution {
    items: Vec<SubstItem>,
}

impl fmt::Display for SubstItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SubstItem::Prod(t) => t.fmt(f),
            SubstItem::Cons(c) => c.fmt(f),
        }
    }
}

impl fmt::Display for Substitution {
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

impl From<Term> for SubstItem {
    fn from(t: Term) -> SubstItem {
        SubstItem::Prod(t)
    }
}

impl From<Covariable> for SubstItem {
    fn from(c: Covariable) -> SubstItem {
        SubstItem::Cons(c)
    }
}

#[cfg(test)]
mod substitution_test {
    use super::{SubstItem, Substitution, Term};

    fn example_proditem() -> SubstItem {
        SubstItem::Prod(Term::Var("x".to_owned()))
    }

    fn example_consitem() -> SubstItem {
        SubstItem::Cons("a".to_owned())
    }

    fn example_subst() -> Substitution {
        Substitution {
            items: vec![example_proditem(), example_consitem()],
        }
    }

    #[test]
    fn display_prod_item() {
        let result = format!("{}", example_proditem());
        let expected = "x";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_cons_item() {
        let result = format!("{}", example_consitem());
        let expected = "a";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_subst() {
        let result = format!("{}", example_subst());
        let expected = "x, a";
        assert_eq!(result, expected)
    }
}
