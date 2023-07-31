use flexi_logger::{Cleanup, Criterion, Duplicate, FileSpec, Logger, LoggerHandle, Naming};

use crate::{prelude::*, AppCliEntry};

pub fn init(cli: &AppCliEntry, path_proiver: &impl PathProvider) -> AppResult<LoggerHandle> {
    let is_debug = cfg!(debug_assertions);
    // let (file_log_level, term_log_level) = get_logger_level(cli, is_debug);

    let logger_folder_path = path_proiver.logger_folder_location().unwrap();
    let base_name = if is_debug {
        constants::PREFIX_FILE_DEV_LOG
    } else {
        constants::APP_NAME
    };

    let fs_specs = FileSpec::default()
        .directory(logger_folder_path)
        .basename(base_name)
        .suffix(constants::SUFFIX_FILE_LOG)
        .suppress_timestamp();

    let level = get_logger_settings(cli, is_debug)
        .format_for_files(flexi_logger::detailed_format)
        .format_for_stderr(flexi_logger::colored_default_format);

    let up_to_file = level.log_to_file(fs_specs).rotate(
        Criterion::Size(constants::MAX_SIZE_MEGA_BYTES),
        if is_debug {
            Naming::Numbers
        } else {
            Naming::Timestamps
        },
        Cleanup::KeepLogFiles(constants::NUMBER_OF_FILES),
    );

    let up_to_file = {
        let up_to_file = if is_debug {
            up_to_file
        } else {
            up_to_file.append()
        };

        if *cli.term_logging() {
            up_to_file.duplicate_to_stderr(Duplicate::All)
        } else {
            up_to_file
        }
        .start()
    }?;

    let banner = "=".repeat(100);
    info!("{}", banner);

    info!("Starting application with name: {}", constants::APP_NAME);
    Ok(up_to_file)
}

fn get_logger_settings(cli_entry: &AppCliEntry, is_debug: bool) -> Logger {
    match (is_debug, cli_entry.log_settings()) {
        (true, None) => Logger::try_with_str("debug"),
        (false, None) => Logger::try_with_str("info"),
        (_, Some(settings)) => Logger::try_with_str(settings),
    }
    .unwrap()
}
