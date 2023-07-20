use log::LevelFilter;
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};
pub fn init_logger() {
    let logging_level = LevelFilter::Debug;

    TermLogger::init(
        logging_level,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .expect("Could initialize logger");
}
