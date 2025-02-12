use crate::commands::Adoc;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "oz-llmstxt")]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[clap(about = "Generate a llmstxt file from a directory of adoc files")]
    Adoc(Adoc),
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}
