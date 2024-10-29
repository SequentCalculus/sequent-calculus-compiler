use super::{NamingTransformation, TransformState};
use crate::syntax::{
    context::{context_covars, context_vars},
    program::Declaration,
    Def, Prog,
};

pub fn transform_def(def: Def, st: &mut TransformState) -> Def {
    st.used_vars = context_vars(&def.context);
    st.used_covars = context_covars(&def.context);

    Def {
        name: def.name,
        context: def.context,
        body: def.body.transform(st),
    }
}

pub fn transform_decl(decl: Declaration, st: &mut TransformState) -> Declaration {
    match decl {
        Declaration::Definition(def) => transform_def(def, st).into(),
        _ => decl,
    }
}

pub fn transform_prog(prog: Prog) -> Prog {
    let mut state = TransformState::from(&prog);
    Prog {
        prog_decls: prog
            .prog_decls
            .into_iter()
            .map(|decl| transform_decl(decl, &mut state))
            .collect(),
    }
}

#[cfg(test)]
mod transform_prog_tests {
    use super::{transform_def, transform_prog, TransformState};
    use crate::syntax::{
        context::ContextBinding,
        program::Declaration,
        statement::Cut,
        term::{Cns, Prd, XVar},
        types::Ty,
        Def, Prog, Statement,
    };
    use std::rc::Rc;

    fn example_def1() -> Def {
        Def {
            name: "done".to_owned(),
            context: vec![],
            body: Statement::Done(),
        }
    }

    fn example_def2() -> Def {
        Def {
            name: "cut".to_owned(),
            context: vec![
                ContextBinding::VarBinding {
                    var: "x".to_owned(),
                    ty: Ty::Int(),
                },
                ContextBinding::CovarBinding {
                    covar: "a".to_owned(),
                    ty: Ty::Int(),
                },
            ],
            body: Cut {
                producer: Rc::new(
                    XVar {
                        prdcns: Prd,
                        var: "x".to_owned(),
                    }
                    .into(),
                ),
                ty: Ty::Int(),
                consumer: Rc::new(
                    XVar {
                        prdcns: Cns,
                        var: "a".to_owned(),
                    }
                    .into(),
                ),
            }
            .into(),
        }
    }

    fn example_prog1() -> Prog {
        Prog { prog_decls: vec![] }
    }

    fn example_prog2() -> Prog {
        Prog {
            prog_decls: vec![example_def1().into()],
        }
    }

    #[test]
    fn transform_def1() {
        let result = transform_def(example_def1(), &mut TransformState::default());
        let expected = example_def1();
        assert_eq!(result.name, expected.name);
        assert_eq!(result.context, expected.context);
        assert_eq!(result.body, expected.body);
    }

    #[test]
    fn transform_def2() {
        let result = transform_def(example_def2(), &mut TransformState::default());
        let expected = example_def2();
        assert_eq!(result.name, expected.name);
        assert_eq!(result.context, expected.context);
        assert_eq!(result.body, expected.body);
    }

    #[test]
    fn transform_prog1() {
        let result = transform_prog(example_prog1());
        assert!(result.prog_decls.is_empty())
    }

    #[test]
    fn transform_prog2() {
        let result = transform_prog(example_prog2());
        assert_eq!(result.prog_decls.len(), 1);
        let def1 = result.prog_decls.get(0);
        assert!(def1.is_some());
        let def1un = def1.unwrap();
        let def = if let Declaration::Definition(def) = def1un {
            Some(def)
        } else {
            None
        }
        .unwrap();
        let ex = example_def1();
        assert_eq!(def.name, ex.name);
        assert_eq!(def.context, ex.context);
        assert_eq!(def.body, ex.body);
    }
}
