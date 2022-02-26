// use chrono::{DateTime, Local};
use serde::Serialize;
use serde_with::SerializeDisplay;

#[derive(Debug, Clone, SerializeDisplay)]
pub enum SearchFacet {
    Category(String),
    Version(String),
    License(String),
    ProjectType(String),
}

impl std::fmt::Display for SearchFacet {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&match *self {
            Self::Category(ref value) => format!("categories:'{}'", value),
            Self::Version(ref value) => format!("versions:'{}'", value),
            Self::License(ref value) => format!("license:'{}'", value),
            Self::ProjectType(ref value) => format!("project_type:'{}'", value),
        })
    }
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
