use std::{fmt, path::PathBuf};

#[derive(Debug)]
pub enum Error {
    ReadDir {
        path: PathBuf,
        msg: String,
    },
    PathAccess {
        path: PathBuf,
        tried: String,
    },
    FileAccess {
        path: PathBuf,
        tried: String,
        msg: String,
    },
    WorkingDir {
        tried: String,
        msg: String,
    },
    TestFailure {
        num_fail: usize,
    },
}

impl Error {
    pub fn read_dir<T: std::error::Error>(path: &PathBuf, err: T) -> Error {
        Error::ReadDir {
            path: path.clone(),
            msg: err.to_string(),
        }
    }

    pub fn path_access(path: &PathBuf, tried: &str) -> Error {
        Error::PathAccess {
            path: path.clone(),
            tried: tried.to_owned(),
        }
    }

    pub fn file_access<T: std::error::Error>(path: &PathBuf, tried: &str, err: T) -> Error {
        Error::FileAccess {
            path: path.clone(),
            tried: tried.to_owned(),
            msg: err.to_string(),
        }
    }

    pub fn working_dir<T: std::error::Error>(tried: &str, err: T) -> Error {
        Error::WorkingDir {
            tried: tried.to_owned(),
            msg: err.to_string(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::ReadDir { path, msg } => {
                write!(f, "Could not load contents of {path:?}:\n\t{msg}")
            }
            Error::PathAccess { path, tried } => write!(f, "Could not get {tried} for {path:?}"),
            Error::FileAccess { path, tried, msg } => {
                write!(f, "Could not {tried} file {path:?}:\n\t{msg}")
            }
            Error::WorkingDir { tried, msg } => {
                write!(f, "Could not {tried} working dir:\n\t{msg}")
            }
            Error::TestFailure { num_fail } => write!(f, "{num_fail} tests have failed"),
        }
    }
}

impl std::error::Error for Error {}
