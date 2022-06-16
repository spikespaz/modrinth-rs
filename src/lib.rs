pub mod client;
pub mod endpoints;
pub mod request;
pub mod types;

pub(crate) mod serde_with;
pub(crate) mod utils;

// pub mod prelude {}

/// The main error type used throughout the crate.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// This is the most useful variant. This will be returned if the API
    /// response failed to parse either as valid JSON, or according to the
    /// policy for handling unknown fields set by the enabled Cargo features.
    /// See the crate documentation for [conditional
    /// compilation](crate#conditional-compilation).
    #[error("there was an error deserializing a response\n{error}\nencountered at:\n{uri}")]
    Deserialize {
        /// The URI that the initial request was sent to.
        uri: url::Url,
        /// The source error that this variant was constructed from.
        #[source]
        error: serde_path_to_error::Error<serde_json::Error>,
        /// The bytes the body content bytes of the response.
        bytes: Box<Vec<u8>>,
    },
    /// Sometimes the backend can throw an error, either because something was
    /// configured wrongly or an internal error such as connection loss could
    /// have happened.
    #[error("there was an error constructing or receiving a request\n{0}")]
    Request(#[from] isahc::Error),
    /// A request to a URI that was expected to return successfully with `200:
    /// OK` has failed to do so. This contains the status code that was recieved
    /// instead, and the bytes in the body of the response.
    #[error("response was expected to be status 200 OK but got {status}\nencountered at:\n{uri}")]
    StatusNotOk {
        /// The URI that the initial request was sent to.
        uri: url::Url,
        /// The response status code that was returned, not `200: OK`.
        status: isahc::http::StatusCode,
        /// The bytes the body content bytes of the response.
        bytes: Box<Vec<u8>>,
    },
    /// This variant will wrap an [`isahc::http::Error`] when configuring the
    /// client has failed to produce a stable instance of the backend.
    #[error("there was an error constructing the request\n{0}")]
    Http(#[from] isahc::http::Error),
    /// Variant specifically for when parsing the base URL fails.
    #[error("the string provided failed to parse as a URL\n{0}")]
    ParseUrl(#[from] url::ParseError),
    /// The URl that was provided cannot be used as a base.
    #[error("the URL provided cannot be a base")]
    BadBaseUrl,
}
