use anyhow::{Context, Result};
use regex::Regex;
use std::path::Path;

use crate::config::Config;
use crate::errors::Errors;
use crate::generate::common::add_title_and_descriptions;

pub(crate) fn generate_full_llmstxt(
    dir: &Path,
    nav_file: &Path,
    config: &Config,
) -> Result<String> {
    let mut llmstxt = String::new();

    // Add the title, description and long description to the llmstxt file
    add_title_and_descriptions(&mut llmstxt, config);

    let nav_file_content =
        std::fs::read_to_string(nav_file).context(Errors::ReadFile(nav_file.to_path_buf()))?;

    let lines = nav_file_content.lines();
    let regex = Regex::new(r"^\*+ xref:\/?(?<path>.*)\.adoc\[(?<title>.*)\]$").unwrap();

    for line in lines {
        if regex.is_match(line) {
            let captures = regex.captures(line).unwrap();
            let path = captures.get(1).unwrap().as_str().to_string();

            // Ignore pages in other antora submodules
            if path.contains("::") {
                continue;
            }

            let page_path = dir.join("pages").join(path).with_extension("adoc");
            llmstxt.push_str(&process_page(&page_path)?);

            // TODO: Implement the full llmstxt generation
        }
    }

    Ok(llmstxt)
}

fn process_page(page_path: &Path) -> Result<String> {
    let page_content = std::fs::read_to_string(page_path).context(Errors::ReadFile(page_path.to_path_buf()))?;

    Ok(page_content)
}
