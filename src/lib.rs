#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate log;

mod cli;
mod not_empty_text;
pub use absolue_existing_path::AbsoluteExistingPath;
pub use not_empty_text::NotEmptyText;
mod absolue_existing_path;
pub mod constants;
mod file_management;
pub mod handle_commands;
pub mod logging;
pub mod prelude;
pub use cli::AppCliEntry;
pub use cli::SubCommands;
