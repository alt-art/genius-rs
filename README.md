# genius_rs

Rust library that allows interact with Genius API.

##  Searching for a Song

```rust
use genius_rs::Genius;

fn main() {
    let genius = Genius::new("#TOKEN#");
    let result = genius.search("Ariana Grande").unwrap();
    println!("{}", result.response.hits[0].result.full_title);
}
```

## Getting lyrics

```rust
use genius_rs::Genius;

fn main() {
    let genius = Genius::new("#TOKEN#");
    let result = genius.search("Sia").unwrap();
    let lyrics = genius.get_lyrics(&result.response.hits[0].result.url).unwrap();
    for verse in lyrics {
        println!("{}", verse);
    }
}
```