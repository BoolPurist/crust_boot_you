use std::{io, path::PathBuf};

pub use crate::constants;
pub use anyhow::Context;

pub type IoResult<T = ()> = Result<T, io::Error>;
pub type AppError = anyhow::Error;
pub type AppResult<T = ()> = Result<T, AppError>;

pub type PathResult = AppResult<PathBuf>;
pub use crate::app_traits::file_manipulator::FileManipulator;
pub use crate::app_traits::path_provider::PathProvider;
pub use crate::NotEmptyText;
