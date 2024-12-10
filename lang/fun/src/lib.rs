pub mod parser;
pub mod syntax;
pub mod typing;

#[cfg(test)]
pub mod test_common {
    use super::syntax::{
        context::{ContextBinding, TypingContext},
        declarations::{CtorSig, DataDeclaration},
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
}
