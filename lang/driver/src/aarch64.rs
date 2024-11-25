//! Compiler logic for generating aarch64 assembly files, and subsequent compilation to object files and linking.

use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::{Path, PathBuf},
    process::Command,
};

use axcut2backend::{code::pretty, coder::compile};

use crate::{
    paths::{AARCH64_PATH, ASSEMBLY_PATH, BIN_PATH, INFRA_PATH, OBJECT_PATH, TARGET_PATH},
    result::DriverError,
    Driver,
};

impl Driver {
    pub fn print_aarch64(&mut self, path: &PathBuf) -> Result<(), DriverError> {
        let linearized = self.linearized(path)?;
        let code = compile(linearized, &axcut2aarch64::Backend);
        let code_str =
            axcut2aarch64::into_routine::into_aarch64_routine(&pretty(code.0), code.1).to_string();

        let aarch_path = Path::new(TARGET_PATH)
            .join(ASSEMBLY_PATH)
            .join(AARCH64_PATH);
        create_dir_all(aarch_path.clone()).expect("Could not create path");

        let mut filename = PathBuf::from(path.file_name().unwrap());
        filename.set_extension("asm");
        let filename = aarch_path.clone().join(filename);

        let mut file = File::create(filename).expect("Could not create file");
        file.write_all(code_str.as_bytes())
            .expect("Could not write to file");

        Ok(())
    }

    pub fn compile_aarch64(&mut self, path: &PathBuf, is_debug: bool) -> Result<(), DriverError> {
        self.print_aarch64(path)?;

        let file_base_name = path.file_name().unwrap();

        let mut source_path = Path::new(TARGET_PATH)
            .join(ASSEMBLY_PATH)
            .join(AARCH64_PATH)
            .join(file_base_name);
        source_path.set_extension("asm");

        let aarch64_object_path = Path::new(TARGET_PATH).join(OBJECT_PATH).join(AARCH64_PATH);
        create_dir_all(aarch64_object_path.clone()).expect("Could not create path");

        let mut dist_path = aarch64_object_path.join(file_base_name);
        dist_path.set_extension("o");

        // as -o filename.aarch64.o filename.aarch64.asm
        Command::new("as")
            .args(["-o", dist_path.to_str().unwrap()])
            .arg(source_path)
            .status()
            .expect("failed to execute as");

        let aarch64_bin_path = Path::new(TARGET_PATH).join(BIN_PATH).join(AARCH64_PATH);
        create_dir_all(aarch64_bin_path.clone()).expect("Could not create path");

        let mut bin_path = aarch64_bin_path.join(file_base_name);
        bin_path.set_extension("");

        let infra_path = if is_debug {
            Path::new(INFRA_PATH)
                .join(AARCH64_PATH)
                .join("driverDebug.c")
        } else {
            Path::new(INFRA_PATH)
                .join(AARCH64_PATH)
                .join("driverArgs.c")
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
