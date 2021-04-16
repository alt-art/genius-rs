use reqwest;
use dotenv;

mod search_response;

use search_response::SearchResponse;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_test() {
        let genius = Genius::new();
        genius.search("Ariana Grande").unwrap();
    }
}

const URL:&str = "https://api.genius.com/";

pub struct Genius {
    reqwest: reqwest::Client,
    token: String
}

impl Genius {
    pub fn new() -> Self {
        dotenv::dotenv().expect("Can't load dot env file");
        Self {
            reqwest: reqwest::Client::new(),
            token: format!("Bearer {}", dotenv::var("TOKEN").unwrap())
        }
    }

    #[tokio::main]
    pub async fn search(self, q: &str) -> Result<SearchResponse, reqwest::Error> {
    let res = &self.reqwest.get(format!("{}{}{}", URL, "search?q=", q))
    .header("Authorization", self.token).send().await?.text().await?;
    let result: SearchResponse = serde_json::from_str(&res.as_str()).unwrap();
    Ok(result)
    }
}