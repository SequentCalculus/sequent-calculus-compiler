//! This module defines the control operator for capturing current continuation/program context in
//! Fun.

use codespan::Span;
use derivative::Derivative;
use printer::tokens::LABEL;
use printer::*;

use crate::syntax::*;
use crate::traits::*;
use crate::typing::*;

use std::{collections::HashSet, rc::Rc};

/// This struct defines the control operator capturing the current continuation/program context. It
/// consists of a covariable to which the continuation is bound, the body in which the continuation
/// is available, and after typechecking also of the inferred type.
///
/// Example:
/// `label a { goto a (5)}` captures the current continuation, binds it to covariable `a` and
/// invokes it with argument `5`.
#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct Label {
    // The source location
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    /// The covariable to which the continuation is bound
    pub label: Var,
    /// The body in which the continuation is in scope
    pub term: Rc<Term>,
    /// The (inferred) type of the term
    pub ty: Option<Ty>,
}

impl OptTyped for Label {
    fn get_type(&self) -> Option<Ty> {
        self.ty.clone()
    }
}

impl Print for Label {
    fn print<'a>(&'a self, cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        alloc
            .keyword(LABEL)
            .append(alloc.space())
            .append(self.label.print(cfg, alloc))
            .append(alloc.space())
            .append(
                alloc
                    .line()
                    .append(self.term.print(cfg, alloc).group())
                    .nest(cfg.indent)
                    .append(alloc.line())
                    .braces_anno()
                    .group(),
            )
    }
}
impl From<Label> for Term {
    fn from(value: Label) -> Self {
        Term::Label(value)
    }
}

impl Check for Label {
    fn check(
        mut self,
        symbol_table: &mut SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<Self, Error> {
        let mut new_context = context.clone();
        new_context.add_covar(self.label.clone(), expected.clone());
        self.term = self.term.check(symbol_table, &new_context, expected)?;

        self.ty = Some(expected.clone());
        Ok(self)
    }
}

impl UsedBinders for Label {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        used.insert(self.label.clone());
        self.term.used_binders(used);
    }
}

#[cfg(test)]
mod test {
    use codespan::Span;
    use printer::Print;

    use crate::parser::fun;
    use crate::syntax::*;
    use crate::typing::*;

    use std::rc::Rc;

    #[test]
    fn check_label() {
        let result = Label {
            span: Span::default(),
            label: Var {
                name: "a".to_owned(),
                id: 0,
            },
            ty: None,
            term: Rc::new(Lit::mk(1).into()),
        }
        .check(
            &mut SymbolTable::default(),
            &TypingContext::default(),
            &Ty::mk_i64(),
        )
        .unwrap();
        let expected = Label {
            span: Span::default(),
            label: Var {
                name: "a".to_owned(),
                id: 0,
            },
            ty: Some(Ty::mk_i64()),
            term: Rc::new(Lit::mk(1).into()),
        };
        assert_eq!(result, expected)
    }
    #[test]
    fn check_label_fail() {
        let mut ctx = TypingContext::default();
        ctx.add_var(
            Var {
                name: "x".to_string(),
                id: 0,
            },
            Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_i64()])),
        );
        let result = Label {
            span: Span::default(),
            label: Var {
                name: "a".to_owned(),
                id: 0,
            },
            term: Rc::new(
                XVar::mk(Var {
                    name: "x".to_string(),
                    id: 0,
                })
                .into(),
            ),
            ty: None,
        }
        .check(&mut SymbolTable::default(), &ctx, &Ty::mk_i64());
        assert!(result.is_err())
    }

    fn example() -> Label {
        Label {
            span: Span::default(),
            label: Var {
                name: "x".to_string(),
                id: 0,
            },
            term: Rc::new(Term::Lit(Lit::mk(2))),
            ty: None,
        }
    }

    #[test]
    fn parse() {
        let parser = fun::TermParser::new();
        assert_eq!(parser.parse("label x { 2 }"), Ok(example().into()));
    }

    #[test]
    fn display() {
        assert_eq!(
            example().print_to_string(Default::default()),
            "label x { 2 }"
        )
    }
}
