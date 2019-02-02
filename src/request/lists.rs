use failure::Error;
use reqwest::Url;

use super::{response::ListsResponse, Request, UrlBuilder};

/// Request for fetching image lists (`/lists.json`).
/// ```
/// use derpiboorust::Lists;
///
/// let request = Lists::new()
///     .page(2)
///     .last("2d");
/// ```

#[derive(Debug, Default)]
pub struct Lists<'a> {
    page: Option<u64>,
    last: Option<&'a str>,
}
impl<'a> Lists<'a> {
    /// Create new lists request.
    pub fn new() -> Self {
        Lists::default()
    }
    /// The page offset.
    pub fn page(mut self, page: u64) -> Self {
        self.page = Some(page);
        self
    }
    /// Sampling period, specified in weeks, days, or hours.
    pub fn last(mut self, last: &'a str) -> Self {
        self.last = Some(last);
        self
    }
}

impl<'a> Request<'a> for Lists<'a> {
    type ResponseValue = ListsResponse;

    fn build(&self) -> Result<Url, Error> {
        let mut url = UrlBuilder::new("lists.json");

        if let Some(page) = self.page {
            url.append_query_pair("page", page);
        }
        if let Some(last) = self.last {
            url.append_query_pair("last", last);
        }

        url.build()
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
