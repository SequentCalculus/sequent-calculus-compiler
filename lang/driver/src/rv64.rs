//! Compiler logic for generating Risc-V assembly files, and subsequent compilation to object files and linking.

use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::{Path, PathBuf},
};

use axcut2backend::{code::pretty, coder::compile};

use crate::{
    paths::{ASSEMBLY_PATH, RV_64_PATH, TARGET_PATH},
    result::DriverError,
    Driver,
};

impl Driver {
    pub fn print_rv_64(&mut self, path: &PathBuf) -> Result<(), DriverError> {
        let linearized = self.linearized(path)?;
        let code = compile(linearized, &axcut2rv64::Backend);
        let code_str =
            axcut2rv64::into_routine::into_rv64_routine(&pretty(code.0), code.1).to_string();

        let rv_64_path = Path::new(TARGET_PATH).join(ASSEMBLY_PATH).join(RV_64_PATH);
        create_dir_all(rv_64_path.clone()).expect("Could not create path");

        let mut filename = PathBuf::from(path.file_name().unwrap());
        filename.set_extension("asm");
        let filename = rv_64_path.clone().join(filename);

        let mut file = File::create(filename).expect("Could not create file");
        file.write_all(code_str.as_bytes())
            .expect("Could not write to file");

        Ok(())
    }
}
