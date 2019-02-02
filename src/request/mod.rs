//! API methods.
use failure::Error;
use reqwest::Url;
use serde::Deserialize;
use std::fmt::Display;

mod galleries;
mod gallery;
mod image;
mod image_list;
mod lists;
pub mod response;
mod search;
pub use self::{
    galleries::Galleries,
    gallery::Gallery,
    image::Image,
    image_list::{Bound, Images, Order, Watched},
    lists::Lists,
    search::Search,
};

static DERPIBOORU_API_BASE: &str = "https://derpibooru.org";

/// Base trait for requests.
pub trait Request<'de> {
    type ResponseValue: Deserialize<'de>;

    fn build(&self) -> Result<Url, Error>;
}

struct UrlBuilder<'a> {
    path: &'a str,
    query: Vec<(&'a str, String)>,
}
impl<'a> UrlBuilder<'a> {
    fn new(path: &'a str) -> Self {
        let query = Vec::new();

        UrlBuilder { path, query }
    }
    fn append_query_pair<D: Display>(&mut self, key: &'a str, value: D) {
        self.query.push((key, value.to_string()));
    }
    fn build(self) -> Result<Url, Error> {
        let mut url = Url::parse(DERPIBOORU_API_BASE)?.join(self.path)?;
        if !self.query.is_empty() {
            url.query_pairs_mut().extend_pairs(self.query);
        }

        Ok(url)
    }
}
