use super::{check_type, context::check_typing_context, terms::Check};
use crate::{
    syntax::declarations::{
        CodataDeclaration, CtorSig, DataDeclaration, Declaration, Definition, DtorSig, Module,
    },
    typing::{
        errors::Error,
        symbol_table::{build_symbol_table, SymbolTable},
    },
};

pub fn check_module(module: Module) -> Result<Module, Error> {
    let symbol_table = build_symbol_table(&module)?;
    check_module_with_table(module, &symbol_table)
}

fn check_module_with_table(module: Module, symbol_table: &SymbolTable) -> Result<Module, Error> {
    let mut new_decls = vec![];
    for decl in module.declarations.into_iter() {
        let new_decl = check_declaration(decl, symbol_table)?;
        new_decls.push(new_decl);
    }
    Ok(Module {
        declarations: new_decls,
    })
}

// Checking toplevel declarations
//
//

fn check_declaration(decl: Declaration, symbol_table: &SymbolTable) -> Result<Declaration, Error> {
    match decl {
        Declaration::Definition(definition) => {
            Ok(check_definition(definition, symbol_table)?.into())
        }
        Declaration::DataDeclaration(data_declaration) => {
            check_data_declaration(&data_declaration, symbol_table)?;
            Ok(data_declaration.into())
        }
        Declaration::CodataDeclaration(codata_declaration) => {
            check_codata_declaration(&codata_declaration, symbol_table)?;
            Ok(codata_declaration.into())
        }
    }
}

fn check_definition(def: Definition, symbol_table: &SymbolTable) -> Result<Definition, Error> {
    check_typing_context(&def.context, symbol_table)?;
    check_type(&def.ret_ty, symbol_table)?;
    let body_checked = def.body.check(symbol_table, &def.context, &def.ret_ty)?;
    Ok(Definition {
        body: body_checked,
        span: def.span,
        name: def.name,
        context: def.context,
        ret_ty: def.ret_ty,
    })
}

fn check_data_declaration(decl: &DataDeclaration, symbol_table: &SymbolTable) -> Result<(), Error> {
    for ctor in decl.ctors.iter() {
        check_ctor_sig(&ctor, symbol_table)?;
    }
    Ok(())
}

fn check_codata_declaration(
    decl: &CodataDeclaration,
    symbol_table: &SymbolTable,
) -> Result<(), Error> {
    for dtor in decl.dtors.iter() {
        check_dtor_sig(dtor, symbol_table)?;
    }
    Ok(())
}

fn check_ctor_sig(ctor: &CtorSig, symbol_table: &SymbolTable) -> Result<(), Error> {
    check_typing_context(&ctor.args, symbol_table)?;
    Ok(())
}

fn check_dtor_sig(dtor: &DtorSig, symbol_table: &SymbolTable) -> Result<(), Error> {
    check_typing_context(&dtor.args, symbol_table)?;
    check_type(&dtor.cont_ty, symbol_table)?;
    Ok(())
}
