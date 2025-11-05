//! This crate contains the driver that manages the various compilation steps of a file and
//! that contains the logic for computing all intermediate representations. The representations can
//! be printed in textual mode or as LaTeX code.

use std::{
    collections::HashMap,
    fmt::Write as _,
    fs::{self, File, remove_dir_all},
    io::{self, Write},
    path::{Path, PathBuf},
    process::Command,
};

use core2axcut::program::shrink_prog;
use fun::{
    self,
    parser::parse_module,
    syntax::program::{CheckedProgram, Program},
};
use fun2core::program::compile_prog;
use latex::{Arch, LATEX_END, LATEX_PRINT_CFG, latex_all_template, latex_start};
use optimizations::inline_prog;
use paths::{Paths, TARGET_PATH};
use printer::{Print, PrintCfg};
use result::DriverError;

pub mod backends;
pub mod latex;
pub mod paths;
pub mod result;

/// This constant defines the font size for LaTeX code.
const FONTSIZE: &str = "scriptsize";

/// This struct defines the driver that manages the various compilation steps of a file and that
/// contains the logic for computing all intermediate representations. It is able to cache
/// intermediate results for better performance.
pub struct Driver {
    /// File sources
    sources: HashMap<PathBuf, String>,
    /// Parsed but not typechecked
    parsed: HashMap<PathBuf, Program>,
    /// Typechecked
    checked: HashMap<PathBuf, CheckedProgram>,
    /// Compiled to core, but not yet focused
    compiled: HashMap<PathBuf, core_lang::syntax::Prog>,
    /// Compiled to core and focused
    focused: HashMap<PathBuf, core_lang::syntax::program::FsProg>,
    /// Compiled to non-linearized axcut
    shrunk: HashMap<PathBuf, axcut::syntax::Prog>,
    /// Compiled to linearized axcut
    linearized: HashMap<PathBuf, axcut::syntax::Prog>,
    /// With inlined definitions
    inlined: HashMap<PathBuf, axcut::syntax::Prog>,
}

/// This enum encodes whether the representations are printed in textual mode or as LaTeX code.
#[derive(Clone, Copy)]
pub enum PrintMode {
    Textual,
    Latex,
}

impl Driver {
    /// This function creates a new driver.
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
            inlined: HashMap::new(),
        }
    }

    /// This function returns the unparsed source code for the given file.
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

    /// This function returns the parsed source code for the given file.
    pub fn parsed(&mut self, path: &PathBuf) -> Result<Program, DriverError> {
        // Check for a cache hit.
        if let Some(res) = self.parsed.get(path) {
            return Ok(res.clone());
        }

        let content = self.source(path)?;
        let parsed = parse_module(&content).map_err(DriverError::ParseError)?;
        self.parsed.insert(path.clone(), parsed.clone());
        Ok(parsed)
    }

    /// This function returns the typechecked source code of the given file.
    pub fn checked(&mut self, path: &PathBuf) -> Result<CheckedProgram, DriverError> {
        // Check for cache hit.
        if let Some(res) = self.checked.get(path) {
            return Ok(res.clone());
        }

        let parsed = self.parsed(path)?;
        let checked = parsed.check().map_err(DriverError::TypeError)?;
        self.checked.insert(path.clone(), checked.clone());
        Ok(checked)
    }

    /// This function returns the [Core](core_lang) code of the given file.
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

    /// This function prints the compiled code to a file in the target directory.
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
                    .expect("Could not write to file");
            }
            PrintMode::Latex => {
                file.write_all(latex_start(FONTSIZE).as_bytes()).unwrap();
                compiled
                    .print_latex(&LATEX_PRINT_CFG, &mut file)
                    .expect("Could not write to file");
                file.write_all(LATEX_END.as_bytes()).unwrap();
            }
        }
        Ok(())
    }

    /// This function returns the focused version of the [Core](core_lang) code.
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

    /// This function prints the focused code to a file in the target directory.
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
                    .expect("Could not write to file");
            }
            PrintMode::Latex => {
                file.write_all(latex_start(FONTSIZE).as_bytes()).unwrap();
                focused
                    .print_latex(&LATEX_PRINT_CFG, &mut file)
                    .expect("Could not write to file");
                file.write_all(LATEX_END.as_bytes()).unwrap();
            }
        }
        Ok(())
    }

    /// This function returns the non-linearized [AxCut](axcut) version of the file.
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

    /// This function prints the non-linearized [AxCut](axcut) code to a file in the target directory.
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
                    .expect("Could not write to file");
            }
            PrintMode::Latex => {
                file.write_all(latex_start(FONTSIZE).as_bytes()).unwrap();
                shrunk
                    .print_latex(&LATEX_PRINT_CFG, &mut file)
                    .expect("Could not write to file");
                file.write_all(LATEX_END.as_bytes()).unwrap();
            }
        }
        Ok(())
    }

    /// This function returns the linearized [AxCut](axcut) version of the file.
    pub fn linearized(&mut self, path: &PathBuf) -> Result<axcut::syntax::Prog, DriverError> {
        // Check for cache hit.
        if let Some(res) = self.linearized.get(path) {
            return Ok(res.clone());
        }

        let shrunk = self.shrunk(path)?;
        let linearized = shrunk.linearize();
        self.linearized.insert(path.clone(), linearized.clone());
        Ok(linearized)
    }

    /// This function prints the linearized [AxCut](axcut) code to a file in the target directory.
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
                    .expect("Could not write to file");
            }
            PrintMode::Latex => {
                file.write_all(latex_start(FONTSIZE).as_bytes()).unwrap();
                linearized
                    .print_latex(&LATEX_PRINT_CFG, &mut file)
                    .expect("Could not write to file");
                file.write_all(LATEX_END.as_bytes()).unwrap();
            }
        }
        Ok(())
    }

    /// This function returns the inlined [AxCut](axcut) version of the file.
    pub fn inlined(&mut self, path: &PathBuf) -> Result<axcut::syntax::Prog, DriverError> {
        // Check for cache hit.
        if let Some(res) = self.inlined.get(path) {
            return Ok(res.clone());
        }

        let linearized = self.linearized(path)?;
        let inlined = inline_prog(linearized)?;
        self.inlined.insert(path.clone(), inlined.clone());
        Ok(inlined)
    }

    /// This function prints the inlined [AxCut](axcut) code to a file in the target directory.
    pub fn print_inlined(&mut self, path: &PathBuf, mode: PrintMode) -> Result<(), DriverError> {
        let inlined = self.inlined(path)?;

        Paths::create_inlined_dir();

        let mut filename = PathBuf::from(path.file_name().unwrap());
        match mode {
            PrintMode::Textual => {
                filename.set_extension("txt");
            }
            PrintMode::Latex => {
                filename.set_extension("tex");
            }
        }
        let filename = Paths::inlined_dir().join(filename);

        let mut file = File::create(filename).expect("Could not create file");
        match mode {
            PrintMode::Textual => {
                inlined
                    .print_io(&PrintCfg::default(), &mut file)
                    .expect("Could not write to file");
            }
            PrintMode::Latex => {
                file.write_all(latex_start(FONTSIZE).as_bytes()).unwrap();
                inlined
                    .print_latex(&LATEX_PRINT_CFG, &mut file)
                    .expect("Could not write to file");
                file.write_all(LATEX_END.as_bytes()).unwrap();
            }
        }
        Ok(())
    }

    /// This function prints the parsed source code as LaTeX code.
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

    /// This function prints representations as LaTeX code.
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

    /// This function opens the PDF containing the LaTeX code.
    pub fn open_pdf(&mut self, path: &Path) -> Result<(), DriverError> {
        let filename = path.file_stem().unwrap();
        let filepath = append_to_path(&Paths::pdf_dir().join(filename), "All.pdf");
        let _ = opener::open(filepath);
        Ok(())
    }

    /// This function converts a [`DriverError`] to a [`miette`] report.
    pub fn error_to_report(&mut self, err: DriverError, path: &PathBuf) -> miette::Report {
        let content = self.source(path).expect("Couldn't find source file");
        let err: miette::Error = err.into();
        err.with_source_code(content)
    }

    /// This function deletes all files in the target directory.
    pub fn clean() {
        remove_dir_all(TARGET_PATH).expect("Could not delete target directory");
    }
}

/// This function appends a string to a path.
fn append_to_path(p: &Path, s: &str) -> PathBuf {
    let mut p_osstr = p.as_os_str().to_owned();
    p_osstr.push(s);
    p_osstr.into()
}

/// This constant contains the template of the C driver.
pub const C_DRIVER_TEMPLATE: &str = include_str!("../../../infrastructure/driver-template.c");

/// This function generates the correct version of the C driver from the
/// [template](C_DRIVER_TEMPLATE). It is parametrized by the number of arguments of the program and
/// the heap size the program needs.
pub fn generate_c_driver(number_of_arguments: usize, heap_size: Option<usize>) -> PathBuf {
    let mut asm_main_prototype = "asm_main(void *heap".to_string();
    for i in 1..=number_of_arguments {
        write!(&mut asm_main_prototype, ", int64_t input{i}")
            .expect("Could not append to String in generation of C driver");
    }
    asm_main_prototype.push(')');

    let mut asm_main_call = "asm_main(heap".to_string();
    for i in 1..=number_of_arguments {
        write!(&mut asm_main_call, ", atoi(argv[{i}])")
            .expect("Could not append to String in generation of C driver");
    }
    asm_main_call.push(')');

    let c_driver = C_DRIVER_TEMPLATE
        .replace("asm_main(void *heap)", &asm_main_prototype)
        .replace(
            "(argc != 1 + 0)",
            &format!("(argc != 1 + {number_of_arguments})"),
        )
        .replace("asm_main(heap)", &asm_main_call);
    let c_driver = if let Some(heap_size) = heap_size {
        c_driver.replace(
            "heapsize = UINT64_C(1024 * 1024) * 32",
            &format!("heapsize = UINT64_C(1024 * 1024) * {heap_size}"),
        )
    } else {
        c_driver
    };

    Paths::create_infrastructure_dir();
    let filename = if let Some(heap_size) = heap_size {
        format!("driver{number_of_arguments}_{heap_size}.c")
    } else {
        format!("driver{number_of_arguments}.c")
    };
    let filepath = Paths::infrastructure_dir().join(filename);
    if !filepath.exists() {
        let mut file = File::create(&filepath).expect("Could not create C-driver file");
        file.write_all(c_driver.as_bytes())
            .expect("Could not write to C-driver file");
    }

    filepath
}

/// This constant contains the code of the IO runtime.
pub const IO_RUNTIME: &[u8] = include_bytes!("../../../infrastructure/io.c");

/// This function generates the puts the IO runtime into the correct place.
pub fn generate_io_runtime() -> PathBuf {
    Paths::create_infrastructure_dir();
    let filepath = Paths::infrastructure_dir().join("io.c");
    if !filepath.exists() {
        let mut file = File::create(&filepath).expect("Could not create IO runtime file");
        file.write_all(IO_RUNTIME)
            .expect("Could not write IO runtime file");
    }

    filepath
}
