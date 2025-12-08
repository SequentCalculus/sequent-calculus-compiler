use std::{
    fmt,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub enum Error {
    ReadConfig { path: PathBuf, msg: String },
    Toml { path: PathBuf, msg: String },
}

impl Error {
    pub fn read_conf<Err>(path: &Path, err: Err) -> Error
    where
        Err: std::error::Error,
    {
        Error::ReadConfig {
            path: path.to_path_buf(),
            msg: err.to_string(),
        }
    }

    pub fn toml<Err>(path: &Path, err: Err) -> Error
    where
        Err: std::error::Error,
    {
        Error::Toml {
            path: path.to_path_buf(),
            msg: err.to_string(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::ReadConfig { path, msg } => {
                write!(f, "Could not read config {}:\n{msg}", path.display())
            }
            Error::Toml { path, msg } => {
                write!(f, "Could not read toml {}:\n{msg}", path.display())
            }
        }
    }
}

impl std::error::Error for Error {}
