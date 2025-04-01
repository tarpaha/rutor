use std::error::Error;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[allow(dead_code)]
pub enum Source {
    Url(String),
    File(String)
}

pub async fn load(source: Source) -> Result<String, Box<dyn Error>> {
    match source {
        Source::Url(url) => {
            let content = reqwest::get(url).await?.text().await?;
            Ok(content)
        }
        Source::File(path) => {
            let mut file = File::open(path).await?;
            let mut content = String::new();
            file.read_to_string(&mut content).await?;
            Ok(content)
        }
    }
}