use clap::Parser;
use colored::*;
use crust_boot_you::app_traits::file_manipulator::DevOsFileManipulator;
use crust_boot_you::app_traits::file_manipulator::DryFileManipulator;
use crust_boot_you::app_traits::path_provider::DevPathProvider;
use crust_boot_you::cli::CliLogLevel;
use crust_boot_you::constants;
use crust_boot_you::handle_commands;
use crust_boot_you::logging;
use crust_boot_you::prelude::AppResult;
use crust_boot_you::template_augmentation::RegexTemplateAugmentor;
use crust_boot_you::AppCliEntry;
use std::process::ExitCode;

fn main() -> ExitCode {
    let is_in_debug = cfg!(debug_assertions);

    let args = AppCliEntry::parse();

    let is_in_dry = args.dry();
    let mut augmentor = RegexTemplateAugmentor::default();

    let output = match (is_in_debug, is_in_dry) {
        (true, true) => {
            let paths = DevPathProvider::default();
            logging::init(&args, &paths);
            handle_commands::handle(
                &paths,
                &DryFileManipulator::default(),
                &mut augmentor,
                &args,
            )
        }
        (true, false) => {
            let paths = DevPathProvider::default();
            logging::init(&args, &paths);
            handle_commands::handle(
                &DevPathProvider::default(),
                &DevOsFileManipulator::default(),
                &mut augmentor,
                &args,
            )
        }
        (false, _) => todo!("Not implemented for production"),
    };

    handle_result_from_command(output, &args)
}

fn handle_result_from_command(output: AppResult<String>, args: &AppCliEntry) -> ExitCode {
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
            if is_rust_backtrace_on() {
                log::error!("{:?}", error_message);
            } else {
                log::error!("{}", error_message);

                let (loggin_in_term, logging_allowed) = (
                    args.term_logging(),
                    args.log_level()
                        .map(|level| level != CliLogLevel::None)
                        .unwrap_or(true),
                );
                match (*loggin_in_term, logging_allowed) {
                    (false, _) => eprint_output(error_message.to_string()),
                    (true, false) => eprint_output(error_message.to_string()),
                    (true, true) => (),
                }
            }
            ExitCode::FAILURE
        }
    };

    fn eprint_output(error_message: String) {
        eprintln!("{}: {}", "Error".red(), error_message);
    }
}

fn is_rust_backtrace_on() -> bool {
    std::env::var("RUST_BACKTRACE")
        .map(|var| matches!(var.as_str(), "1" | "full"))
        .unwrap_or(false)
}
