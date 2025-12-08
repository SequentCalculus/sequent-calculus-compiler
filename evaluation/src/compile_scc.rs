use crate::{BIN_OUT, SCC_BIN, errors::Error};
use std::{
    fs::{create_dir_all, rename},
    path::{Path, PathBuf},
    process::Command,
};

pub fn compile_versions(hashes: &[String]) -> Result<(), Error> {
    let current_branch = get_current_branch()?;

    let bin_path = PathBuf::from(BIN_OUT);
    create_dir_all(&bin_path).map_err(|err| Error::create_dir(&bin_path, err))?;
    let compiled_path = PathBuf::from(SCC_BIN);

    for (index, hash) in hashes.iter().enumerate() {
        checkout_branch(hash)?;
        compile_current(index, &bin_path, &compiled_path)?;
    }

    checkout_branch(&current_branch)?;
    Ok(())
}

fn get_current_branch() -> Result<String, Error> {
    let branch_res = Command::new("git")
        .arg("branch")
        .arg("--show-current")
        .output()
        .map_err(|err| Error::start_cmd("git", "get current branch", err))?;

    if !branch_res.status.success() {
        let stdout_str =
            String::from_utf8(branch_res.stdout).map_err(|err| Error::parse_out("git", err))?;
        let stderr_str =
            String::from_utf8(branch_res.stderr).map_err(|err| Error::parse_out("git", err))?;
        return Err(Error::run_cmd(
            "git",
            branch_res.status,
            &stdout_str,
            &stderr_str,
        ));
    }
    Ok(String::from_utf8(branch_res.stdout)
        .map_err(|err| Error::parse_out("git branch", err))?
        .trim()
        .to_owned())
}

fn checkout_branch(branch: &str) -> Result<(), Error> {
    let checkout_res = Command::new("git")
        .arg("checkout")
        .arg(branch)
        .output()
        .map_err(|err| Error::start_cmd("git", "checkout branch", err))?;
    if !checkout_res.status.success() {
        let stdout_str =
            String::from_utf8(checkout_res.stdout).map_err(|err| Error::parse_out("git", err))?;
        let stderr_str =
            String::from_utf8(checkout_res.stderr).map_err(|err| Error::parse_out("git", err))?;

        return Err(Error::run_cmd(
            "git",
            checkout_res.status,
            &stdout_str,
            &stderr_str,
        ));
    }
    Ok(())
}

fn compile_current(index: usize, bin_path: &Path, compiled_path: &Path) -> Result<(), Error> {
    let cargo_res = Command::new("cargo")
        .arg("build")
        .arg("--release")
        .output()
        .map_err(|err| Error::start_cmd("cargo", "build version", err))?;

    if !cargo_res.status.success() {
        let stdout_str =
            String::from_utf8(cargo_res.stdout).map_err(|err| Error::parse_out("git", err))?;
        let stderr_str =
            String::from_utf8(cargo_res.stderr).map_err(|err| Error::parse_out("git", err))?;

        return Err(Error::run_cmd(
            "cargo",
            cargo_res.status,
            &stdout_str,
            &stderr_str,
        ));
    }

    let out_path = bin_path.join(format!("scc_{index}"));
    rename(compiled_path, &out_path)
        .map_err(|err| Error::move_file(compiled_path, &out_path, err))?;
    Ok(())
}
