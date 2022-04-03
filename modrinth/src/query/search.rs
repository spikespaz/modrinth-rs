use std::{collections::VecDeque, hash::Hash};

use chrono::{DateTime, Utc};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use serde_with::SerializeDisplay;
use strum::EnumString;

use crate::{
    base62::Base62,
    query::query_string::JsonQueryParams,
    types::{ProjectType, SideSupport},
};

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
pub type SearchFilters<T> = Vec<Vec<T>>;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize)]
pub struct SearchParams {
    pub query: Option<String>,
    /// <https://docs.modrinth.com/docs/tutorials/api_search/#facets>
    pub facets: Option<SearchFilters<SearchFacet>>,
    pub index: Option<SearchIndex>,
    pub offset: Option<usize>,
    pub limit: Option<usize>,
    pub filters: Option<SearchFilters<String>>,
    // #[deprecated]
    // pub version: Option<SearchFilters<String>>,
}

impl JsonQueryParams<'_> for SearchParams {}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Display, SerializeDisplay)]
pub enum SearchFacet {
    #[display(fmt = "categories:'{}'", _0)]
    Category(String),
    #[display(fmt = "versions:'{}'", _0)]
    Version(String),
    #[display(fmt = "license:'{}'", _0)]
    License(String),
    #[display(fmt = "project_type:'{}'", _0)]
    ProjectType(String),
    #[display(fmt = "{}:'{}'", _0, _1)]
    Custom(String, String),
}

impl SearchFacet {
    pub fn category<S>(value: S) -> Self
    where
        S: AsRef<str>,
    {
        Self::Category(value.as_ref().to_owned())
    }

    pub fn version<S>(value: S) -> Self
    where
        S: AsRef<str>,
    {
        Self::Version(value.as_ref().to_owned())
    }

    pub fn license<S>(value: S) -> Self
    where
        S: AsRef<str>,
    {
        Self::License(value.as_ref().to_owned())
    }

    pub fn project_type<S>(value: S) -> Self
    where
        S: AsRef<str>,
    {
        Self::ProjectType(value.as_ref().to_owned())
    }

    pub fn custom<N, S>(name: N, value: S) -> Self
    where
        N: AsRef<str>,
        S: AsRef<str>,
    {
        Self::Custom(name.as_ref().to_owned(), value.as_ref().to_owned())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, EnumString, Serialize)]
#[serde(deny_unknown_fields)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum SearchIndex {
    Relevance,
    Downloads,
    Follows,
    Newest,
    Updated,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SearchResults {
    pub hits: VecDeque<ProjectResult>,
    pub offset: usize,
    pub limit: usize,
    pub total_hits: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectResult {
    pub project_id: Base62,
    pub project_type: ProjectType,
    pub slug: Option<String>,
    pub author: String,
    pub title: String,
    pub description: String,
    pub categories: Vec<String>,
    pub versions: Vec<String>,
    pub latest_version: Option<String>,
    // Should `downloads` and `follows`be a usize but the API returns -1 sometimes
    // Reference:
    // > `labrinth::models::projects::Project` and
    // > `labrinth::database::models::project_item::Project`
    pub downloads: isize,
    pub follows: isize,
    pub icon_url: String,
    pub date_created: DateTime<Utc>,
    pub date_modified: DateTime<Utc>,
    pub license: String,
    pub client_side: SideSupport,
    pub server_side: SideSupport,
    pub gallery: Vec<String>,
}

#[derive(Debug)]
pub struct SearchResultsPaginator<'a> {
    params: SearchParams,
    token: Option<&'a str>,
    results: VecDeque<ProjectResult>,
    total_hits: Option<usize>,
    errored: bool,
}

impl<'a> SearchResultsPaginator<'a> {
    pub fn new(params: SearchParams, token: Option<&'a str>) -> Self {
        Self {
            params,
            token,
            results: VecDeque::new(),
            total_hits: None,
            errored: false,
        }
    }
}

impl<'a> Iterator for SearchResultsPaginator<'a> {
    type Item = Result<ProjectResult>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.errored {
            return None;
        }

        if self.results.is_empty() {
            let results = if self.total_hits.is_none() {
                let mut params = self.params.clone();
                params.limit = Some(1);
                get_search(&params, self.token)
            } else {
                get_search(&self.params, self.token)
            };

            let mut results = match results {
                Ok(results) => results,
                Err(error) => {
                    self.errored = true;
                    return Some(Err(error));
                }
            };

            if self.total_hits.is_none() {
                self.total_hits = Some(results.total_hits);
            }

            self.results.append(&mut results.hits);
            self.params.offset = Some(self.params.offset.unwrap_or(0) + self.results.len());
        }

        self.results.pop_front().map(Ok)
    }

    /// Requires one item to have been recieved with [`Self::next`],
    /// otherwise the upper bound will be `None`.
    /// This cannot be precomputed because this method cannot have mutable access to `self`
    /// and therefore cannot process the results of an initial get request.
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, self.total_hits)
    }
}
