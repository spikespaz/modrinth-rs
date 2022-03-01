mod projects;
mod search;

pub use projects::*;
pub use search::*;

use serde::de::DeserializeOwned;
use serde_path_to_error::Error as SerdePathError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("there was an issue making the request")]
    Ureq(#[from] ureq::Error),
    #[error("there was an issue deserializing a response to JSON")]
    Io(#[from] std::io::Error),
    #[error("there was an issue fitting JSON to a strong type")]
    Json(#[from] SerdePathError<serde_json::Error>),
}

pub type Result<T> = std::result::Result<T, Error>;

fn get<T>(endpoint: &str, token: Option<&str>) -> Result<T>
where
    T: DeserializeOwned,
{
    let mut request = ureq::get(endpoint);

    if let Some(token) = token {
        request = request.set("Authorization", token);
    }

    let content = request.call()?.into_reader();
    let deserializer = &mut serde_json::Deserializer::from_reader(content);

    Ok(serde_path_to_error::deserialize(deserializer)?)
}
