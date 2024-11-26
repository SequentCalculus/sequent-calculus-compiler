use std::rc::Rc;

use codespan::Span;
use derivative::Derivative;
use printer::{
    theme::ThemeExt,
    tokens::{COMMA, IFE, IFL},
    DocAllocator, Print,
};

use super::Term;
use crate::{
    syntax::{
        context::TypingContext,
        types::{OptTyped, Ty},
    },
    typing::{check::Check, errors::Error, symbol_table::SymbolTable},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IfSort {
    Equal,
    Less,
}

#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct IfC {
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    pub sort: IfSort,
    pub fst: Rc<Term>,
    pub snd: Rc<Term>,
    pub thenc: Rc<Term>,
    pub elsec: Rc<Term>,
    pub ty: Option<Ty>,
}

impl OptTyped for IfC {
    fn get_type(&self) -> Option<Ty> {
        self.ty.clone()
    }
}

impl Print for IfC {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        let start = match self.sort {
            IfSort::Equal => alloc.keyword(IFE),
            IfSort::Less => alloc.keyword(IFL),
        };
        start.append(
            self.fst
                .print(cfg, alloc)
                .append(COMMA)
                .append(alloc.space())
                .append(self.snd.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append(self.thenc.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append(self.elsec.print(cfg, alloc))
                .parens(),
        )
    }
}

impl From<IfC> for Term {
    fn from(value: IfC) -> Self {
        Term::IfC(value)
    }
}

impl Check for IfC {
    fn check(
        self,
        symbol_table: &SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<Self, Error> {
        let fst_checked = self.fst.check(symbol_table, context, &Ty::mk_int())?;
        let snd_checked = self.snd.check(symbol_table, context, &Ty::mk_int())?;
        let thenc_checked = self.thenc.check(symbol_table, context, expected)?;
        let elsec_checked = self.elsec.check(symbol_table, context, expected)?;
        Ok(IfC {
            fst: fst_checked,
            snd: snd_checked,
            thenc: thenc_checked,
            elsec: elsec_checked,
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
    use crate::syntax::terms::IfSort;
    use crate::{
        syntax::{
            context::ContextBinding,
            terms::{IfC, Lit, Var},
            types::Ty,
        },
        typing::symbol_table::SymbolTable,
    };
    use codespan::Span;
    use printer::Print;
    use std::rc::Rc;

    #[test]
    fn check_ife() {
        let result = IfC {
            span: Span::default(),
            sort: IfSort::Equal,
            fst: Rc::new(Lit::mk(2).into()),
            snd: Rc::new(Lit::mk(1).into()),
            thenc: Rc::new(Lit::mk(2).into()),
            elsec: Rc::new(Lit::mk(3).into()),
            ty: None,
        }
        .check(
            &SymbolTable::default(),
            &TypingContext { bindings: vec![] },
            &Ty::mk_int(),
        )
        .unwrap();
        let expected = IfC {
            span: Span::default(),
            sort: IfSort::Equal,
            fst: Rc::new(Lit::mk(2).into()),
            snd: Rc::new(Lit::mk(1).into()),
            thenc: Rc::new(Lit::mk(2).into()),
            elsec: Rc::new(Lit::mk(3).into()),
            ty: Some(Ty::mk_int()),
        };
        assert_eq!(result, expected)
    }
    #[test]
    fn check_ife_fail() {
        let result = IfC {
            span: Span::default(),
            sort: IfSort::Equal,
            fst: Rc::new(Var::mk("x").into()),
            snd: Rc::new(Var::mk("x").into()),
            thenc: Rc::new(Lit::mk(1).into()),
            elsec: Rc::new(Lit::mk(2).into()),
            ty: None,
        }
        .check(
            &SymbolTable::default(),
            &TypingContext {
                bindings: vec![ContextBinding::TypedVar {
                    var: "x".to_owned(),
                    ty: Ty::mk_decl("ListInt"),
                }],
            },
            &Ty::mk_int(),
        );
        assert!(result.is_err())
    }

    fn example() -> IfC {
        IfC {
            span: Span::default(),
            sort: IfSort::Equal,
            fst: Rc::new(Term::Lit(Lit::mk(0))),
            snd: Rc::new(Term::Lit(Lit::mk(0))),
            thenc: Rc::new(Term::Lit(Lit::mk(2))),
            elsec: Rc::new(Term::Lit(Lit::mk(4))),
            ty: None,
        }
    }

    #[test]
    fn display() {
        assert_eq!(
            example().print_to_string(Default::default()),
            "ife(0, 0, 2, 4)"
        )
    }

    #[test]
    fn parse() {
        let parser = fun::TermParser::new();
        assert_eq!(parser.parse("ife(0, 0, 2, 4)"), Ok(example().into()));
    }
}
