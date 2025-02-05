use adoc_to_llmstxt::generate_from_dir;
use anyhow::{bail, Result};
use clap::Parser;

mod cli;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = cli::Cli::parse();

    match cli.dir {
        Some(dir) => {
            let llmstxt = generate_from_dir(dir, cli.full).await?;
            println!("{}", llmstxt);
        }
        None => {
            bail!("No directory provided");
        }
    }

    Ok(())
}
