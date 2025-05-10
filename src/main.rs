use std::collections::HashMap;
use std::error::Error;
use itertools::Itertools;
use tokio::sync::mpsc;

mod loader;
mod index_parser;
mod desc_parser;

use index_parser::Torrent;

use loader::{Source, load};

const N: usize = 5;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    //let content = load(Source::File("page.html".to_string())).await?;
    let content = load(Source::Url("https://rutor.info/browse/0/1/0/2".to_string())).await?;
    let torrents = index_parser::parse(&content)?;

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

    let best_torrents: Vec<Torrent> = best_seeders_dict
        .into_values()
        .sorted_by(|a, b| b.seeders.cmp(&a.seeders))
        .take(N)
        .collect();

    let (tx, mut rx) = mpsc::channel::<(usize, String)>(N);

    for idx in 0..best_torrents.len() {
        let tx = tx.clone();
        let url = best_torrents[idx].url.clone();
        tokio::spawn(async move {
            let description = get_torrent_description(&url).await;
            let _ = tx
                .send((idx, description))
                .await;
        });
    }
    drop(tx);

    let mut buffer: Vec<Option<String>> = vec![None; N];

    let mut next_to_print = 0;
    while let Some((idx, res)) = rx.recv().await {
        buffer[idx] = Some(res);

        while next_to_print < N {
            if let Some(description) = buffer[next_to_print].take() {
                print_torrent(&best_torrents[next_to_print], &description);
                next_to_print += 1;
            } else {
                break;
            }
        }
    }

    Ok(())
}

async fn get_torrent_description(url: &String) -> String {
    let url = format!("https://rutor.info{}", url);
    let content = load(Source::Url(url.clone())).await.unwrap();
    desc_parser::parse(&content).unwrap_or_else(|| "No description found".to_string())
}

fn print_torrent(torrent: &Torrent, description: &String) {
    println!(
        "{}: \x1B]8;;https://rutor.info{}\x07{}\x1B]8;;\x07",
        torrent.seeders, torrent.url, torrent.title);
    println!(
        "\x1B[38;5;245m    {}\x1B[0m",
        textwrap::fill(
            description,
            textwrap::Options::new(80).subsequent_indent("    ")));
    println!()
}