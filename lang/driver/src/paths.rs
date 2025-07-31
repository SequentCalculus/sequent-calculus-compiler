//! This moduel defines various file paths used by the compiler.

use std::{
    fs::create_dir_all,
    path::{Path, PathBuf},
};

/// Base path for benchmarks
pub const BENCHMARKS_PATH: &str = "benchmarks/suite";

/// Path for benchmarking results
pub const BENCHMARKS_RESULTS: &str = "benchmarks/results";

/// Path for benchmarking reports
pub const BENCHMARKS_REPORTS: &str = "benchmarks/reports";

/// Base path for examples
pub const EXAMPLES_PATH: &str = "examples";

/// Base path for all build artefacts
pub const TARGET_PATH: &str = "target_scc";

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

/// Path for generated C files
pub const INFRA_PATH: &str = "infrastructure";

/// Path for generated binaries
pub const BIN_PATH: &str = "bin";

/// Path for generated pdfs and latex
pub const PDF_PATH: &str = "pdf";

pub struct Paths {}

impl Paths {
    /// Return the directory for `tex` and `pdf` files.
    /// ```rust
    /// use driver::paths::Paths;
    /// assert_eq!(Paths::pdf_dir().to_str().unwrap(), "target_scc/pdf")
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
    /// assert_eq!(Paths::compiled_dir().to_str().unwrap(), "target_scc/compiled")
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
    /// assert_eq!(Paths::focused_dir().to_str().unwrap(), "target_scc/focused")
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
    /// assert_eq!(Paths::shrunk_dir().to_str().unwrap(), "target_scc/shrunk")
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
    /// assert_eq!(Paths::linearized_dir().to_str().unwrap(), "target_scc/linearized")
    /// ```
    pub fn linearized_dir() -> PathBuf {
        Path::new(TARGET_PATH).join(LINEARIZED_PATH)
    }

    /// Create the directory for files after linearization, if it doesn't exist yet.
    pub fn create_linearized_dir() {
        create_dir_all(Paths::linearized_dir()).expect("Could not create path");
    }

    /// Return the directory for the infrastructure files.
    /// ```rust
    /// use driver::paths::Paths;
    /// assert_eq!(Paths::infrastructure_dir().to_str().unwrap(), "target_scc/infrastructure")
    /// ```
    pub fn infrastructure_dir() -> PathBuf {
        Path::new(TARGET_PATH).join(INFRA_PATH)
    }

    /// Create the directory for the infrastructure, if it doesn't exist yet.
    pub fn create_infrastructure_dir() {
        create_dir_all(Paths::infrastructure_dir()).expect("Could not create path");
    }

    //
    // RISC-V
    //

    /// Return the directory for Risc-V assembly files.
    /// ```rust
    /// use driver::paths::Paths;
    /// assert_eq!(Paths::risc_v_assembly_dir().to_str().unwrap(), "target_scc/assembly/rv_64")
    /// ```
    pub fn risc_v_assembly_dir() -> PathBuf {
        Path::new(TARGET_PATH).join(ASSEMBLY_PATH).join(RV_64_PATH)
    }

    /// Create the directory for Risc-V assembly, if it doesn't exist yet.
    pub fn create_risc_v_assembly_dir() {
        create_dir_all(Paths::risc_v_assembly_dir()).expect("Could not create path");
    }

    //
    // x86-64
    //

    /// Return the directory for x86-64 assembly files.
    /// ```rust
    /// use driver::paths::Paths;
    /// assert_eq!(Paths::x86_64_assembly_dir().to_str().unwrap(), "target_scc/assembly/x86_64")
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
    /// assert_eq!(Paths::x86_64_object_dir().to_str().unwrap(), "target_scc/object/x86_64")
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
    /// assert_eq!(Paths::x86_64_binary_dir().to_str().unwrap(), "target_scc/bin/x86_64")
    /// ```
    pub fn x86_64_binary_dir() -> PathBuf {
        Path::new(TARGET_PATH).join(BIN_PATH).join(X86_64_PATH)
    }

    /// Create the directory for x86_64 binaries, if it doesn't exist yet.
    pub fn create_x86_64_binary_dir() {
        create_dir_all(Paths::x86_64_binary_dir()).expect("Could not create path");
    }

    //
    // Aarch64
    //

    /// Return the directory for aarch64 assembly files.
    /// ```rust
    /// use driver::paths::Paths;
    /// assert_eq!(Paths::aarch64_assembly_dir().to_str().unwrap(), "target_scc/assembly/aarch_64")
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
    /// assert_eq!(Paths::aarch64_object_dir().to_str().unwrap(), "target_scc/object/aarch_64")
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
    /// assert_eq!(Paths::aarch64_binary_dir().to_str().unwrap(), "target_scc/bin/aarch_64")
    /// ```
    pub fn aarch64_binary_dir() -> PathBuf {
        Path::new(TARGET_PATH).join(BIN_PATH).join(AARCH64_PATH)
    }

    /// Create the directory for aarch64 binaries, if it doesn't exist yet.
    pub fn create_aarch64_binary_dir() {
        create_dir_all(Paths::aarch64_binary_dir()).expect("Could not create path");
    }
}
