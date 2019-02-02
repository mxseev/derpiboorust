use failure::Error;
use reqwest::Url;

use super::{response::GalleryResponse, Request, UrlBuilder};

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
    page: Option<u64>,
}
impl<'a> Gallery<'a> {
    /// Create new gallery request.
    pub fn new(username: &'a str, id: u64) -> Self {
        Gallery {
            username,
            id,
            page: None,
        }
    }
    /// The page offset.
    pub fn page(mut self, page: u64) -> Self {
        self.page = Some(page);
        self
    }
}

impl<'a> Request<'a> for Gallery<'a> {
    type ResponseValue = GalleryResponse;

    fn build(&self) -> Result<Url, Error> {
        let gallery_url = format!("galleries/{}/{}.json", self.username, self.id);
        let mut url = UrlBuilder::new(&gallery_url);

        if let Some(page) = self.page {
            url.append_query_pair("page", page);
        }

        url.build()
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
