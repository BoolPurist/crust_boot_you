#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate log;

mod cli;

pub mod app_traits;
pub mod not_empty_text;
pub use not_empty_text::NotEmptyText;
pub mod constants;
mod file_management;
pub mod handle_commands;
pub mod logging;
pub mod prelude;
pub use cli::AppCliEntry;
pub use cli::SubCommands;
