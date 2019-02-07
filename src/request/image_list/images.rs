use failure::Error;
use reqwest::Url;

use super::{Bound, Order};
use crate::request::{build_url, response::ImagesResponse, QueryPairs, Request};

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

#[derive(Debug)]
pub struct Images<'a> {
    query: QueryPairs<'a>,
}
impl<'a> Images<'a> {
    /// Create new images request.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let query = QueryPairs::new();

        Images { query }
    }
    /// Search and sort by a specific field.
    pub fn constraint(mut self, name: &'a str) -> Self {
        self.query.insert("constraint", name);
        self
    }
    /// When specified, constraint field must be match bound.
    pub fn constraint_bound(mut self, bound: Bound<'a>) -> Self {
        let bound_query = bound.query_pair();
        self.query.insert(bound_query.0, bound_query.1);

        self
    }
    /// Sort order for constraint.
    pub fn constraint_order(mut self, order: Order) -> Self {
        self.query.insert("order", order);
        self
    }
    /// The page offset.
    pub fn page(mut self, page: u64) -> Self {
        self.query.insert("page", page);
        self
    }
    /// When set, order the images randomly.
    pub fn random(mut self) -> Self {
        self.query.insert("random", true);
        self
    }
}

impl<'a> Request<'a> for Images<'a> {
    type ResponseValue = ImagesResponse;

    fn build(&self) -> Result<Url, Error> {
        build_url("images.json", &self.query)
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
            ("order", "d"),
            ("page", "2"),
            ("random", "true"),
        ],
    )
    .unwrap();

    assert_eq!(req, expected);
}
