//! This crate provides Rust bindings for [Derpibooru API](https://derpibooru.org/pages/api).
//! # Example usage:
//! ```no_run
//! use derpiboorust::{SyncAdapter, Search};
//!
//! let adapter = SyncAdapter::new();
//! let request = Search::new("69 position,safe")
//!     .min_score(70)
//!     .max_score(120);
//!
//! let response = adapter.send(request).unwrap();
//! println!("{:?}", response.search); // Vec<Image>
//! ```

pub mod adapter;
pub mod models;
pub mod request;

pub use adapter::{AsyncAdapter, SyncAdapter};
pub use request::{Bound, Galleries, Gallery, Image, Images, Lists, Order, Search, Watched};
