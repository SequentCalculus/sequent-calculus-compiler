//! This module defines the translation of a destructor.

use std::{collections::HashMap, rc::Rc};

use crate::{
    arguments::compile_subst,
    compile::{Compile, CompileState, bind_many},
    types::{compile_ty, compile_type_args},
};
use core_lang::syntax::{
    names::Identifier,
    terms::Cns,
    type_params::{ParamPolarity, TypeParam},
    types::TypeArgs,
};
use fun::traits::OptTyped;

impl Compile for fun::syntax::terms::Destructor {
    /// This implementation of [Compile::compile_with_cont] proceeds as follows.
    /// ```text
    /// 〚t.D(t_1, ...) 〛_{c} = bind_many_v(〚t_1, ...〛)[λas.〚t〛_{D(as, c)}]
    /// ```
    ///
    /// # Panics
    ///
    /// A panic is caused if the types are not annotated in the program.
    fn compile_with_cont(
        self,
        cont: core_lang::syntax::terms::Term<Cns>,
        state: &mut CompileState,
        type_params: Rc<HashMap<String, (Identifier, ParamPolarity)>>,
    ) -> core_lang::syntax::Statement {
        let ambient_type_params: Rc<Vec<TypeParam>> = Rc::new(
            type_params
                .values()
                .map(|(id, polarity)| TypeParam {
                    name: id.clone(),
                    polarity: *polarity,
                })
                .collect(),
        );
        bind_many(
            compile_subst(self.args, state, type_params.clone()).into(),
            Box::new(move |mut bindings, state| {
                bindings.push_back(cont.into());

                let scrutinee_ty = self
                    .scrutinee
                    .get_type()
                    .expect("Types should be annotated before translation");

                // The number of type arguments belonging to the codata type itself (as opposed to
                // the destructor's own type arguments) equals the number of type arguments the
                // scrutinee's type is instantiated with.
                let codata_arity = match &scrutinee_ty {
                    fun::syntax::types::Ty::Decl { type_args, .. } => type_args.args.len(),
                    fun::syntax::types::Ty::I64 { .. } => 0,
                };

                // Split off the leading type arguments (belonging to the codata type, already
                // reflected in `ty` below) from the trailing type arguments belonging to the
                // destructor itself.
                let type_args = TypeArgs {
                    args: compile_type_args(&self.type_args, type_params.clone())
                        .args
                        .split_off(codata_arity),
                };

                // lookup the concret name of the destructor in the codata types
                let Some(name) = state.codata_types.iter().find_map(|codata_decl| {
                    codata_decl
                        .xtors
                        .iter()
                        .find(|dtor| dtor.name.name == self.id)
                        .map(|dtor| dtor.name.clone())
                }) else {
                    panic!("Destructor {} not found in codata types", self.id);
                };

                // new continuation: D(〚t_1〛, ..., c)
                let new_cont = core_lang::syntax::terms::Xtor {
                    prdcns: Cns,
                    name,
                    type_args,
                    args: bindings.into(),
                    ty: compile_ty(
                        &self
                            .scrutinee
                            .get_type()
                            .expect("Types should be annotated before translation"),
                        type_params.clone(),
                    ),
                }
                .into();

                // 〚t〛_{new_cont}
                self.scrutinee
                    .compile_with_cont(new_cont, state, type_params)
            }),
            state,
            ambient_type_params,
        )
    }
}

#[cfg(test)]
mod compile_tests {
    use crate::compile::{Compile, CompileState};
    use core_lang::syntax::{CodataDeclaration, terms::Prd};
    use core_macros::{
        bind, clause, cns, cocase, codata, covar, cut, dtor, dtor_sig, id, lit, mu, prd, tparam,
        tvar, ty,
    };
    use fun::{parse_term, test_common::symbol_table_lpair, typing::check::Check};
    use std::{
        collections::{HashSet, VecDeque},
        rc::Rc,
    };

    fn pair() -> CodataDeclaration {
        return codata!(
            id!("LPair"),
            [
                dtor_sig!(
                    id!("fst"),
                    [],
                    [bind!(id!("out"), prd!(), tvar!(id!("A", 1)))]
                ),
                dtor_sig!(
                    id!("snd"),
                    [],
                    [bind!(id!("out"), prd!(), tvar!(id!("B", 2)))]
                )
            ],
            [tparam!(id!("A", 1), "+"), tparam!(id!("B", 2), "+")]
        );
    }

    #[test]
    fn compile_fst() {
        let term = parse_term!("new { fst => 1, snd => 2}.fst[i64, i64]");
        let term_typed = term
            .check(
                &mut symbol_table_lpair(),
                &fun::syntax::context::TypingContext::default(),
                &fun::syntax::types::Ty::mk_i64(),
            )
            .unwrap();

        let mut state = CompileState {
            used_vars: HashSet::default(),
            codata_types: &[pair()],
            data_types: &[],
            used_labels: &mut HashSet::default(),
            current_label: "",
            lifted_statements: &mut VecDeque::default(),
            max_id: &mut 0,
        };
        let result =
            term_typed.compile(&mut state, core_lang::syntax::types::Ty::I64, Rc::default());

        let expected = mu!(
            id!("a0"),
            cut!(
                cocase!(
                    [
                        clause!(
                            Prd,
                            id!("fst"),
                            [],
                            [bind!(id!("a1"), cns!())],
                            cut!(lit!(1), covar!(id!("a1")))
                        ),
                        clause!(
                            Prd,
                            id!("snd"),
                            [],
                            [bind!(id!("a2"), cns!())],
                            cut!(lit!(2), covar!(id!("a2")))
                        )
                    ],
                    ty!(id!("LPair"), vec![ty!("int"), ty!("int")])
                ),
                dtor!(
                    id!("fst"),
                    [],
                    [covar!(id!("a0"))],
                    ty!(id!("LPair"), vec![ty!("int"), ty!("int")])
                ),
                ty!(id!("LPair"), vec![ty!("int"), ty!("int")])
            )
        )
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn compile_snd() {
        let term = parse_term!("new { fst => 1, snd => 2}.snd[i64, i64]");
        let term_typed = term
            .check(
                &mut symbol_table_lpair(),
                &fun::syntax::context::TypingContext::default(),
                &fun::syntax::types::Ty::mk_i64(),
            )
            .unwrap();

        let mut state = CompileState {
            used_vars: HashSet::default(),
            codata_types: &[pair()],
            data_types: &[],
            used_labels: &mut HashSet::default(),
            current_label: "",
            lifted_statements: &mut VecDeque::default(),
            max_id: &mut 0,
        };
        let result = term_typed.compile(&mut state, ty!("int"), Rc::default());

        let expected = mu!(
            id!("a0"),
            cut!(
                cocase!(
                    [
                        clause!(
                            Prd,
                            id!("fst"),
                            [],
                            [bind!(id!("a1"), cns!())],
                            cut!(lit!(1), covar!(id!("a1")))
                        ),
                        clause!(
                            Prd,
                            id!("snd"),
                            [],
                            [bind!(id!("a2"), cns!())],
                            cut!(lit!(2), covar!(id!("a2")))
                        )
                    ],
                    ty!(id!("LPair"), vec![ty!("int"), ty!("int")])
                ),
                dtor!(
                    id!("snd"),
                    [],
                    [covar!(id!("a0"))],
                    ty!(id!("LPair"), vec![ty!("int"), ty!("int")])
                ),
                ty!(id!("LPair"), vec![ty!("int"), ty!("int")])
            )
        )
        .into();
        assert_eq!(result, expected)
    }
}
