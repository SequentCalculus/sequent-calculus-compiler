use axcut::syntax::*;
use axcut::syntax::{BinOp, Chirality, ContextBinding, Return, Ty, TypeDeclaration, XtorSig};
use printer::Print;

use std::rc::Rc;

fn main() {
    let ty_box = TypeDeclaration {
        name: "Box".to_string(),
        xtors: vec![XtorSig {
            name: "B".to_string(),
            args: vec![ContextBinding {
                var: "b".to_string(),
                chi: Chirality::Ext,
                ty: Ty::Int,
            }],
        }],
    };
    let ty_box_box = TypeDeclaration {
        name: "BoxBox".to_string(),
        xtors: vec![XtorSig {
            name: "BB".to_string(),
            args: vec![ContextBinding {
                var: "bb".to_string(),
                chi: Chirality::Prd,
                ty: Ty::Decl("Box".to_string()),
            }],
        }],
    };

    let main_body_switch_switch = Statement::Switch(Switch {
        var: "a2".to_string(),
        ty: Ty::Decl("Box".to_string()),
        clauses: vec![Clause {
            xtor: "B".to_string(),
            context: vec![ContextBinding {
                var: "y2".to_string(),
                chi: Chirality::Ext,
                ty: Ty::Int,
            }],
            case: Rc::new(Statement::Switch(Switch {
                var: "a1".to_string(),
                ty: Ty::Decl("Box".to_string()),
                clauses: vec![Clause {
                    xtor: "B".to_string(),
                    context: vec![ContextBinding {
                        var: "y1".to_string(),
                        chi: Chirality::Ext,
                        ty: Ty::Int,
                    }],
                    case: Rc::new(Statement::Op(Op {
                        fst: "y1".to_string(),
                        op: BinOp::Sum,
                        snd: "y2".to_string(),
                        var: "res".to_string(),
                        case: Rc::new(Statement::Return(Return {
                            var: "res".to_string(),
                        })),
                    })),
                }],
            })),
        }],
    });
    let main_body_switch = Statement::Switch(Switch {
        var: "bb".to_string(),
        ty: Ty::Decl("BoxBox".to_string()),
        clauses: vec![Clause {
            xtor: "BB".to_string(),
            context: vec![ContextBinding {
                var: "b1".to_string(),
                chi: Chirality::Prd,
                ty: Ty::Decl("Box".to_string()),
            }],
            case: Rc::new(Statement::Switch(Switch {
                var: "b1".to_string(),
                ty: Ty::Decl("Box".to_string()),
                clauses: vec![Clause {
                    xtor: "B".to_string(),
                    context: vec![ContextBinding {
                        var: "x1".to_string(),
                        chi: Chirality::Ext,
                        ty: Ty::Int,
                    }],
                    case: Rc::new(Statement::Leta(Leta {
                        var: "d1".to_string(),
                        ty: Ty::Decl("Box".to_string()),
                        tag: "B".to_string(),
                        args: vec!["x1".to_string()],
                        next: Rc::new(Statement::Leta(Leta {
                            var: "dd1".to_string(),
                            ty: Ty::Decl("BoxBox".to_string()),
                            tag: "BB".to_string(),
                            args: vec!["d1".to_string()],
                            next: Rc::new(Statement::Literal(Literal {
                                lit: 4,
                                var: "y".to_string(),
                                case: Rc::new(Statement::Leta(Leta {
                                    var: "a1".to_string(),
                                    ty: Ty::Decl("Box".to_string()),
                                    tag: "B".to_string(),
                                    args: vec!["y".to_string()],
                                    next: Rc::new(Statement::Switch(Switch {
                                        var: "bb".to_string(),
                                        ty: Ty::Decl("BoxBox".to_string()),
                                        clauses: vec![Clause {
                                            xtor: "BB".to_string(),
                                            context: vec![ContextBinding {
                                                var: "b2".to_string(),
                                                chi: Chirality::Prd,
                                                ty: Ty::Decl("Box".to_string()),
                                            }],
                                            case: Rc::new(Statement::Switch(Switch {
                                                var: "b2".to_string(),
                                                ty: Ty::Decl("Box".to_string()),
                                                clauses: vec![Clause {
                                                    xtor: "B".to_string(),
                                                    context: vec![ContextBinding {
                                                        var: "x2".to_string(),
                                                        chi: Chirality::Ext,
                                                        ty: Ty::Int,
                                                    }],
                                                    case: Rc::new(Statement::Leta(Leta {
                                                        var: "a2".to_string(),
                                                        ty: Ty::Decl("Box".to_string()),
                                                        tag: "B".to_string(),
                                                        args: vec!["x2".to_string()],
                                                        next: Rc::new(main_body_switch_switch),
                                                    })),
                                                }],
                                            })),
                                        }],
                                    })),
                                })),
                            })),
                        })),
                    })),
                }],
            })),
        }],
    });
    let main_body = Statement::Literal(Literal {
        lit: 3,
        var: "f1".to_string(),
        case: Rc::new(Statement::Literal(Literal {
            lit: 3,
            var: "f2".to_string(),
            case: Rc::new(Statement::Literal(Literal {
                lit: 3,
                var: "x".to_string(),
                case: Rc::new(Statement::Leta(Leta {
                    var: "b".to_string(),
                    ty: Ty::Decl("Box".to_string()),
                    tag: "B".to_string(),
                    args: vec!["x".to_string()],
                    next: Rc::new(Statement::Leta(Leta {
                        var: "bb".to_string(),
                        ty: Ty::Decl("BoxBox".to_string()),
                        tag: "BB".to_string(),
                        args: vec!["b".to_string()],
                        next: Rc::new(main_body_switch),
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

    let program = Prog {
        defs: vec![main],
        types: vec![ty_box, ty_box_box],
    };

    println!("{}", program::linearize(program).print_to_string(None))
}
