//! This module defines programs in Core.

use printer::*;
use std::collections::{HashSet, HashMap,};

use crate::syntax::*;
use crate::typing::*;

/// This struct defines a module consisting of a list of [`Declaration`]s.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program {
    pub module_declarations: Vec<ModuleDeclaration>,
    pub declarations: Vec<Declaration>,
}

/// This struct defines a program consiting of mulitple files/submodules
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModuleProgram {
    /// The imports found in the file
    //pub imports: Vec<ModuleProgram>,
    pub imports: Vec<ModuleProgram>,
    /// The submodule declared in the file
    pub modules: Vec<ModuleProgram>,
    /// The public top-level functions in the file
    pub declarations: Vec<Declaration>,
    /// The name of the module
    pub name: String,
    /// The imports that a parent module needs to import
    pub imports_to_parent: HashMap<Name, ModuleProgram>,
    /// The declarations of the parent module(only if there is one)
    pub parent_declarations: Option<(String, Vec<Declaration>)>,
    /// The public declarations of the module
    pub public_declarations: Vec<Declaration>
}


/// This struct defines a typechecked module created from a [`Program`] by checking each contained
/// [`Declaration`]. The checked module only contans monomorphic instances of data and codata types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckedProgram {
    /// Checked data type instances
    pub data_types: Vec<Data>,
    /// Checked codata type instances
    pub codata_types: Vec<Codata>,
    /// Checked top-level functions
    pub defs: Vec<Def>,
    /// The name of the module
    pub name: String,
}

impl ModuleProgram {
    /// This function typechecks all declarations in a module, creating a checked module with
    /// monomorphic type instances.
    pub fn check(self, has_parent: bool) -> Result<CheckedProgram, Error> {
        let mut import_decl = Vec::<(String, Vec::<Declaration>)>::new();
        for import in &self.imports {
            if !import.public_declarations.is_empty() {
                import_decl.push((import.name.clone(), import.public_declarations.clone()));
            }
        }
        let mut module_decl = Vec::<(String, Vec::<Declaration>)>::new();
        for module in &self.modules {
            if !module.public_declarations.is_empty() {
                module_decl.push((module.name.clone(), module.public_declarations.clone()));
            }
        }
        let symbol_table = build_symbol_table(&self, import_decl, module_decl)?;
        let mut checked = self.clone().check_with_table(symbol_table, if has_parent {&self.name} else {""})?;

        let mut checked_submodules = Vec::<CheckedProgram>::new();
        if !has_parent {
            for import in &self.imports {
                checked_submodules.push(import.clone().check(true)?);
            }
        }
        for module in &self.modules {
            checked_submodules.push(module.clone().check(true)?);
        }

        for submodule in checked_submodules {
            if !submodule.data_types.is_empty() {
                checked.data_types.extend(submodule.data_types.clone());
            }
            if !submodule.codata_types.is_empty() {
                checked.codata_types.extend(submodule.codata_types.clone());
            }
            if !submodule.defs.is_empty() {
                checked.defs.extend(submodule.defs.clone());
            }
        }
        
        Ok(checked)
    }

    /// This function typechecks a module, creating a checked module with monomorphic type
    /// instances, with given symbol table.
    fn check_with_table(self, mut symbol_table: SymbolTable, prefix: &str) -> Result<CheckedProgram, Error> {
        let mut defs = Vec::new();
        // we check the well-formedness of type declarations first
        for decl in self.declarations {
            match decl {
                Declaration::Data(data) => {
                    data.check(&symbol_table)?;
                }
                Declaration::Codata(codata) => {
                    codata.check(&symbol_table)?;
                }
                Declaration::Def(def) => {
                    if !prefix.is_empty() {
                        defs.push(Def {span: def.span.clone(), name: prefix.to_owned() + "::" + def.name.as_str(), context: def.context.clone(), ret_ty: def.ret_ty.clone(), body: def.body.clone(), is_public: def.is_public});
                    }
                    else {
                        defs.push(def);
                    }
                }
            }
        }

        let defs = defs
            .into_iter()
            .map(|def| def.check(&mut symbol_table))
            .collect::<Result<_, Error>>()?;

        // collect all instances of type templates from the symbol table
        let mut data_types = Vec::new();
        let mut codata_types = Vec::new();
        for (name, (pol, type_args, xtors)) in symbol_table.types {
            match pol {
                Polarity::Data => {
                    let ctors = xtors
                        .into_iter()
                        .map(|base_name| {
                            let full_name = base_name.clone() + &type_args.print_to_string(None);
                            let args = symbol_table
                                .ctors
                                .get(&full_name)
                                .unwrap_or_else(|| {
                                    panic!("Couldn't find constructor {full_name} in symbol_table.")
                                })
                                .clone();
                            CtorSig {
                                span: None,
                                // keep base name for xtor in all instances
                                name: base_name,
                                args,
                            }
                        })
                        .collect();
                    let declaration = Data {
                        span: None,
                        name,
                        type_params: TypeContext::default(),
                        ctors,
                        is_public: false
                    };
                    data_types.push(declaration);
                }
                Polarity::Codata => {
                    let dtors = xtors
                        .into_iter()
                        .map(|base_name| {
                            let full_name = base_name.clone() + &type_args.print_to_string(None);
                            let (args, cont_ty) = symbol_table
                                .dtors
                                .get(&full_name)
                                .unwrap_or_else(|| {
                                    panic!("Couldn't find destructor {full_name} in symbol_table.")
                                })
                                .clone();
                            DtorSig {
                                span: None,
                                // keep base name for xtor in all instances
                                name: base_name,
                                args,
                                cont_ty,
                            }
                        })
                        .collect();
                    let declaration = Codata {
                        span: None,
                        name,
                        type_params: TypeContext::default(),
                        dtors,
                        is_public: false,
                    };
                    codata_types.push(declaration);
                }
            }
        }

        Ok(CheckedProgram {
            defs,
            data_types,
            codata_types,
            name: self.name,
        })
    }

    /// This function returns the names of all data type templates in a module.
    pub fn data_types(&self) -> HashSet<Name> {
        let mut names = HashSet::new();

        for declaration in &self.declarations {
            if let Declaration::Data(data) = declaration {
                names.insert(data.name.clone());
            }
        }

        names
    }

    /// This function returns the names of all codata type templates in a module.
    pub fn codata_types(&self) -> HashSet<Name> {
        let mut names = HashSet::new();

        for declaration in &self.declarations {
            if let Declaration::Codata(codata) = declaration {
                names.insert(codata.name.clone());
            }
        }
        names
    }
}

trait RenameTerms {
    fn rename(self, prefix: &str) -> Result<Term ,Error>;
}

impl RenameTerms for Term {
    fn rename(self, prefix: &str) -> Result<Term, Error> {
        match self {
            Term::Call(mut call) => {
                call.name = prefix.to_owned() + "::" + &call.name;
                Ok(Term::Call(call.clone()))
            }
            Term::Case(mut case) => {
                case.scrutinee = <terms::Term as Clone>::clone(&case.scrutinee).rename(prefix).expect("Should have been renamed").into();
                for clause in &mut case.clauses {
                    clause.body = <terms::Term as Clone>::clone(&clause.body).rename(prefix).expect("Should have been renamed");
                }
                Ok(Term::Case(case.clone()))
            }
            Term::Constructor(ref constructor) => {
                Ok(Term::Constructor(constructor.clone()))
            }
            Term::Destructor(ref destructor) => {
                Ok(Term::Destructor(destructor.clone()))
            }
            Term::Exit(mut exit) => {
                exit.arg = <terms::Term as Clone>::clone(&exit.arg).rename(prefix).expect("Should have been renamed").into();
                Ok(Term::Exit(exit.clone()))
            }
            Term::Goto(mut goto) => {
                goto.term = <terms::Term as Clone>::clone(&goto.term).rename(prefix).expect("Should have been renamed").into();
                Ok(Term::Goto(goto.clone()))
            }
            Term::IfC(mut ifc) => {
                if ifc.snd.is_some() {
                    ifc.snd = Some(<terms::Term as Clone>::clone(&ifc.snd.unwrap()).rename(prefix).expect("Should have been renamed").into());
                }
                ifc.fst = <terms::Term as Clone>::clone(&ifc.fst).rename(prefix).expect("Should have been renamed").into();
                ifc.thenc = <terms::Term as Clone>::clone(&ifc.thenc).rename(prefix).expect("Should have been renamed").into();
                ifc.elsec = <terms::Term as Clone>::clone(&ifc.elsec).rename(prefix).expect("Should have been renamed").into();
                Ok(Term::IfC(ifc.clone()))
            }
            Term::Label(mut label) => {
                label.term = <terms::Term as Clone>::clone(&label.term).rename(prefix).expect("Should have been renamed").into();
                Ok(Term::Label(label.clone()))
            }
            Term::Let(mut mlet) => {
                mlet.bound_term = <terms::Term as Clone>::clone(&mlet.bound_term).rename(prefix).expect("Should have been renamed").into();
                mlet.in_term = <terms::Term as Clone>::clone(&mlet.in_term).rename(prefix).expect("Should have been renamed").into();
                Ok(Term::Let(mlet.clone()))
            }
            Term::Lit(ref lit) => {
                Ok(Term::Lit(lit.clone()))
            }
            Term::New(mut new) => {
                for clause in &mut new.clauses {
                    clause.body = <terms::Term as Clone>::clone(&clause.body).rename(prefix).expect("Should have been renamed");
                }
                Ok(Term::New(new.clone()))
            }
            Term::Op(mut op) => {
                op.fst = <terms::Term as Clone>::clone(&op.fst).rename(prefix).expect("Should have been renamed").into();
                op.snd = <terms::Term as Clone>::clone(&op.snd).rename(prefix).expect("Should have been renamed").into();
                Ok(Term::Op(op.clone()))
            }
            Term::Paren(mut paren) => {
                paren.inner = <terms::Term as Clone>::clone(&paren.inner).rename(prefix).expect("Should have been renamed").into();
                Ok(Term::Paren(paren.clone()))
            }
            Term::PrintI64(mut print) => {
                print.arg = <terms::Term as Clone>::clone(&print.arg).rename(prefix).expect("Should have been renamed").into();
                print.next = <terms::Term as Clone>::clone(&print.next).rename(prefix).expect("Should have been renamed").into();
                Ok(Term::PrintI64(print.clone()))
            }
            Term::XVar(ref xvar) => {
                Ok(Term::XVar(xvar.clone()))
            }
        }
    }
}

impl Print for ModuleProgram {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        // We usually separate declarations with an empty line, except when the `omit_decl_sep`
        // option is set. This is useful for typesetting examples in papers which have to make
        // economic use of vertical space.
        let sep = if cfg.omit_decl_sep {
            alloc.line()
        } else {
            alloc.line().append(alloc.line())
        };

        let imports = self.imports.iter().map(|imp| imp.print(cfg, alloc));
        //let imports = self.imports.iter().map(|imp| imp.print(cfg, alloc));
        let modules = self.modules.iter().map(|modu| modu.print(cfg, alloc));
        let declarations = self.declarations.iter().map(|decl| decl.print(cfg, alloc));

        alloc.intersperse(imports, sep.clone());
        alloc.intersperse(modules, sep.clone());
        alloc.intersperse(declarations, sep)
    }
}

#[cfg(test)]
mod program_tests {
    use printer::Print;

    use crate::{
        parser::fun,
        syntax::{
            context::TypingContext,
            declarations::Def,
            program::Program,
            terms::{Lit, Term},
            types::Ty,
            util::dummy_span,
        },
    };
    use std::collections::HashSet;

    // Program with one definition without arguments
    //
    //

    fn example_simple() -> Program {
        Program {
            declarations: vec![
                Def {
                    span: dummy_span(),
                    name: "x".to_string(),
                    context: TypingContext::default(),
                    body: Term::Lit(Lit::mk(4)),
                    ret_ty: Ty::mk_i64(),
                }
                .into(),
            ],
        }
    }

    #[test]
    fn display_simple() {
        assert_eq!(
            example_simple().print_to_string(Default::default()),
            "def x(): i64 {\n    4\n}".to_string()
        )
    }

    #[test]
    fn parse_simple() {
        let parser = fun::ProgParser::new();
        assert_eq!(
            parser.parse("def x: i64 { 4 }"),
            Ok(example_simple().into())
        );
    }

    #[test]
    fn data_simple() {
        let result = example_simple().data_types();
        let expected = HashSet::new();
        assert_eq!(result, expected)
    }

    #[test]
    fn codata_simple() {
        let result = example_simple().codata_types();
        let expected = HashSet::new();
        assert_eq!(result, expected)
    }

    // Program with one definition which takes arguments
    //
    //

    fn example_args() -> Program {
        let mut ctx = TypingContext::default();
        ctx.add_var("x", Ty::mk_i64());
        ctx.add_covar("a", Ty::mk_i64());
        Program {
            declarations: vec![
                Def {
                    span: dummy_span(),
                    name: "f".to_string(),
                    context: ctx,
                    body: Term::Lit(Lit::mk(4)),
                    ret_ty: Ty::mk_i64(),
                }
                .into(),
            ],
        }
    }

    #[test]
    fn display_args() {
        assert_eq!(
            example_args().print_to_string(Default::default()),
            "def f(x: i64, a: cns i64): i64 {\n    4\n}".to_string(),
        )
    }

    #[test]
    fn parse_args() {
        let parser = fun::ProgParser::new();
        assert_eq!(
            parser.parse("def f(x: i64, a:cns i64): i64 {\n    4\n}"),
            Ok(example_args().into())
        )
    }

    // Program with two definitions
    //
    //

    fn example_two() -> Program {
        let d1 = Def {
            span: dummy_span(),
            name: "f".to_string(),
            context: TypingContext::default(),
            body: Term::Lit(Lit::mk(2)),
            ret_ty: Ty::mk_i64(),
        };

        let d2 = Def {
            span: dummy_span(),
            name: "g".to_string(),
            context: TypingContext::default(),
            body: Term::Lit(Lit::mk(4)),
            ret_ty: Ty::mk_i64(),
        };
        Program {
            declarations: vec![d1.into(), d2.into()],
        }
    }

    #[test]
    fn display_two() {
        assert_eq!(
            example_two().print_to_string(Default::default()),
            "def f(): i64 {\n    2\n}\n\ndef g(): i64 {\n    4\n}".to_string(),
        )
    }

    #[test]
    fn parse_two() {
        let parser = fun::ProgParser::new();
        assert_eq!(
            parser.parse("def f(): i64 { 2 }\n def g(): i64 { 4 }"),
            Ok(example_two().into())
        )
    }
}
