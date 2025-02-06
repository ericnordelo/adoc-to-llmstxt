use crate::errors::Errors;
use anyhow::{Context, Result};
use regex::Regex;
use std::path::Path;

const BASE_URL: &str = "https://docs.openzeppelin.com/contracts-cairo/";

/// A link to a page in the documentation.
///
/// This is used to generate the llmstxt standard file sections.
pub struct Link {
    pub title: String,
    pub url: String,
    pub details: Option<String>,
}

impl Link {
    /// Create a new link from an adoc path and title.
    ///
    /// # Requirements
    ///
    /// - `adoc_path` must be a valid path to an adoc file, and MUST NOT start with a `/`.
    pub fn new(dir: &Path, title: String, adoc_path: String) -> Result<Self> {
        let url = format!("{BASE_URL}{}", adoc_path);
        let details = try_get_details(dir, adoc_path)?;
        Ok(Self {
            title,
            url,
            details,
        })
    }

    /// Create a new link from an adoc path and title with details.
    ///
    /// # Requirements
    ///
    /// - `adoc_path` must be a valid path to an adoc file, and MUST NOT start with a `/`.
    pub fn _new_with_details(title: String, adoc_path: String, details: String) -> Self {
        let url = format!("{BASE_URL}{}", adoc_path);
        Self {
            title,
            url,
            details: Some(details),
        }
    }
}

fn try_get_details(dir: &Path, adoc_path: String) -> Result<Option<String>> {
    let mut file_path = dir.join("pages").join(adoc_path);
    file_path.set_extension("adoc");

    let file_content = std::fs::read_to_string(&file_path).context(Errors::ReadFile(file_path))?;
    let first_line = file_content.lines().next().unwrap_or("");
    let regex = Regex::new(r"^\/\/ llmstxt-short-description: (.*)$").unwrap();
    let captures = regex.captures(first_line);

    if captures.is_some() {
        let details = captures.unwrap().get(1).unwrap().as_str().to_string();
        return Ok(Some(details));
    }

    Ok(None)
}
