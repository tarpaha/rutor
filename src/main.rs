use std::collections::HashMap;
use std::error::Error;

mod loader;
mod parser;
use parser::Torrent;

use loader::{Source, load};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    //let content = load(Source::File("page.html".to_string())).await?;
    let content = load(Source::Url("https://rutor.info/browse/0/1/0/2".to_string())).await?;
    let torrents = parser::parse_torrents(&content)?;

    let mut by_name: HashMap<String, Torrent> = HashMap::new();
    for torrent in &torrents {
        let title_before_slash = torrent
            .title
            .split('/')
            .next()
            .unwrap_or("")
            .trim()
            .to_string();

        by_name
            .entry(title_before_slash.to_string())
            .and_modify(|existing| {
                if torrent.seeders > existing.seeders {
                    *existing = torrent.clone();
                }
            })
            .or_insert_with(|| torrent.clone());
    }

    let mut filtered_torrents: Vec<Torrent> = by_name.into_values().collect();
    filtered_torrents.sort_by(|a, b| b.seeders.cmp(&a.seeders));
    for f in filtered_torrents {
        println!("{}: {}", f.seeders, f.title);
    }

    Ok(())
}