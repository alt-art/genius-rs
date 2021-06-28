//! # genius_rs
//!
//!  Rust library that allows interact with Genius API (Under development)
//!
//! ## Searching for a Song
//!
//! ```rust
//! use genius_rs;
//!
//! fn main() {
//!     let genius = Genius::new("#TOKEN#");
//!     let result = genius.search("Ariana Grande").unwrap();
//!     println!("{}", result.response.hits[0].result.full_title);
//! }
//! ```

use reqwest::Client;
use serde::{Deserialize};

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv;

    #[test]
    fn search_test() {
        dotenv::dotenv().expect("Can't load dot env file");
        let genius = Genius::new(dotenv::var("TOKEN").unwrap());
        let result = genius.search("Ariana Grande").unwrap();
        println!("{}", result.response.hits[0].result.full_title);
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

    #[tokio::main]
    /// Search for a song in Genius the result will be genius_rs::search::SearchResponse
    pub async fn search(self, q: &str) -> Result<SearchResponse, reqwest::Error> {
        let res = &self.reqwest.get(format!("{}{}{}", URL, "search?q=", q))
        .header("Authorization", self.token).send().await?.text().await?;
        let result: SearchResponse = serde_json::from_str(&res.as_str()).unwrap();
        Ok(result)
    }
}


#[derive(Deserialize, Debug)]
pub struct SearchResponse {
    pub meta: Meta,
    pub response: Hits
}

#[derive(Deserialize, Debug)]
pub struct Meta {
    pub status: u32
}

#[derive(Deserialize, Debug)]
pub struct Hits {
    pub hits: Vec<Hit>
}

#[derive(Deserialize, Debug)]
pub struct Hit {
    pub highlights: [String;0],
    pub index: String,
    pub r#type: String,
    pub result: SongSearch
}

#[derive(Deserialize, Debug)]
pub struct SongSearch {
    pub annotation_count: u32,
    pub api_path: String,
    pub full_title: String,
    pub header_image_thumbnail_url: String,
    pub header_image_url: String,
    pub id: u32,
    pub lyrics_owner_id: u32,
    pub lyrics_state: String,
    pub path: String,
    pub pyongs_count: u32,
    pub song_art_image_thumbnail_url: String,
    pub song_art_image_url: String,
    pub song_art_primary_color: Option<String>,
    pub song_art_secondary_color: Option<String>,
    pub song_art_text_color: Option<String>,
    pub stats: SongStatus,
    pub title: String,
    pub title_with_featured: String,
    pub url: String,
    pub primary_artist: Artist
}

#[derive(Deserialize, Debug)]
pub struct SongStatus {
    pub unreviewed_annotations: u32,
    pub hot: bool,
    pub pageviews: u32
}

#[derive(Deserialize, Debug)]
pub struct Artist {
    pub api_path: String,
    pub header_image_url: String,
    pub id: u32,
    pub image_url: String,
    pub is_meme_verified: bool,
    pub is_verified: bool,
    pub name: String,
    pub url: String,
    pub iq: Option<u32>,
}