use failure::Error;
use futures::{
    future::{self, Either},
    Future,
};
use reqwest::r#async::Client;
use serde::Deserialize;

use super::check_status;
use crate::request::Request;

/// Async adapter powered by asynchronous reqwest's [Client](reqwest::async::Client).
pub struct AsyncAdapter {
    client: Client,
}
impl AsyncAdapter {
    /// Creates new async asynchronous with default [Client](reqwest::async::Client).
    pub fn new() -> Self {
        let client = Client::new();

        AsyncAdapter { client }
    }
    /// Send a request.
    pub fn send<R: Request<'static>>(
        &self,
        request: R,
    ) -> impl Future<Item = R::ResponseValue, Error = Error>
    where
        R::ResponseValue: for<'de> Deserialize<'de>,
    {
        let url = match request.build() {
            Ok(url) => url,
            Err(error) => return Either::B(future::err(error)),
        };

        let fut = self
            .client
            .get(url)
            .send()
            .from_err()
            .and_then(|mut response| {
                if let Some(error) = check_status(response.status()) {
                    return Either::B(future::err(error));
                }

                Either::A(response.json().from_err())
            });

        Either::A(fut)
    }
}

impl From<Client> for AsyncAdapter {
    fn from(client: Client) -> Self {
        AsyncAdapter { client }
    }
}

impl Default for AsyncAdapter {
    fn default() -> Self {
        Self::new()
    }
}
