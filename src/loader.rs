use async_trait::async_trait;
use std::error::Error;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[async_trait]
trait HtmlLoader {
    async fn load_html(&self) -> Result<String, Box<dyn Error>>;
}

#[allow(dead_code)]
struct UrlHtmlLoader {
    url: String
}

#[async_trait]
impl HtmlLoader for UrlHtmlLoader {
    async fn load_html(&self) -> Result<String, Box<dyn Error>> {
        let content = reqwest::get(&self.url).await?.text().await?;
        Ok(content)
    }
}

struct FileHtmlLoader {
    path: String
}

#[async_trait]
impl HtmlLoader for FileHtmlLoader {
    async fn load_html(&self) -> Result<String, Box<dyn Error>> {
        let mut file = File::open(&self.path).await?;
        let mut content = String::new();
        file.read_to_string(&mut content).await?;
        Ok(content)
    }
}

pub async fn load() -> Result<String, Box<dyn Error>> {
    FileHtmlLoader {  path: "page.html".to_string() }.load_html().await
}