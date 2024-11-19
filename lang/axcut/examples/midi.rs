use axcut::syntax::statements::*;
use axcut::syntax::*;
use printer::Print;

use std::rc::Rc;

fn main() {
    let ty_list = TypeDeclaration {
        name: "List".to_string(),
        xtors: vec![
            XtorSig {
                name: "Nil".to_string(),
                args: vec![],
            },
            XtorSig {
                name: "Cons".to_string(),
                args: vec![
                    ContextBinding {
                        var: "xs".to_string(),
                        chi: Chirality::Prd,
                        ty: Ty::Decl("List".to_string()),
                    },
                    ContextBinding {
                        var: "x".to_string(),
                        chi: Chirality::Ext,
                        ty: Ty::Int,
                    },
                ],
            },
        ],
    };

    let ty_cont_list = TypeDeclaration {
        name: "ContList".to_string(),
        xtors: vec![XtorSig {
            name: "Retl".to_string(),
            args: vec![ContextBinding {
                var: "kl".to_string(),
                chi: Chirality::Prd,
                ty: Ty::Decl("List".to_string()),
            }],
        }],
    };

    let ty_cont_int = TypeDeclaration {
        name: "ContInt".to_string(),
        xtors: vec![XtorSig {
            name: "Reti".to_string(),
            args: vec![ContextBinding {
                var: "ki".to_string(),
                chi: Chirality::Ext,
                ty: Ty::Int,
            }],
        }],
    };

    let main_body = Statement::New(New {
        var: "t".to_string(),
        ty: Ty::Decl("ContInt".to_string()),
        context: None,
        clauses: vec![Clause {
            xtor: "Reti".to_string(),
            context: vec![ContextBinding {
                var: "r".to_string(),
                chi: Chirality::Ext,
                ty: Ty::Int,
            }],
            case: Rc::new(Statement::Return(Return {
                var: "r".to_string(),
            })),
        }],
        next: Rc::new(Statement::New(New {
            var: "k".to_string(),
            ty: Ty::Decl("ContList".to_string()),
            context: None,
            clauses: vec![Clause {
                xtor: "Retl".to_string(),
                context: vec![ContextBinding {
                    var: "as".to_string(),
                    chi: Chirality::Prd,
                    ty: Ty::Decl("List".to_string()),
                }],
                case: Rc::new(Statement::Call(Call {
                    label: "sum".to_string(),
                    args: vec!["t".to_string(), "as".to_string()],
                })),
            }],
            next: Rc::new(Statement::Leta(Leta {
                var: "zs".to_string(),
                ty: Ty::Decl("List".to_string()),
                tag: "Nil".to_string(),
                args: vec![],
                next: Rc::new(Statement::Literal(Literal {
                    lit: 3,
                    var: "n".to_string(),
                    case: Rc::new(Statement::Call(Call {
                        label: "range".to_string(),
                        args: vec!["k".to_string(), "zs".to_string(), "n".to_string()],
                    })),
                })),
            })),
        })),
    });
    let main = Def {
        name: "main".to_string(),
        context: Vec::new(),
        body: main_body,
    };

    let range_body = Statement::IfZ(IfZ {
        ifc: "i".to_string(),
        thenc: Rc::new(Statement::Invoke(Invoke {
            var: "k".to_string(),
            tag: "Retl".to_string(),
            ty: Ty::Decl("ContList".to_string()),
            args: vec!["xs".to_string()],
        })),
        elsec: Rc::new(Statement::Leta(Leta {
            var: "ys".to_string(),
            ty: Ty::Decl("List".to_string()),
            tag: "Cons".to_string(),
            args: vec!["xs".to_string(), "i".to_string()],
            next: Rc::new(Statement::Literal(Literal {
                lit: -1,
                var: "o".to_string(),
                case: Rc::new(Statement::Op(Op {
                    fst: "i".to_string(),
                    op: BinOp::Sum,
                    snd: "o".to_string(),
                    var: "j".to_string(),
                    case: Rc::new(Statement::Call(Call {
                        label: "range".to_string(),
                        args: vec!["k".to_string(), "ys".to_string(), "j".to_string()],
                    })),
                })),
            })),
        })),
    });
    let range = Def {
        name: "range".to_string(),
        context: vec![
            ContextBinding {
                var: "k".to_string(),
                chi: Chirality::Cns,
                ty: Ty::Decl("ContList".to_string()),
            },
            ContextBinding {
                var: "xs".to_string(),
                chi: Chirality::Prd,
                ty: Ty::Decl("List".to_string()),
            },
            ContextBinding {
                var: "i".to_string(),
                chi: Chirality::Ext,
                ty: Ty::Int,
            },
        ],
        body: range_body,
    };

    let sum_body = Statement::Switch(Switch {
        var: "xs".to_string(),
        ty: Ty::Decl("List".to_string()),
        clauses: vec![
            Clause {
                xtor: "Nil".to_string(),
                context: vec![],
                case: Rc::new(Statement::Literal(Literal {
                    lit: 0,
                    var: "z".to_string(),
                    case: Rc::new(Statement::Invoke(Invoke {
                        var: "k".to_string(),
                        tag: "Reti".to_string(),
                        ty: Ty::Decl("ContInt".to_string()),
                        args: vec!["z".to_string()],
                    })),
                })),
            },
            Clause {
                xtor: "Cons".to_string(),
                context: vec![
                    ContextBinding {
                        var: "ys".to_string(),
                        chi: Chirality::Prd,
                        ty: Ty::Decl("List".to_string()),
                    },
                    ContextBinding {
                        var: "y".to_string(),
                        chi: Chirality::Ext,
                        ty: Ty::Int,
                    },
                ],
                case: Rc::new(Statement::New(New {
                    var: "j".to_string(),
                    ty: Ty::Decl("ContInt".to_string()),
                    context: None,
                    clauses: vec![Clause {
                        xtor: "Reti".to_string(),
                        context: vec![ContextBinding {
                            var: "r".to_string(),
                            chi: Chirality::Ext,
                            ty: Ty::Int,
                        }],
                        case: Rc::new(Statement::Op(Op {
                            fst: "y".to_string(),
                            op: BinOp::Sum,
                            snd: "r".to_string(),
                            var: "s".to_string(),
                            case: Rc::new(Statement::Invoke(Invoke {
                                var: "k".to_string(),
                                tag: "Reti".to_string(),
                                ty: Ty::Decl("ContInt".to_string()),
                                args: vec!["s".to_string()],
                            })),
                        })),
                    }],
                    next: Rc::new(Statement::Call(Call {
                        label: "sum".to_string(),
                        args: vec!["j".to_string(), "ys".to_string()],
                    })),
                })),
            },
        ],
    });
    let sum = Def {
        name: "sum".to_string(),
        context: vec![
            ContextBinding {
                var: "k".to_string(),
                chi: Chirality::Cns,
                ty: Ty::Decl("ContList".to_string()),
            },
            ContextBinding {
                var: "xs".to_string(),
                chi: Chirality::Prd,
                ty: Ty::Decl("List".to_string()),
            },
        ],
        body: sum_body,
    };

    let program = Prog {
        defs: vec![main, range, sum],
        types: vec![ty_list, ty_cont_list, ty_cont_int],
    };

    println!("{}", program::linearize(program).print_to_string(None))
}
