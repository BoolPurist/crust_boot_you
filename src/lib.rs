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

pub mod app_traits;
pub mod cli;
pub mod constants;
pub mod file_management;
pub mod handle_commands;
pub mod logging;
pub mod template_augmentation;
pub mod valid_template_name;

pub use app_traits::path_provider::DevPathProvider;
pub use app_traits::path_provider::ProdPathProvider;
pub use app_traits::path_provider::TestPathProvider;
pub use app_traits::path_resolver::OsPathResolver;
pub use cli::AppCliEntry;
pub use cli::SubCommands;
pub use valid_placeholder_border::ValidPlaceholderBorder;
pub use valid_template_name::ValidTemplateName;

mod valid_placeholder_border;

#[cfg_attr(not(debug_assertions), allow(unused_imports))]
use app_traits::path_resolver::DevPathResolver;

#[cfg(any(debug_assertions, test))]
pub type UsedPathResolver = DevPathResolver;
#[cfg(not(any(debug_assertions, test)))]
pub type UsedPathResolver = OsPathResolver;

#[cfg(any(debug_assertions, test))]
pub type UsedPathProvider = DevPathProvider;
#[cfg(not(any(debug_assertions, test)))]
pub type UsedPathProvider = ProdPathProvider;

pub fn create_path_resolver() -> UsedPathResolver {
    UsedPathResolver::default()
}

pub fn print_dry(to_print: &str) {
    println!("{} {}", *constants::DRY_LABEL, to_print);
}
