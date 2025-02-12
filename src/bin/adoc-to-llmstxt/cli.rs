use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "adoc-to-llmstxt")]
#[command(version, about, long_about = None)]
pub struct Cli {
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

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}
