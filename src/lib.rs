use anyhow::{Context, Error, Result};
use indoc::formatdoc;
use std::ffi::OsStr;
use std::path::PathBuf;

pub mod errors;
use errors::Errors;

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
    let nav_file = find_nav_file(dir)?;

    Ok(if !full {
        generate_llmstxt(nav_file)?
    } else {
        generate_full_llmstxt(nav_file)?
    })
}

fn find_nav_file(dir: PathBuf) -> Result<PathBuf> {
    let files = std::fs::read_dir(dir).context(Errors::FailedToReadDir)?;
    for file in files {
        let file = file.context(Errors::FailedToReadFile)?;
        let path = file.path();

        if path.file_name() == Some(OsStr::new("nav.adoc")) {
            return Ok(path);
        }
    }
    Err(Error::msg(Errors::FailedToFindNavFile))
}

fn generate_llmstxt(nav_file: PathBuf) -> Result<String> {
    let mut llmstxt = String::new();

    // TODO: Get the title and descriptions from the directory
    let title = "OpenZeppelin Contracts for Cairo";
    let description = "OpenZeppelin Contracts written in Cairo for Starknet, a decentralized ZK Rollup.";
    let long_description = "A library for secure smart contract development written in Cairo for Starknet. This library consists of a set of reusable components to build custom smart contracts, as well as ready-to-deploy presets. You can also find other utilities including interfaces and dispatchers and test utilities that facilitate testing with Starknet Foundry.";

    // Add the title, description and long description to the llmstxt
    llmstxt.push_str(&formatdoc! {
      "
      # {title}

      > {description}

      {long_description}
      "
    });

    let nav_file_content = std::fs::read_to_string(nav_file).context(Errors::FailedToReadFile)?;
    let lines = nav_file_content.lines();
    for line in lines {
        if line.starts_with("*") {
            // llmstxt.push_str(line);
        }
    }

    Ok(llmstxt)
}

fn generate_full_llmstxt(_nav_file: PathBuf) -> Result<String> {
    let llmstxt = "TODO: Implement the full llmstxt generation".to_string();
    Ok(llmstxt)
}
