use async_trait::async_trait;
use awaur::paginator::{PaginatedStream, PaginationDelegate};
use crate::request::params::ProjectSearchParams;
use crate::types::Project;

pub struct ProjectSearchDelegate<'cu> {
    client: &'cu isahc::HttpClient,
    base: &'cu url::Url,
    params: ProjectSearchParams,
    total_hits: usize,
}

impl <'cu> ProjectSearchDelegate<'cu> {
    pub fn new(
        client: &'cu isahc::HttpClient,
        base: &'cu url::Url,
        params: ProjectSearchParams,
    ) -> Self {
        Self {
            client,
            base,
            params,
            total_hits: 0,
        }
    }
}

impl PaginationDelegate for ProjectSearchDelegate<'_> {
    type Item = Project;
    type Error = crate::Error;

    async fn next_page(&mut self) -> Result<Vec<Self::Item>, Self::Error> {
        let result = crate::endpoints::search_projects(
                self.client,
                self.base,
                &self.params
            )
            .await?;

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
