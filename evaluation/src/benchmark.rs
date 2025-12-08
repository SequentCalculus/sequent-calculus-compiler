use crate::{errors::Error, examples::Example};
use std::process::Command;

pub fn benchmark_examples(examples: &[Example], compiler_names: &[String]) -> Result<(), Error> {
    for example in examples {
        for compiler_name in compiler_names {
            let compiled_path = example.compiled_path(&compiler_name);
            println!("Benchmarking {}", compiled_path.display());
            let mut command = Command::new("hyperfine");
            command.arg(compiled_path);
            command.output().map_err(|err| {
                Error::start_cmd("hyperfine", &format!("benchmark {}", example.name), err)
            })?;
        }
    }
    Ok(())
}
