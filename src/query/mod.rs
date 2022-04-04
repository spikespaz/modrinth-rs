mod query_string;

pub mod search;

use {async_stream::try_stream, futures_core::stream::Stream};

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
    pub fn new(token: Option<String>) -> Self {
        let mut config = surf::Config::new();

        // The unwrapping is safe because the URL is constant and guaranteed to be parsed.
        config = config.set_base_url(surf::Url::parse(MODRINTH_ENDPOINT).unwrap());

        if let Some(token) = token {
            // The unwrapping is safe because the token should be a known format,
            // and if it is not, crashing the program is appropriate.
            config = config.add_header("Authorization", token).unwrap();
        }

        Self {
            // The `Config` was created using defaults from Surf,
            // and the only point of failure would have been adding the token.
            // Unwrapping should be safe.
            inner: config.try_into().unwrap(),
        }
    }

    pub fn with_config(
        mut config: surf::Config,
    ) -> Result<Self, <surf::Client as TryFrom<surf::Config>>::Error> {
        config = config.set_base_url(surf::Url::parse(MODRINTH_ENDPOINT).unwrap());

        Ok(Self {
            inner: config.try_into()?,
        })
    }

    pub async fn search(&self, params: &SearchParams) -> surf::Result<SearchResults> {
        self.inner
            .get(&format!("search?{}", &params.to_query_string()))
            .recv_json()
            .await
    }

    pub fn search_iter(
        &self,
        mut params: SearchParams,
    ) -> impl Stream<Item = surf::Result<ProjectResult>> + '_ {
        try_stream! {
            let (mut projects, total) = {
                // The first search needs to have a limit of `1`,
                // otherwise the API seems to return an incorrect `total_hits`.
                let limit = std::mem::replace(&mut params.limit, Some(1));
                let search = self.search(&params).await?;
                params.limit = limit;
                // Because the loop won't have updated this for the first
                // project it returns, this needs to be set.
                params.offset = Some(params.offset.unwrap_or(0) + 1);

                (search.hits, search.total_hits)
            };

            loop {
                if projects.is_empty() {
                    // Only check this if we are out of projects,
                    // if this is checked outside we will be comparing too early as the
                    // offset is updated when the next page is received.
                    if params.offset.unwrap() >= total {
                        break
                    }

                    projects = self.search(&params).await?.hits;
                    params.offset = Some(params.offset.unwrap() + projects.len())
                }

                yield projects.pop_front().unwrap()
            }
        }
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
