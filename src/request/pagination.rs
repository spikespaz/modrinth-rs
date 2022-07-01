use async_trait::async_trait;
use awaur::paginator::{PaginatedStream, PaginationDelegate};
use serde::{Deserialize, Serialize};

use crate::endpoints as e;
use crate::request::params::ProjectSearchParams;
use crate::types::ProjectSearchResult;

pub struct ProjectSearchDelegate<'cu> {
    client: &'cu isahc::HttpClient,
    base: &'cu url::Url,
    params: ProjectSearchParams,
    total_hits: usize,
}

impl<'cu> ProjectSearchDelegate<'cu> {
    pub fn new(
        client: &'cu isahc::HttpClient,
        base: &'cu url::Url,
        mut params: ProjectSearchParams,
    ) -> Self {
        params.offset = params.offset.or(Some(0));

        Self {
            client,
            base,
            params,
            total_hits: 0,
        }
    }
}

#[async_trait]
impl PaginationDelegate for ProjectSearchDelegate<'_> {
    type Item = ProjectSearchResult;
    type Error = e::Error;

    async fn next_page(&mut self) -> Result<Vec<Self::Item>, Self::Error> {
        let result = e::search_projects(self.client, self.base, &self.params)
            .await?
            .into_value();

        self.total_hits = result.total_hits;

        Ok(result.hits)
    }

    fn offset(&self) -> usize {
        self.params.offset.unwrap()
    }

    fn set_offset(&mut self, value: usize) {
        self.params.offset = Some(value);
    }

    fn total_items(&self) -> Option<usize> {
        Some(self.total_hits)
    }
}

/// See the documentation for [`PaginatedStream`].
pub type ProjectSearchStream<'cu, 'f> = PaginatedStream<'f, ProjectSearchDelegate<'cu>>;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PaginatedResponse<T> {
    pub hits: Vec<T>,
    pub offset: usize,
    pub limit: usize,
    pub total_hits: usize,
}
