use clap::Parser;
use colored::*;
use crust_boot_you::app_traits::file_manipulator::DevOsFileManipulator;
use crust_boot_you::app_traits::file_manipulator::DryFileManipulator;
use crust_boot_you::app_traits::file_manipulator::OsFileManipulator;
#[cfg_attr(not(debug_assertions), allow(dead_code, unused_imports))]
use crust_boot_you::constants;
use crust_boot_you::handle_commands;
use crust_boot_you::logging;
use crust_boot_you::prelude::AppResult;
use crust_boot_you::prelude::PathProvider;
use crust_boot_you::prelude::ReturnToUser;
use crust_boot_you::AppCliEntry;
use crust_boot_you::UsedPathProvider;
use log::log_enabled;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args = AppCliEntry::parse();
    let path_provider = UsedPathProvider::default();

    logging::init(&args, &path_provider).expect("Failed to initialize logger.");

    let output = process_command(&args, &path_provider);
    print_result(output, &args)
}

fn process_command(args: &AppCliEntry, paths: &impl PathProvider) -> ReturnToUser {
    let is_debug = cfg!(debug_assertions);
    let is_dry = *args.dry();

    let output = match (is_debug, is_dry) {
        (true, true) => handle_commands::handle(paths, &DryFileManipulator::default(), args),
        (true, false) => handle_commands::handle(paths, &DevOsFileManipulator::default(), args),
        (false, false) => handle_commands::handle(paths, &OsFileManipulator::default(), args),
        (false, true) => handle_commands::handle(paths, &DryFileManipulator::default(), args),
    }?;

    Ok(output)
}

fn print_result(output: AppResult<String>, args: &AppCliEntry) -> ExitCode {
    return match output {
        Ok(success_message) => {
            if *args.dry() {
                crust_boot_you::print_dry(&success_message);
            } else {
                println!("{}: {}", *constants::SUCCESS_LABEL, success_message);
            }
            ExitCode::SUCCESS
        }
        Err(error_message) => {
            log::error!("{:?}", error_message);
            let (loggin_in_term, logging_allowed) =
                (args.term_logging(), log_enabled!(log::Level::Error));
            match (*loggin_in_term, logging_allowed) {
                (false, _) => eprint_output(&error_message),
                (true, false) => eprint_output(&error_message),
                (true, true) => (),
            }

            ExitCode::FAILURE
        }
    };

    fn eprint_output<T>(error_message: &T)
    where
        T: std::fmt::Display + std::fmt::Debug,
    {
        eprintln!("{}: {:?}", "Error".red(), error_message);
    }
}
