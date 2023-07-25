use clap::ValueEnum;
use log::LevelFilter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum CliLogLevel {
    None,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl From<CliLogLevel> for LevelFilter {
    fn from(value: CliLogLevel) -> Self {
        match value {
            CliLogLevel::None => LevelFilter::Off,
            CliLogLevel::Error => LevelFilter::Error,
            CliLogLevel::Warn => LevelFilter::Warn,
            CliLogLevel::Info => LevelFilter::Info,
            CliLogLevel::Debug => LevelFilter::Debug,
            CliLogLevel::Trace => LevelFilter::Trace,
        }
    }
}
