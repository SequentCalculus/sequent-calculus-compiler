//! Compiler logic for generating x86_64 assembly files, and subsequent compilation to object files and linking.

use std::{fs::File, io::Write, path::PathBuf, process::Command};

use axcut2backend::coder::compile;
use printer::Print;

use crate::{
    Driver, FONTSIZE, PrintMode, generate_c_driver, generate_io_runtime,
    latex::{LATEX_END, LATEX_PRINT_CFG, latex_start},
    paths::{IO_RUNTIME_PATH, Paths},
    result::DriverError,
};

impl Driver {
    pub fn print_x86_64(&mut self, path: &PathBuf, mode: PrintMode) -> Result<usize, DriverError> {
        let linearized = self.linearized(path)?;
        let code = compile::<axcut2x86_64::Backend, _, _, _>(linearized);
        let number_of_arguments = code.number_of_arguments;
        Paths::create_x86_64_assembly_dir();

        let mut filename = PathBuf::from(path.file_name().unwrap());
        match mode {
            PrintMode::Textual => {
                filename.set_extension("asm");
            }
            PrintMode::Latex => {
                filename.set_extension("tex");
            }
        }

        let filename = Paths::x86_64_assembly_dir().join(filename);

        let mut file = File::create(filename).expect("Could not create file");

        match mode {
            PrintMode::Textual => {
                let code_str =
                    axcut2x86_64::into_routine::into_x86_64_routine(code).print_to_string(None);
                file.write_all(code_str.as_bytes())
                    .expect("Could not write to file");
            }
            PrintMode::Latex => {
                file.write_all(latex_start(FONTSIZE).as_bytes()).unwrap();
                let code = axcut2x86_64::into_routine::into_x86_64_routine(code);
                code.print_latex(&LATEX_PRINT_CFG, &mut file)
                    .expect("Could not write to file");
                file.write_all(LATEX_END.as_bytes()).unwrap();
            }
        }
        Ok(number_of_arguments)
    }

    pub fn compile_x86_64(
        &mut self,
        path: &PathBuf,
        heap_size: Option<usize>,
    ) -> Result<(), DriverError> {
        let number_of_arguments = self.print_x86_64(path, PrintMode::Textual)?;

        let file_base_name = path.file_name().unwrap();

        let mut source_path = Paths::x86_64_assembly_dir().join(file_base_name);
        source_path.set_extension("asm");

        Paths::create_x86_64_object_dir();

        let mut dist_path = Paths::x86_64_object_dir().join(file_base_name);
        dist_path.set_extension("o");

        // yasm -f elf64 filename.asm
        Command::new("yasm")
            .args(["-f", "elf64"])
            .args(["-o", dist_path.to_str().unwrap()])
            .arg(source_path)
            .status()
            .map_err(|_| DriverError::BinaryNotFound {
                bin_name: "yasm".to_string(),
            })?;

        Paths::create_x86_64_binary_dir();

        let mut bin_path = Paths::x86_64_binary_dir().join(file_base_name);
        bin_path.set_extension("");

        generate_c_driver(number_of_arguments, heap_size);
        let filename = if let Some(heap_size) = heap_size {
            format!("driver{number_of_arguments}_{heap_size}.c")
        } else {
            format!("driver{number_of_arguments}.c")
        };
        let c_driver_path = Paths::infrastructure_dir().join(filename);

        generate_io_runtime();
        let io_runtime_path = Paths::infrastructure_dir().join(IO_RUNTIME_PATH);

        // gcc -o filename path/to/driver.c filename.o
        Command::new("gcc")
            .args(["-o", bin_path.to_str().unwrap()])
            .arg(c_driver_path.to_str().unwrap())
            .arg(io_runtime_path.to_str().unwrap())
            .arg(dist_path)
            .status()
            .map_err(|_| DriverError::BinaryNotFound {
                bin_name: "gcc".to_string(),
            })?;
        Ok(())
    }
}
