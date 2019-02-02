# derpiboorust - Rust bindings for Derpibooru API 
This crate provides Rust bindings for [Derpibooru API](https://derpibooru.org/pages/api).

[![Travis CI Status](https://api.travis-ci.org/Ralvke/derpiboorust.svg?branch=master)](https://travis-ci.org/Ralvke/derpiboorust)
[![crates.io](https://img.shields.io/crates/v/derpiboorust.svg)](https://crates.io/crates/derpiboorust)
[![Documentation](https://docs.rs/derpiboorust/badge.svg)](https://docs.rs/derpiboorust)

## Example usage:
```rust
use derpibooru::{SyncAdapter, Search};

let adapter = SyncAdapter::new();
let request = Search::new("69 position,safe")
    .min_score(70)
    .max_score(120);

let response = adapter.send(request)?;
println!("{:?}", response.search); // Vec<Image>
```

