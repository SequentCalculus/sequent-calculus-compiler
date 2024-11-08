use std::rc::Rc;

use codespan::Span;
use derivative::Derivative;
use printer::{
    theme::ThemeExt,
    tokens::{COMMA, IFZ},
    DocAllocator, Print,
};

use super::Term;
use crate::syntax::types::{OptTyped, Ty};

#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct IfZ {
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    pub ifc: Rc<Term>,
    pub thenc: Rc<Term>,
    pub elsec: Rc<Term>,
    pub ty: Option<Ty>,
}

impl OptTyped for IfZ {
    fn get_type(&self) -> Option<Ty> {
        self.ty.clone()
    }
}

impl Print for IfZ {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc.keyword(IFZ).append(
            self.ifc
                .print(cfg, alloc)
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

impl From<IfZ> for Term {
    fn from(value: IfZ) -> Self {
        Term::IfZ(value)
    }
}

#[cfg(test)]
mod ifz_tests {
    use codespan::Span;
    use printer::Print;

    use crate::{parser::fun, syntax::terms::Lit};
    use std::rc::Rc;

    use super::{IfZ, Term};

    fn example() -> IfZ {
        IfZ {
            span: Span::default(),
            ifc: Rc::new(Term::Lit(Lit::mk(0))),
            thenc: Rc::new(Term::Lit(Lit::mk(2))),
            elsec: Rc::new(Term::Lit(Lit::mk(4))),
            ty: None,
        }
    }

    #[test]
    fn display() {
        assert_eq!(
            example().print_to_string(Default::default()),
            "ifz(0, 2, 4)"
        )
    }

    #[test]
    fn parse() {
        let parser = fun::TermParser::new();
        assert_eq!(parser.parse("ifz(0, 2, 4)"), Ok(example().into()));
    }
}
