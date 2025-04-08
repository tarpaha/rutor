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
            let response = reqwest::get(&url)
                .await
                .map_err(|e| format!("Failed to get response from '{}': {}", &url, e))?
                .error_for_status()
                .map_err(|e| format!("Server returned an error for '{}': {}", &url, e))?;
            let content = response.text()
                .await
                .map_err(|e| format!("Failed to load data from'{}': {}", &url, e))?;
            Ok(content)
        }
        Source::File(path) => {
            let mut file = File::open(&path)
                .await
                .map_err(|e| format!("Could not open file '{}': {}", &path, e))?;
            let mut content = String::new();
            file.read_to_string(&mut content)
                .await
                .map_err(|e| format!("Could not read file '{}': {}", &path, e))?;
            Ok(content)
        }
    }
}