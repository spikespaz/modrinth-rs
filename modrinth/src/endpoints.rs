use crate::{prelude::*, query_string::JsonQueryParams};
use derive_more::Display;
use serde::de::DeserializeOwned;
use serde_path_to_error::Error as SerdePathError;
use serde_with::SerializeDisplay;
use thiserror::Error;

// const API_BASE: &str = "https://api.modrinth.com/v2/";

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

pub fn get_search(params: &SearchParams, token: Option<&str>) -> Result<SearchResults> {
    get(
        &format!(
            "https://api.modrinth.com/v2/search?{}",
            &params.to_query_string()
        ),
        token,
    )
}

pub fn get_search_iter(params: SearchParams, token: Option<&str>) -> SearchResultsPaginator {
    SearchResultsPaginator::new(params, token)
}

pub fn get_project(identifier: &ProjectIdentifier, token: Option<&str>) -> Result<Project> {
    get(
        &format!("https://api.modrinth.com/v2/project/{}", identifier),
        token,
    )
}

#[derive(Debug, Clone, Display, SerializeDisplay)]
pub enum ProjectIdentifier {
    Id(Base62),
    Slug(String),
}

impl ProjectIdentifier {
    pub fn id(other: Base62) -> Self {
        Self::from(other)
    }

    pub fn slug<S>(other: S) -> Self
    where
        S: AsRef<str>,
    {
        Self::from(other)
    }
}

impl From<Base62> for ProjectIdentifier {
    fn from(other: Base62) -> Self {
        Self::Id(other)
    }
}

impl<S> From<S> for ProjectIdentifier
where
    S: AsRef<str>,
{
    fn from(other: S) -> Self {
        Self::Slug(other.as_ref().to_owned())
    }
}
