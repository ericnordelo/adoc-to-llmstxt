use std::fmt::Display;
use std::path::PathBuf;

/// List of errors that can occur.
#[derive(Debug)]
pub enum Errors {
    ReadDir(PathBuf),
    ReadFile(PathBuf),
    FindNavFile(PathBuf),
    DeserializeConfig(PathBuf),
    ReadConfig(PathBuf),
}

impl Display for Errors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Errors::ReadDir(dir) => write!(f, "Failed to read directory: {:?}", dir),
            Errors::ReadFile(file) => write!(f, "Failed to read file: {:?}", file),
            Errors::FindNavFile(dir) => {
                write!(f, "Failed to find nav.adoc file in directory: {:?}", dir)
            }
            Errors::DeserializeConfig(file) => {
                write!(f, "Failed to deserialize config at path: {:?}", file)
            }
            Errors::ReadConfig(file) => {
                write!(f, "Failed to read config at path: {:?}", file)
            }
        }
    }
}
