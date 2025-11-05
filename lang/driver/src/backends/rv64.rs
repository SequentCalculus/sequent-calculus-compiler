//! This module contains the compiler logic for generating RISC-V assembly files. There is
//! currently no compilation to object files and linking.

use std::{fs::File, io::Write, path::PathBuf};

use axcut2backend::coder::compile;

use crate::{Driver, PrintMode, paths::Paths, result::DriverError};

impl Driver {
    /// This function compiles a source file to assembly code and prints it to a file either as
    /// text or as LaTeX code.
    /// - `path` is the path to the source file.
    /// - `mode` determines whether the assembly code is printed in textual mode or as LaTeX code.
    pub fn print_rv_64(&mut self, path: &PathBuf, _mode: PrintMode) -> Result<(), DriverError> {
        let inlined = self.inlined(path)?;
        let code = compile::<axcut2rv64::Backend, _, _, _>(inlined);
        let code_str = axcut2rv64::into_routine::into_rv64_routine(code).to_string();

        Paths::create_risc_v_assembly_dir();

        let mut filename = PathBuf::from(path.file_name().unwrap());
        filename.set_extension("asm");
        let filename = Paths::risc_v_assembly_dir().join(filename);

        let mut file = File::create(filename).expect("Could not create file");
        file.write_all(code_str.as_bytes())
            .expect("Could not write to file");

        Ok(())
    }
}
