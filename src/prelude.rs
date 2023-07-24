use std::io;

pub use crate::constants;
pub use anyhow::Context;

#[cfg(test)]
pub use serde::{Deserialize, Serialize};

pub use derive_getters::Getters;
pub use derive_new::new;
pub type IoResult<T = ()> = Result<T, io::Error>;
pub type AppError = anyhow::Error;
pub type AppResult<T = ()> = Result<T, AppError>;

pub type ReturnToUser = AppResult<String>;
pub type PathResult = AppResult<PathBuf>;
pub type AppIoResult<T = ()> = Result<T, AppIoError>;

pub use crate::app_traits::file_manipulator::FileManipulator;
pub use crate::app_traits::path_provider::PathProvider;
pub use crate::app_traits::path_resolver::PathResolver;
pub use crate::file_management::AppIoError;
pub use crate::ValidTemplateName;
pub use std::path::{Path, PathBuf};

#[cfg(test)]
pub use testing::*;
#[cfg(test)]
pub mod testing {

    pub const TEST_INPUT_FOLDER_NAME: &str = "test_input";
    #[macro_export]
    macro_rules! from_ron_input_file {
        ($input:literal) => {{
            let path = const_format::concatcp!(
                TEST_INPUT_FOLDER_NAME,
                std::path::MAIN_SEPARATOR_STR,
                $input
            );
            ron::from_str(load_str!(path)).expect(concat!($input, "is not in a valid format"))
        }};
    }
    pub use from_ron_input_file;
}
