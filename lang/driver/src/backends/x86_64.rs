//! Compiler logic for generating x86_64 assembly files, and subsequent compilation to object files and linking.

use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::{Path, PathBuf},
    process::Command,
};

use axcut2backend::{code::pretty, coder::compile};

use crate::{
    paths::{ASSEMBLY_PATH, BIN_PATH, INFRA_PATH, OBJECT_PATH, TARGET_PATH, X86_64_PATH},
    result::DriverError,
    Driver,
};

impl Driver {
    pub fn print_x86_64(&mut self, path: &PathBuf) -> Result<(), DriverError> {
        let linearized = self.linearized(path)?;
        let code = compile(linearized, &axcut2x86_64::Backend);
        let code_str =
            axcut2x86_64::into_routine::into_x86_64_routine(&pretty(code.0), code.1).to_string();

        let x86_64_path = Path::new(TARGET_PATH).join(ASSEMBLY_PATH).join(X86_64_PATH);
        create_dir_all(x86_64_path.clone()).expect("Could not create path");

        let mut filename = PathBuf::from(path.file_name().unwrap());
        filename.set_extension("asm");
        let filename = x86_64_path.clone().join(filename);

        let mut file = File::create(filename).expect("Could not create file");
        file.write_all(code_str.as_bytes())
            .expect("Could not write to file");

        Ok(())
    }

    pub fn compile_x86_64(&mut self, path: &PathBuf, is_debug: bool) -> Result<(), DriverError> {
        self.print_x86_64(path)?;

        let file_base_name = path.file_name().unwrap();

        let mut source_path = Path::new(TARGET_PATH)
            .join(ASSEMBLY_PATH)
            .join(X86_64_PATH)
            .join(file_base_name);
        source_path.set_extension("asm");

        let x86_64_object_path = Path::new(TARGET_PATH).join(OBJECT_PATH).join(X86_64_PATH);
        create_dir_all(x86_64_object_path.clone()).expect("Could not create path");

        let mut dist_path = x86_64_object_path.join(file_base_name);
        dist_path.set_extension("o");

        // nasm -f elf64 filename.x86_64.asm
        Command::new("nasm")
            .args(["-f", "elf64"])
            .args(["-o", dist_path.to_str().unwrap()])
            .arg(source_path)
            .status()
            .expect("Failed to execute nasm");

        let x86_64_bin_path = Path::new(TARGET_PATH).join(BIN_PATH).join(X86_64_PATH);
        create_dir_all(x86_64_bin_path.clone()).expect("Could not create path");

        let mut bin_path = x86_64_bin_path.join(file_base_name);
        bin_path.set_extension("");

        let infra_path = if is_debug {
            Path::new(INFRA_PATH)
                .join(X86_64_PATH)
                .join("driverDebug.c")
        } else {
            Path::new(INFRA_PATH).join(X86_64_PATH).join("driverArgs.c")
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
