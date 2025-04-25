use codespan::Span;
use derivative::Derivative;
use printer::{
    DocAllocator, Print,
    theme::ThemeExt,
    tokens::{RETURN, TO},
};

use super::Term;
use crate::{
    parser::util::ToMiette,
    syntax::{
        Covar, Var,
        context::TypingContext,
        types::{OptTyped, Ty},
    },
    traits::used_binders::UsedBinders,
    typing::{check::Check, errors::Error, symbol_table::SymbolTable},
};

use std::{collections::HashSet, rc::Rc};

#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct ReturnTo {
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    pub term: Rc<Term>,
    pub target: Covar,
    pub ty: Option<Ty>,
}

impl OptTyped for ReturnTo {
    fn get_type(&self) -> Option<Ty> {
        self.ty.clone()
    }
}

impl Print for ReturnTo {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc
            .keyword(RETURN)
            .append(alloc.space())
            .append(self.term.print(cfg, alloc))
            .append(alloc.space())
            .append(alloc.keyword(TO))
            .append(alloc.space())
            .append(self.target.print(cfg, alloc))
    }
}

impl From<ReturnTo> for Term {
    fn from(value: ReturnTo) -> Self {
        Term::ReturnTo(value)
    }
}

impl Check for ReturnTo {
    fn check(
        mut self,
        symbol_table: &mut SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<Self, Error> {
        let cont_type = context.lookup_covar(&self.target, &self.span.to_miette())?;
        self.term = self.term.check(symbol_table, context, &cont_type)?;

        self.ty = Some(expected.clone());
        Ok(self)
    }
}

impl UsedBinders for ReturnTo {
    fn used_binders(&self, used: &mut HashSet<Var>) {
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
            terms::{Lit, ReturnTo},
            types::Ty,
        },
        typing::symbol_table::SymbolTable,
    };
    use codespan::Span;
    use printer::Print;
    use std::rc::Rc;

    #[test]
    fn check_return_to() {
        let mut ctx = TypingContext::default();
        ctx.add_covar("a", Ty::mk_i64());
        let result = ReturnTo {
            span: Span::default(),
            target: "a".to_owned(),
            term: Rc::new(Lit::mk(1).into()),
            ty: None,
        }
        .check(&mut SymbolTable::default(), &ctx, &Ty::mk_i64())
        .unwrap();
        let expected = ReturnTo {
            span: Span::default(),
            target: "a".to_owned(),
            term: Rc::new(Lit::mk(1).into()),
            ty: Some(Ty::mk_i64()),
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn check_return_to_fail() {
        let result = ReturnTo {
            span: Span::default(),
            target: "a".to_owned(),
            term: Rc::new(Lit::mk(1).into()),
            ty: None,
        }
        .check(
            &mut SymbolTable::default(),
            &TypingContext::default(),
            &Ty::mk_i64(),
        );
        assert!(result.is_err())
    }

    fn example() -> ReturnTo {
        ReturnTo {
            span: Span::default(),
            term: Rc::new(Term::Lit(Lit::mk(2))),
            target: "x".to_string(),
            ty: None,
        }
    }

    #[test]
    fn display() {
        assert_eq!(
            example().print_to_string(Default::default()),
            "return 2 to x"
        )
    }

    #[test]
    fn parse() {
        let parser = fun::TermParser::new();
        assert_eq!(parser.parse("return 2 to x"), Ok(example().into()));
    }
}
