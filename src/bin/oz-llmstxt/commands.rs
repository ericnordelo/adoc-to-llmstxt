mod adoc;
pub use adoc::Adoc;

use anyhow::Result;
use async_trait::async_trait;

/// Common trait for Cli commands
#[async_trait]
pub trait CliCommand {
    async fn run(&self) -> Result<()>;
}
