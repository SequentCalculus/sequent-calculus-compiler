use core::syntax::program::transform_prog;
use std::{
    collections::HashMap,
    fs::{self, create_dir_all, remove_dir_all, File},
    io::Write,
    path::{Path, PathBuf},
    process::Command,
};

use axcut::syntax::program::linearize;
use axcut2backend::{code::pretty, coder::compile};
use core2axcut::program::translate_prog;
use fun::{self, parser::parse_module, syntax::declarations::Module, typing::check::check_module};
use fun2core::program::compile_prog;
use paths::{
    AARCH64_PATH, ASSEMBLY_PATH, BIN_PATH, COMPILED_PATH, FOCUSED_PATH, INFRA_PATH,
    LINEARIZED_PATH, OBJECT_PATH, RV_64_PATH, SHRUNK_PATH, TARGET_PATH, X86_64_PATH,
};
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

    pub fn print_aarch64(&mut self, path: &PathBuf) -> Result<(), DriverError> {
        let linearized = self.linearized(path)?;
        let code = compile(linearized, &axcut2aarch64::Backend);
        let code_str =
            axcut2aarch64::into_routine::into_aarch64_routine("filename", &pretty(code.0), code.1)
                .to_string();

        let aarch_path = Path::new(TARGET_PATH)
            .join(ASSEMBLY_PATH)
            .join(AARCH64_PATH);
        create_dir_all(aarch_path.clone()).expect("Could not create path");

        let mut filename = PathBuf::from(path.file_name().unwrap());
        filename.set_extension("asm");
        let filename = aarch_path.clone().join(filename);

        let mut file = File::create(filename).expect("Could not create file");
        file.write_all(code_str.as_bytes())
            .expect("Could not write to file");

        Ok(())
    }

    pub fn compile_aarch64(&mut self, path: &PathBuf) -> Result<(), DriverError> {
        self.print_aarch64(path)?;

        let file_base_name = path.file_name().unwrap();

        let mut source_path = Path::new(TARGET_PATH)
            .join(ASSEMBLY_PATH)
            .join(AARCH64_PATH)
            .join(file_base_name);
        source_path.set_extension("asm");

        let aarch64_object_path = Path::new(TARGET_PATH).join(OBJECT_PATH).join(AARCH64_PATH);
        create_dir_all(aarch64_object_path.clone()).expect("Could not create path");

        let mut dist_path = aarch64_object_path.join(file_base_name);
        dist_path.set_extension("o");

        // as -o filename.aarch64.o filename.aarch64.asm
        Command::new("as")
            .args(["-o", dist_path.to_str().unwrap()])
            .arg(source_path)
            .spawn()
            .expect("failed to execute process");

        // gcc -o filename path/to/AARCH64-infrastructure/driver$MODE.c filename.aarch64.o

        Ok(())
    }

    pub fn print_x86_64(&mut self, path: &PathBuf) -> Result<(), DriverError> {
        let linearized = self.linearized(path)?;
        let code = compile(linearized, &axcut2x86_64::Backend);
        let code_str =
            axcut2x86_64::into_routine::into_x86_64_routine("filename", &pretty(code.0), code.1)
                .to_string();

        let x86_64_path = Path::new(TARGET_PATH).join(ASSEMBLY_PATH).join(X86_64_PATH);
        create_dir_all(x86_64_path.clone()).expect("Could not create path");

        let mut filename = PathBuf::from(path.file_name().unwrap());
        filename.set_extension("asm");
        let filename = x86_64_path.clone().join(filename);

        let mut file = File::create(filename).expect("Could not create file");
        file.write_all(code_str.as_bytes())
            .expect("Could not write to file");

        Ok(())
    }

    pub fn compile_x86_64(&mut self, path: &PathBuf) -> Result<(), DriverError> {
        self.print_x86_64(path)?;

        let file_base_name = path.file_name().unwrap();

        let mut source_path = Path::new(TARGET_PATH)
            .join(ASSEMBLY_PATH)
            .join(X86_64_PATH)
            .join(file_base_name);
        source_path.set_extension("asm");

        let x86_64_object_path = Path::new(TARGET_PATH).join(OBJECT_PATH).join(X86_64_PATH);
        create_dir_all(x86_64_object_path.clone()).expect("Could not create path");

        let mut dist_path = x86_64_object_path.join(file_base_name);
        dist_path.set_extension("o");

        // nasm -f elf64 filename.x86_64.asm
        Command::new("nasm")
            .args(["-f", "elf64"])
            .args(["-o", dist_path.to_str().unwrap()])
            .arg(source_path)
            .spawn()
            .expect("Failed to execute nasm");

        let x86_64_bin_path = Path::new(TARGET_PATH).join(BIN_PATH).join(X86_64_PATH);
        create_dir_all(x86_64_bin_path.clone()).expect("Could not create path");

        let mut bin_path = x86_64_bin_path.join(file_base_name);
        bin_path.set_extension("");

        let infra_path = Path::new(INFRA_PATH).join(X86_64_PATH).join("driverArgs.c");

        // gcc -o filename path/to/X86_64-infrastructure/driver$MODE.c filename.x86_64.o
        // where $MODE = Args | Debug
        Command::new("gcc")
            .args(["-o", bin_path.to_str().unwrap()])
            .arg(infra_path.to_str().unwrap())
            .arg(dist_path)
            .spawn()
            .expect("Failed to execute gcc");
        Ok(())
    }

    pub fn print_rv_64(&mut self, path: &PathBuf) -> Result<(), DriverError> {
        let linearized = self.linearized(path)?;
        let code = compile(linearized, &axcut2rv64::Backend);
        let code_str =
            axcut2rv64::into_routine::into_rv64_routine("filename", &pretty(code.0), code.1)
                .to_string();

        let rv_64_path = Path::new(TARGET_PATH).join(ASSEMBLY_PATH).join(RV_64_PATH);
        create_dir_all(rv_64_path.clone()).expect("Could not create path");

        let mut filename = PathBuf::from(path.file_name().unwrap());
        filename.set_extension("asm");
        let filename = rv_64_path.clone().join(filename);

        let mut file = File::create(filename).expect("Could not create file");
        file.write_all(code_str.as_bytes())
            .expect("Could not write to file");

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
