use once_cell::sync::Lazy;
use std::path::Path;
const PROJECT_ROOT: &str = env!("CARGO_MANIFEST_DIR");

pub static PROJECT_ROOT_PATH: Lazy<&Path> = Lazy::new(|| &Path::new(PROJECT_ROOT));

pub const TEMPLATES_FOLDER: &str = "templates";
pub const TEMPLATE_META_FILE_NAME: &str = "template_mate.json";

pub mod dev {
    pub const ENTRY_FOLDER: &str = ".dev_data";
    pub const DATA_FOLDER: &str = "data";
}
