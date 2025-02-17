use std::{
    collections::HashMap,
    fs::{self, remove_dir_all, File},
    io::{self, Write},
    path::{Path, PathBuf},
    process::Command,
};

use axcut::syntax::program::linearize;
use core2axcut::program::shrink_prog;
use fun::{
    self,
    parser::parse_module,
    syntax::declarations::{CheckedModule, Module},
};
use fun2core::program::compile_prog;
use latex::{latex_all_template, latex_start, Arch, LATEX_END, LATEX_PRINT_CFG};
use paths::{Paths, TARGET_PATH};
use printer::{Print, PrintCfg};
use result::DriverError;

pub mod backends;
pub mod latex;
pub mod paths;
pub mod result;

const FONTSIZE: &str = "scriptsize";

/// The driver manages the various compilation steps of a file and
/// contains the logic for computing all intermediate steps.
pub struct Driver {
    /// File sources
    sources: HashMap<PathBuf, String>,
    /// Parsed but not typechecked
    parsed: HashMap<PathBuf, Module>,
    /// Typechecked
    checked: HashMap<PathBuf, CheckedModule>,
    /// Compiled to core, but not yet focused
    compiled: HashMap<PathBuf, core_lang::syntax::Prog>,
    /// Compiled to core and focused
    focused: HashMap<PathBuf, core_lang::syntax::program::FsProg>,
    /// Compiled to non-linearized axcut
    shrunk: HashMap<PathBuf, axcut::syntax::Prog>,
    /// Compiled to linearized axcut
    linearized: HashMap<PathBuf, axcut::syntax::Prog>,
}

#[derive(Clone, Copy)]
pub enum PrintMode {
    Textual,
    Latex,
}

impl Driver {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Driver {
            sources: HashMap::new(),
            parsed: HashMap::new(),
            checked: HashMap::new(),
            compiled: HashMap::new(),
            focused: HashMap::new(),
            shrunk: HashMap::new(),
            linearized: HashMap::new(),
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
    pub fn checked(&mut self, path: &PathBuf) -> Result<CheckedModule, DriverError> {
        // Check for cache hit.
        if let Some(res) = self.checked.get(path) {
            return Ok(res.clone());
        }

        let parsed = self.parsed(path)?;
        let checked = parsed.check().map_err(DriverError::TypeError)?;
        self.checked.insert(path.clone(), checked.clone());
        Ok(checked)
    }

    /// Return the core code of the given file.
    pub fn compiled(&mut self, path: &PathBuf) -> Result<core_lang::syntax::Prog, DriverError> {
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
    pub fn print_compiled(&mut self, path: &PathBuf, mode: PrintMode) -> Result<(), DriverError> {
        let compiled = self.compiled(path)?;

        Paths::create_compiled_dir();

        let mut filename = PathBuf::from(path.file_name().unwrap());
        match mode {
            PrintMode::Textual => {
                filename.set_extension("txt");
            }
            PrintMode::Latex => {
                filename.set_extension("tex");
            }
        }

        let filename = Paths::compiled_dir().join(filename);
        let mut file = File::create(filename).expect("Could not create file");
        match mode {
            PrintMode::Textual => {
                compiled
                    .print_io(&PrintCfg::default(), &mut file)
                    .expect("Could not write to file.");
            }
            PrintMode::Latex => {
                file.write_all(latex_start(FONTSIZE).as_bytes()).unwrap();
                compiled
                    .print_latex(&LATEX_PRINT_CFG, &mut file)
                    .expect("Could not write to file.");
                file.write_all(LATEX_END.as_bytes()).unwrap();
            }
        }
        Ok(())
    }

    /// Return the focused version of the Core code.
    pub fn focused(
        &mut self,
        path: &PathBuf,
    ) -> Result<core_lang::syntax::program::FsProg, DriverError> {
        // Check for cache hit.
        if let Some(res) = self.focused.get(path) {
            return Ok(res.clone());
        }

        let compiled = self.compiled(path)?;
        let focused = compiled.focus();
        self.focused.insert(path.clone(), focused.clone());
        Ok(focused)
    }

    /// Print the focused code to a file in the target directory.
    pub fn print_focused(&mut self, path: &PathBuf, mode: PrintMode) -> Result<(), DriverError> {
        let focused = self.focused(path)?;

        Paths::create_focused_dir();

        let mut filename = PathBuf::from(path.file_name().unwrap());
        match mode {
            PrintMode::Textual => {
                filename.set_extension("txt");
            }
            PrintMode::Latex => {
                filename.set_extension("tex");
            }
        }
        let filename = Paths::focused_dir().join(filename);

        let mut file = File::create(filename).expect("Could not create file");
        match mode {
            PrintMode::Textual => {
                focused
                    .print_io(&PrintCfg::default(), &mut file)
                    .expect("Could not write to file.");
            }
            PrintMode::Latex => {
                file.write_all(latex_start(FONTSIZE).as_bytes()).unwrap();
                focused
                    .print_latex(&LATEX_PRINT_CFG, &mut file)
                    .expect("Could not write to file.");
                file.write_all(LATEX_END.as_bytes()).unwrap();
            }
        }
        Ok(())
    }

    /// Return the non-linearized axcut version of the file.
    pub fn shrunk(&mut self, path: &PathBuf) -> Result<axcut::syntax::Prog, DriverError> {
        // Check for cache hit.
        if let Some(res) = self.shrunk.get(path) {
            return Ok(res.clone());
        }

        let focused = self.focused(path)?;
        let shrunk = shrink_prog(focused);
        self.shrunk.insert(path.clone(), shrunk.clone());
        Ok(shrunk)
    }

    /// Print the shrunk code to a file in the target directory.
    pub fn print_shrunk(&mut self, path: &PathBuf, mode: PrintMode) -> Result<(), DriverError> {
        let shrunk = self.shrunk(path)?;

        Paths::create_shrunk_dir();

        let mut filename = PathBuf::from(path.file_name().unwrap());
        match mode {
            PrintMode::Textual => {
                filename.set_extension("txt");
            }
            PrintMode::Latex => {
                filename.set_extension("tex");
            }
        }
        let filename = Paths::shrunk_dir().join(filename);

        let mut file = File::create(filename).expect("Could not create file");
        match mode {
            PrintMode::Textual => {
                shrunk
                    .print_io(&PrintCfg::default(), &mut file)
                    .expect("Could not write to file.");
            }
            PrintMode::Latex => {
                file.write_all(latex_start(FONTSIZE).as_bytes()).unwrap();
                shrunk
                    .print_latex(&LATEX_PRINT_CFG, &mut file)
                    .expect("Could not write to file.");
                file.write_all(LATEX_END.as_bytes()).unwrap();
            }
        }
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
    pub fn print_linearized(&mut self, path: &PathBuf, mode: PrintMode) -> Result<(), DriverError> {
        let linearized = self.linearized(path)?;

        Paths::create_linearized_dir();

        let mut filename = PathBuf::from(path.file_name().unwrap());
        match mode {
            PrintMode::Textual => {
                filename.set_extension("txt");
            }
            PrintMode::Latex => {
                filename.set_extension("tex");
            }
        }
        let filename = Paths::linearized_dir().join(filename);

        let mut file = File::create(filename).expect("Could not create file");
        match mode {
            PrintMode::Textual => {
                linearized
                    .print_io(&PrintCfg::default(), &mut file)
                    .expect("Could not write to file.");
            }
            PrintMode::Latex => {
                file.write_all(latex_start(FONTSIZE).as_bytes()).unwrap();
                linearized
                    .print_latex(&LATEX_PRINT_CFG, &mut file)
                    .expect("Could not write to file.");
                file.write_all(LATEX_END.as_bytes()).unwrap();
            }
        }
        Ok(())
    }

    pub fn print_parsed_tex(
        &mut self,
        path: &PathBuf,
        cfg: &PrintCfg,
        fontsize: &str,
    ) -> Result<(), DriverError> {
        let parsed = self.parsed(path)?;

        Paths::create_pdf_dir();

        let mut filename = PathBuf::from(path.file_name().unwrap());
        filename.set_extension("tex");
        let filename = Paths::pdf_dir().join(filename);

        let mut stream: Box<dyn io::Write> =
            Box::new(fs::File::create(filename).expect("Failed to create file"));

        stream.write_all(latex_start(fontsize).as_bytes()).unwrap();

        parsed
            .print_latex(cfg, &mut stream)
            .expect("Failed to print to stdout");
        println!();

        stream.write_all(LATEX_END.as_bytes()).unwrap();
        Ok(())
    }

    pub fn print_latex_all(&mut self, path: &Path, backend: &Arch) -> Result<(), DriverError> {
        Paths::create_pdf_dir();

        let filename = path.file_stem().unwrap();
        let contents = latex_all_template(filename.to_str().unwrap(), backend);

        let filepath = append_to_path(&Paths::pdf_dir().join(filename), "All.tex");

        let mut file = fs::File::create(filepath.clone()).expect("Failed to create file");
        file.write_all(contents.as_bytes()).unwrap();

        Command::new("pdflatex")
            .current_dir(Paths::pdf_dir())
            .arg(filepath.file_name().unwrap())
            .status()
            .map_err(|_| DriverError::BinaryNotFound {
                bin_name: "pdflatex".to_string(),
            })?;

        Ok(())
    }

    pub fn open_pdf(&mut self, path: &Path) -> Result<(), DriverError> {
        let filename = path.file_stem().unwrap();
        let filepath = append_to_path(&Paths::pdf_dir().join(filename), "All.pdf");
        let _ = opener::open(filepath);
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
        remove_dir_all(TARGET_PATH).expect("Could not delete target directory");
    }
}

fn append_to_path(p: &Path, s: &str) -> PathBuf {
    let mut p_osstr = p.as_os_str().to_owned();
    p_osstr.push(s);
    p_osstr.into()
}

pub fn generate_c_driver(number_of_arguments: usize) {
    Paths::create_c_driver_gen_dir();

    let filename = Paths::c_driver_gen_dir().join(format!("driver{number_of_arguments}.c"));

    let mut file = File::create(filename).expect("Could not create file");

    let mut asm_main_prototype = "asm_main(void *heap".to_string();
    for i in 1..=number_of_arguments {
        asm_main_prototype += &format!(", int64_t input{i}");
    }
    asm_main_prototype.push(')');
    let mut asm_main_call = "asm_main(heap".to_string();
    for i in 1..=number_of_arguments {
        asm_main_call += &format!(", atoi(argv[{i}])");
    }
    asm_main_call.push(')');

    let c_driver_template = fs::read_to_string(Paths::c_driver_template())
        .expect("Should have been able to read the file");
    let c_driver = c_driver_template
        .replace("asm_main(void *heap)", &asm_main_prototype)
        .replace(
            "(argc != 1 + 0)",
            &format!("(argc != 1 + {number_of_arguments})"),
        )
        .replace("asm_main(heap)", &asm_main_call);
    file.write_all(c_driver.as_bytes())
        .expect("Could not write to file");
}
