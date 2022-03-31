mod search;

use std::io::Read;

use serde::de::DeserializeOwned;
use serde_path_to_error::Error as SerdePathError;
use thiserror::Error;

use crate::{
    base62::Base62,
    types::{ProjectIdentifier, Project, ProjectVersion, FileHashes}
};

pub use search::*;

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

pub(crate) fn get<T>(endpoint: &str, token: Option<&str>) -> Result<T>
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

pub fn get_project(identifier: &ProjectIdentifier, token: Option<&str>) -> Result<Project> {
    get(
        &format!("https://api.modrinth.com/v2/project/{}", identifier),
        token,
    )
}

pub fn get_project_versions(
    identifier: &ProjectIdentifier,
    token: Option<&str>,
) -> Result<Vec<ProjectVersion>> {
    get(
        &format!("https://api.modrinth.com/v2/project/{}/version", identifier),
        token,
    )
}

pub fn get_version(identifier: &Base62, token: Option<&str>) -> Result<ProjectVersion> {
    get(
        &format!("https://api.modrinth.com/v2/version/{}", identifier),
        token,
    )
}

pub fn get_version_by_hash(hash: &FileHashes, token: Option<&str>) -> Result<ProjectVersion> {
    get(
        &match hash {
            FileHashes {
                sha512: Some(hash), ..
            } => format!(
                "https://api.modrinth.com/v2/version_file/{}?algorithm=sha512",
                hash
            ),
            FileHashes {
                sha1: Some(hash), ..
            } => format!(
                "https://api.modrinth.com/v2/version_file/{}?algorithm=sha1",
                hash
            ),
            _ => {
                return Err(Error::Input(
                    "the provided 'FileHashes' must have at minimum one `Some` value",
                ));
            }
        },
        token,
    )
}
