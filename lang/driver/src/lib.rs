use core::syntax::program::transform_prog;
use std::{collections::HashMap, fs, path::PathBuf};

use fun::{self, parser::parse_module, syntax::declarations::Module, typing::check::check_module};
use fun2core::program::compile_prog;
use result::DriverError;
pub mod result;

/// The driver manages the various compilation steps of a file and
/// contains the logic for computing all intermediate steps.
pub struct Driver {
    /// File sources
    sources: HashMap<PathBuf, String>,
    /// Parsed but not typechecked
    parsed: HashMap<PathBuf, Module>,
    /// Typechecked
    checked: HashMap<PathBuf, Module>,
    // Compiled to core, but not yet focused
    compiled: HashMap<PathBuf, core::syntax::Prog>,
    // Compiled to core and focused
    focused: HashMap<PathBuf, core::syntax_var::Prog>,
}

impl Driver {
    pub fn new() -> Self {
        Driver {
            sources: Default::default(),
            parsed: Default::default(),
            checked: Default::default(),
            compiled: Default::default(),
            focused: Default::default(),
        }
    }

    /// Return the unparsed source code for the given file.
    pub fn source(&mut self, path: &PathBuf) -> Result<String, DriverError> {
        // Check for a cache hit.
        if let Some(res) = self.sources.get(path) {
            return Ok(res.clone());
        }

        let content =
            fs::read_to_string(path.clone()).expect("Should have been able to read the file");
        self.sources.insert(path.clone(), content.clone());
        Ok(content)
    }

    /// Return the parsed source code for the given file.
    pub fn parsed(&mut self, path: &PathBuf) -> Result<Module, DriverError> {
        // Check for a cache hit.
        if let Some(res) = self.parsed.get(path) {
            return Ok(res.clone());
        }

        let content = self.source(path)?;
        let parsed = parse_module(&content).map_err(|err| DriverError::ParseError(err))?;
        self.parsed.insert(path.clone(), parsed.clone());
        Ok(parsed)
    }

    pub fn checked(&mut self, path: &PathBuf) -> Result<Module, DriverError> {
        // Check for cache hit.
        if let Some(res) = self.checked.get(path) {
            return Ok(res.clone());
        }

        let parsed = self.parsed(path)?;
        let checked = check_module(parsed).map_err(|err| DriverError::TypeError(err))?;
        self.checked.insert(path.clone(), checked.clone());
        Ok(checked)
    }

    pub fn compiled(&mut self, path: &PathBuf) -> Result<core::syntax::Prog, DriverError> {
        // Check for cache hit.
        if let Some(res) = self.compiled.get(path) {
            return Ok(res.clone());
        }

        let checked = self.checked(path)?;
        let compiled = compile_prog(checked);
        self.compiled.insert(path.clone(), compiled.clone());
        Ok(compiled)
    }

    pub fn focused(&mut self, path: &PathBuf) -> Result<core::syntax_var::Prog, DriverError> {
        // Check for cache hit.
        if let Some(res) = self.focused.get(path) {
            return Ok(res.clone());
        }

        let compiled = self.compiled(path)?;
        let focused = transform_prog(compiled);
        self.focused.insert(path.clone(), focused.clone());
        Ok(focused)
    }

    /// Convert a DriverError to a miette report
    pub fn error_to_report(&mut self, err: DriverError, path: &PathBuf) -> miette::Report {
        let content = self.source(path).expect("Couldn't find source file");
        let err: miette::Error = err.into();
        err.with_source_code(content)
    }
}
