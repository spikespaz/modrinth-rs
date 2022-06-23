// pub mod client;
pub mod endpoints;
pub mod request;
pub mod types;

pub(crate) mod serde_with;
pub(crate) mod utils;

use endpoints::{DeserializeError, ResponseError};

// pub mod prelude {}

/// The main error type used throughout the crate.
#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Deserialize(#[from] DeserializeError),
    /// Sometimes the backend can throw an error, either because something was
    /// configured wrongly or because of an internal error such as connection
    /// loss.
    #[error("failed to construct a request or recieve a response\n{0}")]
    Request(#[from] isahc::Error),
    #[error("{0}")]
    Response(#[from] ResponseError),
    /*
    /// This variant will wrap an [`isahc::http::Error`] when configuring the
    /// client has failed to produce a stable instance of the backend.
    #[error("failed to construct a request\n{0}")]
    Http(#[from] isahc::http::Error),
    */
    /*
    /// Variant specifically for when parsing the base URL fails.
    #[error("the string provided failed to parse as a URL\n{0}")]
    ParseUrl(#[from] url::ParseError),
    */
    /*
    /// The URL that was provided cannot be used as a base.
    #[error("the URL provided cannot be a base")]
    BadBaseUrl,
    */
}
