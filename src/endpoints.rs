use awaur::endpoints::{endpoint, ApiResponse, DeserializeError, ResponseError};
use awaur::macros::new_struct;
use awaur::serde_with::{Base62, JsonString};
use serde::Serialize;
use serde_with::serde_as;

use crate::request::pagination::{PaginatedResponse, ProjectSearchDelegate, ProjectSearchStream};
use crate::request::params::{ProjectIdentifier, ProjectSearchParams};
use crate::types::project::Project;
use crate::types::search::ProjectSearchResult;
use crate::types::version::{FileHashes, ProjectVersion};

pub static DEFAULT_API_BASE: &str = "https://api.modrinth.com/v2/";

/// The main error type used for methods associated with REST endpoints.
#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Deserialize(#[from] DeserializeError),
    /// Sometimes the backend can throw an error, either because something was
    /// configured wrongly or because of an internal error such as connection
    /// loss.
    #[error("failed to construct a request or recieve a response\n{0}")]
    Request(#[from] isahc::Error),
    #[error("{0}")]
    Response(#[from] ResponseError),
}

/// See the documentation for [`ApiResponse`].
pub type ApiResult<T> = Result<ApiResponse<T>, Error>;
/// See the documentation for [`ApiResponse`].
pub type ApiPageResult<T> = Result<ApiResponse<PaginatedResponse<T>>, Error>;

pub async fn search_projects(
    client: &isahc::HttpClient,
    base: &url::Url,
    params: &ProjectSearchParams,
) -> ApiPageResult<ProjectSearchResult> {
    endpoint! {
        client GET,
        uri: base / "search",
        params: params,
    }
}

pub fn search_projects_iter<'cu, 'f>(
    client: &'cu isahc::HttpClient,
    base: &'cu url::Url,
    params: ProjectSearchParams,
) -> ProjectSearchStream<'cu, 'f> {
    ProjectSearchDelegate::new(client, base, params).into()
}

pub async fn project(
    client: &isahc::HttpClient,
    base: &url::Url,
    project_id: &ProjectIdentifier,
) -> ApiResult<Project> {
    endpoint! {
        client GET,
        uri: base / "project/{}",
        vars: [project_id],
    }
}

pub async fn projects<I>(
    client: &isahc::HttpClient,
    base: &url::Url,
    project_ids: I,
) -> ApiResult<Project>
where
    I: IntoIterator<Item = u64>,
{
    endpoint! {
        client GET,
        uri: base / "projects",
        params: &new_struct! {
            #[serde_as]
            #[derive(Serialize)]
            RequestParams {
                #[serde_as(as = "JsonString<Vec<Base62>>")]
                ids: Vec<u64> = project_ids.into_iter().collect(),
            }
        },
    }
}

// pub async fn project_dependencies(
//     client: &isahc::HttpClient,
//     base: &url::Url,
//     project_id: &ProjectIdentifier,
// ) -> ApiResult<ProjectDependencies> {
//     endpoint! {
//         client GET,
//         uri: base / "project/{}/dependencies",
//         vars: [project_id],
//     }
// }

pub async fn project_version_list(
    client: &isahc::HttpClient,
    base: &url::Url,
    project_id: &ProjectIdentifier,
) -> ApiResult<ProjectVersion> {
    endpoint! {
        client GET,
        uri: base / "project/{}/version",
        vars: [project_id],
    }
}

pub async fn project_version(
    client: &isahc::HttpClient,
    base: &url::Url,
    version_id: u64,
) -> ApiResult<ProjectVersion> {
    endpoint! {
        client GET,
        uri: base / "version/{}",
        vars: [base62::encode(version_id)],
    }
}

pub async fn project_versions<I>(
    client: &isahc::HttpClient,
    base: &url::Url,
    version_ids: I,
) -> ApiResult<Vec<ProjectVersion>>
where
    I: IntoIterator<Item = u64>,
{
    endpoint! {
        client GET,
        uri: base / "versions",
        params: &new_struct! {
            #[serde_as]
            #[derive(Serialize)]
            RequestParams {
                #[serde_as(as = "JsonString<Vec<Base62>>")]
                ids: Vec<u64> = version_ids.into_iter().collect(),
            }
        },
    }
}

pub async fn project_version_by_hash(
    client: &isahc::HttpClient,
    base: &url::Url,
    hash: &FileHashes,
) -> ApiResult<ProjectVersion> {
    let (hash, kind) = match hash {
        FileHashes {
            sha512: Some(hash), ..
        } => (hash, "sha512"),
        FileHashes {
            sha1: Some(hash), ..
        } => (hash, "sha1"),
        _ => todo!(),
    };

    endpoint! {
        client GET,
        uri: base / "version_file/{}",
        vars: [hash],
        params: &new_struct! {
            #[derive(Serialize)]
            RequestParams<'a> {
                algorithm: &'a str = kind,
            }
        },
    }
}
