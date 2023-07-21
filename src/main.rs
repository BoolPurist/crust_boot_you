#[macro_use]
extern crate log;

use clap::Parser;
use colored::*;
use crust_boot_you::app_traits::file_manipulator::DevOsFileManipulator;
use crust_boot_you::app_traits::file_manipulator::DryFileManipulator;
use crust_boot_you::app_traits::path_provider::DevPathProvider;
use crust_boot_you::constants;
use crust_boot_you::handle_commands;
use crust_boot_you::logging;
use crust_boot_you::prelude::AppResult;
use crust_boot_you::AppCliEntry;
use std::process::ExitCode;

fn main() -> ExitCode {
    logging::init();

    let args = AppCliEntry::parse();
    debug!("Cli arguments are parsed.");

    let (is_in_debug, is_in_dry) = (cfg!(debug_assertions), args.dry());
    let output = match (is_in_debug, is_in_dry) {
        (true, true) => handle_commands::handle(
            DevPathProvider::default(),
            DryFileManipulator::default(),
            &args,
        ),
        (true, false) => handle_commands::handle(
            DevPathProvider::default(),
            DevOsFileManipulator::default(),
            &args,
        ),
        (false, _) => todo!("Not implemented for production"),
    };

    handle_result_from_command(output, &args)
}

fn handle_result_from_command(output: AppResult<String>, args: &AppCliEntry) -> ExitCode {
    match output {
        Ok(success_message) => {
            if args.dry() {
                crust_boot_you::print_dry(&success_message);
            } else {
                println!("{}: {}", *constants::SUCCESS_LABEL, success_message);
            }
            ExitCode::SUCCESS
        }
        Err(error_message) => {
            if is_rust_backtrace_on() {
                eprintln!("{:?}", error_message);
            } else {
                let message_in_red = format!("Error: {}", error_message).red();
                eprintln!("{}", message_in_red);
            }
            ExitCode::FAILURE
        }
    }
}

fn is_rust_backtrace_on() -> bool {
    std::env::var("RUST_BACKTRACE")
        .map(|var| matches!(var.as_str(), "1" | "full"))
        .unwrap_or(false)
}
