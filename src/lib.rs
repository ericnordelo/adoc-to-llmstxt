use anyhow::{Context, Error, Result};
use indoc::formatdoc;
use regex::Regex;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

pub mod errors;
use errors::Errors;

mod link;
use link::Link;

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
    let nav_file = find_nav_file(&dir)?;

    Ok(if !full {
        generate_llmstxt(&dir, &nav_file)?
    } else {
        generate_full_llmstxt(&dir, &nav_file)?
    })
}

fn generate_llmstxt(dir: &Path, nav_file: &Path) -> Result<String> {
    let mut llmstxt = String::new();

    // TODO: Get the title and descriptions from the directory
    let title = "OpenZeppelin Contracts for Cairo";
    let description =
        "OpenZeppelin Contracts written in Cairo for Starknet, a decentralized ZK Rollup.";
    let long_description = "A library for secure smart contract development written in Cairo for Starknet. This library consists of a set of reusable components to build custom smart contracts, as well as ready-to-deploy presets. You can also find other utilities including interfaces and dispatchers and test utilities that facilitate testing with Starknet Foundry.";

    // Add the title, description and long description to the llmstxt file
    llmstxt.push_str(&formatdoc! {
      "
      # {title}

      > {description}

      {long_description}
      "
    });

    let nav_file_content = std::fs::read_to_string(nav_file).context(Errors::FailedToReadFile(nav_file.to_path_buf()))?;

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
                api_links.push(Link::new(dir, title, path)?);
            } else {
                doc_links.push(Link::new(dir, title, path)?);
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

fn find_nav_file(dir: &Path) -> Result<PathBuf> {
    let files = std::fs::read_dir(dir).context(Errors::FailedToReadDir(dir.to_path_buf()))?;
    for file in files {
        let file = file.context(Errors::FailedToReadFile(dir.to_path_buf()))?;
        let path = file.path();

        if path.file_name() == Some(OsStr::new("nav.adoc")) {
            return Ok(path);
        }
    }
    Err(Error::msg(Errors::FailedToFindNavFile(dir.to_path_buf())))
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
