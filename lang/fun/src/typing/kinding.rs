use super::errors::Error;
use crate::syntax::{declarations::Prog, kinds::Kind, types::Ty};

pub fn kind_type(ty: &Ty, prog: &Prog) -> Result<Kind, Error> {
    match ty {
        Ty::Int() => Ok(Kind::Prim),
        Ty::Decl(name) => {
            let is_data = prog.data_types().contains(name);
            let is_codata = prog.codata_types().contains(name);
            if is_data && !is_codata {
                Ok(Kind::Data)
            } else if is_codata && !is_data {
                Ok(Kind::Codata)
            } else if is_data && is_codata {
                Err(Error::DefinedMultipleTimes(name.clone()))
            } else {
                Err(Error::Undefined(name.clone()))
            }
        }
    }
}

#[cfg(test)]
mod kinding_tests {
    use super::kind_type;
    use crate::syntax::{
        declarations::Prog,
        kinds::Kind,
        typedef::{CodataDefinition, DataDefinition},
        types::Ty,
    };

    fn example_prog() -> Prog {
        Prog {
            prog_defs: vec![
                DataDefinition {
                    name: "List".to_owned(),
                    ctors: vec![],
                }
                .into(),
                CodataDefinition {
                    name: "Stream".to_owned(),
                    dtors: vec![],
                }
                .into(),
            ],
        }
    }

    #[test]
    fn kind_int() {
        let result = kind_type(&Ty::Int(), &example_prog()).unwrap();
        let expected = Kind::Prim;
        assert_eq!(result, expected)
    }

    #[test]
    fn kind_list() {
        let result = kind_type(&Ty::Decl("List".to_owned()), &example_prog()).unwrap();
        let expected = Kind::Data;
        assert_eq!(result, expected)
    }

    #[test]
    fn kind_list_mutliple() {
        let mut prog = example_prog();
        prog.prog_defs.push(
            CodataDefinition {
                name: "List".to_owned(),
                dtors: vec![],
            }
            .into(),
        );
        let result = kind_type(&Ty::Decl("List".to_owned()), &prog);
        assert!(result.is_err())
    }

    #[test]
    fn kind_list_undefined() {
        let result = kind_type(&Ty::Decl("List".to_owned()), &Prog { prog_defs: vec![] });
        assert!(result.is_err())
    }

    #[test]
    fn kind_stream() {
        let result = kind_type(&Ty::Decl("Stream".to_owned()), &example_prog()).unwrap();
        let expected = Kind::Codata;
        assert_eq!(result, expected)
    }
}
