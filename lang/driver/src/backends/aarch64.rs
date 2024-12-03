//! Compiler logic for generating aarch64 assembly files, and subsequent compilation to object files and linking.

use std::{fs::File, io::Write, path::PathBuf, process::Command};

use axcut2backend::{code::pretty, coder::compile};

use crate::{paths::Paths, result::DriverError, Driver, PrintMode};

impl Driver {
    pub fn print_aarch64(&mut self, path: &PathBuf, _mode: PrintMode) -> Result<(), DriverError> {
        let linearized = self.linearized(path)?;
        let code = compile(linearized, &axcut2aarch64::Backend);
        let code_str =
            axcut2aarch64::into_routine::into_aarch64_routine(&pretty(code.0), code.1).to_string();

        Paths::create_aarch64_assembly_dir();

        let mut filename = PathBuf::from(path.file_name().unwrap());
        filename.set_extension("asm");
        let filename = Paths::aarch64_assembly_dir().join(filename);

        let mut file = File::create(filename).expect("Could not create file");
        file.write_all(code_str.as_bytes())
            .expect("Could not write to file");

        Ok(())
    }

    pub fn compile_aarch64(&mut self, path: &PathBuf, is_debug: bool) -> Result<(), DriverError> {
        self.print_aarch64(path, PrintMode::Textual)?;

        let file_base_name = path.file_name().unwrap();

        let mut source_path = Paths::aarch64_assembly_dir().join(file_base_name);
        source_path.set_extension("asm");

        Paths::create_aarch64_object_dir();

        let mut dist_path = Paths::aarch64_object_dir().join(file_base_name);
        dist_path.set_extension("o");

        // as -o filename.aarch64.o filename.aarch64.asm
        Command::new("as")
            .args(["-o", dist_path.to_str().unwrap()])
            .arg(source_path)
            .status()
            .expect("failed to execute as");

        Paths::create_aarch64_binary_dir();

        let mut bin_path = Paths::aarch64_binary_dir().join(file_base_name);
        bin_path.set_extension("");

        let infra_path = if is_debug {
            Paths::aarch64_infra_dir().join("driverDebug.c")
        } else {
            Paths::aarch64_infra_dir().join("driverArgs.c")
        };

        // gcc -o filename path/to/AARCH64-infrastructure/driver$MODE.c filename.aarch64.o
        Command::new("gcc")
            .args(["-o", bin_path.to_str().unwrap()])
            .arg(infra_path.to_str().unwrap())
            .arg(dist_path)
            .status()
            .expect("Failed to execute gcc");

        Ok(())
    }
}
