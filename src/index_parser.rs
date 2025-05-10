use std::error::Error;
use scraper::{ElementRef, Html, Selector};

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Torrent {
    pub title: String,
    pub url: String,
    pub comments: usize,
    pub size: String,
    pub seeders: usize,
    pub leechers: usize,
    pub date: String
}

pub fn parse(html_content: &str) -> Result<Vec<Torrent>, Box<dyn Error>> {

    let document = Html::parse_document(html_content);

    let row_selector = Selector::parse("#index table tr")?;
    let td_selector = Selector::parse("td")?;
    let movie_selector = Selector::parse("a[href^=\"/torrent/\"]")?;

    let mut movies = Vec::new();

    for row in document.select(&row_selector) {

        let cells: Vec<_> = row.select(&td_selector).collect();
        if cells.len() != 5 {
            continue;
        }

        let date = text_from_element(&cells[0])?;

        let (title, url) = if let Some(movie_element) = cells[1].select(&movie_selector).next() {
            let name = movie_element.text().collect::<String>().trim().to_string();
            if name.is_empty() {
                return Err("Cannot parse movie name".into());
            }
            let url = movie_element.value().attr("href").ok_or("Cannot parse movie url")?.to_string();
            if url.is_empty() {
                return Err("Cannot parse movie url".into());
            }
            (name, url)
        } else {
            return Err("Cannot parse movie name and url".into());
        };

        let comments = text_from_element(&cells[2])?.parse::<usize>()?;
        let size = text_from_element(&cells[3])?;

        let numbers = text_from_element(&cells[4])?
            .split_whitespace()
            .map(|s| s.parse::<usize>())
            .collect::<Result<Vec<usize>, _>>()?;
        if numbers.len() != 2 {
            return Err("Cannot parse seeders and leechers".into());
        }

        let movie_torrent = Torrent {
            title,
            url,
            comments,
            size,
            seeders: numbers[0],
            leechers: numbers[1],
            date
        };

        movies.push(movie_torrent);
    }

    Ok(movies)
}

fn text_from_element(element: &ElementRef) -> Result<String, Box<dyn Error>> {
    let text = element.text().collect::<String>().trim().to_string();
    if text.is_empty() {
        Err("Cannot parse element".into())
    } else {
        Ok(text)
    }
}