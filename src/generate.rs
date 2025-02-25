use anyhow::{Context, Error, Result};
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use crate::config::get_config;
use crate::errors::Errors;

pub mod common;
pub mod full;
pub mod standard;

/// Process a directory containing adoc files and generate the corresponding llmstxt
/// based on the options provided.
///
/// # Arguments
///
/// * `dir` - The directory containing the adoc files.
/// * `full` - Whether to generate the full llmstxt file.
///
/// # Returns
///
/// * The generated llmstxt file.
pub async fn generate_from_dir(
    dir: &Path,
    full: bool,
    library_version: Option<String>,
) -> Result<String> {
    let config = get_config(dir)?;
    let nav_file = find_nav_file(dir)?;

    Ok(if !full {
        standard::generate_llmstxt(dir, &nav_file, &config, library_version)?
    } else {
        full::generate_full_llmstxt(dir, &nav_file, &config)?
    })
}

fn find_nav_file(dir: &Path) -> Result<PathBuf> {
    let files = std::fs::read_dir(dir).context(Errors::ReadDir(dir.to_path_buf()))?;
    for file in files {
        let file = file.context(Errors::ReadFile(dir.to_path_buf()))?;
        let path = file.path();

        if path.file_name() == Some(OsStr::new("nav.adoc")) {
            return Ok(path);
        }
    }
    Err(Error::msg(Errors::FindNavFile(dir.to_path_buf())))
}
