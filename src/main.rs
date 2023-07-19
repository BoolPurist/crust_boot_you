#[macro_use]
extern crate log;

use clap::Parser;
use colored::*;
use crust_boot_you::app_traits::file_manipulator::OsFileManipulator;
use crust_boot_you::app_traits::path_provider::DevPathProvider;
use crust_boot_you::handle_commands;
use crust_boot_you::logging;
use crust_boot_you::AppCliEntry;
use std::process::ExitCode;

fn main() -> ExitCode {
    logging::init();

    let args = AppCliEntry::parse();
    debug!("Cli arguments are parsed.");
    let file_manipulator = OsFileManipulator;
    let output = if cfg!(debug_assertions) {
        handle_commands::handle(DevPathProvider, file_manipulator, &args)
    } else {
        todo!("Not implemented for production");
    };

    match output {
        Ok(success_message) => {
            let message = format!("Success: {}", success_message).green();
            println!("{}", message);
            ExitCode::SUCCESS
        }
        Err(error_message) => {
            let message = format!("Error: {}", error_message).red();
            eprintln!("{}", message);
            ExitCode::FAILURE
        }
    }
}
