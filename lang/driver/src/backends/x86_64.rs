//! Compiler logic for generating x86_64 assembly files, and subsequent compilation to object files and linking.

use std::{fs::File, io::Write, path::PathBuf, process::Command};

use axcut2backend::{code::pretty, coder::compile};

use crate::{paths::Paths, result::DriverError, Driver, PrintMode};

impl Driver {
    pub fn print_x86_64(&mut self, path: &PathBuf, _mode: PrintMode) -> Result<(), DriverError> {
        let linearized = self.linearized(path)?;
        let code = compile(linearized, &axcut2x86_64::Backend);
        let code_str = axcut2x86_64::into_routine::into_x86_64_routine(
            &pretty(code.instructions),
            code.number_of_arguments,
        )
        .to_string();

        Paths::create_x86_64_assembly_dir();

        let mut filename = PathBuf::from(path.file_name().unwrap());
        filename.set_extension("asm");
        let filename = Paths::x86_64_assembly_dir().join(filename);

        let mut file = File::create(filename).expect("Could not create file");
        file.write_all(code_str.as_bytes())
            .expect("Could not write to file");

        Ok(())
    }

    pub fn compile_x86_64(&mut self, path: &PathBuf, is_debug: bool) -> Result<(), DriverError> {
        self.print_x86_64(path, PrintMode::Textual)?;

        let file_base_name = path.file_name().unwrap();

        let mut source_path = Paths::x86_64_assembly_dir().join(file_base_name);
        source_path.set_extension("asm");

        Paths::create_x86_64_object_dir();

        let mut dist_path = Paths::x86_64_object_dir().join(file_base_name);
        dist_path.set_extension("o");

        // nasm -f elf64 filename.x86_64.asm
        Command::new("nasm")
            .args(["-f", "elf64"])
            .args(["-o", dist_path.to_str().unwrap()])
            .arg(source_path)
            .status()
            .expect("Failed to execute nasm");

        Paths::create_x86_64_binary_dir();

        let mut bin_path = Paths::x86_64_binary_dir().join(file_base_name);
        bin_path.set_extension("");

        let infra_path = if is_debug {
            Paths::x86_64_infra_dir().join("driverDebug.c")
        } else {
            Paths::x86_64_infra_dir().join("driverArgs.c")
        };

        // gcc -o filename path/to/X86_64-infrastructure/driver$MODE.c filename.x86_64.o
        // where $MODE = Args | Debug
        Command::new("gcc")
            .args(["-o", bin_path.to_str().unwrap()])
            .arg(infra_path.to_str().unwrap())
            .arg(dist_path)
            .status()
            .expect("Failed to execute gcc");
        Ok(())
    }
}
