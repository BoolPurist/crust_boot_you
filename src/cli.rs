use derive_getters::Getters;
pub use subcommands::SubCommands;

mod cli_loglevel;
mod init_kind;
mod subcommands;

pub use cli_loglevel::CliLogLevel;
pub use from_cli_path::AbsoluteExistingPath;
pub use init_kind::InitKind;
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
    #[clap(short, long, value_enum, env = app_env_name!("LOG_LEVEL"))]
    log_level: Option<CliLogLevel>,
    #[clap(short, long, num_args = 1.., value_delimiter = ',', env = app_env_name!("LOG_MODULE_FILTER"))]
    module_filter: Option<Vec<String>>,
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
