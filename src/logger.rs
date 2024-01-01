use log::{Level, Metadata, Record};

pub struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("[{}] {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

static LOGGER: SimpleLogger = SimpleLogger;

pub fn init_logger(verbose: bool) {
    log::set_logger(&LOGGER).expect("Failed to set logger");
    log::set_max_level(if verbose {
        Level::Info.to_level_filter()
    } else {
        Level::Warn.to_level_filter()
    });
}
