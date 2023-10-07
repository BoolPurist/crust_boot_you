use derive_getters::Getters;
pub use subcommands::SubCommands;

mod init_kind;
mod load_cli_details;
mod non_file_path;
mod subcommands;

pub use from_cli_path::AbsoluteExistingPath;
pub use init_kind::InitKind;
pub use load_cli_details::LoadCliDetails;
pub use non_file_path::NonFilePath;
pub use subcommands::CreateTemplateArg;
pub use subcommands::LoadTemplateArg;
pub use subcommands::SaveTemplateCli;

mod from_cli_path;

use clap::Parser;

#[macro_export]
macro_rules! app_env_name {
    ($suffix:literal) => {{
        concat!("CRUST_BOOT_YOU_", $suffix)
    }};
}

#[derive(Debug, Parser, Getters)]
pub struct AppCliEntry {
    #[clap(short, long, env = app_env_name!("RUN_DRY"))]
    dry: bool,
    #[clap(short, long, env = app_env_name!("TERM_LOGGER"))]
    term_logging: bool,
    #[clap(short, long, env = app_env_name!("LOG_SETTINGS"))]
    log_settings: Option<String>,
    #[clap(subcommand)]
    sub_commands: SubCommands,
}

#[cfg(test)]
mod testing {
    use super::*;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        AppCliEntry::command().debug_assert()
    }
}
