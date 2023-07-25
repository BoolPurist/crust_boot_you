use std::{ffi::OsStr, fs::File};

use log::LevelFilter;
use simplelog::{
    ColorChoice, CombinedLogger, ConfigBuilder, SharedLogger, TermLogger, TerminalMode, WriteLogger,
};

use crate::{prelude::*, AppCliEntry};
pub fn init(cli: &AppCliEntry, path_proiver: &impl PathProvider) {
    let is_debug = cfg!(debug_assertions);
    let (file_log_level, term_log_level) = get_logger_level(cli, is_debug);
    let mut config = if let Some(filters) = cli.module_filter() {
        let mut config = ConfigBuilder::default();
        config.clear_filter_allow();
        for next in filters {
            config.add_filter_allow(next.clone());
        }
        config
    } else {
        ConfigBuilder::default()
    };

    let logger_path = path_proiver
        .logger_file_location()
        .expect("Failed to get path to logging file.");
    let file_writer = WriteLogger::new(
        file_log_level,
        config.set_location_level(LevelFilter::Error).build(),
        File::options()
            .create(true)
            .append(true)
            .open(logger_path)
            .expect("Failed to create or access logger file."),
    );

    let loggers: Vec<Box<dyn SharedLogger>> = if *cli.term_logging() {
        let term_logger = TermLogger::new(
            term_log_level,
            config.set_location_level(LevelFilter::Off).build(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        );
        vec![file_writer, term_logger]
    } else {
        vec![file_writer]
    };
    const UNKOWN: &str = "Unknown";

    CombinedLogger::init(loggers).expect("Failed to initialze logger");
    let (app_name, app_path) = std::env::current_exe()
        .map(|path| {
            let file_name = path
                .file_name()
                .unwrap_or(OsStr::new(UNKOWN))
                .to_string_lossy()
                .into_owned();
            let stem = path.to_string_lossy().into_owned();
            (file_name, stem)
        })
        .unwrap_or((UNKOWN.to_string(), UNKOWN.to_string()));

    let banner = "=".repeat(100);
    info!("{}", banner);
    info!("Starting application with name: {}", app_name);
    info!("Appliaction loaction: {}", app_path)
}

fn get_logger_level(cli: &AppCliEntry, is_debug: bool) -> (LevelFilter, LevelFilter) {
    if let Some(given) = cli.log_level() {
        let for_both: LevelFilter = (*given).into();
        (for_both, for_both)
    } else {
        let term_file_logging_level = if is_debug {
            LevelFilter::Debug
        } else {
            LevelFilter::Info
        };
        let term_logging_level = if is_debug {
            LevelFilter::Debug
        } else {
            LevelFilter::Warn
        };
        (term_file_logging_level, term_logging_level)
    }
}
