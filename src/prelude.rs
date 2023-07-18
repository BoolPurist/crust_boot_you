use std::io;

pub use crate::constants;
pub use anyhow::Context;
pub type IoResult<T = ()> = Result<T, io::Error>;
pub type AppError = anyhow::Error;
pub type AppResult<T = ()> = Result<T, AppError>;
