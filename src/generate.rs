use anyhow::{Context, Error, Result};
use indoc::formatdoc;
use regex::Regex;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use crate::config::{get_config, Config};
use crate::errors::Errors;
use crate::link::Link;

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
pub async fn generate_from_dir(dir: PathBuf, full: bool) -> Result<String> {
    let config = get_config(&dir)?;
    let nav_file = find_nav_file(&dir)?;

    Ok(if !full {
        generate_llmstxt(&dir, &nav_file, &config)?
    } else {
        generate_full_llmstxt(&dir, &nav_file)?
    })
}

fn generate_llmstxt(dir: &Path, nav_file: &Path, config: &Config) -> Result<String> {
    let mut llmstxt = String::new();

    let title = &config.title;
    let description = &config.description;
    let long_description = &config.long_description;

    // Add the title, description and long description to the llmstxt file
    llmstxt.push_str(&formatdoc! {
      "
    # {title}

    > {description}

    {long_description}
    "
    });

    let nav_file_content =
        std::fs::read_to_string(nav_file).context(Errors::ReadFile(nav_file.to_path_buf()))?;

    let lines = nav_file_content.lines();
    let mut doc_links: Vec<Link> = vec![];
    let mut api_links: Vec<Link> = vec![];
    let regex = Regex::new(r"^\*+ xref:\/?(?<path>.*)\.adoc\[(?<title>.*)\]$").unwrap();

    for line in lines {
        if regex.is_match(line) {
            let captures = regex.captures(line).unwrap();
            let path = captures.get(1).unwrap().as_str().to_string();
            let title = captures.get(2).unwrap().as_str().to_string();

            // Ignore pages in other antora submodules
            if path.contains("::") {
                continue;
            } else if path.contains("api/") {
                api_links.push(Link::new(dir, &title, &path)?);
            } else {
                doc_links.push(Link::new(dir, &title, &path)?);
            }
        }
    }

    add_links_section(&mut llmstxt, doc_links, "Documentation");
    add_links_section(&mut llmstxt, api_links, "API Reference");

    Ok(llmstxt)
}

fn generate_full_llmstxt(_dir: &Path, _nav_file: &Path) -> Result<String> {
    let llmstxt = "TODO: Implement the full llmstxt generation".to_string();
    Ok(llmstxt)
}

fn add_links_section(llmstxt: &mut String, links: Vec<Link>, section_name: &str) {
    if !links.is_empty() {
        llmstxt.push_str(&format!("\n## {section_name}\n\n"));

        for link in links {
            if let Some(details) = link.details {
                llmstxt.push_str(&format!("[{}]({}): {details}\n", link.title, link.url));
            } else {
                llmstxt.push_str(&format!("[{}]({})\n", link.title, link.url));
            }
        }
    }
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
