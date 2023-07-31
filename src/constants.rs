use colored::Colorize;
use once_cell::sync::Lazy;
use std::path::Path;

use crate::UsedPathResolver;
pub const fn project_root() -> &'static str {
    env!("CARGO_MANIFEST_DIR")
}

pub const APP_NAME: &str = env!("CARGO_PKG_NAME");
pub static DRY_LABEL: Lazy<String> = Lazy::new(|| "DRY:".blue().to_string());
pub static SUCCESS_LABEL: Lazy<String> = Lazy::new(|| "SUCCESS:".green().to_string());
pub static PROJECT_ROOT_PATH: Lazy<&Path> = Lazy::new(|| Path::new(project_root()));

pub const DEFAULT_LEFT_DELIMITER: &str = "{{";
pub const DEFAULT_RIGHT_DELIMITER: &str = "}}";
pub const PROD_LOG_FILE: &str = "crust_boot_you.log";
pub const TEMPLATES_FOLDER: &str = "templates";
pub const TEMPLATE_META_FILE_NAME: &str = "meta_data.json";
pub const SCRIPT_FOLDER_NAME: &str = "scripts";
pub const DICTIONARY_FILE: &str = "dict.toml";
pub const FILES_FOLDER: &str = "files";
pub const TITLE_LIST_RESULT: &str = "List of all templates:";
pub const LOG_FOLDER_NAME: &str = "logs";

pub const INIT_KIND_ONLY_EMPTY: &str = "only_empty";
pub const INIT_KIND_NO_NAME_CONFLICT: &str = "no_name_conflict";
pub const INIT_KIND_OVERRIDE: &str = "override";
pub const INIT_KIND_PURGE: &str = "purge";
pub const SEPERATOR_BETWEEN_DEFAULT_AND_VALUE: char = '?';

pub const MAX_SIZE_MEGA_BYTES: u64 = 10 * 1024 * 1024;
pub const NUMBER_OF_FILES: usize = 10;
pub const PREFIX_FILE_DEV_LOG: &str = "dev";
pub const SUFFIX_FILE_LOG: &str = "log";

pub static USED_PATH_PROVIDER: Lazy<UsedPathResolver> = Lazy::new(UsedPathResolver::default);

pub mod dev {
    pub const TMP_ROOT: &str = "crust_boot_you_tmp";
    pub const TMP_CWD_FOLDER: &str = "CWD";
    pub const DATA_FOLDER: &str = "data";
    pub const CONFIG_FOLDER: &str = "config";
}
