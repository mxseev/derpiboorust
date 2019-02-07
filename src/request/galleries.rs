use failure::Error;
use reqwest::Url;

use super::{build_url, response::GalleriesResponse, QueryPairs, Request};

/// Request for fetching user galleries (`/galleries/username.json`).
/// ```
/// use derpiboorust::Galleries;
///
/// let request = Galleries::new("Blossomforth")
///     .page(2)
///     .include_images();
/// ```

#[derive(Debug)]
pub struct Galleries<'a> {
    username: &'a str,
    query: QueryPairs<'a>,
}
impl<'a> Galleries<'a> {
    /// Create new galleries request.
    pub fn new(username: &'a str) -> Self {
        let query = QueryPairs::new();

        Galleries { username, query }
    }
    /// The page offset.
    pub fn page(mut self, page: u64) -> Self {
        self.query.insert("page", page);
        self
    }
    /// When set, include arrays of image IDs featured in each gallery.
    pub fn include_images(mut self) -> Self {
        self.query.insert("include_images", true);
        self
    }
}

impl<'a> Request<'a> for Galleries<'a> {
    type ResponseValue = GalleriesResponse;

    fn build(&self) -> Result<Url, Error> {
        let galleries_url = format!("galleries/{}.json", self.username);
        build_url(&galleries_url, &self.query)
    }
}

#[test]
fn request() {
    let req = Galleries::new("Blossomforth")
        .page(2)
        .include_images()
        .build()
        .unwrap();

    let expected = Url::parse_with_params(
        "https://derpibooru.org/galleries/Blossomforth.json",
        &[("page", "2"), ("include_images", "true")],
    )
    .unwrap();

    assert_eq!(req, expected);
}
