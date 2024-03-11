use lazy_static::lazy_static;
use log::{Level, Log, Metadata, Record};

struct Logger;

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!(
                "{} - {} - {} - {} - {}",
                record.level(),
                record.target(),
                record.file().unwrap(),
                record.line().unwrap(),
                record.args()
            );
        }
    }

    fn flush(&self) {}
}

lazy_static! {
    static ref LOGGER: Logger = Logger;
}

pub fn init_logger() {
    log::set_logger(&*LOGGER).unwrap();
    log::set_max_level(Level::Info.to_level_filter());
}
