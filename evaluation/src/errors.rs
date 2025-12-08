use std::{
    fmt,
    path::{Path, PathBuf},
    process::ExitStatus,
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
    StartCommand {
        cmd: String,
        tried: String,
        msg: String,
    },
    RunCommand {
        cmd: String,
        status: ExitStatus,
        stdout: String,
        stderr: String,
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

    pub fn start_cmd<Err>(cmd: &str, tried: &str, err: Err) -> Error
    where
        Err: std::error::Error,
    {
        Error::StartCommand {
            cmd: cmd.to_owned(),
            tried: tried.to_owned(),
            msg: err.to_string(),
        }
    }

    pub fn run_cmd(cmd: &str, status: ExitStatus, stdout: &str, stderr: &str) -> Error
where {
        Error::RunCommand {
            cmd: cmd.to_owned(),
            status,
            stdout: stdout.to_owned(),
            stderr: stderr.to_owned(),
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
            Error::RunCommand {
                cmd,
                status,
                stdout,
                stderr,
            } => write!(
                f,
                "Command {cmd} exited with status {status}, stdout: {stdout}, stderr: {stderr}"
            ),

            Error::StartCommand { cmd, tried, msg } => write!(
                f,
                "Error during {tried}, could not run command {cmd}: {msg}"
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
