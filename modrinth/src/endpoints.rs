use crate::{base62::Base62, query::*, response::*};
use derive_more::Display;
use serde_with::SerializeDisplay;
use thiserror::Error;

const API_BASE: &str = "https://api.modrinth.com/v2/";

#[derive(Debug, Error)]
pub enum Error {
    #[error("there was an issue making the request")]
    Ureq(#[from] ureq::Error),
    #[error("there was an issue deserializing a response to JSON")]
    Io(#[from] std::io::Error),
    #[error("there was an issue fitting JSON to a strong type")]
    Json(#[from] serde_path_to_error::Error<serde_json::Error>),
}

pub type Result<T> = std::result::Result<T, Error>;

fn get<T>(endpoint: &str, token: Option<&str>) -> Result<T>
where
    T: serde::de::DeserializeOwned,
{
    let request = ureq::get(endpoint);

    if let Some(token) = token {
        request.set("Authorization", token);
    }

    let response = ureq::get(endpoint).call()?;
    let string = response.into_string()?;
    let deserializer = &mut serde_json::Deserializer::from_str(&string);

    Ok(serde_path_to_error::deserialize(deserializer)?)
}

pub fn get_search() -> ! {
    todo!();
}

pub fn get_project(identifier: &ProjectIdentifier) -> Result<Project> {
    get(
        &format!("https://api.modrinth.com/v2/project/{}", identifier),
        None,
    )
}

#[derive(Debug, Clone, Display, SerializeDisplay)]
pub enum ProjectIdentifier {
    Id(Base62),
    Slug(String),
}
