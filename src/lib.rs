//! # genius_rs
//!
//!  Rust library that allows interact with Genius API.
//!
//!  Create an API Client at <https://genius.com/developers> and get the token to get Genius API access.
//! ## Searching for a Song
//!
//! ```rust
//! use genius_rs::Genius;
//!
//! #[tokio::main]
//! async fn main() {
//!     let genius = Genius::new(dotenv::var("TOKEN").unwrap());
//!     let response = genius.search("Ariana Grande").await.unwrap();
//!     println!("{}", response[0].result.full_title);
//! }
//! ```
//!
//! ## Getting lyrics
//!
//! ```rust
//! use genius_rs::Genius;
//!
//! #[tokio::main]
//! async fn main() {
//!     let genius = Genius::new(dotenv::var("TOKEN").unwrap());
//!     let response = genius.search("Sia").await.unwrap();
//!     let lyrics = genius.get_lyrics(&response[0].result.url).await.unwrap();
//!     for verse in lyrics {
//!         println!("{}", verse);
//!     }
//! }
//! ```
//! 
//! ## Getting deeper information for a song by id
//!
//! ```rust
//! use genius_rs::Genius;
//!
//! #[tokio::main]
//! async fn main() {
//!     let genius = Genius::new(dotenv::var("TOKEN").unwrap());
//!     let response = genius.search("Weeknd").await.unwrap();
//!     let song = genius.get_song(response[0].result.id, "plain").await.unwrap();
//!     println!("{}", song.media.unwrap()[0].url)
//! }
//! ```

pub mod search;
pub mod song;
pub mod user;
pub mod annotation;

use song::Song;
use search::Hit;
use reqwest::Client;
use serde::Deserialize;
use scraper::{Html, Selector};

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv;

    #[tokio::test]
    async fn search_test() {
        dotenv::dotenv().expect("Can't load dot env file");
        let genius = Genius::new(dotenv::var("TOKEN").unwrap());
        let result = genius.search("Ariana Grande").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn get_lyrics_test() {
        dotenv::dotenv().expect("Can't load dot env file");
        let genius = Genius::new(dotenv::var("TOKEN").unwrap());
        let lyrics = genius.get_lyrics("https://genius.com/Sia-chandelier-lyrics").await;
        assert!(lyrics.is_ok());
    }

    #[tokio::test]
    async fn get_song_test() {
        dotenv::dotenv().expect("Can't load dot env file");
        let genius = Genius::new(dotenv::var("TOKEN").unwrap());
        let result = genius.get_song(378195, "plain").await;
        assert!(result.is_ok())
    }
}

const URL:&str = "https://api.genius.com/";

pub struct Genius {
    reqwest: Client,
    token: String
}

/// The main hub for interacting with the Genius API
impl Genius {
    /// Create an API Client at <https://genius.com/developers> and get the token to get Genius API access
    pub fn new(token: String) -> Self {
        Self {
            reqwest: reqwest::Client::new(),
            token: format!("Bearer {}", token)
        }
    }

    /// Search for a song in Genius the result will be [`search::Hit`]
    pub async fn search(&self, q: &str) -> Result<Vec<Hit>, reqwest::Error> {
        let res = &self.reqwest.get(format!("{}{}{}", URL, "search?q=", q))
        .header("Authorization", self.token.as_str()).send().await?.text().await?;
        let result: SearchResponse = serde_json::from_str(&res.as_str()).unwrap();
        Ok(result.response.hits)
    }

    /// Get lyrics with an url of genius song like: <https://genius.com/Sia-chandelier-lyrics>
    pub async fn get_lyrics(&self, url: &str) -> Result<Vec<String>, reqwest::Error> {
        let res = &self.reqwest.get(url).header("Cookie","_genius_ab_test_cohort=33").send().await?.text().await?;
        let document = Html::parse_document(res);
        let lyrics_selector = Selector::parse("div.Lyrics__Container-sc-1ynbvzw-8").unwrap();
        let mut lyrics = vec![];
        document.select(&lyrics_selector).for_each(|elem|
            elem.text().for_each(|text|
                lyrics.push(text.to_string())
            )
        );
        Ok(lyrics)
    }

    /// Get deeper information from a song by it's id, `text_format` is the field for the format of text bodies related to the document. Avaliabe text formats are `plain` and `html`
    pub async fn get_song(&self, id: u32, text_format: &str) -> Result<Song, reqwest::Error> {
        let res = &self.reqwest.get(format!("{}{}{}{}{}", URL, "songs/", id, "?text_format=", text_format))
        .header("Authorization", self.token.as_str()).send().await?.text().await?;
        let result: SongResponse = serde_json::from_str(&res.as_str()).unwrap();
        Ok(result.response.song)
    }
}

#[derive(Deserialize, Debug)]
pub struct Body {
    pub plain: Option<String>,
    pub html: Option<String>
}

#[derive(Deserialize, Debug)]
struct Meta {
    pub status: u32,
    pub message: Option<String>
}

#[derive(Deserialize, Debug)]
struct SearchResponse {
    pub meta: Meta,
    pub response: Hits
}

#[derive(Deserialize, Debug)]
struct Hits {
    pub hits: Vec<Hit>
}

#[derive(Deserialize, Debug)]
struct SongResponse {
    pub meta: Meta,
    pub response: SongGetter
}

#[derive(Deserialize, Debug)]
struct SongGetter {
    pub song: Song
}