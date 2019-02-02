use failure::Error;
use reqwest::Url;

use super::{response::SearchResponse, Request, UrlBuilder};

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

pub struct Search<'a> {
    query: &'a str,
    page: Option<u64>,
    min_score: Option<i64>,
    max_score: Option<i64>,
    perpage: Option<u64>,
    key: Option<&'a str>,
}
impl<'a> Search<'a> {
    /// Create new search request.
    pub fn new(query: &'a str) -> Self {
        Search {
            query,
            page: None,
            min_score: None,
            max_score: None,
            perpage: None,
            key: None,
        }
    }
    /// The page offset.
    pub fn page(mut self, page: u64) -> Self {
        self.page = Some(page);
        self
    }
    /// Minimum score for images.
    pub fn min_score(mut self, score: i64) -> Self {
        self.min_score = Some(score);
        self
    }
    /// Maximum score for images.
    pub fn max_score(mut self, score: i64) -> Self {
        self.max_score = Some(score);
        self
    }
    /// How many results to return on each page (must be between 1 and 50).
    pub fn perpage(mut self, perpage: u64) -> Self {
        self.perpage = Some(perpage);
        self
    }
    /// User key.
    pub fn key(mut self, key: &'a str) -> Self {
        self.key = Some(key);
        self
    }
}

impl<'a> Request<'a> for Search<'a> {
    type ResponseValue = SearchResponse;

    fn build(&self) -> Result<Url, Error> {
        let mut url = UrlBuilder::new("search.json");
        url.append_query_pair("q", self.query);

        if let Some(page) = self.page {
            url.append_query_pair("page", page);
        }
        if let Some(score) = self.min_score {
            url.append_query_pair("min_score", score);
        }
        if let Some(score) = self.max_score {
            url.append_query_pair("max_score", score);
        }
        if let Some(perpage) = self.perpage {
            url.append_query_pair("perpage", perpage);
        }
        if let Some(key) = self.key {
            url.append_query_pair("key", key);
        }

        url.build()
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
