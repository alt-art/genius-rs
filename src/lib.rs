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

impl Genius {
    pub fn new(token: String) -> Self {
        Self {
            reqwest: reqwest::Client::new(),
            token: format!("Bearer {}", token)
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