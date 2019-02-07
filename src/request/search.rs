use failure::Error;
use reqwest::Url;

use super::{build_url, response::SearchResponse, QueryPairs, Request};

/// Request for searching images (`/search.json`).
/// ```
/// use derpiboorust::Search;
///
/// let request = Search::new("69 position,safe")
///     .page(2)
///     .min_score(70)
///     .max_score(120)
///     .perpage(10);
/// ```

#[derive(Debug)]
pub struct Search<'a> {
    query: QueryPairs<'a>,
}
impl<'a> Search<'a> {
    /// Create new search request.
    pub fn new(q: &'a str) -> Self {
        let mut query = QueryPairs::new();
        query.insert("q", q);

        Search { query }
    }
    /// The page offset.
    pub fn page(mut self, page: u64) -> Self {
        self.query.insert("page", page);
        self
    }
    /// Minimum score for images.
    pub fn min_score(mut self, score: i64) -> Self {
        self.query.insert("min_score", score);
        self
    }
    /// Maximum score for images.
    pub fn max_score(mut self, score: i64) -> Self {
        self.query.insert("max_score", score);
        self
    }
    /// How many results to return on each page (must be between 1 and 50).
    pub fn perpage(mut self, perpage: u64) -> Self {
        self.query.insert("perpage", perpage);
        self
    }
    /// User key.
    pub fn key(mut self, key: &'a str) -> Self {
        self.query.insert("key", key);
        self
    }
}

impl<'a> Request<'a> for Search<'a> {
    type ResponseValue = SearchResponse;

    fn build(&self) -> Result<Url, Error> {
        build_url("search.json", &self.query)
    }
}

#[test]
fn request() {
    let req = Search::new("luna, safe")
        .page(2)
        .min_score(42)
        .max_score(322)
        .perpage(5)
        .key("qwezxc123")
        .build()
        .unwrap();

    let expected = Url::parse_with_params(
        "https://derpibooru.org/search.json",
        &[
            ("q", "luna, safe"),
            ("page", "2"),
            ("min_score", "42"),
            ("max_score", "322"),
            ("perpage", "5"),
            ("key", "qwezxc123"),
        ],
    )
    .unwrap();

    assert_eq!(req, expected);
}
