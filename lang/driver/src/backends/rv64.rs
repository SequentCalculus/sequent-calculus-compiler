//! Compiler logic for generating Risc-V assembly files, and subsequent compilation to object files and linking.

use std::{fs::File, io::Write, path::PathBuf};

use axcut2backend::coder::compile;

use crate::{paths::Paths, result::DriverError, Driver, PrintMode};

impl Driver {
    pub fn print_rv_64(&mut self, path: &PathBuf, _mode: PrintMode) -> Result<(), DriverError> {
        let linearized = self.linearized(path)?;
        let code = compile::<axcut2rv64::Backend, _, _, _>(linearized);
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
