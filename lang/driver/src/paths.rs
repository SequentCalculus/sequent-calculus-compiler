//! Various file paths used by the compiler

use std::{
    fs::create_dir_all,
    path::{Path, PathBuf},
};

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

/// Path for infrastructure files
pub const INFRA_PATH: &str = "infrastructure";

/// Path for generated binaries
pub const BIN_PATH: &str = "bin";

/// Path for generated pdfs and latex
pub const PDF_PATH: &str = "pdf";

pub struct Paths {}

impl Paths {
    /// Return the path for `tex` and `pdf` files.
    /// ```rust
    /// use driver::paths::Paths;
    /// assert_eq!(Paths::pdf_path().to_str().unwrap(), "target_grk/pdf")
    /// ```
    pub fn pdf_dir() -> PathBuf {
        Path::new(TARGET_PATH).join(PDF_PATH)
    }

    /// Create the directory for `tex` and `pdf` files, if it doesn't exist yet.
    pub fn create_pdf_dir() {
        let path = Paths::pdf_dir();
        create_dir_all(path).expect("Could not create path");
    }
}
