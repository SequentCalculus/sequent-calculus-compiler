use std::{
    fmt,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub enum Error {
    ReadConfig {
        path: PathBuf,
        msg: String,
    },
    Toml {
        path: PathBuf,
        msg: String,
    },
    Command {
        cmd: String,
        tried: String,
        msg: String,
    },
    ParseStdOut {
        cmd: String,
        msg: String,
    },
    CreateDir {
        path: PathBuf,
        msg: String,
    },
    MoveFile {
        from: PathBuf,
        to: PathBuf,
        msg: String,
    },
    ReadDir {
        path: PathBuf,
        msg: String,
    },
    ReadFileName {
        path: PathBuf,
    },
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

    pub fn cmd<T>(cmd: &str, tried: &str, err: T) -> Error
    where
        T: fmt::Display,
    {
        Error::Command {
            cmd: cmd.to_owned(),
            tried: tried.to_owned(),
            msg: err.to_string(),
        }
    }

    pub fn parse_out<Err>(cmd: &str, err: Err) -> Error
    where
        Err: std::error::Error,
    {
        Error::ParseStdOut {
            cmd: cmd.to_owned(),
            msg: err.to_string(),
        }
    }

    pub fn create_dir<Err>(path: &Path, err: Err) -> Error
    where
        Err: std::error::Error,
    {
        Error::CreateDir {
            path: path.to_path_buf(),
            msg: err.to_string(),
        }
    }

    pub fn move_file<Err>(from: &Path, to: &Path, err: Err) -> Error
    where
        Err: std::error::Error,
    {
        Error::MoveFile {
            from: from.to_path_buf(),
            to: to.to_path_buf(),
            msg: err.to_string(),
        }
    }

    pub fn read_dir<Err>(path: &Path, err: Err) -> Error
    where
        Err: std::error::Error,
    {
        Error::ReadDir {
            path: path.to_path_buf(),
            msg: err.to_string(),
        }
    }

    pub fn read_file_name(path: &Path) -> Error {
        Error::ReadFileName {
            path: path.to_path_buf(),
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
            Error::Command { cmd, tried, msg } => write!(
                f,
                "Could not {tried}, command {cmd} exited with error {msg}"
            ),
            Error::ParseStdOut { cmd, msg } => {
                write!(f, "Could not read std out from {cmd}: {msg}")
            }
            Error::CreateDir { path, msg } => {
                write!(f, "Could not create directory {}:{msg}", path.display())
            }
            Error::MoveFile { from, to, msg } => write!(
                f,
                "Could not move {} -> {}: {msg}",
                from.display(),
                to.display()
            ),
            Error::ReadDir { path, msg } => {
                write!(f, "Could not read dir {}:{msg}", path.display())
            }
            Error::ReadFileName { path } => {
                write!(f, "Could not get file name of {}", path.display())
            }
        }
    }
}

impl std::error::Error for Error {}
