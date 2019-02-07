use failure::Error;
use reqwest::Url;

use super::{Bound, Order};
use crate::request::{build_url, response::ImagesResponse, QueryPairs, Request};

/// Request for fetching user watched images (`/images/watched.json`).
/// ```
/// use derpiboorust::{Watched, Bound, Order};
///
/// let request = Watched::new("user_account_key")
///     .constraint("id")
///     .constraint_bound(Bound::Gte("12345"))
///     .constraint_order(Order::Descending)
///     .random();
/// ```

#[derive(Debug)]
pub struct Watched<'a> {
    query: QueryPairs<'a>,
}
impl<'a> Watched<'a> {
    /// Create new watched images request.
    pub fn new(key: &'a str) -> Self {
        let mut query = QueryPairs::new();
        query.insert("key", key);

        Watched { query }
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

impl<'a> Request<'a> for Watched<'a> {
    type ResponseValue = ImagesResponse;

    fn build(&self) -> Result<Url, Error> {
        build_url("images/watched.json", &self.query)
    }
}

#[test]
fn request() {
    let req = Watched::new("qwezxc123")
        .constraint("id")
        .constraint_bound(Bound::Gte("1941825"))
        .constraint_order(Order::Descending)
        .page(2)
        .random()
        .build()
        .unwrap();

    let expected = Url::parse_with_params(
        "https://derpibooru.org/images/watched.json",
        &[
            ("key", "qwezxc123"),
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
