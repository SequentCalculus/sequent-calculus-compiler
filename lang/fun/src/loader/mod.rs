//! This module conatains the loader for programs in the sufrace language Fun.
use std::{
    collections::HashMap,
    env::{consts::OS, home_dir},
    ffi::OsString,
    path::PathBuf,
};

use crate::syntax::{
    module_declarations::*,
    {
        Declaration, Name,
        program::{ModuleProgram, Program},
    },
};

use result::LoaderError;

use regex::Regex;

pub mod result;

pub trait DriverTrait {
    fn parsed(&mut self, path: &PathBuf) -> Result<Program, LoaderError>;
    fn loaded(
        &mut self,
        path: &PathBuf,
        parent_decl: Option<(String, Vec<Declaration>)>,
        visited: &mut HashMap<OsString, Option<ModuleProgram>>,
    ) -> Result<ModuleProgram, LoaderError>;

    fn new() -> Self
    where
        Self: Sized;
}

/// This functions loads a module the specified modules ans submodules of the given file
pub fn load_module<'a>(
    parsed: &Program,
    path: &PathBuf,
    parent_decl: Option<(String, Vec<Declaration>)>,
    visited: &mut HashMap<OsString, Option<ModuleProgram>>,
    create_driver: impl Fn() -> Box<dyn DriverTrait>,
) -> Result<ModuleProgram, LoaderError> {
    visited.insert(
        path.canonicalize()
            .expect("Could not get absoule path")
            .to_path_buf()
            .file_name()
            .expect("Should have filename")
            .to_owned(),
        None,
    );
    let mut imports = HashMap::<Name, ModuleProgram>::new();
    let mut imports_from_children = HashMap::<Name, ModuleProgram>::new();
    let mut modules = Vec::<ModuleProgram>::new();
    let name = path
        .file_stem()
        .map(|os_str| {
            os_str
                .to_str()
                .expect("Modulename conatins invalid Unicode")
        })
        .expect("No filename given")
        .to_string();

    for decl in &parsed.module_declarations {
        match decl {
            ModuleDeclaration::Import(import) => {
                let mut search_path = path
                    .canonicalize()
                    .expect("Could not get absoule path")
                    .clone();
                search_path.pop();
                let path_to_import = find_given_file(&import.name, &mut search_path, true)?;
                let mut subdrv: Box<dyn DriverTrait> = create_driver();
                if !visited.contains_key(
                    &path_to_import
                        .file_name()
                        .expect("Should have filename")
                        .to_owned(),
                ) {
                    let loaded = subdrv.loaded(&path_to_import, None, visited)?;
                    visited.insert(
                        path_to_import
                            .file_name()
                            .expect("Should have filename")
                            .to_owned(),
                        Some(loaded.clone()),
                    );
                    if !loaded.imports_to_parent.is_empty() {
                        imports_from_children.extend(loaded.imports_to_parent.clone());
                    }
                    imports.insert(
                        get_actual_name(import.name.clone()).unwrap(),
                        loaded.clone(),
                    );
                    imports_from_children
                        .insert(get_actual_name(import.name.clone()).unwrap(), loaded);
                } else {
                    let subparsed = subdrv.parsed(&path_to_import)?;
                    let mut sub_public_decl = Vec::<Declaration>::new();
                    for decl in &subparsed.declarations {
                        match decl {
                            Declaration::Codata(codata) => {
                                if codata.is_public {
                                    sub_public_decl.push(decl.clone())
                                }
                            }
                            Declaration::Data(data) => {
                                if data.is_public {
                                    sub_public_decl.push(decl.clone())
                                }
                            }
                            Declaration::Def(def) => {
                                if def.is_public {
                                    sub_public_decl.push(decl.clone())
                                }
                            }
                        }
                    }
                    imports.insert(
                        get_actual_name(import.name.clone()).unwrap()
                            + "------TEST----------------",
                        ModuleProgram {
                            imports: HashMap::<Name, ModuleProgram>::new(),
                            modules: Vec::<ModuleProgram>::new(),
                            declarations: subparsed.declarations.clone(),
                            name: path_to_import
                                .file_stem()
                                .map(|os_str| {
                                    os_str
                                        .to_str()
                                        .expect("Modulename conatins invalid Unicode")
                                })
                                .expect("No filename given")
                                .to_string(),
                            imports_to_parent: HashMap::<Name, ModuleProgram>::new(),
                            parent_declarations: None,
                            public_declarations: sub_public_decl,
                        },
                    );
                }
            }
            ModuleDeclaration::Module(module) => {
                let mut search_path = path.clone();
                search_path.pop();
                let mut subdrv: Box<dyn DriverTrait> = create_driver();
                modules.push(subdrv.loaded(
                    &find_given_file(&module.name, &mut search_path, false)?,
                    Some((name.clone(), parsed.declarations.clone())),
                    visited,
                )?);
            }
        }
    }
    let mut public_declarations = Vec::<Declaration>::new();
    for decl in &parsed.declarations {
        match decl {
            Declaration::Codata(codata) => {
                if codata.is_public {
                    public_declarations.push(decl.clone());
                }
            }
            Declaration::Data(data) => {
                if data.is_public {
                    public_declarations.push(decl.clone());
                }
            }
            Declaration::Def(def) => {
                if def.is_public {
                    public_declarations.push(decl.clone());
                }
            }
        }
    }
    let module_program = ModuleProgram {
        modules: modules,
        declarations: parsed.declarations.clone(),
        name: name,
        parent_declarations: parent_decl.clone(),
        imports: imports.clone(),
        imports_to_parent: if !parent_decl.is_some() {
            imports_from_children
        } else {
            imports.clone()
        },
        public_declarations: public_declarations,
    };
    visited.insert(
        path.canonicalize()
            .expect("Could not get absoule path")
            .to_path_buf()
            .file_name()
            .expect("Should have filename")
            .to_owned(),
        Some(module_program.clone()),
    );
    Ok(module_program)
}

/// This function searches for a given file at the specified locations
fn find_given_file<'a>(
    module_call: &'a str,
    path: &'a mut PathBuf,
    is_import: bool,
) -> Result<PathBuf, LoaderError> {
    let reg = Regex::new(r"^[A-z][a-zA-Z0-9_]*(::[A-z][a-zA-Z0-9_]*)+$").unwrap();
    let filename;
    if reg.is_match(module_call) {
        let mut split: Vec<&str> = module_call.split("::").collect();
        let root = split[0];
        let mut abs_path = path.canonicalize().expect("Could not get absoule path");
        while abs_path.file_name().unwrap().to_str().unwrap() != root {
            abs_path.pop();
        }
        split.remove(0);
        let file_name = split.pop().unwrap();
        for dir in split {
            abs_path.push(dir);
            if !abs_path.exists() {
                return Err(LoaderError::FileNotFound {
                    path_to_file: abs_path.to_str().unwrap().to_owned(),
                });
            }
        }
        abs_path.push(file_name);
        filename = file_name;
        *path = abs_path;
        path.set_extension("sc");
        if path.is_file() {
            return Ok(path.to_path_buf());
        }
        path.set_extension("");
    } else {
        filename = module_call;
    }
    path.push(filename);
    path.set_extension("sc");
    //rintln!("{:?}", path);
    if path.is_file() {
        Ok(path.to_path_buf())
    } else {
        path.set_extension("");
        if path.is_dir() {
            path.push(filename);
            path.set_extension("sc");
            if path.is_file() {
                Ok(path.to_path_buf())
            } else {
                Err(LoaderError::FileNotFound {
                    path_to_file: path.to_str().unwrap().to_owned(),
                })
            }
        } else if is_import {
            let mut std_module_path = {
                if OS == "linux" {
                    let mut dir = home_dir().unwrap();
                    dir.push(".local");
                    dir.push("share");
                    dir.push("scc");
                    dir
                } else if OS == "windows" {
                    let mut dir = home_dir().unwrap();
                    dir.push("AppData");
                    dir.push("Local");
                    dir.push("scc");
                    dir
                } else if OS == "macos" {
                    let mut dir = home_dir().unwrap();
                    dir.push("Library");
                    dir.push("Application Support");
                    dir.push("scc");
                    dir
                } else {
                    PathBuf::new()
                }
            };
            if std_module_path.exists() {
                std_module_path.push(filename);
                std_module_path.set_extension("sc");
                if std_module_path.is_file() {
                    Ok(std_module_path)
                } else {
                    std_module_path.set_extension("");
                    if std_module_path.is_dir() {
                        std_module_path.push(filename);
                        std_module_path.set_extension("sc");
                        if std_module_path.is_file() {
                            Ok(std_module_path)
                        } else {
                            Err(LoaderError::FileNotFound {
                                path_to_file: std_module_path.to_str().unwrap().to_owned(),
                            })
                        }
                    } else {
                        Err(LoaderError::FileNotFound {
                            path_to_file: std_module_path.to_str().unwrap().to_owned(),
                        })
                    }
                }
            } else {
                Err(LoaderError::FileNotFound {
                    path_to_file: path.to_str().unwrap().to_owned(),
                })
            }
        } else {
            Err(LoaderError::FileNotFound {
                path_to_file: path.to_str().unwrap().to_owned(),
            })
        }
    }
}

fn get_actual_name(input: String) -> Result<String, String> {
    input
        .rsplit("::")
        .next()
        .map(|s| s.to_string())
        .ok_or_else(|| input)
}
