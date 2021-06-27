# genius_rs

Rust library that allows interact with Genius API (Under development).

##  Searching for a Song

```rust
use genius_rs;

fn main() {
    let genius = Genius::new("#TOKEN#");
    let result = genius.search("Ariana Grande").unwrap();
    println!("{}", result.response.hits[0].result.full_title);
}
```