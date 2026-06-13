use std::{fmt::Debug, fs::File, io::Write};

pub struct Logger {
    log_file: File,
}

impl Logger {
    pub fn new(path: &str) -> Self {
        Self {
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
