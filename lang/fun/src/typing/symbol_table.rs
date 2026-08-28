//! This module define the symbol table used during typechecking.

use std::collections::HashMap;

use miette::SourceSpan;
use printer::Print;

use crate::syntax::{
    context::{TypeContext, TypingContext},
    declarations::{Codata, CtorSig, Data, Declaration, Def, DtorSig, Polarity},
    names::Name,
    program::Program,
    type_params::TypeParams,
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
    pub defs: HashMap<Name, (TypeParams, TypingContext, Ty)>,
    /// Maps names of monomorphic [constructors][CtorSig] (with the type arguments of the
    /// surrounding data type instance already substituted) to their own (still open) type
    /// parameters and their argument list.
    pub ctors: HashMap<Name, (TypeParams, TypingContext)>,
    /// Maps names of monomorphic [destructors][DtorSig] (with the type arguments of the
    /// surrounding codata type instance already substituted) to their own (still open) type
    /// parameters, their argument list, and their return type.
    pub dtors: HashMap<Name, (TypeParams, TypingContext, Ty)>,
    /// Maps names of instances of user-declared [data](Data) and [codata](Codata) types to their
    /// [polarity](Polarity) determinig whether they are data or codata, to their type arguments
    /// instantiating the type parameters of the corresponding template, and to their name of xtors.
    pub types: HashMap<Name, (Polarity, TypeArgs, Vec<Name>)>,
    /// Maps names of [constructors][CtorSig] of a template to their own (existential) type
    /// parameters and their signatures, i.e., their argument list.
    pub ctor_templates: HashMap<Name, (TypeParams, TypingContext)>,
    /// Maps names of [destructors][DtorSig] of a template to their own (existential) type
    /// parameters and their signatures, i.e., their argument list and return type.
    pub dtor_templates: HashMap<Name, (TypeParams, TypingContext, Ty)>,
    /// Maps names of user-declared type templates for [data](Data) and [codata](Codata) types to
    /// their [polarity](Polarity) determining whether they are data or codata, to their type
    /// parameters, and to their name of xtors.
    pub type_templates: HashMap<Name, (Polarity, TypeParams, Vec<Name>)>,
    /// Maps type variables that are currently in scope as abstract type variables to their
    /// declared [polarity](Polarity), e.g. bound by an existential constructor pattern in a
    /// `case` clause (`Cons[B](x, xs)`) or a universal destructor parameter in a `new` clause
    /// (`head[B]`). Such names are treated as valid, opaque monomorphic types without requiring
    /// an instance or template to exist for them.
    pub abstract_type_vars: HashMap<Name, Polarity>,
}

impl SymbolTable {
    /// This function instantiates the signature of a top-level definition with given type
    /// arguments.
    pub fn instantiate_def_signature(
        &mut self,
        span: Option<SourceSpan>,
        name: &Name,
        type_args: &TypeArgs,
    ) -> Result<(TypingContext, Ty), Error> {
        let Some((type_params, context_template, ret_ty_template)) = self.defs.get(name).cloned()
        else {
            return Err(Error::Undefined {
                span,
                name: name.clone(),
            });
        };

        type_args.is_instance(&type_params, self)?;
        let mappings: HashMap<Name, Ty> = type_params
            .names()
            .into_iter()
            .zip(type_args.args.clone())
            .collect();

        Ok((
            context_template.subst_ty(&mappings),
            ret_ty_template.subst_ty(&mappings),
        ))
    }

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
                    name: name.clone(),
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
                    name: name.clone(),
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

    /// This function resolves a constructor invocation against an expected data type.
    ///
    /// It checks the well-formedness of `data_ty` (creating a monomorphic instance of the
    /// corresponding data type template if necessary) and looks up the monomorphic constructor
    /// signature for `ctor` instantiated with `data_ty`'s type arguments.
    ///
    /// Note that the returned argument context may still contain the constructor's own
    /// (existential) type parameters, since those are not determined by `data_ty` and must be
    /// substituted separately by the caller using the returned [TypeContext].
    /// - `span` is the source location of the constructor invocation.
    /// - `ctor` is the name of the constructor.
    /// - `data_ty` is the expected (fully instantiated) data type, e.g. `List[i64]`.
    pub fn lookup_ctor_signature(
        &mut self,
        span: &SourceSpan,
        ctor: &Name,
        data_ty: &Ty,
    ) -> Result<(TypeParams, TypingContext), Error> {
        data_ty.check(&Some(*span), self)?;

        let Ty::Decl { type_args, .. } = data_ty else {
            return Err(Error::ExpectedI64ForConstructor {
                span: *span,
                name: ctor.clone(),
            });
        };

        let ctor_instance_name = ctor.clone() + &type_args.print_to_string(None);
        self.ctors
            .get(&ctor_instance_name)
            .cloned()
            .ok_or_else(|| Error::Undefined {
                span: Some(*span),
                name: ctor.clone(),
            })
    }

    /// This function resolves a destructor invocation from its name and the full list of type
    /// arguments as written at the call site.
    ///
    /// The type arguments syntactically appear as a single list (e.g. `x.head[i64, T]`), but they
    /// actually consist of two logically distinct parts: the type arguments instantiating the
    /// type parameters of the codata type the destructor belongs to (e.g. `A` in `Stream[A]`),
    /// followed by the destructor's own (existential) type parameters (e.g. `T`). This function
    /// splits the list according to the arity known from the codata type template, creates the
    /// codata type instance if necessary, and fully substitutes both the codata type parameters
    /// and the destructor's own type parameters in the argument context and return type.
    /// - `span` is the source location of the destructor invocation.
    /// - `dtor` is the name of the destructor.
    /// - `all_type_args` is the full, unsplit list of type arguments as written at the call site.
    ///
    /// Returns the (fully instantiated) type of the scrutinee, the fully substituted argument
    /// context, and the fully substituted return type of the destructor.
    pub fn lookup_dtor_signature(
        &mut self,
        span: &SourceSpan,
        dtor: &Name,
        all_type_args: &TypeArgs,
    ) -> Result<(Ty, TypingContext, Ty), Error> {
        // Find the codata type template the destructor belongs to, together with its type
        // parameters, in order to determine the split point in `all_type_args`.
        let (codata_name, codata_type_params) = self
            .type_templates
            .iter()
            .find_map(|(name, (pol, type_params, xtors))| {
                (*pol == Polarity::Codata && xtors.contains(dtor))
                    .then(|| (name.clone(), type_params.clone()))
            })
            .ok_or_else(|| Error::Undefined {
                span: Some(*span),
                name: dtor.clone(),
            })?;

        // Look up the destructor's own type parameters to validate the total arity.
        let (own_type_params, _, _) =
            self.dtor_templates
                .get(dtor)
                .cloned()
                .ok_or_else(|| Error::Undefined {
                    span: Some(*span),
                    name: dtor.clone(),
                })?;

        let split = codata_type_params.bindings.len();
        if all_type_args.args.len() < split {
            return Err(Error::WrongNumberOfTypeArguments {
                span: all_type_args.span.to_miette(),
                expected: split + own_type_params.bindings.len(),
                got: all_type_args.args.len(),
            });
        }

        // Split off the leading type arguments instantiating the codata type's own type
        // parameters from the trailing type arguments belonging to the destructor itself.
        let (codata_args, dtor_own_args) = all_type_args.args.split_at(split);
        let codata_type_args = TypeArgs {
            span: all_type_args.span,
            args: codata_args.to_vec(),
        };
        let dtor_own_type_args = TypeArgs {
            span: all_type_args.span,
            args: dtor_own_args.to_vec(),
        };

        // Create/check the codata type instance, e.g. `Stream[i64]`, and check the scrutinee against it.
        let scrutinee_ty = Ty::Decl {
            span: None,
            name: codata_name,
            type_args: codata_type_args.clone(),
        };
        scrutinee_ty.check(&all_type_args.span, self)?;

        // Look up the monomorphic destructor template (with the codata type parameters already
        // substituted by `scrutinee_ty.check`) and substitute the destructor's own type
        // parameters with `dtor_own_type_args`.
        let dtor_instance_name = dtor.clone() + &codata_type_args.print_to_string(None);
        let (own_type_params, args_template, cont_ty_template) = self
            .dtors
            .get(&dtor_instance_name)
            .cloned()
            .ok_or_else(|| Error::Undefined {
                span: Some(*span),
                name: dtor.clone(),
            })?;

        dtor_own_type_args.is_instance(&own_type_params, self)?;

        let mappings: HashMap<Name, Ty> = own_type_params
            .names()
            .into_iter()
            .zip(dtor_own_type_args.args.iter().cloned())
            .collect();

        let args = args_template.subst_ty(&mappings);
        let cont_ty = cont_ty_template.subst_ty(&mappings);

        Ok((scrutinee_ty, args, cont_ty))
    }

    /// This function checks the well-formedness of all lists of type parameters in all type
    /// templates in the symbol table.
    pub fn check_type_params(&self) -> Result<(), Error> {
        for (name, (_, type_params, _)) in &self.type_templates {
            type_params.no_dups(name)?;
            for param in &type_params.bindings {
                if self.type_templates.contains_key(&param.name) {
                    return Err(Error::DefinedMultipleTimes {
                        span: type_params.span.to_miette(),
                        name: param.name.clone(),
                    });
                }
            }
        }

        for (name, (type_params, _, _)) in &self.defs {
            type_params.no_dups(name)?;
            for param in &type_params.bindings {
                if self.type_templates.contains_key(&param.name) {
                    return Err(Error::DefinedMultipleTimes {
                        span: type_params.span.to_miette(),
                        name: param.name.clone(),
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

    /// This function brings the given names into scope as abstract type variables, e.g.
    /// the type parameters bound by a `case`/`new` clause. It returns the corresponding list of
    /// opaque types (one `Ty::Decl` per name, in order), so callers can use them directly for
    /// substitution.
    ///
    /// Fails if any of the given names already denotes a declared data/codata type template, or
    /// is already in scope as a rigid variable (e.g. due to a nested clause shadowing an outer
    /// one), since that would silently shadow an existing type and lead to confusing errors
    /// elsewhere.
    ///
    /// Each user-chosen name in `names` is paired positionally with the corresponding
    /// `TypeParam` in `own_type_params` (the ctor's/dtor's own declared existential/universal
    /// parameter list), so that the name is brought into scope with the *declared* polarity
    /// rather than a guessed default. Callers must ensure both lists have equal length (an arity
    /// mismatch is a separate, earlier error).
    /// - `span` is the source location of the clause introducing the names.
    /// - `names` are the type parameter names to bring into scope.
    /// - `own_type_params` is the ctor's/dtor's own declared type parameter list, in the same
    ///   order as `names`.
    pub fn push_abstract_vars(
        &mut self,
        span: &SourceSpan,
        names: &TypeContext,
        own_type_params: &TypeParams,
    ) -> Result<Vec<Ty>, Error> {
        let mut result = vec![];
        for (name, own_param) in names.bindings.iter().zip(&own_type_params.bindings) {
            if self.type_templates.contains_key(name) {
                return Err(Error::DefinedMultipleTimes {
                    span: Some(*span),
                    name: name.clone(),
                });
            }
            if self.abstract_type_vars.contains_key(name) {
                return Err(Error::DefinedMultipleTimes {
                    span: Some(*span),
                    name: name.clone(),
                });
            }
            self.abstract_type_vars
                .insert(name.clone(), own_param.polarity.clone());
            result.push(Ty::Decl {
                span: None,
                name: name.clone(),
                type_args: TypeArgs::default(),
            });
        }
        Ok(result)
    }

    /// This function removes the given names from the set of abstract type variables again, e.g.
    /// after typechecking the clause body that bound them, so that they cannot leak into later,
    /// unrelated clauses.
    pub fn pop_abstract_vars(&mut self, names: &TypeContext) {
        for name in &names.bindings {
            self.abstract_type_vars.remove(name);
        }
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
                span: Some(self.span),
                name: self.name.clone(),
            });
        }
        symbol_table.defs.insert(
            self.name.clone(),
            (
                self.type_params.clone(),
                self.context.clone(),
                self.ret_ty.clone(),
            ),
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
        symbol_table.ctor_templates.insert(
            self.name.clone(),
            (self.type_params.clone(), self.args.clone()),
        );
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
        symbol_table.dtor_templates.insert(
            self.name.clone(),
            (
                self.type_params.clone(),
                self.args.clone(),
                self.cont_ty.clone(),
            ),
        );
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
            type_params::TypeParams,
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
                TypeParams::default(),
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
                TypeParams::default(),
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
