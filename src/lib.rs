#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate log;
#[macro_use]
extern crate thiserror;

pub mod cli;

pub mod app_traits;
pub mod valid_template_name;
pub use valid_template_name::ValidTemplateName;
pub mod constants;
pub mod file_management;
pub mod handle_commands;
pub mod logging;
pub mod prelude;
pub use app_traits::path_provider::DevPathProvider;
pub use app_traits::path_provider::TestPathProvider;
pub use cli::AppCliEntry;
pub use cli::SubCommands;
pub use file_management::LoadedNode;

pub fn print_dry(to_print: &str) {
    println!("{} {}", *constants::DRY_LABEL, to_print);
}
