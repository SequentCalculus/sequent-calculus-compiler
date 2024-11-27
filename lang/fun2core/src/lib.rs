pub mod definition;
pub mod program;
pub mod terms;

#[cfg(test)]
pub mod symbol_tables {
    use codespan::Span;
    use fun::{
        syntax::{
            context::{ContextBinding, TypingContext},
            types::Ty,
        },
        typing::symbol_table::{Polarity, SymbolTable},
    };
    use std::collections::HashMap;

    fn ctors_list() -> HashMap<String, TypingContext> {
        let mut ctors = HashMap::new();
        ctors.insert(
            "Nil".to_owned(),
            TypingContext {
                span: Span::default(),
                bindings: vec![],
            },
        );
        ctors.insert(
            "Cons".to_owned(),
            TypingContext {
                span: Span::default(),
                bindings: vec![
                    ContextBinding::TypedVar {
                        var: "x".to_owned(),
                        ty: Ty::mk_int(),
                    },
                    ContextBinding::TypedVar {
                        var: "xs".to_owned(),
                        ty: Ty::mk_decl("ListInt"),
                    },
                ],
            },
        );
        ctors
    }

    fn dtors_lpair() -> HashMap<String, (TypingContext, Ty)> {
        let mut dtors = HashMap::new();
        dtors.insert(
            "Fst".to_owned(),
            (
                TypingContext {
                    span: Span::default(),
                    bindings: vec![],
                },
                Ty::mk_int(),
            ),
        );
        dtors.insert(
            "Snd".to_owned(),
            (
                TypingContext {
                    span: Span::default(),
                    bindings: vec![],
                },
                Ty::mk_int(),
            ),
        );
        dtors
    }

    fn ty_ctors_list() -> HashMap<String, (Polarity, Vec<String>)> {
        let mut ty_ctors = HashMap::new();
        ty_ctors.insert(
            "ListInt".to_owned(),
            (Polarity::Data, vec!["Nil".to_owned(), "Cons".to_owned()]),
        );
        ty_ctors
    }

    fn ty_ctors_lpair() -> HashMap<String, (Polarity, Vec<String>)> {
        let mut ty_ctors = HashMap::new();
        ty_ctors.insert(
            "LPairIntInt".to_owned(),
            (Polarity::Codata, vec!["Fst".to_owned(), "Snd".to_owned()]),
        );
        ty_ctors
    }

    pub fn table_list() -> SymbolTable {
        SymbolTable {
            ctors: ctors_list(),
            dtors: HashMap::new(),
            funs: HashMap::new(),
            ty_ctors: ty_ctors_list(),
        }
    }

    pub fn table_lpair() -> SymbolTable {
        SymbolTable {
            funs: HashMap::new(),
            ctors: HashMap::new(),
            dtors: dtors_lpair(),
            ty_ctors: ty_ctors_lpair(),
        }
    }
}
