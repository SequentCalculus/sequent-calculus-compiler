//! This module define the symbol table used during typechecking.

use std::collections::HashMap;

use miette::SourceSpan;
use printer::Print;

use crate::syntax::{
    context::{TypeContext, TypingContext},
    declarations::{Codata, CtorSig, Data, Declaration, Def, DtorSig, Polarity},
    names::Name,
    program::Program,
    types::{Ty, TypeArgs},
};

use super::errors::Error;
use crate::parser::util::ToMiette;

/// This struct defines the symbol table used during typechecking. It contains mappings from names
/// to signatures for
/// - top-level function definitions
/// - monomorphic instances of constructors
/// - monomorphic instances of destructors
/// - monomorphic instances of user-declared data/codata types
/// - constructors of user-declared type templates with type parameters
/// - destructors of user-declared type templates with type parameters
/// - user-declared type templates with type parameters
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SymbolTable {
    /// Maps names of top-level [definitions][Def] to their signatures, i.e., their parameter list
    /// and return type.
    pub defs: HashMap<Name, (TypingContext, Ty)>,
    /// Maps names of monomorphic [constructors][CtorSig] to their signatures, i.e., their argument
    /// list.
    pub ctors: HashMap<Name, TypingContext>,
    /// Maps names of monomorphic [destructors][DtorSig] to their signatures, i.e., their argument
    /// list and return type.
    pub dtors: HashMap<Name, (TypingContext, Ty)>,
    /// Maps names of instances of user-declared [data](Data) and [codata](Codata) types to their
    /// [polarity](Polarity) determinig whether they are data or codata, to their type arguments
    /// instantiating the type parameters of the corresponding template, and to their name of xtors.
    pub types: HashMap<Name, (Polarity, TypeArgs, Vec<Name>)>,
    /// Maps names of [constructors][CtorSig] of a template to their signatures, i.e., their
    /// argument list.
    pub ctor_templates: HashMap<Name, TypingContext>,
    /// Maps names of [destructors][DtorSig] of a template to their signatures, i.e., their argument
    /// list and return type.
    pub dtor_templates: HashMap<Name, (TypingContext, Ty)>,
    /// Maps names of user-declared type templates for [data](Data) and [codata](Codata) types to
    /// their [polarity](Polarity) determining whether they are data or codata, to their type
    /// parameters, and to their name of xtors.
    pub type_templates: HashMap<Name, (Polarity, TypeContext, Vec<Name>)>,
}

impl SymbolTable {
    /// This function returns the monomorphic type of a monomorphic destructor from its name.
    pub fn lookup_ty_for_dtor(&self, span: &SourceSpan, dtor: &Name) -> Result<Ty, Error> {
        for (name, (pol, type_args, xtors)) in &self.types {
            if pol == &Polarity::Codata
                && xtors
                    .iter()
                    .any(|xtor| xtor.clone() + &type_args.print_to_string(None) == *dtor)
            {
                let ty = Ty::Decl {
                    span: None,
                    name: name.replace(&type_args.print_to_string(None), ""),
                    type_args: type_args.clone(),
                };
                return Ok(ty);
            }
        }
        Err(Error::Undefined {
            span: Some(*span),
            name: dtor.clone(),
        })
    }

    /// This function creates an instance of the type template a given non-monomorphic destructor
    /// belongs to and returns the created instance.
    /// - `dtor` is the name of the destructor.
    /// - `type_args` is the list of type arguments the type parameters of the template are
    ///   instantiated with.
    pub fn lookup_ty_template_for_dtor(
        &mut self,
        dtor: &Name,
        type_args: &TypeArgs,
    ) -> Result<Ty, Error> {
        for (name, (pol, _type_params, xtors)) in &self.type_templates {
            if pol == &Polarity::Codata && xtors.contains(dtor) {
                let ty = Ty::Decl {
                    span: None,
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

    /// This function returns the monomorphic type of a monomorphic constructor from its name.
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
                    span: None,
                    name: name.replace(&type_args.print_to_string(None), ""),
                    type_args: type_args.clone(),
                };
                return Ok((ty, xtors.clone()));
            }
        }
        Err(Error::Undefined {
            span: Some(*span),
            name: ctor.clone(),
        })
    }

    /// This function creates an instance of the type template a given non-monomorphic constructor
    /// belongs to and returns the created instance.
    /// - `dtor` is the name of the destructor.
    /// - `type_args` is the list of type arguments the type parameters of the template are
    ///   instantiated with.
    pub fn lookup_ty_template_for_ctor(
        &mut self,
        ctor: &Name,
        type_args: &TypeArgs,
    ) -> Result<(Ty, Vec<String>), Error> {
        for (name, (pol, _type_params, xtors)) in &self.type_templates {
            if pol == &Polarity::Data && xtors.contains(ctor) {
                let ty = Ty::Decl {
                    span: None,
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

    /// This function checks the well-formedness of all lists of type parameters in all type
    /// templates in the symbol table.
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

    /// This function combines two symbol tables into one.
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

/// This function builds a symbol table for a [program](Program).
pub fn build_symbol_table(module: &Program) -> Result<SymbolTable, Error> {
    let mut symbol_table = SymbolTable::default();
    module.build(&mut symbol_table)?;
    symbol_table.check_type_params()?;
    Ok(symbol_table)
}

/// This trait provides a method for adding entries to a symbol table.
pub trait BuildSymbolTable {
    /// This method adds an entry to the given symbol table.
    fn build(&self, symbol_table: &mut SymbolTable) -> Result<(), Error>;
}

impl BuildSymbolTable for Program {
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
                span: Some(self.span.to_miette()),
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
        syntax::{
            context::{Chirality::Prd, ContextBinding, TypingContext},
            program::Program,
            types::{Ty, TypeArgs},
            util::dummy_span,
        },
        test_common::{
            codata_stream, data_list, def_mult, symbol_table_list, symbol_table_list_template,
            symbol_table_lpair, symbol_table_stream_template,
        },
    };

    #[test]
    fn build_module() {
        let mut symbol_table = SymbolTable::default();
        Program {
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
                    span: None,
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
                    span: None,
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
            .lookup_ty_for_dtor(&dummy_span(), &"fst[i64, i64]".to_owned())
            .unwrap();
        let expected = Ty::mk_decl("LPair", TypeArgs::mk(vec![Ty::mk_i64(), Ty::mk_i64()]));
        assert_eq!(result, expected)
    }

    #[test]
    fn dtor_lookup_fail() {
        let result =
            SymbolTable::default().lookup_ty_for_dtor(&dummy_span(), &"snd[i64, i64]".to_owned());
        assert!(result.is_err())
    }

    #[test]
    fn ctor_lookup() {
        let symbol_table = symbol_table_list();
        let result = symbol_table
            .lookup_ty_for_ctor(&dummy_span(), &"Nil[i64]".to_owned())
            .unwrap();
        let expected = (
            Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_i64()])),
            Vec::from(["Nil".to_owned(), "Cons".to_owned()]),
        );
        assert_eq!(result, expected)
    }

    #[test]
    fn ctor_lookup_fail() {
        let result = SymbolTable::default().lookup_ty_for_ctor(&dummy_span(), &"Nil".to_owned());
        assert!(result.is_err())
    }
}
