#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AugmentationError {
    NotFound(String),
    NoValueAndDefaultConsole(String),
}
