use std::fmt::Display;
use std::path::PathBuf;

/// List of errors that can occur.
#[derive(Debug)]
pub enum Errors {
    FailedToReadDir(PathBuf),
    FailedToReadFile(PathBuf),
    FailedToFindNavFile(PathBuf),
}

impl Display for Errors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Errors::FailedToReadDir(dir) => write!(f, "Failed to read directory: {:?}", dir),
            Errors::FailedToReadFile(file) => write!(f, "Failed to read file: {:?}", file),
            Errors::FailedToFindNavFile(dir) => write!(f, "Failed to find nav.adoc file in directory: {:?}", dir),
        }
    }
}
