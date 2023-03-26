use std::string::FromUtf8Error;

#[derive(Debug, Clone, thiserror::Error)]
pub enum CreatorError {
    #[error("IO error: {0}")]
    IoError(String),
    #[error("FromUtf8Error: {0}")]
    FromUtf8Error(#[from] FromUtf8Error),
    #[error("Not Connected to a Wifi Network")]
    NotConnected,
    #[error("Regex Error: {0}")]
    RegexError(String),
    #[error("No Match Found")]
    NoMatchFound,
    #[error("No Matching Disks Found")]
    NoMatchingDisksFound,
    // #[error("Config File Template Not Found")]
    // ConfigFileTemplateNotFound,
}

impl From<std::io::Error> for CreatorError {
    fn from(err: std::io::Error) -> Self {
        CreatorError::IoError(err.to_string())
    }
}

impl From<fancy_regex::Error> for CreatorError {
    fn from(err: fancy_regex::Error) -> Self {
        CreatorError::RegexError(err.to_string())
    }
}
