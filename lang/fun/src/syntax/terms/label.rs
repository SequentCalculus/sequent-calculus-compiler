use codespan::Span;
use derivative::Derivative;
use printer::{theme::ThemeExt, tokens::LABEL, util::BracesExt, DocAllocator, Print};

use super::Term;
use crate::{
    syntax::{
        context::TypingContext,
        types::{OptTyped, Ty},
        Covar, Var,
    },
    traits::used_binders::UsedBinders,
    typing::{check::Check, errors::Error, symbol_table::SymbolTable},
};

use std::{collections::HashSet, rc::Rc};

/// A term defining a label used in a [goto](crate::syntax::terms::Goto) expression
/// Example: `label a { goto a (5)}`
/// Defines the label `a` and immediately jumps to it
#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct Label {
    // The source location
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    /// The label being defined
    pub label: Covar,
    /// The inner term in which the label is in scope
    pub term: Rc<Term>,
    /// The type of the term (inferred)
    pub ty: Option<Ty>,
}

impl OptTyped for Label {
    fn get_type(&self) -> Option<Ty> {
        self.ty.clone()
    }
}

impl Print for Label {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc
            .keyword(LABEL)
            .append(alloc.space())
            .append(self.label.clone())
            .append(alloc.space())
            .append(self.term.print(cfg, alloc).braces_anno())
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
        new_context.add_covar(&self.label, expected.clone());
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
    use super::Check;
    use super::Term;
    use crate::parser::fun;
    use crate::syntax::context::TypingContext;
    use crate::{
        syntax::{
            terms::{Label, Lit, XVar},
            types::{Ty, TypeArgs},
        },
        typing::symbol_table::SymbolTable,
    };
    use codespan::Span;
    use printer::Print;
    use std::rc::Rc;

    #[test]
    fn check_label() {
        let result = Label {
            span: Span::default(),
            label: "a".to_owned(),
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
            label: "a".to_owned(),
            ty: Some(Ty::mk_i64()),
            term: Rc::new(Lit::mk(1).into()),
        };
        assert_eq!(result, expected)
    }
    #[test]
    fn check_label_fail() {
        let mut ctx = TypingContext::default();
        ctx.add_var("x", Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_i64()])));
        let result = Label {
            span: Span::default(),
            label: "a".to_owned(),
            term: Rc::new(XVar::mk("x").into()),
            ty: None,
        }
        .check(&mut SymbolTable::default(), &ctx, &Ty::mk_i64());
        assert!(result.is_err())
    }

    fn example() -> Label {
        Label {
            span: Span::default(),
            label: "x".to_string(),
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
        assert_eq!(example().print_to_string(Default::default()), "label x {2}")
    }
}
