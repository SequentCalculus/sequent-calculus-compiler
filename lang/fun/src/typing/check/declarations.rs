use super::{check_type, context::check_typing_context, terms::Check};
use crate::{
    syntax::declarations::{
        CodataDeclaration, CtorSig, DataDeclaration, Declaration, Definition, DtorSig,
    },
    typing::{errors::Error, symbol_table::SymbolTable},
};

// Checking toplevel declarations
//
//

pub fn check_declaration(decl: &Declaration, symbol_table: &SymbolTable) -> Result<(), Error> {
    match decl {
        Declaration::Definition(definition) => check_definition(definition, symbol_table),
        Declaration::DataDeclaration(data_declaration) => {
            check_data_declaration(data_declaration, symbol_table)
        }
        Declaration::CodataDeclaration(codata_declaration) => {
            check_codata_declaration(codata_declaration, symbol_table)
        }
    }
}

fn check_definition(def: &Definition, symbol_table: &SymbolTable) -> Result<(), Error> {
    check_typing_context(&def.context, symbol_table)?;
    check_type(&def.ret_ty, symbol_table)?;
    def.body.check(symbol_table, &def.context, &def.ret_ty)
}

fn check_data_declaration(decl: &DataDeclaration, symbol_table: &SymbolTable) -> Result<(), Error> {
    for ctor in decl.ctors.iter() {
        check_ctor_sig(ctor, symbol_table)?;
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
