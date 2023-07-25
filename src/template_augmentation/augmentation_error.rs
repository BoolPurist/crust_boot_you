use thiserror::Error;
#[derive(Debug, PartialEq, Eq, Clone, Error)]
pub enum AugmentationError {
    #[error("{0}")]
    NotFound(String),
    #[error(
        "No value was entered for the key {0} in the console. This key has not default value either."
    )]
    NoValueAndDefaultConsole(String),
    #[error("Could read input from std in")]
    StdInProblem,
}
