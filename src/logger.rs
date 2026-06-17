//! Append-only file logger → /opt/rec25/logs/rec25.log

use anyhow::Result;
use chrono::Local;
use std::io::Write;

pub struct Logger {
    path: String,
}

impl Logger {
    /// Open (or create) the log file at `path`.
    pub fn open(path: &str) -> Result<Self> {
        // Ensure the parent directory exists.
        if let Some(parent) = std::path::Path::new(path).parent() {
            std::fs::create_dir_all(parent)?;
        }
        // Touch the file so it exists.
        std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?;
        Ok(Logger { path: path.to_string() })
    }

    fn write_line(&self, level: &str, msg: &str) {
        let ts = Local::now().format("%Y-%m-%dT%H:%M:%SZ");
        let line = format!("[{ts}] [{level}] {msg}\n");
        if let Ok(mut f) = std::fs::OpenOptions::new().append(true).open(&self.path) {
            let _ = f.write_all(line.as_bytes());
        }
    }

    pub fn info(&self, msg: &str)  { self.write_line("INFO",  msg); }
    pub fn warn(&self, msg: &str)  { self.write_line("WARN",  msg); }
    pub fn error(&self, msg: &str) { self.write_line("ERROR", msg); }
}
