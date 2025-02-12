use anyhow::Result;
use clap::Parser;

mod cli;
mod commands;
use commands::CliCommand;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = cli::Cli::parse();
    match cli.command {
        cli::Commands::Adoc(cmd) => {
            cmd.run().await?;
        }
    };

    Ok(())
}
