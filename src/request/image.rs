use failure::Error;
use reqwest::Url;

use super::{response::ImageResponse, Request, UrlBuilder};

/// Request for fetching single image (`/images/1941825.json`).
/// ```
/// use derpiboorust::Image;
///
/// let request = Image::new(1941825);
/// ```

pub struct Image {
    id: u64,
}
impl Image {
    /// Create new image request.
    pub fn new(id: u64) -> Self {
        Image { id }
    }
}

impl<'a> Request<'a> for Image {
    type ResponseValue = ImageResponse;

    fn build(&self) -> Result<Url, Error> {
        let image_url = format!("{}.json", self.id);
        let url = UrlBuilder::new(&image_url);

        url.build()
    }
}

#[test]
fn request() {
    let req = Image::new(1941825).build().unwrap();
    let expected = Url::parse("https://derpibooru.org/1941825.json").unwrap();

    assert_eq!(req, expected);
}
