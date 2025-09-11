//! This module contains the compiler logic for generating x86-64 assembly files and subsequent
//! compilation to object files and linking against the runtime.

use std::{fs::File, io::Write, path::PathBuf, process::Command};

use axcut2backend::coder::compile;
use printer::Print;

use crate::{
    Driver, FONTSIZE, PrintMode, generate_c_driver, generate_io_runtime,
    latex::{LATEX_END, LATEX_PRINT_CFG, latex_start},
    paths::Paths,
    result::DriverError,
};

impl Driver {
    /// This function compiles a source file to assembly code and prints it to a file either as
    /// text or as LaTeX code.
    /// - `path` is the path to the source file.
    /// - `mode` determines whether the assembly code is printed in textual mode or as LaTeX code.
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

    /// This function compiles a source file to an executable, including the generation of object
    /// file and linkage against the runtime.
    /// - `path` is the path to the source file.
    /// - `heap_size` is the heap size the program needs.
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

        let c_driver_path = generate_c_driver(number_of_arguments, heap_size);

        let io_runtime_path = generate_io_runtime();

        // gcc -o filename path/to/driver.c path/to/io.c filename.o
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
