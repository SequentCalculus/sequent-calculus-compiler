use std::{
    fs::{create_dir_all, read_to_string, rename},
    path::PathBuf,
    process::Command,
};

mod errors;
use errors::Error;

const EVAL_CONFIG: &str = "evaluation/config.toml";
const SCC_BIN: &str = "target/release/scc";
const BIN_OUT: &str = "target_scc/versions";

#[derive(Debug, Clone, serde::Deserialize)]
struct EvalConfig {
    version_git_hashes: Vec<String>,
}

impl EvalConfig {
    pub fn load() -> Result<EvalConfig, Error> {
        let config_path = PathBuf::from(EVAL_CONFIG);
        let config_contents =
            read_to_string(&config_path).map_err(|err| Error::read_conf(&config_path, err))?;
        basic_toml::from_str::<EvalConfig>(&config_contents)
            .map_err(|err| Error::toml(&config_path, err))
    }
}

fn main() -> Result<(), Error> {
    let config = EvalConfig::load()?;
    compile_versions(&config.version_git_hashes)?;
    Ok(())
}

fn compile_versions(hashes: &[String]) -> Result<(), Error> {
    let current_branch = String::from_utf8(
        Command::new("git")
            .arg("branch")
            .arg("--show-current")
            .output()
            .map_err(|err| Error::cmd("git", "get current branch", err))?
            .stdout,
    )
    .map_err(|err| Error::parse_out("git branch", err))?;

    let bin_path = PathBuf::from(BIN_OUT);
    create_dir_all(&bin_path).map_err(|err| Error::create_dir(&bin_path, err))?;

    let compiled_path = PathBuf::from(SCC_BIN);

    for (index, hash) in hashes.iter().enumerate() {
        let checkout_res = Command::new("git")
            .arg("checkout")
            .arg(hash)
            .status()
            .map_err(|err| Error::cmd("git", "checkout version branch", err))?;
        if !checkout_res.success() {
            return Err(Error::cmd(
                "git",
                "checkout version branch",
                format!("exited with code {checkout_res}"),
            ));
        }

        let out_path = bin_path.join(format!("scc_{index}"));
        rename(&compiled_path, &out_path)
            .map_err(|err| Error::move_file(&compiled_path, &out_path, err))?;
    }

    let checkout_res = Command::new("git")
        .arg("checkout")
        .arg(current_branch.trim())
        .status()
        .map_err(|err| Error::cmd("git", "checkout current branch", err))?;
    if !checkout_res.success() {
        return Err(Error::cmd(
            "git",
            "checkout current branch",
            format!("exited with {}", checkout_res),
        ));
    }
    Ok(())
}
