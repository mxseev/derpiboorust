mod images;
mod watched;
pub use self::{images::Images, watched::Watched};

use crate::request::QueryPairValue;

/// Constraint bound.
#[derive(Debug)]
pub enum Bound<'a> {
    Gt(&'a str),
    Gte(&'a str),
    Lt(&'a str),
    Lte(&'a str),
}
impl<'a> Bound<'a> {
    fn query_pair(&self) -> (&'a str, &'a str) {
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
impl QueryPairValue for Order {
    fn to_query(&self) -> String {
        match self {
            Order::Ascending => String::from("a"),
            Order::Descending => String::from("d"),
        }
    }
}
