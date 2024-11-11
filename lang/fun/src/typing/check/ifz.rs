use super::terms::Check;
use crate::{
    syntax::{context::TypingContext, terms::IfZ, types::Ty},
    typing::{errors::Error, symbol_table::SymbolTable},
};

impl Check for IfZ {
    fn check(
        self,
        symbol_table: &SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<IfZ, Error> {
        let ifc_checked = self.ifc.check(symbol_table, context, &Ty::mk_int())?;
        let thenc_checked = self.thenc.check(symbol_table, context, expected)?;
        let elsec_checked = self.elsec.check(symbol_table, context, expected)?;
        Ok(IfZ {
            span: self.span,
            ifc: ifc_checked,
            thenc: thenc_checked,
            elsec: elsec_checked,
            ty: Some(expected.clone()),
        })
    }
}

#[cfg(test)]
mod ifz_test {
    use super::Check;
    use crate::{
        syntax::{
            context::ContextBinding,
            terms::{IfZ, Lit, Var},
            types::Ty,
        },
        typing::symbol_table::SymbolTable,
    };
    use codespan::Span;
    use std::rc::Rc;
    #[test]
    fn check_ifz() {
        let result = IfZ {
            span: Span::default(),
            ifc: Rc::new(
                Lit {
                    span: Span::default(),
                    val: 1,
                }
                .into(),
            ),
            thenc: Rc::new(
                Lit {
                    span: Span::default(),
                    val: 2,
                }
                .into(),
            ),
            elsec: Rc::new(
                Lit {
                    span: Span::default(),
                    val: 3,
                }
                .into(),
            ),
            ty: None,
        }
        .check(&SymbolTable::default(), &vec![], &Ty::mk_int())
        .unwrap();
        let expected = IfZ {
            span: Span::default(),
            ifc: Rc::new(
                Lit {
                    span: Span::default(),
                    val: 1,
                }
                .into(),
            ),
            thenc: Rc::new(
                Lit {
                    span: Span::default(),
                    val: 2,
                }
                .into(),
            ),
            elsec: Rc::new(
                Lit {
                    span: Span::default(),
                    val: 3,
                }
                .into(),
            ),
            ty: Some(Ty::mk_int()),
        };
        assert_eq!(result, expected)
    }
    #[test]
    fn check_ifz_fail() {
        let result = IfZ {
            span: Span::default(),
            ifc: Rc::new(
                Var {
                    span: Span::default(),
                    ty: None,
                    var: "x".to_owned(),
                }
                .into(),
            ),
            thenc: Rc::new(
                Lit {
                    span: Span::default(),
                    val: 1,
                }
                .into(),
            ),
            elsec: Rc::new(
                Lit {
                    span: Span::default(),
                    val: 2,
                }
                .into(),
            ),
            ty: None,
        }
        .check(
            &SymbolTable::default(),
            &vec![ContextBinding::TypedVar {
                var: "x".to_owned(),
                ty: Ty::mk_decl("ListInt"),
            }],
            &Ty::mk_int(),
        );
        assert!(result.is_err())
    }
}
