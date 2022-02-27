use crate::query_string::JsonQueryParams;
use derive_more::Display;
use serde::Serialize;
use serde_with::SerializeDisplay;

#[derive(Debug, Clone, Display, SerializeDisplay)]
pub enum SearchFacet {
    #[display(fmt = "categories:'{}'", _0)]
    Category(String),
    #[display(fmt = "versions:'{}'", _0)]
    Version(String),
    #[display(fmt = "license:'{}'", _0)]
    License(String),
    #[display(fmt = "project_type:'{}'", _0)]
    ProjectType(String),
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
}

pub type SearchFacets = Vec<Vec<SearchFacet>>;

#[derive(Debug, Clone, Serialize)]
pub enum SearchIndex {
    Relevance,
    Downloads,
    Follows,
    Newest,
    Updated,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct SearchParams {
    pub query: Option<String>,
    /// <https://docs.modrinth.com/docs/tutorials/api_search/#facets>
    pub facets: Option<SearchFacets>,
    pub index: Option<SearchIndex>,
    pub offset: Option<usize>,
    pub limit: Option<usize>,
    // filters: Option<SearchFilters>,
}

impl JsonQueryParams<'_> for SearchParams {}
