//! Adapters for making requests.
//!
//! Both adapters are just simple wrappers over request's `Client`.
//! They have `From<Client>` implementation, so you should not use the `new` method, better
//! create a customized `Client` (set a timeout, User-Agent, etc.) and create an adapter from it.
//!
//! # Example
//! ```no_run
//! use std::time::Duration;
//! use reqwest::Client;
//! use derpiboorust::{SyncAdapter, Lists};
//!
//! let client = Client::builder()
//!     .timeout(Duration::from_secs(10))
//!     .build()
//!     .unwrap();
//!
//! let adapter = SyncAdapter::from(client);
//! let request = Lists::new().page(2);
//! let response = adapter.send(request).unwrap();
//! ```

use failure::{err_msg, Error};
use reqwest::StatusCode;

mod r#async;
mod sync;

pub use self::{r#async::AsyncAdapter, sync::SyncAdapter};

fn check_status(status: StatusCode) -> Option<Error> {
    if status.is_client_error() || status.is_server_error() {
        let error = format!("HTTP error: {}", status);
        return Some(err_msg(error));
    }

    None
}
