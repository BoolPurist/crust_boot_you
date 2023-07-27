use clap::Parser;
use colored::*;
use crust_boot_you::app_traits::file_manipulator::DevOsFileManipulator;
use crust_boot_you::app_traits::file_manipulator::DryFileManipulator;
use crust_boot_you::app_traits::file_manipulator::OsFileManipulator;
use crust_boot_you::app_traits::path_provider::DevPathProvider;
use crust_boot_you::app_traits::path_provider::ProdPathProvider;
use crust_boot_you::cli::CliLogLevel;
use crust_boot_you::constants;
use crust_boot_you::handle_commands;
use crust_boot_you::logging;
use crust_boot_you::prelude::AppResult;
use crust_boot_you::prelude::ReturnToUser;
use crust_boot_you::template_augmentation::RegexTemplateAugmentor;
use crust_boot_you::AppCliEntry;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args = AppCliEntry::parse();
    let output = process_command(&args);
    print_result(output, &args)
}

fn process_command(args: &AppCliEntry) -> ReturnToUser {
    let is_debug = cfg!(debug_assertions);
    let is_dry = args.dry();
    let mut augmentor = RegexTemplateAugmentor::default();
    let output = match (is_debug, is_dry) {
        (true, true) => {
            let paths = dev_logger_init(args);
            handle_commands::handle(&paths, &DryFileManipulator::default(), &mut augmentor, args)
        }
        (true, false) => {
            let paths = dev_logger_init(args);
            handle_commands::handle(
                &paths,
                &DevOsFileManipulator::default(),
                &mut augmentor,
                args,
            )
        }
        (false, false) => {
            let paths = ProdPathProvider;
            logging::init(args, &paths);
            handle_commands::handle(&paths, &OsFileManipulator::default(), &mut augmentor, args)
        }
        (false, true) => {
            let paths = ProdPathProvider;
            logging::init(args, &paths);
            handle_commands::handle(&paths, &DryFileManipulator::default(), &mut augmentor, args)
        }
    }?;

    return Ok(output);

    fn dev_logger_init(args: &AppCliEntry) -> DevPathProvider {
        let paths = DevPathProvider::default();
        logging::init(args, &paths);
        paths
    }
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
            let (loggin_in_term, logging_allowed) = (
                args.term_logging(),
                args.log_level()
                    .map(|level| level != CliLogLevel::None)
                    .unwrap_or(true),
            );
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
