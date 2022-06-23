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

/// This macro makes use of several calls to [`Result::unwrap`] or
/// [`Option::unwrap`]. The values that are unwrapped are expected to be of
/// types where the operation in question is guaranteed to be successful.
/// It may be the case that an unwrap fails at runtime; if the author making use
/// of the macro is certain that the hard-coded values are correct, but runtime
/// panics and unwinds, this is considered a bug. A panic means that a variant
/// needs to be added to the `Error` type, and that one of the following
/// justification comments is wrong.
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
        // The type of `$params` is expected to have been validated manually,
        // with a guarantee that it can be serialized as a query string with
        // [`serde_qs::to_string`]. This would only fail if runtime values fail
        // to serialize; this won't happen if the type of `$params` has a
        // well-defined structure.
        $(uri.set_query(Some(&serde_qs::to_string($params).unwrap()));)?

        let builder = isahc::Request::builder()
            .method(endpoint!(@str $method))
            .uri(uri.as_str());
        // Use of unwrap:
        // Building the [`isahc::Request`] should realistically never fail,
        // because all of the involved values have already made it past every
        // preceeding point where the runtime had the opprotunity to panic.
        let request = endpoint!(@build, builder $(, $body)?).unwrap();

        // Sending the request can easily fail, so this would get bubbled to
        // [`crate::Error::Request`].
        let response = $client.send_async(request).await?;
        let status = response.status();
        let mut bytes = Vec::new();

        // Use of unwrap:
        // Expect that reading the bytes from a response body is infallible.
        // Responses must always return some data, even an empty slice of bytes,
        // so unwrapping the result of the [`AsyncReadExt::read_to_end`] here
        // should be perfectly acceptable.
        response.into_body().read_to_end(&mut bytes).await.unwrap();

        // If the response status is not 200 OK, bubble the error, passing along
        // the unexpected status, the fully formed URI, and the body bytes in
        // case the server responded with more details.
        if status != 200 {
            return Err(Error::StatusNotOk { uri, status, bytes: Box::new(bytes) });
        }

        let deser = &mut serde_json::Deserializer::from_slice(bytes.as_slice());
        let result = serde_path_to_error::deserialize(deser);

        // Determine if the response's body bytes deserialized correctly into
        // the inferred type (outside the macro), and if not, bubble the error
        // to `Error::Deserialize`.
        match result {
            Ok(value) => Ok(ApiResponse { bytes, value }),
            Err(error) => Err(Error::Deserialize { uri, error, bytes: Box::new(bytes) }),
        }
    }};
    (@uri, $base:ident, $path:literal) => {
        // Use of unwrap:
        // This cannot fail as a result of a malformed `$base`, which is most
        // likely hard-coded, and at the very least, a parsing failure would
        // have already been caught. The `$path` is definitely hard-coded in
        // this branch with no variables. If this fails, the macro input was not
        // correct.
        $base.join($path).unwrap()
    };
    (@uri, $base:ident, $path:literal, [$($var:expr),+]) => {
        // Use of unwrap:
        // The call to [`url::Url::join`] takes a string that is produced by
        // `format!`, where parts of `$path` are replaced, in order, by `$var`
        // items with `ToString`. If it fails, the macro input was not correct.
        $base.join(&format!($path, $($var.to_string()),*)).unwrap()
    };
    (@build, $builder:ident) => {
        $builder.body(())
    };
    (@build, $builder:ident, $body:expr) => {
        // Use of unwrap:
        // The type of `$body` is expected to be validated manually. The user of
        // this macro should be confident that the type will serialize
        // successfully as a valid query string, even if the parameters of are
        // variadic at runtime.
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
