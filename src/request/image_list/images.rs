use failure::Error;
use reqwest::Url;

use super::{Bound, Order};
use crate::request::{response::ImagesResponse, Request, UrlBuilder};

/// Request for fetching images (`/images.json`).
/// ```
/// use derpiboorust::{Images, Bound, Order};
///
/// let request = Images::new()
///     .page(4)
///     .constraint("id")
///     .constraint_bound(Bound::Gte("12345"))
///     .constraint_order(Order::Descending)
///     .random();
/// ```

#[derive(Debug, Default)]
pub struct Images<'a> {
    constraint: Option<&'a str>,
    constraint_bound: Option<Bound<'a>>,
    constraint_order: Option<Order>,
    page: Option<u64>,
    random: bool,
}
impl<'a> Images<'a> {
    /// Create new images request.
    pub fn new() -> Self {
        Images::default()
    }
    /// Search and sort by a specific field.
    pub fn constraint(mut self, name: &'a str) -> Self {
        self.constraint = Some(name);
        self
    }
    /// When specified, constraint field must be match bound.
    pub fn constraint_bound(mut self, bound: Bound<'a>) -> Self {
        self.constraint_bound = Some(bound);
        self
    }
    /// Sort order for constraint.
    pub fn constraint_order(mut self, order: Order) -> Self {
        self.constraint_order = Some(order);
        self
    }
    /// The page offset.
    pub fn page(mut self, page: u64) -> Self {
        self.page = Some(page);
        self
    }
    /// When set, order the images randomly.
    pub fn random(mut self) -> Self {
        self.random = true;
        self
    }
}

impl<'a> Request<'a> for Images<'a> {
    type ResponseValue = ImagesResponse;

    fn build(&self) -> Result<Url, Error> {
        let mut url = UrlBuilder::new("images.json");

        if let Some(constraint) = &self.constraint {
            url.append_query_pair("constraint", constraint);
        }
        if let Some(bound) = &self.constraint_bound {
            let bound_query = bound.query();
            url.append_query_pair(bound_query.0, bound_query.1);
        }
        if let Some(page) = self.page {
            url.append_query_pair("page", page);
        }
        if let Some(order) = &self.constraint_order {
            url.append_query_pair("order", order.query());
        }
        if self.random {
            url.append_query_pair("random", "true");
        }

        url.build()
    }
}

#[test]
fn request() {
    let req = Images::new()
        .constraint("id")
        .constraint_bound(Bound::Gte("1941825"))
        .constraint_order(Order::Descending)
        .page(2)
        .random()
        .build()
        .unwrap();

    let expected = Url::parse_with_params(
        "https://derpibooru.org/images.json",
        &[
            ("constraint", "id"),
            ("gte", "1941825"),
            ("page", "2"),
            ("order", "d"),
            ("random", "true"),
        ],
    )
    .unwrap();

    assert_eq!(req, expected);
}
