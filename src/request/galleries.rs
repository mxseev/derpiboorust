use failure::Error;
use reqwest::Url;

use super::{response::GalleriesResponse, Request, UrlBuilder};

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
    page: Option<u64>,
    include_images: bool,
}
impl<'a> Galleries<'a> {
    /// Create new galleries request.
    pub fn new(username: &'a str) -> Self {
        Galleries {
            username,
            page: None,
            include_images: false,
        }
    }
    /// The page offset.
    pub fn page(mut self, page: u64) -> Self {
        self.page = Some(page);
        self
    }
    /// When set, include arrays of image IDs featured in each gallery.
    pub fn include_images(mut self) -> Self {
        self.include_images = true;
        self
    }
}

impl<'a> Request<'a> for Galleries<'a> {
    type ResponseValue = GalleriesResponse;

    fn build(&self) -> Result<Url, Error> {
        let galleries_url = format!("galleries/{}.json", self.username);
        let mut url = UrlBuilder::new(&galleries_url);

        if let Some(page) = self.page {
            url.append_query_pair("page", page);
        }
        if self.include_images {
            url.append_query_pair("include_images", "true");
        }

        url.build()
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
