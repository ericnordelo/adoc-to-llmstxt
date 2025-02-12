use crate::errors::Errors;
use anyhow::{Context, Result};
use regex::Regex;
use std::path::Path;

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
    /// NOTE: If the title is "API Reference", the first header of the adoc page
    /// will prepended to final title (i.e. "Account API Reference").
    ///
    /// # Requirements
    ///
    /// - `adoc_path` must be a valid path to an adoc file, and MUST NOT start with a `/`.
    pub fn new(
        dir: &Path,
        title: &str,
        adoc_path: &str,
        library_version: &str,
        base_url: &str,
    ) -> Result<Self> {
        let base_url = base_url.trim_end_matches('/');
        let url = if library_version.is_empty() {
            format!("{base_url}/{adoc_path}")
        } else {
            format!("{base_url}/{library_version}/{adoc_path}")
        };
        let file_content = get_file_content(dir, adoc_path)?;
        let title = process_title(&file_content, title);
        let details = get_details(dir, adoc_path)?;
        Ok(Self {
            title,
            url,
            details,
        })
    }
}

fn get_file_content(dir: &Path, adoc_path: &str) -> Result<String> {
    let mut file_path = dir.join("pages").join(adoc_path);
    file_path.set_extension("adoc");

    std::fs::read_to_string(&file_path).context(Errors::ReadFile(file_path))
}

/// In the case of API Reference pages, the title is the first header of the file
/// concatenated with "API Reference" (i.e. "ERC1155 API Reference").
fn process_title(file_content: &str, title: &str) -> String {
    if title == "API Reference" {
        let regex = Regex::new(r"^= .*$").unwrap();

        for line in file_content.lines() {
            if regex.is_match(line) {
                return line[2..].to_string() + " API Reference";
            }
        }
    }
    title.to_string()
}

fn get_details(dir: &Path, adoc_path: &str) -> Result<Option<String>> {
    let file_content = get_file_content(dir, adoc_path)?;
    let first_line = file_content.lines().next().unwrap_or("");
    let regex = Regex::new(r"^\/\/ llmstxt-short-description: (.*)$").unwrap();
    let captures = regex.captures(first_line);

    if captures.is_some() {
        let details = captures.unwrap().get(1).unwrap().as_str().to_string();
        return Ok(Some(details));
    }

    Ok(None)
}
