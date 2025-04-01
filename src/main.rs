use std::error::Error;

mod loader;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let content = loader::load().await?;
    println!("file size: {} bytes", content.len());
    Ok(())
}
