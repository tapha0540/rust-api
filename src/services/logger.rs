use std::{fmt::Debug, fs::File, io::Write};

#[derive(Debug)]
pub struct Logger {
    log_file: File,
    path: String,
}

impl Clone for Logger {
    fn clone(&self) -> Self {
        Self {
            path: self.path.clone(),
            log_file: File::options()
                .append(true)
                .create(true)
                .open(&self.path)
                .expect("Failed to create the log file"),
        }
    }
}

impl Logger {
    pub fn new(path: String) -> Self {
        Self {
            path: path.clone(),
            log_file: File::options()
                .append(true)
                .create(true)
                .open(path)
                .expect("Failed to create the log file"),
        }
    }
    pub fn log<E: Debug>(&mut self, msg: E) {
        self.log_file
            .write_all(format!("{:?}\n", msg).as_bytes())
            .expect("Failed to write to the log file");
    }
}
