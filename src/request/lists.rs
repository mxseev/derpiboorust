use failure::Error;
use reqwest::Url;

use super::{build_url, response::ListsResponse, QueryPairs, Request};

/// Request for fetching image lists (`/lists.json`).
/// ```
/// use derpiboorust::Lists;
///
/// let request = Lists::new()
///     .page(2)
///     .last("2d");
/// ```

#[derive(Debug)]
pub struct Lists<'a> {
    query: QueryPairs<'a>,
}
impl<'a> Lists<'a> {
    /// Create new lists request.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let query = QueryPairs::new();

        Lists { query }
    }
    /// The page offset.
    pub fn page(mut self, page: u64) -> Self {
        self.query.insert("page", page);
        self
    }
    /// Sampling period, specified in weeks, days, or hours.
    pub fn last(mut self, last: &'a str) -> Self {
        self.query.insert("last", last);
        self
    }
}

impl<'a> Request<'a> for Lists<'a> {
    type ResponseValue = ListsResponse;

    fn build(&self) -> Result<Url, Error> {
        build_url("lists.json", &self.query)
    }
}

#[test]
fn request() {
    let req = Lists::new().page(2).last("2w").build().unwrap();
    let expected = Url::parse_with_params(
        "https://derpibooru.org/lists.json",
        &[("page", "2"), ("last", "2w")],
    )
    .unwrap();

    assert_eq!(req, expected);
}
