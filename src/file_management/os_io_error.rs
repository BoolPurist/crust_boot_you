use std::io;

#[derive(Debug, Error)]
pub enum AppIoError {
    #[error("Io error: {0}.")]
    Io(#[source] io::Error),
    #[error("Io error: {0}.")]
    FsExtra(#[source] fs_extra::error::Error),
    #[error("Io error: path does not exits")]
    NotFound,
    #[error("Io error: {0}")]
    Custom(String),
}

impl AppIoError {
    pub fn custom(message: impl AsRef<str>) -> Self {
        let message = message.as_ref();
        Self::Custom(message.to_string())
    }
}
impl From<io::Error> for AppIoError {
    fn from(error: io::Error) -> Self {
        if error.kind() == io::ErrorKind::NotFound {
            Self::NotFound
        } else {
            Self::Io(error)
        }
    }
}

impl From<fs_extra::error::Error> for AppIoError {
    fn from(error: fs_extra::error::Error) -> Self {
        if let fs_extra::error::ErrorKind::NotFound = error.kind {
            Self::NotFound
        } else {
            Self::FsExtra(error)
        }
    }
}
