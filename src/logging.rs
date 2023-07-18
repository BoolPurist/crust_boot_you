use log::LevelFilter;
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};

pub fn init() {
    let logging_level = if cfg!(debug_assertions) {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };

    TermLogger::init(
        logging_level,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .expect("Could initialize logger");
}
