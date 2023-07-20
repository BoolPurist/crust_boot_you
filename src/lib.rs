#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate log;
#[macro_use]
extern crate thiserror;

pub mod cli;

pub mod app_traits;
pub mod not_empty_text;
use colored::Colorize;
pub use not_empty_text::NotEmptyText;
pub mod constants;
mod file_management;
pub mod handle_commands;
pub mod logging;
pub mod prelude;
pub use app_traits::path_provider::DevPathProvider;
pub use cli::AppCliEntry;
pub use cli::SubCommands;
pub use file_management::LoadedNode;

use once_cell::sync::Lazy;

static DRY_LABEL: Lazy<String> = Lazy::new(|| "DRY:".blue().to_string());

pub fn print_dry(to_print: &str) {
    println!("{} {}", *DRY_LABEL, to_print);
}
