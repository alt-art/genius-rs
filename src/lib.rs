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

/// Album response
pub mod album;
/// Annotation response
pub mod annotation;
/// Authentication methods
pub mod auth;
/// Search response
pub mod search;
/// Song response
pub mod song;
/// User response
pub mod user;

use album::Album;
use regex::Regex;
use reqwest::Client;
use scraper::{Html, Selector};
use search::Hit;
use serde::Deserialize;
use song::Song;

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv;
    use std::env;

    #[tokio::test]
    async fn search_test() {
        let token = dotenv::var("TOKEN").unwrap_or_else(|_| {env::var("TOKEN").unwrap().to_string()});
        let genius = Genius::new(token);
        let result = genius.search("Ariana Grande").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn get_lyrics_test() {
        dotenv::dotenv().expect("Can't load dot env file");
        let genius = Genius::new(dotenv::var("TOKEN").unwrap());
        let lyrics = genius
            .get_lyrics("https://genius.com/Lsd-thunderclouds-lyrics")
            .await
            .unwrap();
        for verse in lyrics {
            println!("{}", verse);
        }
    }

    #[tokio::test]
    async fn get_song_test() {
        dotenv::dotenv().expect("Can't load dot env file");
        let genius = Genius::new(dotenv::var("TOKEN").unwrap());
        genius.get_song(378195, "plain").await.unwrap();
    }

    #[tokio::test]
    async fn get_album_test() {
        dotenv::dotenv().expect("Can't load dot env file");
        let genius = Genius::new(dotenv::var("TOKEN").unwrap());
        genius.get_album(27501, "plain").await.unwrap();
    }
}

const URL: &str = "https://api.genius.com";

/// The main hub for interacting with the Genius API
pub struct Genius {
    reqwest: Client,
    token: String,
}

impl Genius {
    /// Create an API Client at <https://genius.com/developers> and get the token to get basic Genius API access. The token will be level client.
    pub fn new(token: String) -> Self {
        Self {
            reqwest: Client::new(),
            token,
        }
    }

    /// Search for a song in Genius the result will be [`search::Hit`]
    pub async fn search(&self, q: &str) -> Result<Vec<Hit>, reqwest::Error> {
        let request = self
            .reqwest
            .get(format!("{}/search?q={}", URL, q))
            .bearer_auth(&self.token)
            .send()
            .await?;
        let res = request.json::<Response>().await?;
        Ok(res.response.hits.unwrap())
    }

    /// Get lyrics with an url of genius song like: <https://genius.com/Sia-chandelier-lyrics>
    pub async fn get_lyrics(&self, url: &str) -> Result<Vec<String>, reqwest::Error> {
        let res = self
            .reqwest
            .get(url)
            .header("Cookie", "_genius_ab_test_cohort=33")
            .send()
            .await?
            .text()
            .await?;
        let regex_italic = Regex::new("</*i>").unwrap();
        let html = regex_italic.replace_all(&res, "");
        let document = Html::parse_document(&html);
        let lyrics_selector = Selector::parse("div.Lyrics__Container-sc-1ynbvzw-8").unwrap();
        let lyrics = document
            .select(&lyrics_selector)
            // Now we iterate over each element that matches the lyrics selector...
            .map(|elem| elem.text())
            // Now, we flatten the iterator over iterators over &strs into an iterator over &strs...
            .flatten()
            // ... map the &strs into owned Strings...
            .map(ToString::to_string)
            // ... and collect them into a vector of strings.
            .collect();
        Ok(lyrics)
    }

    /// Get deeper information from a song by it's id, `text_format` is the field for the format of text bodies related to the document. Available text formats are `plain` and `html`
    pub async fn get_song(&self, id: u32, text_format: &str) -> Result<Song, reqwest::Error> {
        let request = self
            .reqwest
            .get(format!("{}/songs/{}?text_format={}", URL, id, text_format))
            .bearer_auth(&self.token)
            .send()
            .await?;
        let res = request.json::<Response>().await?;
        Ok(res.response.song.unwrap())
    }
    /// Get deeper information from a album by it's id, `text_format` is the field for the format of text bodies related to the document. Available text formats are `plain` and `html`
    pub async fn get_album(&self, id: u32, text_format: &str) -> Result<Album, reqwest::Error> {
        let request = self
            .reqwest
            .get(format!("{}/albums/{}?text_format={}", URL, id, text_format))
            .bearer_auth(&self.token)
            .send()
            .await?;
        let res = request.json::<Response>().await?;
        Ok(res.response.album.unwrap())
    }
}

#[derive(Deserialize, Debug)]
pub struct Body {
    pub plain: Option<String>,
    pub html: Option<String>,
}

#[derive(Deserialize, Debug)]
struct Meta {
    status: u32,
    message: Option<String>,
}

#[derive(Deserialize, Debug)]
struct Response {
    meta: Meta,
    response: BlobResponse,
}

#[derive(Deserialize, Debug)]
struct BlobResponse {
    song: Option<Song>,
    hits: Option<Vec<Hit>>,
    album: Option<Album>,
}
