use crate::{EvalResult, errors::Error, examples::Example};
use std::process::Command;

pub fn benchmark_examples(
    examples: &[Example],
    compiler_names: &[String],
    results: &mut Vec<EvalResult>,
) -> Result<(), Error> {
    for example in examples {
        for compiler_name in compiler_names {
            let compiled_path = example.compiled_path(&compiler_name);
            println!("Benchmarking {}", compiled_path.display());
            let mut command = Command::new("hyperfine");
            let args = example.get_args();
            let mut run_str = format!("\"{}", compiled_path.display());
            for arg in args {
                run_str += &arg;
                run_str += " ";
            }
            run_str += "\"";
            command.arg(run_str);
            command.arg("-u");
            command.arg("microsecond");

            let hyperfine_res = command.output().map_err(|err| {
                Error::start_cmd("hyperfine", &format!("benchmark {}", example.name), err)
            })?;

            let stdout_str = String::from_utf8(hyperfine_res.stdout)
                .map_err(|err| Error::parse_out("hyperfine", err))?;
            if !hyperfine_res.status.success() {
                let stderr_str = String::from_utf8(hyperfine_res.stderr)
                    .map_err(|err| Error::parse_out("hyperfine", err))?;
                return Err(Error::run_cmd(
                    "hyperfine",
                    hyperfine_res.status,
                    &stdout_str,
                    &stderr_str,
                ));
            }

            for line in stdout_str.lines() {
                if line.contains("Time") {
                    let mut line_parts = line.split(":");
                    line_parts.next();
                    let time_str = line_parts.next().expect("Could not get hyperfine time");
                    let mut time_parts = time_str.split(" ");
                    let time = time_parts
                        .next()
                        .expect("Could not get hyperfine time")
                        .trim()
                        .parse::<f64>()
                        .expect("Could not get hyperfine time");
                    let example_results = results
                        .iter_mut()
                        .find(|res| res.example == example.name)
                        .expect("Could not find example results");
                    example_results
                        .benchmark_times
                        .insert(compiler_name.clone(), time);
                }
            }
        }
    }
    Ok(())
}
