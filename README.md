![Crates.io](https://img.shields.io/crates/v/genius-rs?color=%2300aa00&style=flat-square) ![Crates.io (recent)](https://img.shields.io/crates/dr/genius-rs?style=flat-square) ![Crates.io](https://img.shields.io/crates/l/genius-rs?style=flat-square)

# genius_rs

Rust library that allows interact with Genius API.

> [!WARNING]
> This library is not maintained anymore due to my lack of interest in genius and my change of focus in development to developing “[commit](https://github.com/alt-art/commit)”

##  Searching for a song

```rust
use genius_rs::Genius;

#[tokio::main]
async fn main() {
    let genius = Genius::new(dotenv::var("TOKEN").unwrap());
    let response = genius.search("Ariana Grande").await.unwrap();
    println!("{}", response[0].result.full_title);
}
```

## Getting lyrics

```rust
use genius_rs::Genius;

#[tokio::main]
async fn main() {
    let genius = Genius::new(dotenv::var("TOKEN").unwrap());
    let response = genius.search("Sia").await.unwrap();
    let lyrics = genius.get_lyrics(&response[0].result.id).await.unwrap();
    for verse in lyrics {
        println!("{}", verse);
    }
}
```

## Getting deeper information for a song by id

```rust
use genius_rs::Genius;

#[tokio::main]
async fn main() {
    let genius = Genius::new(dotenv::var("TOKEN").unwrap());
    let response = genius.search("Weeknd").await.unwrap();
    let song = genius.get_song(response[0].result.id, "plain").await.unwrap();
    println!("{}", song.media.unwrap()[0].url)
}
```
