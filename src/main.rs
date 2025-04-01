use std::error::Error;

mod loader;
use loader::{Source, load};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let content = load(Source::File("page.html".to_string())).await?;
    println!("file size: {} bytes", content.len());
    Ok(())
}
