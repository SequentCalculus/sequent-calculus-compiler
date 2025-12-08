use crate::{RESULTS_OUT, errors::Error};
use std::{collections::HashMap, fs::File, io::Write, path::PathBuf};

#[derive(Debug)]
pub struct EvalResult {
    pub example: String,
    pub num_passes: u64,
    pub lifted_create: u64,
    pub lifted_switch: u64,
    pub benchmark_times: HashMap<String, f64>,
}

impl EvalResult {
    fn print_csv(&self) -> String {
        format!(
            "{},{},{},{},{}",
            self.example,
            self.num_passes,
            self.lifted_create,
            self.lifted_switch,
            self.benchmark_times
                .iter()
                .map(|(_, time)| time.to_string())
                .collect::<Vec<_>>()
                .join(",")
        )
    }
}

fn results_to_csv(results: Vec<EvalResult>, compiler_names: &[String]) -> String {
    format!(
        "example,num_passes,num_create,num_switch,{}\n{}",
        compiler_names
            .iter()
            .map(|name| format!("time_{name}"))
            .collect::<Vec<_>>()
            .join(","),
        results
            .iter()
            .map(|res| res.print_csv())
            .collect::<Vec<_>>()
            .join("\n")
    )
}

pub fn write_csv(results: Vec<EvalResult>, compiler_names: &[String]) -> Result<(), Error> {
    let out_str = results_to_csv(results, compiler_names);
    let out_path = PathBuf::from(RESULTS_OUT);
    let mut out_file = File::create(&out_path).map_err(|err| Error::create_file(&out_path, err))?;
    out_file
        .write_all(out_str.as_bytes())
        .map_err(|err| Error::write_file(&out_path, err))?;
    Ok(())
}
