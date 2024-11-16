//! Various file paths used by the compiler

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

/// Path for AArch_64 assembly files
pub const AARCH64_PATH: &str = "aarch_64";

/// Path for x86_64 assembly files
pub const X86_64_PATH: &str = "x86_64";

/// Path for 64-Bit Risc-V assembly files
pub const RV_64_PATH: &str = "rv_64";
