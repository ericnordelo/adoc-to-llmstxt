use std::fmt::Display;

/// List of errors that can occur.
#[derive(Debug)]
pub enum Errors {
    FailedToReadDir,
    FailedToReadFile,
    FailedToFindNavFile,
}

impl Display for Errors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Errors::FailedToReadDir => write!(f, "Failed to read directory"),
            Errors::FailedToReadFile => write!(f, "Failed to read file"),
            Errors::FailedToFindNavFile => write!(f, "Failed to find nav.adoc file in directory"),
        }
    }
}
