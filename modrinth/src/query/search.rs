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
    // The next two should be `usize` but the API seems to be returning `-1`.
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
