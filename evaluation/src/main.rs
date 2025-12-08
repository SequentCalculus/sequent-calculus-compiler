use std::{fs::read_to_string, path::PathBuf};

mod errors;
use errors::Error;

const EVAL_CONFIG: &str = "evaluation/config.toml";

#[derive(Clone, serde::Deserialize)]
struct EvalConfig {
    version_git_hashes: Vec<String>,
}

fn main() -> Result<(), Error> {
    println!("{}", std::env::current_dir().unwrap().display());
    let config_path = PathBuf::from(EVAL_CONFIG);
    let config_contents =
        read_to_string(&config_path).map_err(|err| Error::read_conf(&config_path, err))?;
    let config = basic_toml::from_str::<EvalConfig>(&config_contents)
        .map_err(|err| Error::toml(&config_path, err))?;
    Ok(())
}
