# genius_rs

Rust library that allows interact with Genius API.

##  Searching for a Song

```rust
use genius_rs::Genius;

#[tokio::main]
async fn main() {
    let genius = Genius::new(dotenv::var("TOKEN").unwrap());
    let result = genius.search("Ariana Grande").await.unwrap();
    println!("{}", result.response.hits[0].result.full_title);
}
```

## Getting lyrics

```rust
use genius_rs::Genius;

#[tokio::main]
async fn main() {
    let genius = Genius::new(dotenv::var("TOKEN").unwrap());
    let result = genius.search("Sia").await.unwrap();
    let lyrics = genius.get_lyrics(&result.response.hits[0].result.url).await.unwrap();
    for verse in lyrics {
        println!("{}", verse);
    }
}
```