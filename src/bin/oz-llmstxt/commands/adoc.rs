use super::CliCommand;
use anyhow::{bail, Result};
use async_trait::async_trait;
use clap::Parser;
use oz_llmstxt::generate_from_dir;
use std::path::PathBuf;

#[derive(Parser)]
pub struct Adoc {
    /// Path to the directory containing the adoc files
    #[arg(short, long, value_name = "DIR")]
    pub dir: Option<PathBuf>,

    /// Library version
    #[arg(short = 'v', long, value_name = "VERSION")]
    pub library_version: Option<String>,

    /// Activate the llms-full.txt mode
    #[arg(short, long)]
    pub full: bool,
}

#[async_trait]
impl CliCommand for Adoc {
    // Generate a llmstxt file from a directory of adoc files
    async fn run(&self) -> Result<()> {
        match &self.dir {
            Some(dir) => {
                let llmstxt =
                    generate_from_dir(dir, self.full, self.library_version.clone()).await?;
                println!("{}", llmstxt);
            }
            None => {
                bail!("No directory provided");
            }
        }

        Ok(())
    }
}
