# genius-rs

Rust library that allows interact with Genius API (Under development)

## Usage
```rust
use genius-rs;

fn main() {
    let genius = Genius::new("#TOKEN#");
    let result = genius.search("Ariana Grande").unwrap();
    println!("{}", result.response.hits[0].result.full_title);
}
```