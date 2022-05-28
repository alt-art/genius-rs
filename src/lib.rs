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
use reqwest::Client;
use search::Hit;
use serde::Deserialize;
use song::Song;

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv;

    #[tokio::test]
    async fn search_test() {
        let genius = Genius::new(dotenv::var("TOKEN").unwrap());
        let result = genius.search("Ariana Grande").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn get_lyrics_test() {
        let genius = Genius::new(dotenv::var("TOKEN").unwrap());
        let lyrics = genius
            .get_lyrics(1)
            .await
            .unwrap();
        for verse in lyrics {
            println!("{}", verse);
        }
    }

    #[tokio::test]
    async fn get_song_test() {
        let genius = Genius::new(dotenv::var("TOKEN").unwrap());
        genius.get_song(378195, "plain").await.unwrap();
    }

    #[tokio::test]
    async fn get_album_test() {
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
    pub async fn get_lyrics(&self, id: u32) -> Result<Vec<String>, reqwest::Error> {
        let request = self
            .reqwest
            .get(format!("https://lyrics.altart.tk/api/lyrics/{}", id))
            .send()
            .await?;
        let res = request.json::<Body>().await?;
        let lyrics = res
            .plain
            .unwrap()
            .split("\n")
            .map(String::from)
            .collect::<Vec<String>>();
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
pub struct Date {
    pub year: Option<u32>,
    pub month: Option<u32>,
    pub day: Option<u32>,
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
