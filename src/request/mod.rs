//! API methods.
use failure::Error;
use reqwest::Url;
use serde::Deserialize;

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

trait QueryPairValue {
    fn to_query(&self) -> String;
}
impl<T> QueryPairValue for T
where
    T: ToString,
{
    fn to_query(&self) -> String {
        self.to_string()
    }
}

#[derive(Debug)]
struct QueryPairs<'a> {
    pairs: Vec<(&'a str, String)>,
}
impl<'a> QueryPairs<'a> {
    fn new() -> Self {
        let pairs = Vec::new();

        QueryPairs { pairs }
    }
    fn insert<V: QueryPairValue>(&mut self, key: &'a str, value: V) {
        self.pairs.push((key, value.to_query()));
    }
    fn is_empty(&self) -> bool {
        self.pairs.is_empty()
    }
}

fn build_url<'a>(path: &str, query: &QueryPairs<'a>) -> Result<Url, Error> {
    let mut url = Url::parse(DERPIBOORU_API_BASE)?.join(path)?;
    if !query.is_empty() {
        url.query_pairs_mut().extend_pairs(query.pairs.iter());
    }

    Ok(url)
}
