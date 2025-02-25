use anyhow::{Context, Result};
use regex::Regex;
use std::path::Path;

use crate::config::Config;
use crate::errors::Errors;
use crate::generate::common::add_title_and_descriptions;
use crate::link::Link;

pub(crate) fn generate_llmstxt(
    dir: &Path,
    nav_file: &Path,
    config: &Config,
    library_version: Option<String>,
) -> Result<String> {
    let mut llmstxt = String::new();
    let library_version = library_version.unwrap_or("".to_string());

    // Add the title, description and long description to the llmstxt file
    add_title_and_descriptions(&mut llmstxt, config);

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
            }

            let link = Link::new(dir, &title, &path, &library_version, &config.base_url)?;
            if path.contains("api/") {
                api_links.push(link);
            } else {
                doc_links.push(link);
            }
        }
    }

    add_links_section(&mut llmstxt, doc_links, "Documentation");
    add_links_section(&mut llmstxt, api_links, "API Reference");

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
