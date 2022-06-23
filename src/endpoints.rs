use serde::Serialize;
use serde_with::serde_as;

use crate::request::pagination::{ProjectSearchDelegate, ProjectSearchStream};
use crate::request::params::{ProjectIdentifier, ProjectSearchParams};
use crate::request::response::{ApiPageResult, ApiResponse, ApiResult};
use crate::serde_with::{Base62, JsonString};
use crate::types::project::Project;
use crate::types::search::ProjectSearchResult;
use crate::types::version::{FileHashes, ProjectVersion};
use crate::utils::new_struct;
use crate::Error;

pub static DEFAULT_API_BASE: &str = "https://api.modrinth.com/v2/";

macro_rules! endpoint {
    (
        $client:ident $method:ident,
        uri: $base:ident / $path:literal,
        $(vars: [$($var:expr),+],)?
        $(params: $params:expr,)?
        $(body: $body:expr,)?
    ) => {{
        use futures_lite::io::AsyncReadExt;

        #[allow(unused_mut)]
        let mut uri = endpoint!(@uri, $base, $path $(, [$($var),*])?);
        // Use of unwrap:
        // As with the request body and the URI, the value being serialized as
        // query parameters shouldn't realistically fail because it is expected
        // to be strongly-typed, with fields also of types that implement
        // `Serialize` and are strongly-typed themselves.
        $(uri.set_query(Some(&serde_qs::to_string($params).unwrap()));)?

        let builder = isahc::Request::builder()
            .method(endpoint!(@str $method))
            .uri(uri.as_str());
        // Use of unwrap:
        // The request is built by this macro, and there should be enough
        // structured information here that building request (when the body is
        // added in another branch of code) is guaranteed to succeed.
        // Mistakes here *should* have been caught by a compile-time error.
        let request = endpoint!(@build, builder $(, $body)?).unwrap();

        // Sending the request can easily fail, so this would get bubbled to
        // [`crate::Error::Request`].
        let response = $client.send_async(request).await?;
        let status = response.status();
        let mut bytes = Vec::new();

        // Use of unwrap:
        // I expect reading the bytes from a response body to be infallible.
        // Responses must always return some data, it can't just be headers,
        // so unwrapping the result of the `read_to_end` here should be
        // perfectly safe.
        response.into_body().read_to_end(&mut bytes).await.unwrap();

        if status != 200 {
            return Err(Error::StatusNotOk { uri, status, bytes: Box::new(bytes) });
        }

        let deser = &mut serde_json::Deserializer::from_slice(bytes.as_slice());
        let result = serde_path_to_error::deserialize(deser);

        match result {
            Ok(value) => Ok(ApiResponse { bytes, value }),
            Err(error) => Err(Error::Deserialize { uri, error, bytes: Box::new(bytes) }),
        }
    }};
    (@uri, $base:ident, $path:literal) => {
        // Use of unwrap:
        // The `$base` is most likely hard-coded, or at the very least expected
        // to be validated ahead-of-time. The `$path` is definitely hard-coded,
        // and the user of the crate is responsible for ensuring its
        // correctness.
        $base.join($path).unwrap()
    };
    (@uri, $base:ident, $path:literal, [$($var:expr),+]) => {
        // Use of unwrap:
        // Formatting `$path` with `$var` items realistically shouldn't fail as
        // long as the type-system is utilized. Types used as `$var` should have
        // a consistent `Display` implimentation, and no matter the value of the
        // type, should serialize into something that can both be sent over the
        // internet and understood by the recipient.
        $base.join(&format!($path, $($var),*)).unwrap()
    };
    (@build, $builder:ident) => {
        $builder.body(())
    };
    (@build, $builder:ident, $body:expr) => {
        // Use of unwrap:
        // Serializing the request body as JSON should not fail as long as it is
        // strongly-typed, with fields implementing `Serialize`, which is
        // expected here.
        $builder.body(serde_json::to_string($body).unwrap())
    };
    (@str GET) => {
        "GET"
    };
    (@str POST) => {
        "POST"
    };
}

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
                #[serde_as(as = "JsonString<Vec<Base62<u64>>>")]
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
                #[serde_as(as = "JsonString<Vec<Base62<u64>>>")]
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
