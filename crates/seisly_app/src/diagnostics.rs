use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;
use chrono::{DateTime, Local};

#[derive(Debug, Clone)]
pub struct LogEntry {
    pub timestamp: DateTime<Local>,
    pub level: log::Level,
    pub message: String,
}

pub struct LogBuffer {
    entries: Vec<LogEntry>,
    max_entries: usize,
}

impl LogBuffer {
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: Vec::with_capacity(max_entries),
            max_entries,
        }
    }

    pub fn push(&mut self, entry: LogEntry) {
        if self.entries.len() >= self.max_entries {
            self.entries.remove(0);
        }
        self.entries.push(entry);
    }

    pub fn entries(&self) -> &[LogEntry] {
        &self.entries
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }
}

lazy_static! {
    pub static ref GLOBAL_LOGS: Arc<Mutex<LogBuffer>> = Arc::new(Mutex::new(LogBuffer::new(1000)));
}

pub struct SeislyLogger;

impl log::Log for SeislyLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let entry = LogEntry {
                timestamp: Local::now(),
                level: record.level(),
                message: format!("{}", record.args()),
            };
            if let Ok(mut buffer) = GLOBAL_LOGS.lock() {
                buffer.push(entry);
            }
            // Also print to console for development
            println!("[{}] {} - {}", Local::now().format("%H:%M:%S"), record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

static LOGGER: SeislyLogger = SeislyLogger;

pub fn init() -> Result<(), log::SetLoggerError> {
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(log::LevelFilter::Info))
}
