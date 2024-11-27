use std::rc::Rc;

use codespan::Span;
use derivative::Derivative;
use printer::{
    theme::ThemeExt,
    tokens::{COMMA, IFL},
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

#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct IfL {
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    pub fst: Rc<Term>,
    pub snd: Rc<Term>,
    pub thenc: Rc<Term>,
    pub elsec: Rc<Term>,
    pub ty: Option<Ty>,
}

impl OptTyped for IfL {
    fn get_type(&self) -> Option<Ty> {
        self.ty.clone()
    }
}

impl Print for IfL {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc.keyword(IFL).append(
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

impl From<IfL> for Term {
    fn from(value: IfL) -> Self {
        Term::IfL(value)
    }
}

impl Check for IfL {
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
        Ok(IfL {
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
    use crate::{
        syntax::{
            context::ContextBinding,
            terms::{IfL, Lit, Var},
            types::Ty,
        },
        typing::symbol_table::SymbolTable,
    };
    use codespan::Span;
    use printer::Print;
    use std::rc::Rc;

    #[test]
    fn check_ifl() {
        let result = IfL {
            span: Span::default(),
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
        let expected = IfL {
            span: Span::default(),
            fst: Rc::new(Lit::mk(2).into()),
            snd: Rc::new(Lit::mk(1).into()),
            thenc: Rc::new(Lit::mk(2).into()),
            elsec: Rc::new(Lit::mk(3).into()),
            ty: Some(Ty::mk_int()),
        };
        assert_eq!(result, expected)
    }
    #[test]
    fn check_ifl_fail() {
        let result = IfL {
            span: Span::default(),
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

    fn example() -> IfL {
        IfL {
            span: Span::default(),
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
            "ifl(0, 0, 2, 4)"
        )
    }

    #[test]
    fn parse() {
        let parser = fun::TermParser::new();
        assert_eq!(parser.parse("ifl(0, 0, 2, 4)"), Ok(example().into()));
    }
}
