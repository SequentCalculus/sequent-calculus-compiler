use driver::Driver;
use std::{fs, fs::File, io::prelude::Read, path::PathBuf, process::Command};

struct ExamplePaths {
    pub source_file: PathBuf,
    pub expected_file: PathBuf,
}

fn get_file_paths() -> Vec<ExamplePaths> {
    let mut paths = vec![];
    let examples_path = PathBuf::from(driver::paths::EXAMPLES_PATH);
    let expected_path = PathBuf::from(driver::paths::EXPECTED_PATH);
    let path_contents = fs::read_dir(examples_path).expect("Could not find examples");
    for path in path_contents {
        let file_path = path.expect("Could not read filename").path();
        if file_path.extension().expect("Could not get file extension") != "sc" {
            continue;
        }

        let file_name = file_path.file_name().expect("Could not get file name");
        let mut expected = expected_path.clone();
        expected.push(file_name);
        expected.set_extension("expected");

        paths.push(ExamplePaths {
            source_file: file_path,
            expected_file: expected,
        });
    }
    paths
}

fn driver_compile(drv: &mut Driver, path: &PathBuf) -> PathBuf {
    let mut out_path = if cfg!(target_arch = "aarch64") {
        drv.compile_aarch64(path, false)
            .expect("could not compile example");
        driver::paths::Paths::aarch64_binary_dir()
    // risc v does not give a runnable binary
    // not sure how we can include these tests here
    //    } else if cfg!(target_arch = "rv64") {
    //        drv.print_rv_64(path).expect("Could not compile example");
    //        driver::paths::Paths::risc_v_assembly_dir()
    } else {
        // use x86_84 as default
        drv.compile_x86_64(path, false)
            .expect("Could not compile example");
        driver::paths::Paths::x86_64_binary_dir()
    };
    let file_name = path.file_name().expect("Could not get file name");
    out_path.push(file_name);
    out_path.set_extension("");
    out_path
}

fn main() {
    let working_dir = std::env::current_dir()
        .expect("Could not get working dir")
        .join("../");
    std::env::set_current_dir(working_dir).expect("Could not set working dir");

    let paths = get_file_paths();
    let mut driver = Driver::new();

    for example in paths.iter() {
        let binary = driver_compile(&mut driver, &example.source_file);
        println!("running {binary:?}");
        let _result = Command::new(&binary)
            .spawn()
            .expect("Could not run compiled binary");
        let mut expected_file =
            File::open(&example.expected_file).expect("Could not open file for expected output");
        let mut expected = Vec::new();
        expected_file
            .read_to_end(&mut expected)
            .expect("Could not read expected output");
        //assert_eq!(result, expected)
    }
}
