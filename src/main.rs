#[macro_use]
extern crate log;

use log::{Level, Metadata, Record};
use std::fs::{File, OpenOptions};
use std::io::{self, BufWriter, Write};
use std::{error, fmt, result};
use std::sync::RwLock;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
enum FileLoggerError {
    Io(io::Error),
    SetLogger(log::SetLoggerError),
}

type Result<T> = result::Result<T, FileLoggerError>;

impl error::Error for FileLoggerError {
    fn description(&self) -> &str {
        match *self {
            FileLoggerError::Io(ref err) => err.description(),
            FileLoggerError::SetLogger(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            FileLoggerError::Io(ref err) => Some(err),
            FileLoggerError::SetLogger(ref err) => Some(err),
        }
    }
}

impl fmt::Display for FileLoggerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FileLoggerError::Io(ref err) => write!(f, "IO error: {}", err),
            FileLoggerError::SetLogger(ref err) => write!(f, "Parse error: {}", err)
        }
    }
}

struct FileLogger {
    level: Level,
    writer: RwLock<BufWriter<File>>,
}

impl log::Log for FileLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let mut writer = self.writer
            .write()
            .expect("failed to unlock log file writer in write mode");
            let now = SystemTime::now();
            let timestamp = now.duration_since(UNIX_EPOCH).expect("failed to generate timestamp: This system is opertating before unix epoch");
            write!(writer, "{} {} at {}: {}\n", record.level(), timestamp.as_secs(), record.target(), record.args()).expect("failed to log to file");
        }
        self.flush();
    }

    fn flush(&self) {
        self.writer.write()
        .expect("failed to unlock log file writer in write mode")
        .flush()
        .expect("failed to flush log file writer");
    }
}

impl FileLogger {
    fn init(level: Level, file_name: &str) -> Result<()> {
        let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_name)?;
        let writer = RwLock::new(BufWriter::new(file));
        let logger = FileLogger {level, writer};
        log::set_max_level(level.to_level_filter());
        log::set_boxed_logger(Box::new(logger))?;
        Ok(())
    }
}



fn main() {
    FileLogger::init(Level::Info, "./log.txt").expect("Failed to init FileLogger");
    trace!("Beginning the operation");
    info!("It's moving");
    error!("It's alive!");
    debug!("Dr. Frankenstein now knows it feels to be god");
    trace!("End of the operation");
}