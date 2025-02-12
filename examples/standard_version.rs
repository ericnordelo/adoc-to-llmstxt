use anyhow::Result;
use oz_llmstxt::generate_from_dir;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<()> {
    let path = PathBuf::from("./examples/cairo/v0.20.0");
    let llmstxt = generate_from_dir(&path, false, None).await?;
    println!("{}", llmstxt);
    Ok(())
}
