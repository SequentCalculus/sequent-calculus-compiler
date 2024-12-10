pub mod parser;
pub mod syntax;
pub mod typing;

#[cfg(test)]
pub mod test_common {
    use super::syntax::{
        context::{ContextBinding, TypingContext},
        declarations::{CodataDeclaration, CtorSig, DataDeclaration, DtorSig},
        types::Ty,
    };
    use codespan::Span;

    pub fn data_list() -> DataDeclaration {
        DataDeclaration {
            span: Span::default(),
            name: "ListInt".to_owned(),
            ctors: vec![
                CtorSig {
                    span: Span::default(),
                    name: "Nil".to_owned(),
                    args: TypingContext {
                        span: Span::default(),
                        bindings: vec![],
                    },
                },
                CtorSig {
                    span: Span::default(),
                    name: "Cons".to_owned(),
                    args: TypingContext {
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
                },
            ],
        }
    }

    pub fn codata_stream() -> CodataDeclaration {
        CodataDeclaration {
            span: Span::default(),
            name: "StreamInt".to_owned(),
            dtors: vec![
                DtorSig {
                    span: Span::default(),
                    name: "Hd".to_owned(),
                    args: TypingContext {
                        span: Span::default(),
                        bindings: vec![],
                    },
                    cont_ty: Ty::mk_int(),
                },
                DtorSig {
                    span: Span::default(),
                    name: "Tl".to_owned(),
                    args: TypingContext {
                        span: Span::default(),
                        bindings: vec![],
                    },
                    cont_ty: Ty::mk_decl("StreamInt"),
                },
            ],
        }
    }
}
