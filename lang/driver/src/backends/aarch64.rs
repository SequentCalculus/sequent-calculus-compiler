//! Compiler logic for generating aarch64 assembly files, and subsequent compilation to object files and linking.

use std::{fs::File, io::Write, path::PathBuf, process::Command};

use axcut2backend::coder::compile;
use printer::Print;

use crate::{
    generate_c_driver,
    latex::{latex_start, LATEX_END, LATEX_PRINT_CFG},
    paths::Paths,
    result::DriverError,
    Driver, PrintMode, FONTSIZE,
};

impl Driver {
    pub fn print_aarch64(&mut self, path: &PathBuf, mode: PrintMode) -> Result<usize, DriverError> {
        let linearized = self.linearized(path)?;
        let code = compile::<axcut2aarch64::Backend, _, _, _>(linearized);
        let number_of_arguments = code.number_of_arguments;

        Paths::create_aarch64_assembly_dir();

        let mut filename = PathBuf::from(path.file_name().unwrap());
        match mode {
            PrintMode::Textual => {
                filename.set_extension("asm");
            }
            PrintMode::Latex => {
                filename.set_extension("tex");
            }
        }

        let filename = Paths::aarch64_assembly_dir().join(filename);

        let mut file = File::create(filename).expect("Could not create file");

        match mode {
            PrintMode::Textual => {
                let code_str =
                    axcut2aarch64::into_routine::into_aarch64_routine(code).print_to_string(None);
                file.write_all(code_str.as_bytes())
                    .expect("Could not write to file");
            }
            PrintMode::Latex => {
                file.write_all(latex_start(&FONTSIZE.to_string()).as_bytes())
                    .unwrap();
                let code = axcut2aarch64::into_routine::into_aarch64_routine(code);
                code.print_latex(&LATEX_PRINT_CFG, &mut file)
                    .expect("Could not write to file.");
                file.write_all(LATEX_END.as_bytes()).unwrap();
            }
        }

        Ok(number_of_arguments)
    }

    pub fn compile_aarch64(&mut self, path: &PathBuf) -> Result<(), DriverError> {
        let number_of_arguments = self.print_aarch64(path, PrintMode::Textual)?;

        let file_base_name = path.file_name().unwrap();

        let mut source_path = Paths::aarch64_assembly_dir().join(file_base_name);
        source_path.set_extension("asm");

        Paths::create_aarch64_object_dir();

        let mut dist_path = Paths::aarch64_object_dir().join(file_base_name);
        dist_path.set_extension("o");

        // as -o filename.o filename.asm
        Command::new("as")
            .args(["-o", dist_path.to_str().unwrap()])
            .arg(source_path)
            .status()
            .map_err(|_| DriverError::BinaryNotFound {
                bin_name: "as".to_string(),
            })?;

        Paths::create_aarch64_binary_dir();

        let mut bin_path = Paths::aarch64_binary_dir().join(file_base_name);
        bin_path.set_extension("");

        generate_c_driver(number_of_arguments);
        let infra_path = Paths::infra_gen_dir().join(format!("driver{number_of_arguments}.c"));

        // gcc -o filename path/to/driver.c filename.o
        Command::new("gcc")
            .args(["-o", bin_path.to_str().unwrap()])
            .arg(infra_path.to_str().unwrap())
            .arg(dist_path)
            .status()
            .map_err(|_| DriverError::BinaryNotFound {
                bin_name: "gcc".to_string(),
            })?;

        Ok(())
    }
}
