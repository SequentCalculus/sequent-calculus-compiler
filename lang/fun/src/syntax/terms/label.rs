use std::rc::Rc;

use codespan::Span;
use derivative::Derivative;
use printer::{
    theme::ThemeExt,
    tokens::{LABEL, TICK},
    util::BracesExt,
    DocAllocator, Print,
};

use crate::{
    syntax::{
        context::{ContextBinding, TypingContext},
        types::{OptTyped, Ty},
        Covariable,
    },
    typing::{check::Check, errors::Error, symbol_table::SymbolTable},
};

use super::Term;

#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct Label {
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    pub label: Covariable,
    pub term: Rc<Term>,
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
            .append(TICK)
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
        self,
        symbol_table: &SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<Self, Error> {
        let mut new_context = context.clone();
        new_context.bindings.push(ContextBinding::TypedCovar {
            covar: self.label.clone(),
            ty: expected.clone(),
        });
        let term_checked = self.term.check(symbol_table, &new_context, expected)?;
        Ok(Label {
            term: term_checked,
            ty: Some(expected.clone()),
            ..self
        })
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
            terms::{Label, Lit, Var},
            types::Ty,
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
            &SymbolTable::default(),
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
        ctx.add_var("x", Ty::mk_decl("ListInt"));
        let result = Label {
            span: Span::default(),
            label: "a".to_owned(),
            term: Rc::new(Var::mk("x").into()),
            ty: None,
        }
        .check(&SymbolTable::default(), &ctx, &Ty::mk_i64());
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
        assert_eq!(parser.parse("label 'x { 2 }"), Ok(example().into()));
    }

    #[test]
    fn display() {
        assert_eq!(
            example().print_to_string(Default::default()),
            "label 'x {2}"
        )
    }
}
