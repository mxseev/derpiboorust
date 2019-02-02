use failure::Error;
use reqwest::Client;
use serde::Deserialize;

use super::check_status;
use crate::request::Request;

/// Sync adapter powered by synchronous reqwest's [Client](reqwest::Client).
pub struct SyncAdapter {
    client: Client,
}
impl SyncAdapter {
    /// Creates new synchronous adapter with default [Client](reqwest::Client).
    pub fn new() -> Self {
        let client = Client::new();

        SyncAdapter { client }
    }
    /// Send a request.
    pub fn send<'r, R: Request<'r>>(&self, request: R) -> Result<R::ResponseValue, Error>
    where
        R::ResponseValue: for<'de> Deserialize<'de>,
    {
        let url = request.build()?;
        let mut response = self.client.get(url).send()?;

        if let Some(error) = check_status(response.status()) {
            return Err(error);
        }

        Ok(response.json()?)
    }
}

impl From<Client> for SyncAdapter {
    fn from(client: Client) -> Self {
        SyncAdapter { client }
    }
}

impl Default for SyncAdapter {
    fn default() -> Self {
        Self::new()
    }
}
