use std::io;

pub use crate::constants;
pub use anyhow::Context;

pub type IoResult<T = ()> = Result<T, io::Error>;
pub type AppError = anyhow::Error;
pub type AppResult<T = ()> = Result<T, AppError>;

pub type ReturnToUser = AppResult<String>;
pub type PathResult = AppResult<PathBuf>;
pub type AppIoResult<T = ()> = Result<T, AppIoError>;

pub use crate::app_traits::file_manipulator::FileManipulator;
pub use crate::app_traits::path_provider::PathProvider;
pub use crate::file_management::AppIoError;
pub use crate::NotEmptyText;
pub use std::path::{Path, PathBuf};

#[cfg(test)]
pub use mockall::automock;
