use core::syntax::program::transform_prog;
use std::{
    collections::HashMap,
    fs::{self, create_dir_all, remove_dir_all, File},
    path::{Path, PathBuf},
};

use axcut::syntax::program::linearize;
use core2axcut::program::translate_prog;
use fun::{self, parser::parse_module, syntax::declarations::Module, typing::check::check_module};
use fun2core::program::compile_prog;
use paths::{COMPILED_PATH, FOCUSED_PATH, LINEARIZED_PATH, SHRUNK_PATH, TARGET_PATH};
use printer::Print;
use result::DriverError;
pub mod paths;
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
    /// Compiled to core, but not yet focused
    compiled: HashMap<PathBuf, core::syntax::Prog>,
    /// Compiled to core and focused
    focused: HashMap<PathBuf, core::syntax_var::Prog>,
    /// Compiled to non-linearized axcut
    shrunk: HashMap<PathBuf, axcut::syntax::Prog>,
    /// Compiled to linearized axcut
    linearized: HashMap<PathBuf, axcut::syntax::Prog>,
}

impl Driver {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Driver {
            sources: Default::default(),
            parsed: Default::default(),
            checked: Default::default(),
            compiled: Default::default(),
            focused: Default::default(),
            shrunk: Default::default(),
            linearized: Default::default(),
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
        let parsed = parse_module(&content).map_err(DriverError::ParseError)?;
        self.parsed.insert(path.clone(), parsed.clone());
        Ok(parsed)
    }

    /// Return the typechecked source code of the given file.
    pub fn checked(&mut self, path: &PathBuf) -> Result<Module, DriverError> {
        // Check for cache hit.
        if let Some(res) = self.checked.get(path) {
            return Ok(res.clone());
        }

        let parsed = self.parsed(path)?;
        let checked = check_module(parsed).map_err(DriverError::TypeError)?;
        self.checked.insert(path.clone(), checked.clone());
        Ok(checked)
    }

    /// Return the core code of the given file.
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

    /// Print the compiled code to a file in the target directory.
    pub fn print_compiled(&mut self, path: &PathBuf) -> Result<(), DriverError> {
        let compiled = self.compiled(path)?;

        let compiled_path = Path::new(TARGET_PATH).join(COMPILED_PATH);
        create_dir_all(compiled_path.clone()).expect("Could not create path");

        let mut filename = PathBuf::from(path.file_name().unwrap());
        filename.set_extension("txt");
        let filename = compiled_path.clone().join(filename);

        let mut file = File::create(filename).expect("Could not create file");
        compiled
            .print_io(&Default::default(), &mut file)
            .expect("Could not write to file.");
        Ok(())
    }

    /// Return the focused version of the Core code.
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

    /// Print the focused code to a file in the target directory.
    pub fn print_focused(&mut self, path: &PathBuf) -> Result<(), DriverError> {
        let focused = self.focused(path)?;

        let focused_path = Path::new(TARGET_PATH).join(FOCUSED_PATH);
        create_dir_all(focused_path.clone()).expect("Could not create path");

        let mut filename = PathBuf::from(path.file_name().unwrap());
        filename.set_extension("txt");
        let filename = focused_path.clone().join(filename);

        let mut file = File::create(filename).expect("Could not create file");
        focused
            .print_io(&Default::default(), &mut file)
            .expect("Could not write to file.");
        Ok(())
    }

    /// Return the non-linearized axcut version of the file.
    pub fn shrunk(&mut self, path: &PathBuf) -> Result<axcut::syntax::Prog, DriverError> {
        // Check for cache hit.
        if let Some(res) = self.shrunk.get(path) {
            return Ok(res.clone());
        }

        let focused = self.focused(path)?;
        let shrunk = translate_prog(focused);
        self.shrunk.insert(path.clone(), shrunk.clone());
        Ok(shrunk)
    }

    /// Print the shrunk code to a file in the target directory.
    pub fn print_shrunk(&mut self, path: &PathBuf) -> Result<(), DriverError> {
        let shrunk = self.shrunk(path)?;

        let shrunk_path = Path::new(TARGET_PATH).join(SHRUNK_PATH);
        create_dir_all(shrunk_path.clone()).expect("Could not create path");

        let mut filename = PathBuf::from(path.file_name().unwrap());
        filename.set_extension("txt");
        let filename = shrunk_path.clone().join(filename);

        let mut file = File::create(filename).expect("Could not create file");
        shrunk
            .print_io(&Default::default(), &mut file)
            .expect("Could not write to file.");
        Ok(())
    }

    /// Return the linearized axcut version of the file.
    pub fn linearized(&mut self, path: &PathBuf) -> Result<axcut::syntax::Prog, DriverError> {
        // Check for cache hit.
        if let Some(res) = self.linearized.get(path) {
            return Ok(res.clone());
        }

        let shrunk = self.shrunk(path)?;
        let linearized = linearize(shrunk);
        self.linearized.insert(path.clone(), linearized.clone());
        Ok(linearized)
    }

    /// Print the linearized code to a file in the target directory.
    pub fn print_linearized(&mut self, path: &PathBuf) -> Result<(), DriverError> {
        let linearized = self.linearized(path)?;

        let linearized_path = Path::new(TARGET_PATH).join(LINEARIZED_PATH);
        create_dir_all(linearized_path.clone()).expect("Could not create path");

        let mut filename = PathBuf::from(path.file_name().unwrap());
        filename.set_extension("txt");
        let filename = linearized_path.clone().join(filename);

        let mut file = File::create(filename).expect("Could not create file");
        linearized
            .print_io(&Default::default(), &mut file)
            .expect("Could not write to file.");
        Ok(())
    }

    /// Convert a DriverError to a miette report
    pub fn error_to_report(&mut self, err: DriverError, path: &PathBuf) -> miette::Report {
        let content = self.source(path).expect("Couldn't find source file");
        let err: miette::Error = err.into();
        err.with_source_code(content)
    }

    /// Delete all files in the target directory.
    pub fn clean() {
        remove_dir_all(TARGET_PATH).expect("Could not delete target directory")
    }
}
