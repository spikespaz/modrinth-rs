mod query_string;

pub mod search;

use crate::{
    base62::Base62,
    query::{query_string::JsonQueryParams, search::*},
    types::{FileHashes, Project, ProjectIdentifier, ProjectVersion},
};

static MODRINTH_ENDPOINT: &str = "https://api.modrinth.com/v2/";

#[derive(Clone, Debug)]
pub struct Client {
    inner: surf::Client,
}

impl Client {
    pub fn new<S>(token: Option<S>) -> Result<Self, <surf::Client as TryFrom<surf::Config>>::Error>
    where
        S: AsRef<str>,
    {
        let mut config = surf::Config::new();

        config = config.set_base_url(surf::Url::parse(MODRINTH_ENDPOINT).unwrap());

        if let Some(token) = token {
            config = config.add_header("Authorization", token.as_ref()).unwrap();
        }

        Ok(Self {
            inner: config.try_into()?,
        })
    }

    pub fn with_config(
        mut config: surf::Config,
    ) -> Result<Self, <surf::Client as TryFrom<surf::Config>>::Error> {
        config = config.set_base_url(surf::Url::parse(MODRINTH_ENDPOINT).unwrap());

        Ok(Self {
            inner: config.try_into()?,
        })
    }

    pub async fn get_project(&self, identifier: &ProjectIdentifier) -> surf::Result<Project> {
        self.inner
            .get(&format!("project/{}", identifier))
            .recv_json()
            .await
    }

    pub async fn get_project_versions(
        &self,
        identifier: &ProjectIdentifier,
    ) -> surf::Result<Vec<ProjectVersion>> {
        self.inner
            .get(&format!("version/{}", identifier))
            .recv_json()
            .await
    }

    pub async fn get_version(&self, identifier: &Base62) -> surf::Result<ProjectVersion> {
        self.inner
            .get(&format!("version/{}", identifier))
            .recv_json()
            .await
    }

    pub async fn get_version_by_hash(&self, hash: &FileHashes) -> surf::Result<ProjectVersion> {
        self.inner
            .get(&match hash {
                FileHashes {
                    sha512: Some(hash), ..
                } => format!("version_file/{}?algorithm=sha512", hash),
                FileHashes {
                    sha1: Some(hash), ..
                } => format!("version_file/{}?algorithm=sha1", hash),
                _ => panic!("expected at least one field of `hash` to be `Some`"),
            })
            .recv_json()
            .await
    }
}
