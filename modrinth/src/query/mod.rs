mod projects;
mod search;
mod versions;

use std::io::Read;

pub use projects::*;
pub use search::*;
pub use versions::*;

use serde::de::DeserializeOwned;
use serde_path_to_error::Error as SerdePathError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("there was an issue making the request")]
    Ureq(#[from] ureq::Error),
    #[error("there was an issue processing a response")]
    Io(#[from] std::io::Error),
    #[error("there was an issue fitting JSON to a strong type")]
    Json {
        path: SerdePathError<serde_json::Error>,
        data: String,
    },
    #[error("the parameters to a function were invalid")]
    Input(&'static str),
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

    let mut content = String::new();

    request.call()?.into_reader().read_to_string(&mut content)?;

    let deserializer = &mut serde_json::Deserializer::from_str(&content);

    serde_path_to_error::deserialize(deserializer).map_err(|error| Error::Json {
        path: error,
        data: content,
    })
}
