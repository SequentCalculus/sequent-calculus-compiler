use super::errors::Error;
use log::{Level, Log, Metadata, Record};
use std::{
    fs::{File, OpenOptions},
    io::Write,
    path::PathBuf,
};

pub struct Logger {
    log_path: PathBuf,
    log_level: Level,
}

impl Logger {
    pub fn new(log_path: PathBuf) -> Result<Logger, Error> {
        File::create(&log_path)?;
        Ok(Logger {
            log_path,
            log_level: Level::Info,
        })
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.log_level
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
        let log_line = format!("{} - {}\n", record.level(), record.args());
        let mut file = OpenOptions::new()
            .append(true)
            .open(&self.log_path)
            .unwrap();
        file.write_all(log_line.as_bytes()).unwrap();
    }

    fn flush(&self) {}
}
