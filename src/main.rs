#[macro_use]
extern crate log;

use clap::Parser;
use colored::*;
use crust_boot_you::app_traits::file_manipulator::DevOsFileManipulator;
use crust_boot_you::app_traits::file_manipulator::DryFileManipulator;
use crust_boot_you::app_traits::path_provider::DevPathProvider;
use crust_boot_you::handle_commands;
use crust_boot_you::logging;
use crust_boot_you::AppCliEntry;
use std::process::ExitCode;

fn main() -> ExitCode {
    logging::init();

    let args = AppCliEntry::parse();
    debug!("Cli arguments are parsed.");

    let (is_in_debug, is_in_dry) = (cfg!(debug_assertions), args.dry());
    let output = match (is_in_debug, is_in_dry) {
        (true, true) => {
            handle_commands::handle(DevPathProvider, DryFileManipulator::default(), &args)
        }
        (true, false) => {
            handle_commands::handle(DevPathProvider, DevOsFileManipulator::default(), &args)
        }
        (false, _) => todo!("Not implemented for production"),
    };

    match output {
        Ok(success_message) => {
            if args.dry() {
                crust_boot_you::print_dry(&success_message);
            } else {
                let message = format!("Success: {}", success_message).green();
                println!("{}", message);
            }
            ExitCode::SUCCESS
        }
        Err(error_message) => {
            let message = format!("Error: {}", error_message).red();
            eprintln!("{}", message);
            ExitCode::FAILURE
        }
    }
}
