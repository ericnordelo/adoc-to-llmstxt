use adoc_to_llmstxt::generate_from_dir;
use anyhow::Result;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<()> {
    let path = PathBuf::from("./examples/directories/0.20.0");
    let llmstxt = generate_from_dir(path, false, None).await?;
    println!("{}", llmstxt);
    Ok(())
}
