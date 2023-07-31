#[macro_use]
pub mod prelude;

#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate log;
#[macro_use]
extern crate thiserror;

#[macro_use]
extern crate derive_more;

#[cfg(test)]
#[macro_use]
extern crate map_macro;

pub use valid_placeholder_border::ValidPlaceholderBorder;

pub mod app_traits;
pub mod cli;
pub mod template_augmentation;
mod valid_placeholder_border;
pub mod valid_template_name;
#[cfg_attr(not(debug_assertions), allow(unused_imports))]
use app_traits::path_resolver::DevPathResolver;
pub use valid_template_name::ValidTemplateName;
pub mod constants;
pub mod file_management;
pub mod handle_commands;
pub mod logging;
pub use app_traits::path_provider::DevPathProvider;
pub use app_traits::path_provider::TestPathProvider;
pub use cli::AppCliEntry;
pub use cli::SubCommands;

#[cfg(any(debug_assertions, test))]
type UsedPathResolver = DevPathResolver;
#[cfg(not(any(debug_assertions, test)))]
type UsedPathResolver = OsPathResolver;
#[cfg(not(any(debug_assertions, test)))]
use app_traits::path_resolver::OsPathResolver;

pub fn create_path_resolver() -> UsedPathResolver {
    UsedPathResolver::default()
}
pub fn print_dry(to_print: &str) {
    println!("{} {}", *constants::DRY_LABEL, to_print);
}
