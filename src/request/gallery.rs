use failure::Error;
use reqwest::Url;

use super::{build_url, response::GalleryResponse, QueryPairs, Request};

/// Request for fetching user gallery (`/galleries/username/id.json`).
/// ```
/// use derpiboorust::Gallery;
///
/// let request = Gallery::new("Blossomforth", 2683).page(2);
/// ```

#[derive(Debug)]
pub struct Gallery<'a> {
    username: &'a str,
    id: u64,
    query: QueryPairs<'a>,
}
impl<'a> Gallery<'a> {
    /// Create new gallery request.
    pub fn new(username: &'a str, id: u64) -> Self {
        let query = QueryPairs::new();

        Gallery {
            username,
            id,
            query,
        }
    }
    /// The page offset.
    pub fn page(mut self, page: u64) -> Self {
        self.query.insert("page", page);
        self
    }
}

impl<'a> Request<'a> for Gallery<'a> {
    type ResponseValue = GalleryResponse;

    fn build(&self) -> Result<Url, Error> {
        let gallery_url = format!("galleries/{}/{}.json", self.username, self.id);
        build_url(&gallery_url, &self.query)
    }
}

#[test]
fn request() {
    let req = Gallery::new("Blossomforth", 2683).page(2).build().unwrap();
    let expected = Url::parse_with_params(
        "https://derpibooru.org/galleries/Blossomforth/2683.json",
        &[("page", "2")],
    )
    .unwrap();

    assert_eq!(req, expected);
}
