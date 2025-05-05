//! Various file paths used by the compiler

use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::{Path, PathBuf},
};

pub const C_DRIVER_TEMPLATE: &str = include_str!("../../../infrastructure/driver-template.c");
pub const C_DRIVER_PATH: &str = "driver-template.c";

pub const RUNTIME_IO: &[u8] = include_bytes!("../../../infrastructure/io.c");
pub const RUNTIME_IO_PATH: &str = "io.c";

/// Base path for benchmarks
pub const BENCHMARKS_PATH: &str = "benchmarks/suite";

/// Path for benchmarking results
pub const BENCHMARKS_RESULTS: &str = "benchmarks/results";

/// Path for benchmarking reports
pub const BENCHMARKS_REPORTS: &str = "benchmarks/reports";

/// Base path for examples
pub const EXAMPLES_PATH: &str = "examples";

/// Base path for all build artefacts
pub const TARGET_PATH: &str = "target_grk";

/// Path for compiled files
pub const COMPILED_PATH: &str = "compiled";

/// Path for focused files
pub const FOCUSED_PATH: &str = "focused";

/// Path for non-linearized axcut files
pub const SHRUNK_PATH: &str = "shrunk";

/// Path for linearized axcut files
pub const LINEARIZED_PATH: &str = "linearized";

/// Path for assembly files
pub const ASSEMBLY_PATH: &str = "assembly";

/// Path for assembled object files
pub const OBJECT_PATH: &str = "object";

/// Path for AArch_64 assembly files
pub const AARCH64_PATH: &str = "aarch_64";

/// Path for x86_64 assembly files
pub const X86_64_PATH: &str = "x86_64";

/// Path for 64-Bit Risc-V assembly files
pub const RV_64_PATH: &str = "rv_64";

/// Path for generated binaries
pub const BIN_PATH: &str = "bin";

/// Path for generated pdfs and latex
pub const PDF_PATH: &str = "pdf";

/// Path for generated c files
pub const INFRA_PATH: &str = "infrastructure";

pub struct Paths {}

impl Paths {
    /// Return the directory for `tex` and `pdf` files.
    /// ```rust
    /// use driver::paths::Paths;
    /// assert_eq!(Paths::pdf_dir().to_str().unwrap(), "target_grk/pdf")
    /// ```
    pub fn pdf_dir() -> PathBuf {
        Path::new(TARGET_PATH).join(PDF_PATH)
    }

    /// Create the directory for `tex` and `pdf` files, if it doesn't exist yet.
    pub fn create_pdf_dir() {
        create_dir_all(Paths::pdf_dir()).expect("Could not create path");
    }

    /// Return the directory for files after compilation to sequent calculus.
    /// ```rust
    /// use driver::paths::Paths;
    /// assert_eq!(Paths::compiled_dir().to_str().unwrap(), "target_grk/compiled")
    /// ```
    pub fn compiled_dir() -> PathBuf {
        Path::new(TARGET_PATH).join(COMPILED_PATH)
    }

    /// Create the directory for files after compilation to sequent calculus, if it doesn't exist yet.
    pub fn create_compiled_dir() {
        create_dir_all(Paths::compiled_dir()).expect("Could not create path");
    }

    /// Return the directory for files after focusing.
    /// ```rust
    /// use driver::paths::Paths;
    /// assert_eq!(Paths::focused_dir().to_str().unwrap(), "target_grk/focused")
    /// ```
    pub fn focused_dir() -> PathBuf {
        Path::new(TARGET_PATH).join(FOCUSED_PATH)
    }

    /// Create the directory for files after focusing, if it doesn't exist yet.
    pub fn create_focused_dir() {
        create_dir_all(Paths::focused_dir()).expect("Could not create path");
    }

    /// Return the directory for files after shrinking.
    /// ```rust
    /// use driver::paths::Paths;
    /// assert_eq!(Paths::shrunk_dir().to_str().unwrap(), "target_grk/shrunk")
    /// ```
    pub fn shrunk_dir() -> PathBuf {
        Path::new(TARGET_PATH).join(SHRUNK_PATH)
    }

    /// Create the directory for files after shrinking, if it doesn't exist yet.
    pub fn create_shrunk_dir() {
        create_dir_all(Paths::shrunk_dir()).expect("Could not create path");
    }

    /// Return the directory for files after linearization.
    /// ```rust
    /// use driver::paths::Paths;
    /// assert_eq!(Paths::linearized_dir().to_str().unwrap(), "target_grk/linearized")
    /// ```
    pub fn linearized_dir() -> PathBuf {
        Path::new(TARGET_PATH).join(LINEARIZED_PATH)
    }

    /// Create the directory for files after linearization, if it doesn't exist yet.
    pub fn create_linearized_dir() {
        create_dir_all(Paths::linearized_dir()).expect("Could not create path");
    }

    /// Creates runtime io from included bytes
    fn create_runtime_io() -> PathBuf {
        let io_path = Path::new(TARGET_PATH)
            .join(INFRA_PATH)
            .join(RUNTIME_IO_PATH);
        if !io_path.exists() {
            let mut io_file = File::create(&io_path).expect("Could not create runtime io");
            io_file
                .write_all(RUNTIME_IO)
                .expect("Could not write runtime io");
        }
        io_path
    }

    /// Return the path of the file containing IO runtime functions.
    pub fn runtime_io() -> PathBuf {
        Self::create_runtime_io()
    }

    /// Return the directory for the generated C driver.
    /// ```rust
    /// use driver::paths::Paths;
    /// assert_eq!(Paths::c_driver_gen_dir().to_str().unwrap(), "target_grk/infrastructure")
    /// ```
    pub fn c_driver_gen_dir() -> PathBuf {
        Path::new(TARGET_PATH).join(INFRA_PATH)
    }

    /// Create the directory for the generated C driver, if it doesn't exist yet.
    pub fn create_c_driver_gen_dir() {
        create_dir_all(Paths::c_driver_gen_dir()).expect("Could not create path");
    }

    // Risc-V
    //
    //

    /// Return the directory for Risc-V assembly files.
    /// ```rust
    /// use driver::paths::Paths;
    /// assert_eq!(Paths::risc_v_assembly_dir().to_str().unwrap(), "target_grk/assembly/rv_64")
    /// ```
    pub fn risc_v_assembly_dir() -> PathBuf {
        Path::new(TARGET_PATH).join(ASSEMBLY_PATH).join(RV_64_PATH)
    }

    /// Create the directory for Risc-V assembly, if it doesn't exist yet.
    pub fn create_risc_v_assembly_dir() {
        create_dir_all(Paths::risc_v_assembly_dir()).expect("Could not create path");
    }

    // X86-64
    //
    //

    /// Return the directory for x86-64 assembly files.
    /// ```rust
    /// use driver::paths::Paths;
    /// assert_eq!(Paths::x86_64_assembly_dir().to_str().unwrap(), "target_grk/assembly/x86_64")
    /// ```
    pub fn x86_64_assembly_dir() -> PathBuf {
        Path::new(TARGET_PATH).join(ASSEMBLY_PATH).join(X86_64_PATH)
    }

    /// Create the directory for x86_64 assembly, if it doesn't exist yet.
    pub fn create_x86_64_assembly_dir() {
        create_dir_all(Paths::x86_64_assembly_dir()).expect("Could not create path");
    }

    /// Return the directory for x86-64 object files.
    /// ```rust
    /// use driver::paths::Paths;
    /// assert_eq!(Paths::x86_64_object_dir().to_str().unwrap(), "target_grk/object/x86_64")
    /// ```
    pub fn x86_64_object_dir() -> PathBuf {
        Path::new(TARGET_PATH).join(OBJECT_PATH).join(X86_64_PATH)
    }

    /// Create the directory for x86_64 object files, if it doesn't exist yet.
    pub fn create_x86_64_object_dir() {
        create_dir_all(Paths::x86_64_object_dir()).expect("Could not create path");
    }

    /// Return the directory for x86-64 binaries.
    /// ```rust
    /// use driver::paths::Paths;
    /// assert_eq!(Paths::x86_64_binary_dir().to_str().unwrap(), "target_grk/bin/x86_64")
    /// ```
    pub fn x86_64_binary_dir() -> PathBuf {
        Path::new(TARGET_PATH).join(BIN_PATH).join(X86_64_PATH)
    }

    /// Create the directory for x86_64 binaries, if it doesn't exist yet.
    pub fn create_x86_64_binary_dir() {
        create_dir_all(Paths::x86_64_binary_dir()).expect("Could not create path");
    }

    // aarch64
    //
    //

    /// Return the directory for aarch64 assembly files.
    /// ```rust
    /// use driver::paths::Paths;
    /// assert_eq!(Paths::aarch64_assembly_dir().to_str().unwrap(), "target_grk/assembly/aarch_64")
    /// ```
    pub fn aarch64_assembly_dir() -> PathBuf {
        Path::new(TARGET_PATH)
            .join(ASSEMBLY_PATH)
            .join(AARCH64_PATH)
    }

    /// Create the directory for aarch64 assembly, if it doesn't exist yet.
    pub fn create_aarch64_assembly_dir() {
        create_dir_all(Paths::aarch64_assembly_dir()).expect("Could not create path");
    }

    /// Return the directory for aarch64 object files.
    /// ```rust
    /// use driver::paths::Paths;
    /// assert_eq!(Paths::aarch64_object_dir().to_str().unwrap(), "target_grk/object/aarch_64")
    /// ```
    pub fn aarch64_object_dir() -> PathBuf {
        Path::new(TARGET_PATH).join(OBJECT_PATH).join(AARCH64_PATH)
    }

    /// Create the directory for aarch64 object files, if it doesn't exist yet.
    pub fn create_aarch64_object_dir() {
        create_dir_all(Paths::aarch64_object_dir()).expect("Could not create path");
    }

    /// Return the directory for aarch64 binaries.
    /// ```rust
    /// use driver::paths::Paths;
    /// assert_eq!(Paths::aarch64_binary_dir().to_str().unwrap(), "target_grk/bin/aarch_64")
    /// ```
    pub fn aarch64_binary_dir() -> PathBuf {
        Path::new(TARGET_PATH).join(BIN_PATH).join(AARCH64_PATH)
    }

    /// Create the directory for aarch64 binaries, if it doesn't exist yet.
    pub fn create_aarch64_binary_dir() {
        create_dir_all(Paths::aarch64_binary_dir()).expect("Could not create path");
    }
}
