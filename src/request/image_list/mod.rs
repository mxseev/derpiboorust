mod images;
mod watched;
pub use self::{images::Images, watched::Watched};

/// Constraint bound.
#[derive(Debug)]
pub enum Bound<'a> {
    Gt(&'a str),
    Gte(&'a str),
    Lt(&'a str),
    Lte(&'a str),
}
impl<'a> Bound<'a> {
    fn query(&self) -> (&'a str, &'a str) {
        match self {
            Bound::Gt(value) => ("gt", value),
            Bound::Gte(value) => ("gte", value),
            Bound::Lt(value) => ("lt", value),
            Bound::Lte(value) => ("lte", value),
        }
    }
}

/// Constraint order.
#[derive(Debug)]
pub enum Order {
    Ascending,
    Descending,
}
impl Order {
    fn query(&self) -> &str {
        match self {
            Order::Ascending => "a",
            Order::Descending => "d",
        }
    }
}
