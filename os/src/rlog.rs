use log::{Level, LevelFilter, Metadata, Record, SetLoggerError};

struct ColorLogger;

impl log::Log for ColorLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let color = match record.level() {
            Level::Error => {
                31
            }
            Level::Warn => {
                93
            }
            Level::Info => {
                34
            }
            Level::Debug => {
                32
            }
            Level::Trace => {
                90
            }
        };

        println!("\x1b[{}m{}\x1b[0m", color, record.args());
    }

    fn flush(&self) {}
}

pub fn init() -> Result<(), SetLoggerError> {
    static LOGGER: ColorLogger = ColorLogger;

    let log_level = match option_env!("LOG") {
        Some("error") => {
            LevelFilter::Error
        }
        Some("warn") => {
            LevelFilter::Warn
        }
        Some("info") => {
            LevelFilter::Info
        }
        Some("debug") => {
            LevelFilter::Debug
        }
        Some("trace") => {
            LevelFilter::Trace
        }
        _ => {
            LevelFilter::Off
        }
    };

    log::set_logger(&LOGGER).map(|()| {
        log::set_max_level(log_level);
    })
}
