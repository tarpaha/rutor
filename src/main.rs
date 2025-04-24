use std::collections::HashMap;
use std::error::Error;
use itertools::Itertools;


mod loader;
mod parser;
use parser::Torrent;

use loader::{Source, load};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    //let content = load(Source::File("page.html".to_string())).await?;
    let content = load(Source::Url("https://rutor.info/browse/0/1/0/2".to_string())).await?;
    let torrents = parser::parse_torrents(&content)?;

    let mut best_seeders_dict: HashMap<String, Torrent> = HashMap::new();
    for torrent in &torrents {
        let title_before_slash = torrent
            .title
            .split('/')
            .next()
            .unwrap_or("")
            .trim()
            .to_string();

        best_seeders_dict
            .entry(title_before_slash.to_string())
            .and_modify(|existing| {
                if torrent.seeders > existing.seeders {
                    *existing = torrent.clone();
                }
            })
            .or_insert_with(|| torrent.clone());
    }

    let best_torrents = best_seeders_dict
        .into_values()
        .sorted_by(|a, b| b.seeders.cmp(&a.seeders))
        .take(10);
    
    for torrent in best_torrents {
        print_torrent(&torrent);
    }

    Ok(())
}

fn print_torrent(torrent: &Torrent) {
    println!(
        "{}: \x1B]8;;https://rutor.info{}\x07{}\x1B]8;;\x07",
        torrent.seeders, torrent.url, torrent.title);
}