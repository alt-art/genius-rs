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

pub mod search;
use search::SearchResponse;

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