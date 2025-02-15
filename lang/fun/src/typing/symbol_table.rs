use std::collections::HashMap;

use codespan::Span;
use miette::SourceSpan;
use printer::Print;

use crate::syntax::{
    context::{TypeContext, TypingContext},
    declarations::{Codata, CtorSig, Data, Declaration, Def, DtorSig, Module, Polarity},
    types::{Ty, TypeArgs},
    Name,
};

use super::errors::Error;
use crate::parser::util::ToMiette;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SymbolTable {
    pub defs: HashMap<Name, (TypingContext, Ty)>,
    pub ctors: HashMap<Name, TypingContext>,
    pub dtors: HashMap<Name, (TypingContext, Ty)>,
    pub types: HashMap<Name, (Polarity, TypeArgs, Vec<Name>)>,
    pub ctor_templates: HashMap<Name, TypingContext>,
    pub dtor_templates: HashMap<Name, (TypingContext, Ty)>,
    pub type_templates: HashMap<Name, (Polarity, TypeContext, Vec<Name>)>,
}

impl SymbolTable {
    pub fn lookup_ty_for_dtor(&self, span: &SourceSpan, dtor: &Name) -> Result<Ty, Error> {
        for (name, (pol, type_args, xtors)) in &self.types {
            if pol == &Polarity::Codata
                && xtors
                    .iter()
                    .any(|xtor| xtor.clone() + &type_args.print_to_string(None) == *dtor)
            {
                let ty = Ty::Decl {
                    span: Span::default(),
                    name: name.replace(&type_args.print_to_string(None), ""),
                    type_args: type_args.clone(),
                };
                return Ok(ty);
            }
        }
        Err(Error::Undefined {
            span: *span,
            name: dtor.clone(),
        })
    }

    pub fn lookup_ty_template_for_dtor(
        &mut self,
        dtor: &Name,
        type_args: &TypeArgs,
    ) -> Result<Ty, Error> {
        for (name, (pol, _type_params, xtors)) in &self.type_templates {
            if pol == &Polarity::Codata && xtors.contains(dtor) {
                let ty = Ty::Decl {
                    span: Span::default(),
                    name: name.to_string(),
                    type_args: type_args.clone(),
                };
                ty.check(&type_args.span, self)?;
                return Ok(ty);
            }
        }
        Err(Error::UndefinedWrongTypeArguments {
            span: type_args.span.to_miette(),
            name: dtor.clone(),
            type_args: type_args.print_to_string(None),
        })
    }

    pub fn lookup_ty_for_ctor(
        &self,
        span: &SourceSpan,
        ctor: &Name,
    ) -> Result<(Ty, Vec<String>), Error> {
        for (name, (pol, type_args, xtors)) in &self.types {
            if pol == &Polarity::Data
                && xtors
                    .iter()
                    .any(|xtor| xtor.clone() + &type_args.print_to_string(None) == *ctor)
            {
                let ty = Ty::Decl {
                    span: Span::default(),
                    name: name.replace(&type_args.print_to_string(None), ""),
                    type_args: type_args.clone(),
                };
                return Ok((ty, xtors.clone()));
            }
        }
        Err(Error::Undefined {
            span: *span,
            name: ctor.clone(),
        })
    }

    pub fn lookup_ty_template_for_ctor(
        &mut self,
        ctor: &Name,
        type_args: &TypeArgs,
    ) -> Result<(Ty, Vec<String>), Error> {
        for (name, (pol, _type_params, xtors)) in &self.type_templates {
            if pol == &Polarity::Data && xtors.contains(ctor) {
                let ty = Ty::Decl {
                    span: Span::default(),
                    name: name.to_string(),
                    type_args: type_args.clone(),
                };
                let xtors = xtors.clone();
                ty.check(&type_args.span, self)?;
                return Ok((ty, xtors));
            }
        }
        Err(Error::UndefinedWrongTypeArguments {
            span: type_args.span.to_miette(),
            name: ctor.clone() + &type_args.print_to_string(None),
            type_args: type_args.print_to_string(None),
        })
    }

    pub fn check_type_params(&self) -> Result<(), Error> {
        for (name, (_, type_params, _)) in &self.type_templates {
            type_params.no_dups(name)?;
            for param in &type_params.bindings {
                if self.type_templates.contains_key(param) {
                    return Err(Error::DefinedMultipleTimes {
                        span: type_params.span.to_miette(),
                        name: param.clone(),
                    });
                }
            }
        }
        Ok(())
    }

    pub fn combine(&mut self, other: SymbolTable) {
        self.defs.extend(other.defs);
        self.ctors.extend(other.ctors);
        self.dtors.extend(other.dtors);
        self.types.extend(other.types);
        self.ctor_templates.extend(other.ctor_templates);
        self.dtor_templates.extend(other.dtor_templates);
        self.type_templates.extend(other.type_templates);
    }
}

pub fn build_symbol_table(module: &Module) -> Result<SymbolTable, Error> {
    let mut symbol_table = SymbolTable::default();
    module.build(&mut symbol_table)?;
    symbol_table.check_type_params()?;
    Ok(symbol_table)
}

pub trait BuildSymbolTable {
    fn build(&self, symbol_table: &mut SymbolTable) -> Result<(), Error>;
}

impl BuildSymbolTable for Module {
    fn build(&self, symbol_table: &mut SymbolTable) -> Result<(), Error> {
        for declaration in &self.declarations {
            declaration.build(symbol_table)?;
        }
        Ok(())
    }
}

impl BuildSymbolTable for Declaration {
    fn build(&self, symbol_table: &mut SymbolTable) -> Result<(), Error> {
        match self {
            Declaration::Def(def) => def.build(symbol_table),
            Declaration::Data(data) => data.build(symbol_table),
            Declaration::Codata(codata) => codata.build(symbol_table),
        }
    }
}

impl BuildSymbolTable for Def {
    fn build(&self, symbol_table: &mut SymbolTable) -> Result<(), Error> {
        if symbol_table.defs.contains_key(&self.name) {
            return Err(Error::DefinedMultipleTimes {
                span: self.span.to_miette(),
                name: self.name.clone(),
            });
        }
        symbol_table.defs.insert(
            self.name.clone(),
            (self.context.clone(), self.ret_ty.clone()),
        );
        Ok(())
    }
}

impl BuildSymbolTable for Data {
    fn build(&self, symbol_table: &mut SymbolTable) -> Result<(), Error> {
        if symbol_table.type_templates.contains_key(&self.name) {
            return Err(Error::DefinedMultipleTimes {
                span: self.span.to_miette(),
                name: self.name.clone(),
            });
        }
        symbol_table.type_templates.insert(
            self.name.clone(),
            (
                Polarity::Data,
                self.type_params.clone(),
                self.ctors.iter().map(|ctor| ctor.name.clone()).collect(),
            ),
        );

        for ctor in &self.ctors {
            ctor.build(symbol_table)?;
        }
        Ok(())
    }
}

impl BuildSymbolTable for CtorSig {
    fn build(&self, symbol_table: &mut SymbolTable) -> Result<(), Error> {
        if symbol_table.ctor_templates.contains_key(&self.name) {
            return Err(Error::DefinedMultipleTimes {
                span: self.span.to_miette(),
                name: self.name.clone(),
            });
        }
        symbol_table
            .ctor_templates
            .insert(self.name.clone(), self.args.clone());
        Ok(())
    }
}

impl BuildSymbolTable for Codata {
    fn build(&self, symbol_table: &mut SymbolTable) -> Result<(), Error> {
        if symbol_table.type_templates.contains_key(&self.name) {
            return Err(Error::DefinedMultipleTimes {
                span: self.span.to_miette(),
                name: self.name.clone(),
            });
        }
        symbol_table.type_templates.insert(
            self.name.clone(),
            (
                Polarity::Codata,
                self.type_params.clone(),
                self.dtors.iter().map(|ctor| ctor.name.clone()).collect(),
            ),
        );

        for dtor in &self.dtors {
            dtor.build(symbol_table)?;
        }
        Ok(())
    }
}

impl BuildSymbolTable for DtorSig {
    fn build(&self, symbol_table: &mut SymbolTable) -> Result<(), Error> {
        if symbol_table.dtor_templates.contains_key(&self.name) {
            return Err(Error::DefinedMultipleTimes {
                span: self.span.to_miette(),
                name: self.name.clone(),
            });
        }
        symbol_table
            .dtor_templates
            .insert(self.name.clone(), (self.args.clone(), self.cont_ty.clone()));
        Ok(())
    }
}

#[cfg(test)]
mod symbol_table_tests {
    use super::{BuildSymbolTable, SymbolTable};
    use crate::{
        parser::util::ToMiette,
        syntax::{
            context::{Chirality::Prd, ContextBinding, TypingContext},
            declarations::Module,
            types::{Ty, TypeArgs},
        },
        test_common::{
            codata_stream, data_list, def_mult, symbol_table_list, symbol_table_list_template,
            symbol_table_lpair, symbol_table_stream_template,
        },
    };
    use codespan::Span;

    #[test]
    fn build_module() {
        let mut symbol_table = SymbolTable::default();
        Module {
            declarations: vec![
                data_list().into(),
                codata_stream().into(),
                def_mult().into(),
            ],
        }
        .build(&mut symbol_table)
        .unwrap();
        let mut expected = symbol_table_list_template();
        expected.combine(symbol_table_stream_template());
        expected.defs.insert(
            "mult".to_owned(),
            (
                TypingContext {
                    span: Span::default(),
                    bindings: vec![ContextBinding {
                        var: "l".to_owned(),
                        chi: Prd,
                        ty: Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_i64()])),
                    }],
                },
                Ty::mk_i64(),
            ),
        );
        assert_eq!(symbol_table, expected)
    }

    #[test]
    fn build_data() {
        let mut symbol_table = SymbolTable::default();
        data_list().build(&mut symbol_table).unwrap();
        let expected = symbol_table_list_template();
        assert_eq!(symbol_table, expected)
    }

    #[test]
    fn build_codata() {
        let mut symbol_table = SymbolTable::default();
        codata_stream().build(&mut symbol_table).unwrap();
        let expected = symbol_table_stream_template();
        assert_eq!(symbol_table, expected)
    }

    #[test]
    fn build_def() {
        let mut symbol_table = SymbolTable::default();
        def_mult().build(&mut symbol_table).unwrap();
        let mut expected = SymbolTable::default();
        expected.defs.insert(
            "mult".to_owned(),
            (
                TypingContext {
                    span: Span::default(),
                    bindings: vec![ContextBinding {
                        var: "l".to_owned(),
                        chi: Prd,
                        ty: Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_i64()])),
                    }],
                },
                Ty::mk_i64(),
            ),
        );
        assert_eq!(symbol_table, expected)
    }

    #[test]
    fn dtor_lookup() {
        let symbol_table = symbol_table_lpair();
        let result = symbol_table
            .lookup_ty_for_dtor(&Span::default().to_miette(), &"Fst[i64, i64]".to_owned())
            .unwrap();
        let expected = Ty::mk_decl("LPair", TypeArgs::mk(vec![Ty::mk_i64(), Ty::mk_i64()]));
        assert_eq!(result, expected)
    }

    #[test]
    fn dtor_lookup_fail() {
        let result = SymbolTable::default()
            .lookup_ty_for_dtor(&Span::default().to_miette(), &"Snd[i64, i64]".to_owned());
        assert!(result.is_err())
    }

    #[test]
    fn ctor_lookup() {
        let symbol_table = symbol_table_list();
        let result = symbol_table
            .lookup_ty_for_ctor(&Span::default().to_miette(), &"Nil[i64]".to_owned())
            .unwrap();
        let expected = (
            Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_i64()])),
            Vec::from(["Nil".to_owned(), "Cons".to_owned()]),
        );
        assert_eq!(result, expected)
    }

    #[test]
    fn ctor_lookup_fail() {
        let result = SymbolTable::default()
            .lookup_ty_for_ctor(&Span::default().to_miette(), &"Nil".to_owned());
        assert!(result.is_err())
    }
}
